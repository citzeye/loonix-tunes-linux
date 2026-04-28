/* --- loonixtunesv2/src/audio/engine/engine.rs | engine --- */

use crate::audio::io::audiooutput::OutputState;
use crate::audio::io::decoder;
use crate::audio::io::audiooutput::AudioOutput;
use crate::audio::io::decoder::{DecoderControl, DecoderEvent, DecoderHandle};
use crate::audio::dsp::DspSettings;
use crate::core::library::scanner;
use ringbuf::traits::Split;
use ringbuf::{HeapProd, HeapRb};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc, Mutex};

/* ------------------------------------------------ */
/* OUTPUT MODE                                      */
/* ------------------------------------------------ */

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Mono,
    Stereo,
    Surround,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OutputConfig {
    pub mode: OutputMode,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            mode: OutputMode::Stereo,
        }
    }
}

/* ------------------------------------------------ */
/* SEEK STATE                                       */
/* ------------------------------------------------ */

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeekState {
    Idle,
    Seeking,
    Decoding,
    Buffering,
    Ready,
}

impl Default for SeekState {
    fn default() -> Self {
        SeekState::Idle
    }
}

/* ------------------------------------------------ */
/* DURATION MODE                                    */
/* ------------------------------------------------ */

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DurationMode {
    Metadata,
    Decoder,
    Final,
}

impl Default for DurationMode {
    fn default() -> Self {
        DurationMode::Metadata
    }
}

/* ------------------------------------------------ */
/* PLAYBACK STATE                                   */
/* ------------------------------------------------ */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Loading,
    Priming,
    Playing,
    Paused,
}

impl Default for PlaybackState {
    fn default() -> Self {
        PlaybackState::Stopped
    }
}

impl PlaybackState {
    pub fn can_play(&self) -> bool {
        matches!(self, PlaybackState::Stopped | PlaybackState::Paused)
    }

    pub fn can_pause(&self) -> bool {
        matches!(self, PlaybackState::Playing)
    }

    pub fn can_resume(&self) -> bool {
        matches!(self, PlaybackState::Paused)
    }

    pub fn can_stop(&self) -> bool {
        !matches!(self, PlaybackState::Stopped)
    }
}

/* ------------------------------------------------ */
/* ENGINE STRUCT                                    */
/* ------------------------------------------------ */

pub struct Engine {
    pub producer: Option<HeapProd<f32>>,

    pub decoder_control: Option<Arc<DecoderControl>>,

    decoder_handle: Option<DecoderHandle>,

    pub audiooutput: Option<AudioOutput>,

    pub volume: f32,
    pub balance: f32,

    pub output_mode: OutputMode,
    pub is_mono: bool,
    pub dsp_settings: DspSettings,
    pub dsp_enabled: bool,

    pub playback_state: PlaybackState,

    // MASTER CLOCK - timer-based position (reliable)
    pub samples_played: u64,
    pub sample_rate: u64,
    pub channels: u32,
    pub duration_ms: u64,
    duration_mode: DurationMode,
    metadata_duration_ms: u64,
    decoder_total_samples: u64,

    // Position when paused (stored so we can resume from correct spot)
    paused_samples_played: u64,

    // Event receiver for seek completion
    event_rx: Option<mpsc::Receiver<DecoderEvent>>,

    // End of track flag
    end_of_track: bool,

    // Decoder EOF flag - set on EndOfTrack, cleared when buffer drains
    decoder_eof: bool,
}

/* ------------------------------------------------ */
/* ENGINE IMPLEMENTATION                            */
/* ------------------------------------------------ */

impl Engine {
    pub fn new() -> Self {
        Self {
            producer: None,
            decoder_control: None,
            decoder_handle: None,
            audiooutput: None,
            volume: 1.0,
            balance: 0.0,
            output_mode: OutputMode::Stereo,
            is_mono: false,
            dsp_settings: DspSettings::default(),
            dsp_enabled: true,
            playback_state: PlaybackState::Stopped,

            samples_played: 0,
            sample_rate: 48000,
            channels: 2,
            duration_ms: 0,
            duration_mode: DurationMode::Metadata,
            metadata_duration_ms: 0,
            decoder_total_samples: 0,

            paused_samples_played: 0,

            event_rx: None,
            end_of_track: false,
            decoder_eof: false,
        }
    }

