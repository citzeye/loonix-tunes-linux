/* --- LOONIX-TUNES src/audio/engine/engine.rs | The Boss --- */

use crate::audio::audio_output::AudioOutput;
use crate::audio::dac::DacManager;
use crate::audio::decoder::{DecoderControl, DecoderEvent, SEEK_STATE_DECODING};
use crate::audio::dsp::DspSettings;
use crate::audio::scanner;
use ringbuf::traits::Split;
use ringbuf::{HeapProd, HeapRb};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc};

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
/* ENGINE STRUCT                                    */
/* ------------------------------------------------ */

pub struct Engine {
    pub producer: Option<HeapProd<f32>>,

    pub decoder_control: Option<Arc<DecoderControl>>,

    pub audio_output: Option<AudioOutput>,

    pub dac_manager: DacManager,

    pub volume: f32,
    pub balance: f32,

    pub output_mode: OutputMode,
    pub is_mono: bool,
    pub dsp_settings: DspSettings,
    pub dsp_enabled: bool,

    pub is_playing: bool,

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
            audio_output: None,
            dac_manager: DacManager::new(),
            volume: 1.0,
            balance: 0.0,
            output_mode: OutputMode::Stereo,
            is_mono: false,
            dsp_settings: DspSettings::default(),
            dsp_enabled: true,
            is_playing: false,

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