    /* ------------------------------------------------ */
    /* START AUDIO                                      */
    /* ------------------------------------------------ */

    pub fn start_audiooutput(&mut self, path: String, ab_loop: Arc<Mutex<crate::audio::engine::abloop::ABLoop>>) {
        // State transition: Stopped -> Loading
        self.playback_state = PlaybackState::Loading;

        // Reset A-B Loop on new track
        if let Ok(mut ab) = ab_loop.lock() {
            ab.reset();
        }

        // 1. Setup Ring Buffer - 120ms for low latency
        let sample_rate = 48000; // frames per second
        let channels = 2; // Output always forced to STEREO by resampler (see decoder.rs)
        self.channels = channels;
        let buffer_ms = 120;
        // Calculate buffer size in SAMPLES (f32 values), not frames
        // 120ms @ 48kHz stereo = 11520 samples (~46 KB)
        let buffer_size = (sample_rate * channels * buffer_ms / 1000) as usize;

        let rb = HeapRb::<f32>::new(buffer_size);
        let (prod, cons) = rb.split();
        self.producer = Some(prod);

        // 2. Create event channel for seek completion
        let (tx, rx) = mpsc::channel();

        // 3. Setup Decoder Control
        let control = Arc::new(DecoderControl::new());
        let control_for_decoder = control.clone();
        control.set_event_sender(tx); // Connect sender to decoder
        self.decoder_control = Some(control);

        // 4. Store receiver in Engine
        self.event_rx = Some(rx);

        // 5. Force 48kHz sample rate
        let actual_sample_rate = 48000;
        self.sample_rate = actual_sample_rate as u64;
        self.samples_played = 0;
        self.duration_ms = 0; // Reset so get_duration() uses metadata initially
        self.duration_mode = DurationMode::Metadata;
        self.metadata_duration_ms = 0;
        self.decoder_total_samples = 0;

        // 6. Spawn Decoder Thread
        let path_clone = path.clone();
        if let Some(producer) = self.producer.take() {
self.decoder_handle = Some(decoder::spawn_decoder_with_sample_rate(
    path_clone,
    producer,
    control_for_decoder.clone(),
    actual_sample_rate,
    ab_loop.clone(),
));
        } else {
            eprintln!("[Engine] Failed to start playback: producer not available");
            return;
        }

        // 7. Setup Audio Output - reuse existing for crossfade (persistent device)
        if let Some(ref mut audiooutput) = self.audiooutput {
            // Reuse existing AudioOutput - device stream stays open
            audiooutput.mode = self.output_mode;
            audiooutput.update_mode_internal();
            audiooutput.set_volume(self.volume);
            audiooutput.set_balance(self.balance);
            // Chain already built at startup - no need to rebuild on track change
            audiooutput.reset_dsp();
            audiooutput.reset_samples_played(0);
            // clear_old=true: fresh track start, don't crossfade from old track's buffer
            audiooutput.start(cons, true, buffer_size);
        } else {
            // First track - create new AudioOutput (LAZY INIT TERJADI DI SINI)
            let mut audiooutput = AudioOutput::new();
            audiooutput.mode = self.output_mode;
            audiooutput.update_mode_internal();
            audiooutput.set_volume(self.volume);
            audiooutput.set_balance(self.balance);

            // 🔥 INJEKSI STATUS BOOT KE AUDIO OUTPUT BARU
            audiooutput.set_dsp_enabled(self.dsp_enabled);

            audiooutput.update_dsp(&self.dsp_settings);
            audiooutput.reset_dsp();
            audiooutput.reset_samples_played(0);
            audiooutput.start(cons, true, buffer_size);
            self.audiooutput = Some(audiooutput);
        }

        // State transition: Loading -> Priming
        // Decoder is now buffering, wait for InitialBufferReady to transition to Playing
        self.playback_state = PlaybackState::Priming;
    }