    pub fn start_audio_output(&mut self, path: String) {
        // 1. Setup Ring Buffer - 500ms buffer
        let sample_rate = 48000; // frames per second
        let channels = 2; // Output always forced to STEREO by resampler (see decoder.rs)
        self.channels = channels;
        let buffer_ms = 500;
        // Calculate buffer size in SAMPLES (f32 values), not frames
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
        let producer = self.producer.take().unwrap();
        crate::audio::decoder::spawn_decoder_with_sample_rate(
            path_clone,
            producer,
            control_for_decoder.clone(),
            actual_sample_rate,
        );

        // 7. Setup Audio Output - reuse existing for crossfade (persistent device)
        if let Some(ref mut audio_output) = self.audio_output {
            // Reuse existing AudioOutput - device stream stays open
            audio_output.mode = self.output_mode;
            audio_output.update_mode_internal();
            audio_output.set_volume(self.volume);
            audio_output.set_balance(self.balance);
            audio_output.update_dsp(&self.dsp_settings);
            audio_output.reset_dsp();
            audio_output.reset_samples_played(0);
            // clear_old=true: fresh track start, don't crossfade from old track's buffer
            audio_output.start(cons, true, buffer_size);
        } else {
            // First track - create new AudioOutput
            let mut audio_output = AudioOutput::new();
            audio_output.mode = self.output_mode;
            audio_output.update_mode_internal();
            audio_output.set_volume(self.volume);
            audio_output.set_balance(self.balance);
            audio_output.update_dsp(&self.dsp_settings);
            audio_output.reset_dsp();
            audio_output.reset_samples_played(0);
            audio_output.start(cons, true, buffer_size);
            self.audio_output = Some(audio_output);
        }

        self.is_playing = true;
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
                    DecoderEvent::EndOfTrack { total_samples } => {
                        self.decoder_eof = true;
                        self.decoder_total_samples = total_samples;
                        // Duration finalize happens at AUDIO EOF, not decoder EOF
                    }
                }
            }
            self.event_rx = Some(rx);
        }

        if !self.is_playing {
            return;
        }

        if let Some(ref audio_output) = self.audio_output {
            self.samples_played = audio_output.get_samples_played();

            if audio_output.is_seek_mode() && audio_output.get_buffer_len() > 4096 {
                audio_output.set_seek_mode(false);
                audio_output.trigger_delayed_resume();
            }
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
                            println!(
                                "[DURATION] Metadata→Decoder: meta={}ms, decoded_so_far={}ms",
                                meta_dur, decoder_duration_ms
                            );
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

        if self.decoder_eof && self.is_playing {
            let starvation_ok = self
                .audio_output
                .as_ref()
                .map(|ao| ao.is_truly_buffer_empty())
                .unwrap_or(false);

            let buffer_physically_empty = self
                .audio_output
                .as_ref()
                .map(|ao| ao.get_buffer_len() == 0)
                .unwrap_or(true);

            let prebuffer_done = self
                .decoder_control
                .as_ref()
                .map(|c| c.seeking_state.load(Ordering::SeqCst) != SEEK_STATE_DECODING)
                .unwrap_or(true);

            if starvation_ok && buffer_physically_empty && prebuffer_done {
                // FINALIZE DURATION AT AUDIO EOF — slider is at 100% naturally
                if self.duration_mode != DurationMode::Final
                    && self.decoder_total_samples > 0
                    && self.sample_rate > 0
                {
                    self.duration_mode = DurationMode::Final;
                    let true_duration_ms = ((self.decoder_total_samples as f64 * 1000.0)
                        / (self.sample_rate as f64 * self.channels as f64))
                        as u64;
                    self.duration_ms = true_duration_ms;

                    if let Some(ref control) = self.decoder_control {
                        control.set_duration(true_duration_ms);
                    }

                    println!(
                        "[DURATION] Final: {}ms (decoder={}, playback={}, expected={})",
                        true_duration_ms,
                        self.decoder_total_samples,
                        self.samples_played,
                        (self.decoder_total_samples as f64 * 1000.0)
                            / (self.sample_rate as f64 * self.channels as f64)
                    );
                }

                self.end_of_track = true;
                self.decoder_eof = false;
            }
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
        self.is_playing = false;
        self.end_of_track = false;
        self.decoder_eof = false;

        if let Some(ref control) = self.decoder_control {
            control.should_stop.store(true, Ordering::SeqCst);
        }

        self.decoder_control = None;

        // 🔥 CROSSFADE: keep AudioOutput alive for track transitions
        // stop() moves the consumer to the crossfade shadow slot
        // The cpal stream stays open (persistent device)
        // Only on explicit FfmpegEngine::stop() is AudioOutput dropped
        if let Some(ref audio_output) = self.audio_output {
            audio_output.stop();
        }

        self.producer = None;
    }

    /* ------------------------------------------------ */
    /* PAUSE/RESUME                                     */
    /* ------------------------------------------------ */

    pub fn pause(&mut self) {
        if let Some(ref audio_output) = self.audio_output {
            self.paused_samples_played = audio_output.get_samples_played();
        }
        self.is_playing = false;

        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.pause();
        }
    }

    pub fn resume(&mut self) {
        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.set_samples_played(self.paused_samples_played);
            audio_output.resume();
        }
        self.is_playing = true;
    }

    /* ------------------------------------------------ */

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;

        if let Some(ref audio_output) = self.audio_output {
            audio_output.set_volume(volume);
        }
    }

    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance;

        if let Some(ref audio_output) = self.audio_output {
            audio_output.set_balance(balance);
        }
    }

    pub fn set_dsp_settings(&mut self, settings: DspSettings) {
        self.dsp_settings = settings;
        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.update_dsp(&self.dsp_settings);
        }
    }

    pub fn set_dsp_enabled(&mut self, enabled: bool) {
        self.dsp_enabled = enabled;
        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.set_dsp_enabled(enabled);
        }
    }

    pub fn set_exclusive_mode(&mut self, enabled: bool) {
        self.dac_manager.set_exclusive_mode(enabled);

        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.set_exclusive_mode(enabled);
        }
    }

    pub fn set_normalizer_enabled(&mut self, enabled: bool) {
        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.set_normalizer_enabled(enabled);
        }
    }

    pub fn set_normalizer_gain(&mut self, gain: f32) {
        if let Some(ref audio_output) = self.audio_output {
            audio_output.set_normalizer_gain(gain);
        }
    }

    pub fn get_normalizer_arc(
        &self,
    ) -> Option<std::sync::Arc<std::sync::Mutex<crate::audio::dsp::AudioNormalizer>>> {
        self.audio_output.as_ref().map(|ao| ao.get_normalizer_arc())
    }

    pub fn set_normalizer_smoothing(&mut self, smoothing: f32) {
        // Smoothing is stored in a static atomic in normalizer.rs
        // No need to route through AudioOutput
        let arc = crate::audio::dsp::normalizer::get_normalizer_smoothing_arc();
        arc.store(smoothing.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }

    pub fn reset_dsp(&mut self) {
        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.reset_dsp();
        }
    }

    /* ------------------------------------------------ */
    /* OUTPUT MODE                                      */
    /* ------------------------------------------------ */

    pub fn set_output_mode(&mut self, mode: OutputMode) {
        self.output_mode = mode;
        self.is_mono = mode == OutputMode::Mono;

        if let Some(ref mut audio_output) = self.audio_output {
            audio_output.mode = mode;
            audio_output.update_mode_internal();
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
        if self.audio_output.is_none() {
            return 0.0;
        }
        // samples_played / sample_rate = seconds
        self.samples_played as f64 / (self.sample_rate as f64 * self.channels as f64)
    }

    pub fn get_duration(&self) -> f64 {
        // Prioritaskan self.duration_ms yang sudah dikoreksi oleh EndOfTrack
        // Fallback ke metadata kalau belum tersedia
        if self.audio_output.is_none() {
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
    pub fn get_position_ms(&self) -> u64 {
        if self.is_playing {
            if let Some(ref audio_output) = self.audio_output {
                let samples = audio_output.get_samples_played();
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
        // 🔥 FIX 1: CLAMP TARGET! Jangan biarin UI ngirim posisi ngelewatin durasi asli!
        let duration_sec = self.get_duration();
        if duration_sec > 0.0 && seconds >= duration_sec {
            // Kalau UI minta lebih dari durasi, kita potong ke 0.5 detik sebelum lagu abis
            // Biar engine tetep sempet muterin sisa buffer dan nge-trigger EndOfTrack natural
            seconds = duration_sec - 0.5;
        }

        let target_ms = (seconds * 1000.0) as u64;
        let target_samples = (seconds * self.sample_rate as f64 * self.channels as f64) as u64;

        // 1. Update samples_played (callback-based clock)
        self.samples_played = target_samples;

        // 2. Audio: set seek mode + CLEAR OLD BUFFER
        if let Some(ref audio_output) = self.audio_output {
            audio_output.set_seek_mode(true);
            audio_output.reset_samples_played(target_samples);
            audio_output.reset_dsp();
            audio_output.clear_buffer(); // Clear old data before decoder seeks
        }

        // 3. Command decoder to seek and pre-buffer
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
    pub fn on_buffer_ready(&mut self) {
        // Get exact position from decoder (VBR corrected)
        let target_ms = if let Some(ref control) = self.decoder_control {
            control.seek_request.load(Ordering::SeqCst)
        } else {
            return;
        };
        let exact_samples =
            ((target_ms as f64 * self.sample_rate as f64 * self.channels as f64) / 1000.0) as u64;

        // Update engine clock with exact value
        self.samples_played = exact_samples;

        // FIX 2: Double validation - decoder state + buffer level
        let decoder_ready = if let Some(ref control) = self.decoder_control {
            control.seeking_state.load(Ordering::SeqCst) == 2 // SEEK_STATE_READY
        } else {
            false
        };

        // FIX 1: Hard buffer threshold with failsafe
        if let Some(ref audio_output) = self.audio_output {
            let buffer_len = audio_output.get_buffer_len();
            let min_required = 4096;

            if buffer_len >= min_required && decoder_ready {
                // Normal path - buffer is ready
                audio_output.reset_samples_played(exact_samples);
                audio_output.set_seek_mode(false);
                audio_output.trigger_delayed_resume();
                audio_output.trigger_crossfade();
            } else if buffer_len > 0 {
                // Failsafe path - buffer has some data, force unmute
                // Worst case: slight audio glitch, but never stuck muted
                audio_output.set_seek_mode(false);
                audio_output.trigger_delayed_resume();
            }
        }
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
        self.is_playing
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
}

impl FfmpegEngine {
    pub fn new() -> Self {
        let engine = Engine::new();
        Self {
            engine: Some(engine),
            current_path: None,
            is_finished: false,
            scan_params: scanner::ScanParams::default(),
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

    pub fn play(&mut self, path: &str) {
        self.ensure_engine();
        self.is_finished = false;

        if let Some(engine) = &mut self.engine {
            // Stop any current playback first
            engine.stop();

            // Reset samples_played and flags for new track
            engine.samples_played = 0;
            engine.end_of_track = false;
            engine.decoder_eof = false;

            // Start playback INSTANTLY with gain 1.0 (no blocking!)
            engine.set_normalizer_gain(1.0);

            // Start new playback
            engine.start_audio_output(path.to_string());
            self.current_path = Some(path.to_string());

            // Get normalizer handle for background scanner callback
            let normalizer_arc = engine.get_normalizer_arc();
            let path_owned = path.to_string();
            let params = self.scan_params.clone();

            // Spawn loudness scan in background thread (non-blocking)
            let _ = std::thread::Builder::new()
                .name("loudness-scanner".to_string())
                .spawn(move || {
                    let gain = scanner::calculate_track_gain(&path_owned, &params);

                    // Update gain on the running normalizer (smoothing handles transition)
                    if let Some(norm) = normalizer_arc {
                        if let Ok(mut n) = norm.lock() {
                            n.set_fixed_gain(gain);
                            println!("[SCANNER] Background scan done, gain={:.3}", gain);
                        }
                    }
                });
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

    pub fn set_exclusive_mode(&mut self, enabled: bool) {
        self.ensure_engine();
        if let Some(engine) = &mut self.engine {
            engine.set_exclusive_mode(enabled);
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
}

pub struct AudioState {
    pub is_playing: bool,
}

impl Default for AudioState {
    fn default() -> Self {
        Self { is_playing: false }
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

/* --- END --- */