    /* ------------------------------------------------ */
    /* UPDATE TICK                                      */
    /* ------------------------------------------------ */

    pub fn update_tick(&mut self) {
        if let Some(rx) = self.event_rx.take() {
            while let Ok(event) = rx.try_recv() {
                match event {
                    DecoderEvent::SeekComplete => {
                        self.on_seek_complete();
                    }
                    DecoderEvent::BufferReady => {
                        self.on_buffer_ready();
                    }
                    DecoderEvent::InitialBufferReady => {
                        let current = self.playback_state;

                        match current {
                            PlaybackState::Priming => {
                                self.playback_state = PlaybackState::Playing;
                                self.samples_played = 0;
                                if let Some(ref mut audiooutput) = self.audiooutput {
                                    audiooutput.reset_samples_played(0);
                                    audiooutput.set_output_state(
                                         OutputState::Running,
                                    );
                                }
                            }
                            PlaybackState::Paused => {
                                // Ignored - stay in Paused
                            }
                            _ => {
                                // Ignored - invalid state
                            }
                        }
                    }
                    DecoderEvent::EndOfTrack { total_samples } => {
                        self.decoder_eof = true;
                        self.decoder_total_samples = total_samples;
                        // Signal audio output that decoder is done
                        if let Some(ref audiooutput) = self.audiooutput {
                            audiooutput.set_decoder_eof(true);
                        }
                    }
                }
            }
            self.event_rx = Some(rx);
        }

        // Only update timer if we're in Playing or Paused state
        if self.playback_state == PlaybackState::Stopped
            || self.playback_state == PlaybackState::Loading
        {
            return;
        }

        if let Some(ref mut audiooutput) = self.audiooutput {
            self.samples_played = audiooutput.get_samples_played();
            // Note: seek_mode is handled exclusively by on_buffer_ready()
            // No duplicate logic here to avoid race condition
        }

        // Progressive duration update from decoder output samples
        if self.duration_mode != DurationMode::Final {
            if let Some(ref control) = self.decoder_control {
                let decoded_samples = control.output_samples.load(Ordering::SeqCst);
                if decoded_samples > 0 && self.sample_rate > 0 {
                    let decoder_duration_ms = ((decoded_samples as f64 * 1000.0)
                        / (self.sample_rate as f64 * self.channels as f64))
                        as u64;

                    if self.duration_mode == DurationMode::Metadata {
                        // Capture metadata duration on first update
                        let meta_dur = self
                            .decoder_control
                            .as_ref()
                            .map(|c| c.get_duration())
                            .unwrap_or(0);
                        if meta_dur > 0 {
                            self.metadata_duration_ms = meta_dur;
                            self.duration_mode = DurationMode::Decoder;
                        }
                    }

                    // Use whichever is larger (decoder may lag behind metadata for CBR)
                    let corrected = decoder_duration_ms.max(self.metadata_duration_ms);
                    if corrected > 0 {
                        self.duration_ms = corrected;
                    }
                }
            }
        }

        // Simpler fix: as soon as decoder says EOF, mark track as finished
        // No need for complex buffer/samples comparison
        if self.decoder_eof && self.playback_state == PlaybackState::Playing {
            self.end_of_track = true;
        }
    }

    pub fn has_end_of_track(&mut self) -> bool {
        if self.end_of_track {
            self.end_of_track = false;
            true
        } else {
            false
        }
    }

    /* ------------------------------------------------ */
    /* STOP                                             */
    /* ------------------------------------------------ */

    pub fn stop(&mut self) {
        // Only stop if not already stopped
        if !self.playback_state.can_stop() {
            return;
        }

        self.playback_state = PlaybackState::Stopped;
        self.end_of_track = false;
        self.decoder_eof = false;

        // Stop and join decoder thread
        self.decoder_handle = None;

        // 🔥 CROSSFADE: keep AudioOutput alive for track transitions
        // stop() moves the consumer to the crossfade shadow preset
        // The cpal stream stays open (persistent device)
        // Only on explicit FfmpegEngine::stop() is AudioOutput dropped
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_decoder_eof(false);
            audiooutput.stop();
        }

        self.producer = None;
    }

    /* ------------------------------------------------ */
    /* PAUSE/RESUME                                     */
    /* ------------------------------------------------ */

    pub fn pause(&mut self) {
        println!(
            "[ENGINE] pause() called, current state={:?}",
            self.playback_state
        );

        // GUARD: Only pause if currently playing
        if !self.playback_state.can_pause() {
            println!(
                "[ENGINE] pause() IGNORED - not in Playing state (state={:?})",
                self.playback_state
            );
            return;
        }

        if let Some(ref mut audiooutput) = self.audiooutput {
            self.paused_samples_played = audiooutput.get_samples_played();
        }

        self.playback_state = PlaybackState::Paused;

        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.pause();
        }
    }

    pub fn resume(&mut self) {
        // GUARD: Only resume if currently paused
        if !self.playback_state.can_resume() {
            return;
        }

        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_samples_played(self.paused_samples_played);
            audiooutput.resume();
        }

        self.playback_state = PlaybackState::Playing;
    }

    /* ------------------------------------------------ */
    /* SETTERS                                          */
    /* ------------------------------------------------ */

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;

        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_volume(volume);
        }
    }

    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance;

        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_balance(balance);
        }
    }

    pub fn set_dsp_settings(&mut self, settings: DspSettings) {
        self.dsp_settings = settings;
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.update_dsp(&self.dsp_settings);
        }
    }

    pub fn set_dsp_enabled(&mut self, enabled: bool) {
        self.dsp_enabled = enabled;
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_dsp_enabled(enabled);
        }
    }

    pub fn set_normalizer_enabled(&mut self, enabled: bool) {
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_normalizer_enabled(enabled);
        }
    }

    pub fn set_normalizer_gain(&mut self, gain: f32) {
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_normalizer_gain(gain);
        }
    }

    pub fn get_normalizer_arc(
        &self,
    ) -> Option<std::sync::Arc<std::sync::Mutex<crate::audio::dsp::normalizer::AudioNormalizer>>>
    {
        self.audiooutput.as_ref().map(|ao| ao.get_normalizer_arc())
    }

    pub fn set_normalizer_smoothing(&mut self, smoothing: f32) {
        // Smoothing is stored in a static atomic in normalizer.rs
        // No need to route through AudioOutput
        let arc = crate::audio::dsp::normalizer::get_normalizer_smoothing_arc();
        arc.store(smoothing.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }

    pub fn reset_dsp(&mut self) {
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.reset_dsp();
        }
    }

    /* ------------------------------------------------ */
    /* OUTPUT MODE                                      */
    /* ------------------------------------------------ */

    pub fn set_output_mode(&mut self, mode: OutputMode) {
        self.output_mode = mode;
        self.is_mono = mode == OutputMode::Mono;

        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.mode = mode;
            audiooutput.update_mode_internal();
        }
    }

    pub fn set_mono(&mut self, mono: bool) {
        let mode = if mono {
            OutputMode::Mono
        } else {
            OutputMode::Stereo
        };

        self.set_output_mode(mode);
    }

    /* ------------------------------------------------ */
    /* POSITION                                         */
    /* ------------------------------------------------ */

    pub fn get_position(&self) -> f64 {
        // SELALU hitung dari master clock: samples_played / sample_rate = seconds
        if self.sample_rate == 0 {
            return 0.0;
        }
        // Guard: don't return garbage if audio output not initialized
        if self.audiooutput.is_none() {
            return 0.0;
        }
        // samples_played / sample_rate = seconds
        self.samples_played as f64 / (self.sample_rate as f64 * self.channels as f64)
    }

    pub fn get_duration(&self) -> f64 {
        // Prioritaskan self.duration_ms yang sudah dikoreksi oleh EndOfTrack
        // Fallback ke metadata kalau belum tersedia
        if self.audiooutput.is_none() {
            return 0.0;
        }

        // Jika duration_ms belum ada (sebelum EndOfTrack), ambil dari metadata
        if self.duration_ms == 0 {
            if let Some(ref control) = self.decoder_control {
                return control.get_duration() as f64 / 1000.0;
            }
        }

        self.duration_ms as f64 / 1000.0
    }

    /// Get duration in samples (at output sample rate)
    pub fn get_duration_samples(&self) -> u64 {
        let duration_ms = self.get_duration_ms();
        // duration_ms * sample_rate * channels / 1000 = total SAMPLES at output rate
        (duration_ms * self.sample_rate * self.channels as u64) / 1000
    }

    // Single source of truth: callback-based position from audio clock
    pub fn get_position_ms(&mut self) -> u64 {
        if self.playback_state == PlaybackState::Playing
            || self.playback_state == PlaybackState::Paused
        {
            if let Some(ref mut audiooutput) = self.audiooutput {
                let samples = audiooutput.get_samples_played();
                if self.sample_rate > 0 {
                    return ((samples as f64 * 1000.0)
                        / (self.sample_rate as f64 * self.channels as f64))
                        as u64;
                }
            }
        }
        ((self.samples_played as f64 * 1000.0) / (self.sample_rate as f64 * self.channels as f64))
            as u64
    }

    pub fn get_duration_ms(&self) -> u64 {
        // Prioritaskan corrected VBR duration dari EndOfTrack
        if self.duration_ms > 0 {
            return self.duration_ms;
        }
        // Fallback ke metadata sebelum EndOfTrack tiba
        if let Some(ref control) = self.decoder_control {
            return control.get_duration();
        }
        0
    }

    /* ------------------------------------------------ */
    /* SEEK                                             */
    /* ------------------------------------------------ */

    pub fn seek(&mut self, mut seconds: f64) {
        // CLAMP TARGET: Jangan biarin UI ngirim posisi ngelewatin durasi asli!
        let duration_sec = self.get_duration();
        if duration_sec > 0.0 && seconds >= duration_sec {
            seconds = duration_sec - 0.5;
        }

        let target_ms = (seconds * 1000.0) as u64;
        let _target_samples = (seconds * self.sample_rate as f64 * self.channels as f64) as u64;

        // STEP 1: Set state = Seeking (only if currently playing)
        // Engine state machine - single authority
        if self.playback_state == PlaybackState::Playing {
            self.playback_state = PlaybackState::Priming; // Go back to priming during seek
        }

        // STEP 2: Audio: set seek mode - audio thread mulai kirim silence
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_seek_mode(true);
        }

        // STEP 3: Ring buffer clear - audio callback akan drain consumer
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.clear_buffer();
        }

        // STEP 4: Command decoder to seek and pre-buffer
        // Decoder akan: flush, av_seek_frame, prebuffer, send_event(BufferReady)
        if let Some(ref control) = self.decoder_control {
            control.request_seek(target_ms);
        }
    }

    /// Called when decoder signals READY (pre-buffered)
    pub fn on_seek_complete(&mut self) {
        // Clear seek state in decoder
        if let Some(ref control) = self.decoder_control {
            control.clear_seek();
        }
    }

    /// Called when decoder signals BufferReady (buffer is full and ready to play)
    /// THIS IS THE SINGLE AUTHORITY FOR SEEK COMPLETION
    pub fn on_buffer_ready(&mut self) {
        // STEP 1: Get exact position from decoder (VBR corrected)
        let target_ms = if let Some(ref control) = self.decoder_control {
            control.seek_request.load(Ordering::SeqCst)
        } else {
            return;
        };
        let exact_samples =
            ((target_ms as f64 * self.sample_rate as f64 * self.channels as f64) / 1000.0) as u64;

        // STEP 2: Set samples_played EXACT - single source of truth
        self.samples_played = exact_samples;

        // Also update audiooutput's sample counter for consistent UI
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.reset_samples_played(exact_samples);
        }

        // STEP 3: Reset ALL DSP effects (EQ, compressor, etc.)
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.reset_dsp();
        }

        // STEP 4: Audio - set_seek_mode(false) - izinkan audio thread baca buffer
        if let Some(ref mut audiooutput) = self.audiooutput {
            audiooutput.set_seek_mode(false);
            // Trigger seek fade-in for clean transition (25ms smooth ramp)
            audiooutput.trigger_seek_fade();
        }

        // STEP 5: Clear all seek flags in decoder control
        if let Some(ref control) = self.decoder_control {
            control.clear_seek();
        }

        // STEP 6: Set state = Playing - seek complete
        self.playback_state = PlaybackState::Playing;
    }

    /// Called from update_tick - no longer needed with frame counter approach
    #[allow(dead_code)]
    pub fn process_resume(&mut self) {
        // Frame counter handles delay in audio callback
        // This method kept for compatibility
    }

    /* ------------------------------------------------ */
    /* STATE                                            */
    /* ------------------------------------------------ */

    pub fn is_playing(&self) -> bool {
        self.playback_state == PlaybackState::Playing
    }

    pub fn is_paused(&self) -> bool {
        self.playback_state == PlaybackState::Paused
    }

    pub fn get_playback_state(&self) -> PlaybackState {
        self.playback_state
    }
}

/* ------------------------------------------------ */
/* MISSING TYPES AND FUNCTIONS                     */
/* ------------------------------------------------ */

#[derive(Debug, Clone)]
pub struct MusicItem {
    pub name: String,
    pub path: String,
    pub is_folder: bool,
    pub parent_folder: Option<String>,
}

#[allow(dead_code)]
fn get_duration_ffprobe(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let duration_sec: f64 = output_str.trim().parse()?;
    Ok((duration_sec * 1000.0) as u64)
}

#[derive(Default)]
pub struct FfmpegEngine {
    engine: Option<Engine>,
    current_path: Option<String>,
    is_finished: bool,
    scan_params: scanner::ScanParams,
    pub ab_loop: Arc<Mutex<crate::audio::engine::abloop::ABLoop>>,
}

impl FfmpegEngine {
    pub fn new() -> Self {
        let engine = Engine::new();
        Self {
            engine: Some(engine),
            current_path: None,
            is_finished: false,
            scan_params: scanner::ScanParams::default(),
            ab_loop: Arc::new(Mutex::new(crate::audio::engine::abloop::ABLoop::default())),
        }
    }

    fn ensure_engine(&mut self) {
        if self.engine.is_none() {
            self.engine = Some(Engine::new());
        }
    }

    pub fn set_normalizer_params(
        &mut self,
        target_lufs: f32,
        true_peak_dbtp: f32,
        max_gain_db: f32,
    ) {
        self.scan_params.target_lufs = target_lufs;
        self.scan_params.true_peak_dbtp = true_peak_dbtp;
        self.scan_params.max_gain_db = max_gain_db;
        // Params changed - invalidate cache so next play re-scans
        scanner::clear_cache();
    }

    pub fn load(&mut self, path: &str) {
        self.ensure_engine();
        self.is_finished = false;

        if let Some(engine) = &mut self.engine {
            engine.stop();

            engine.samples_played = 0;
            engine.end_of_track = false;
            engine.decoder_eof = false;

            engine.set_normalizer_gain(1.0);

            engine.start_audiooutput(path.to_string(), self.ab_loop.clone());

            engine.playback_state = PlaybackState::Paused;

            if let Some(ref mut audiooutput) = engine.audiooutput {
                audiooutput.pause();
            }

            self.current_path = Some(path.to_string());

            let path_owned = path.to_string();
            let params = self.scan_params.clone();

            let _ = std::thread::Builder::new()
                .name("loudness-scanner".to_string())
                .spawn(move || {
                    let gain = scanner::calculate_track_gain(&path_owned, &params);
                    let gain_arc = crate::audio::dsp::normalizer::get_normalizer_gain_arc();
                    gain_arc.store(gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
                });
        }
    }

    pub fn play(&mut self) {
        if let Some(engine) = &mut self.engine {
            if engine.playback_state == PlaybackState::Paused {
                engine.playback_state = PlaybackState::Playing;
                if let Some(ref mut audiooutput) = engine.audiooutput {
                    audiooutput.resume();
                }
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(engine) = &mut self.engine {
            engine.stop();
        }
    }

    pub fn pause(&mut self) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.pause();
        }
    }

    pub fn resume(&mut self) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.resume();
        }
    }

    pub fn seek(&mut self, seconds: f64) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.seek(seconds);
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_volume(volume);
        }
    }

    pub fn set_balance(&mut self, balance: f32) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_balance(balance);
        }
    }

    pub fn set_dsp_settings(&mut self, settings: DspSettings) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_dsp_settings(settings);
        }
    }

    pub fn set_dsp_enabled(&mut self, enabled: bool) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_dsp_enabled(enabled);
        }
    }

    pub fn set_normalizer_enabled(&mut self, enabled: bool) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_normalizer_enabled(enabled);
        }
    }

    pub fn set_normalizer_gain(&mut self, gain: f32) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_normalizer_gain(gain);
        }
    }

    pub fn set_normalizer_smoothing(&mut self, smoothing: f32) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_normalizer_smoothing(smoothing);
        }
    }

    pub fn reset_dsp(&mut self) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.reset_dsp();
        }
    }

    pub fn get_position(&mut self) -> f64 {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.get_position()
        } else {
            0.0
        }
    }

    pub fn get_duration(&self) -> f64 {
        if let Some(ref engine) = self.engine {
            engine.get_duration()
        } else {
            0.0
        }
    }

    /// Get position without requiring &mut self (reads from samples_played)
    pub fn get_position_immut(&self) -> f64 {
        if let Some(ref engine) = self.engine {
            if engine.sample_rate == 0 {
                return 0.0;
            }
            // samples_played / sample_rate = seconds
            engine.samples_played as f64 / (engine.sample_rate as f64 * engine.channels as f64)
        } else {
            0.0
        }
    }

    pub fn update_tick(&mut self) {
        if let Some(engine) = &mut self.engine {
            engine.update_tick();

            // Check if EndOfTrack event was processed
            if engine.has_end_of_track() {
                self.is_finished = true;
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    // Auto-next: take and reset finished flag (fires once)
    pub fn take_finished(&mut self) -> bool {
        if self.is_finished {
            self.is_finished = false;
            true
        } else {
            false
        }
    }

    pub fn is_playing(&self) -> bool {
        self.engine
            .as_ref()
            .map(|e| e.is_playing())
            .unwrap_or(false)
    }

    pub fn is_paused(&self) -> bool {
        self.engine.as_ref().map(|e| e.is_paused()).unwrap_or(false)
    }

    pub fn get_current_path(&self) -> Option<String> {
        self.current_path.clone()
    }

    pub fn get_playback_state(&self) -> PlaybackState {
        self.engine
            .as_ref()
            .map(|e| e.get_playback_state())
            .unwrap_or(PlaybackState::Stopped)
    }
}

pub struct AudioState {
    pub playback_state: PlaybackState,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            playback_state: PlaybackState::Stopped,
        }
    }
}

pub struct CustomFolder {
    pub name: String,
    pub path: String,
}

pub fn load_output_config() -> OutputConfig {
    OutputConfig::default()
}

pub fn is_audio_file(path: &std::path::Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        return matches!(
            ext_str.as_str(),
            "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "wma"
        );
    }
    false
}

pub struct ProAudioEngine {
    pub eq_bands: [f32; 10],
}

impl ProAudioEngine {
    pub fn new() -> Self {
        Self {
            eq_bands: [0.0; 10],
        }
    }

    pub fn set_eq_band_gain(&mut self, band_index: i32, gain: f32) {
        if band_index >= 0 && band_index < 10 {
            self.eq_bands[band_index as usize] = gain;
        }
    }

    pub fn set_eq_bands(&mut self, low: f32, mid: f32, high: f32) {
        // Simple mapping to 10-band EQ
        self.eq_bands[0] = low;
        self.eq_bands[4] = mid;
        self.eq_bands[8] = high;
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        // Ensure clean shutdown when Engine is dropped
        // This is critical for proper audio thread cleanup
        self.stop();
    }
}

/* --- END --- */
