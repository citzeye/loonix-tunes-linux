/* --- loonixtunesv2/src/audio/io/audiooutput.rs | audiooutput --- */
#[allow(non_snake_case)]
use crate::audio::dsp::{DspChain, DspProcessor};
use crate::audio::engine::OutputMode;
use libpulse_binding::def::BufferAttr;
use libpulse_binding::sample::{Format, Spec};
use libpulse_binding::stream::Direction;
use libpulse_simple_binding as pa_simple;
use ringbuf::traits::{Consumer, Observer};
use ringbuf::HeapCons;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use thread_priority::{set_current_thread_priority, ThreadPriority};

// Audio Commands for the background thread (Lock-Free Communication)
pub enum AudioCommand {
    Play {
        handle: pa_simple::Simple,
        consumer: HeapCons<f32>,
        should_stop: Arc<AtomicBool>,
        seek_mode: Arc<AtomicBool>,
        paused: Arc<AtomicBool>,
        flush_requested: Arc<AtomicBool>,
        seek_fade_remaining: Arc<AtomicU32>,
        volume_bits: Arc<AtomicU32>,
        balance_bits: Arc<AtomicU32>,
        mode: Arc<Mutex<OutputMode>>,
        dsp_chain: DspChain,
        dsp_enabled: Arc<AtomicBool>,
        normalizer_enabled: Arc<AtomicBool>,
        normalizer: Arc<Mutex<crate::audio::dsp::normalizer::AudioNormalizer>>,
        samples_played: Arc<AtomicU64>,
        empty_callback_count: Arc<AtomicU32>,
        device_name: Option<String>,
        output_state: Arc<AtomicU32>,
        decoder_eof: Arc<AtomicBool>,
        is_bluetooth_detected: Arc<AtomicBool>,
        reconnecting: Arc<AtomicBool>,
        reconnect_attempts: Arc<AtomicU32>,
    },
    Stop,
    Flush,
    ChangeDevice {
        device_name: Option<String>,
        result_tx: std::sync::mpsc::Sender<Result<(), String>>,
    },
    Exit,
    ReconnectDevice {
        device_name: Option<String>,
        retry_count: u32,
    },
}

// Audio device info for enumeration
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub description: String,
    pub index: u32,
}

fn f32_to_bits(f: f32) -> u32 {
    f.to_bits()
}

const BUFFER_EMPTY_THRESHOLD: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputState {
    Priming,
    Running,
    Stopping,
}

impl Default for OutputState {
    fn default() -> Self {
        OutputState::Priming
    }
}

const OUTPUT_STATE_PRIMING: u32 = 0;
    const OUTPUT_STATE_RUNNING: u32 = 1;
    const OUTPUT_STATE_STOPPING: u32 = 2;
    const OUTPUT_STATE_ERROR: u32 = 3;

// Latency target: 60ms for tlength
const TARGET_LATENCY_MS: u32 = 60;
const TARGET_MINREQ_MS: u32 = 15;

pub struct AudioOutput {
    is_running: Arc<AtomicBool>,
    is_started: Arc<AtomicBool>,
    should_stop: Arc<AtomicBool>,
    volume_bits: Arc<AtomicU32>,
    balance_bits: Arc<AtomicU32>,
    mode_shared: Arc<Mutex<OutputMode>>,
    pub mode: OutputMode,
    command_tx: mpsc::Sender<AudioCommand>,
    thread_handle: Option<thread::JoinHandle<()>>,
    dsp_chain: DspChain,
    pub dsp_enabled: Arc<AtomicBool>,
    samples_played: Arc<AtomicU64>,
    sample_rate: u32,
    ring_buffer_capacity: usize,
    empty_callback_count: Arc<AtomicU32>,
    loop_reset: Arc<AtomicBool>,
    _consumer: Option<HeapCons<f32>>,
    clear_request: Arc<AtomicBool>,
    seek_fade_remaining: Arc<AtomicU32>,
    seek_mode: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    flush_requested: Arc<AtomicBool>,
    resume_frame_counter: Arc<AtomicU32>,
    shared_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
    old_track_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
    normalizer_enabled: Arc<AtomicBool>,
    normalizer: Arc<Mutex<crate::audio::dsp::normalizer::AudioNormalizer>>,
    selected_device_index: Arc<Mutex<Option<usize>>>,
    is_bluetooth_detected: Arc<AtomicBool>,
    switching: Arc<AtomicBool>,
    pub reconnecting: Arc<AtomicBool>,
    pub reconnect_attempts: Arc<AtomicU32>,
    current_device_name: Arc<Mutex<Option<String>>>,
    available_devices: Arc<Mutex<Vec<AudioDevice>>>,
    output_state: Arc<AtomicU32>,
    decoder_eof: Arc<AtomicBool>,
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        let thread_handle = thread::Builder::new()
            .name("pulseaudio".to_string())
            .spawn(move || {
                Self::audio_thread_loop(rx);
            })
            .ok();

         Self {
             is_running: Arc::new(AtomicBool::new(false)),
             is_started: Arc::new(AtomicBool::new(false)),
             should_stop: Arc::new(AtomicBool::new(false)),
             volume_bits: Arc::new(AtomicU32::new(f32_to_bits(1.0))),
             balance_bits: Arc::new(AtomicU32::new(f32_to_bits(0.0))),
             mode_shared: Arc::new(Mutex::new(OutputMode::Stereo)),
             mode: OutputMode::Stereo,
             command_tx: tx,
             thread_handle,
             dsp_chain: DspChain::default(),
             dsp_enabled: Arc::new(AtomicBool::new(true)),
             samples_played: Arc::new(AtomicU64::new(0)),
             sample_rate: 48000,
             ring_buffer_capacity: 0,
             empty_callback_count: Arc::new(AtomicU32::new(0)),
             loop_reset: Arc::new(AtomicBool::new(false)),
             _consumer: None,
             clear_request: Arc::new(AtomicBool::new(false)),
             seek_fade_remaining: Arc::new(AtomicU32::new(0)),
             seek_mode: Arc::new(AtomicBool::new(false)),
             paused: Arc::new(AtomicBool::new(false)),
             flush_requested: Arc::new(AtomicBool::new(false)),
             resume_frame_counter: Arc::new(AtomicU32::new(0)),
             shared_consumer: Arc::new(Mutex::new(None)),
             old_track_consumer: Arc::new(Mutex::new(None)),
             normalizer_enabled: Arc::new(AtomicBool::new(false)),
             normalizer: Arc::new(Mutex::new(
                 crate::audio::dsp::normalizer::AudioNormalizer::new(true, -14.0),
             )),
             selected_device_index: Arc::new(Mutex::new(None)),
             is_bluetooth_detected: Arc::new(AtomicBool::new(false)),
             switching: Arc::new(AtomicBool::new(false)),
reconnecting: Arc::new(AtomicBool::new(false)),
              reconnect_attempts: Arc::new(AtomicU32::new(0)),
             current_device_name: Arc::new(Mutex::new(None)),
             available_devices: Arc::new(Mutex::new(Vec::new())),
             output_state: Arc::new(AtomicU32::new(OUTPUT_STATE_PRIMING)),
             decoder_eof: Arc::new(AtomicBool::new(false)),
         }
    }

    pub fn request_loop_reset(&self) {
        self.loop_reset.store(true, Ordering::SeqCst);
    }

    pub fn get_dsp_chain(&self) -> DspChain {
        self.dsp_chain.clone()
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        if !self.is_started.load(Ordering::SeqCst) {
            self.sample_rate = sample_rate;
        }
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_samples_played_arc(&self) -> Arc<AtomicU64> {
        self.samples_played.clone()
    }

    pub fn get_samples_played(&self) -> u64 {
        self.samples_played.load(Ordering::SeqCst)
    }

    pub fn set_samples_played(&self, samples: u64) {
        self.samples_played.store(samples, Ordering::SeqCst);
    }

    pub fn reset_samples_played(&self, samples: u64) {
        self.samples_played.store(samples, Ordering::SeqCst);
    }

    pub fn get_reconnecting_status(&self) -> bool {
        self.reconnecting.load(Ordering::Relaxed)
    }

    pub fn get_reconnect_attempts(&self) -> u32 {
        self.reconnect_attempts.load(Ordering::Relaxed)
    }

    pub fn force_reconnect(&self) {
        self.reconnecting.store(true, Ordering::Relaxed);
        self.reconnect_attempts.store(0, Ordering::Relaxed);
    }

    pub fn clear_buffer(&self) {
        self.flush_requested.store(true, Ordering::SeqCst);
    }

    pub fn is_buffer_empty(&self) -> bool {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                return c.is_empty();
            }
        }
        true
    }

    pub fn is_ring_buffer_empty(&self) -> bool {
        self.is_buffer_empty()
    }

    pub fn is_truly_buffer_empty(&self) -> bool {
        self.empty_callback_count.load(Ordering::Relaxed) >= BUFFER_EMPTY_THRESHOLD
    }

    pub fn get_buffer_len(&self) -> usize {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                if !c.is_empty() {
                    return self.ring_buffer_capacity;
                }
                return 0;
            }
        }
        0
    }

    pub fn is_ring_buffer_ready(&self) -> bool {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                return !c.is_empty();
            }
        }
        false
    }

    pub fn is_seek_mode(&self) -> bool {
        self.seek_mode.load(Ordering::SeqCst)
    }

    pub fn reset_dsp(&self) {
        self.dsp_chain.reset();
    }

    pub fn update_dsp(&mut self, _settings: &crate::audio::dsp::DspSettings) {
        let rack = crate::audio::dsp::DspRack::build_rack(false);
        self.dsp_chain.swap_chain(rack);
    }

    pub fn update_mode_internal(&self) {
        if let Ok(mut m) = self.mode_shared.lock() {
            *m = self.mode;
        }
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume_bits
            .store(f32_to_bits(volume), Ordering::SeqCst);
    }

    pub fn set_balance(&self, balance: f32) {
        self.balance_bits
            .store(f32_to_bits(balance), Ordering::SeqCst);
    }

    pub fn set_dsp_enabled(&self, enabled: bool) {
        self.dsp_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn is_dsp_enabled(&self) -> bool {
        self.dsp_enabled.load(Ordering::SeqCst)
    }

    pub fn set_normalizer_enabled(&mut self, enabled: bool) {
        self.normalizer_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn set_normalizer_gain(&self, gain: f32) {
        if let Ok(mut norm) = self.normalizer.lock() {
            norm.set_fixed_gain(gain);
        }
    }

    pub fn get_normalizer_arc(&self) -> Arc<Mutex<crate::audio::dsp::normalizer::AudioNormalizer>> {
        self.normalizer.clone()
    }

    pub fn get_available_devices(&self) -> Vec<AudioDevice> {
        self.available_devices
            .lock()
            .ok()
            .map(|d| d.clone())
            .unwrap_or_default()
    }

    pub fn get_output_devices(&self) -> Vec<String> {
        if let Ok(devs) = self.available_devices.lock() {
            if !devs.is_empty() {
                return devs.iter().map(|d| d.description.clone()).collect();
            }
        }
        vec!["Default Output".to_string()]
    }

    pub fn set_output_device(&self, index: usize) {
        if let Ok(mut selected) = self.selected_device_index.lock() {
            *selected = Some(index);
        }
    }

    pub fn get_selected_device_index(&self) -> Option<usize> {
        self.selected_device_index
            .lock()
            .ok()
            .and_then(|guard| *guard)
    }

    pub fn select_device(&mut self, device_name: String) {
        let is_bluetooth = device_name.to_lowercase().contains("bluetooth");
        self.is_bluetooth_detected.store(is_bluetooth, Ordering::SeqCst);
        
        // Logic untuk switch device
        if let Ok(mut selected) = self.selected_device_index.lock() {
            *selected = Some(device_name.parse().unwrap_or(0));
        }
    }

    pub fn change_device(&self, device_name: Option<String>) -> Result<(), String> {
        let (tx, rx) = mpsc::channel();
        self.switching.store(true, Ordering::SeqCst);

        self.command_tx
            .send(AudioCommand::ChangeDevice {
                device_name,
                result_tx: tx,
            })
            .map_err(|e| format!("Failed to send device change command: {}", e))?;

        rx.recv()
            .map_err(|e| format!("Device change failed: {}", e))?
    }

    pub fn get_current_device_name(&self) -> Option<String> {
        self.current_device_name.lock().ok()?.clone()
    }

    pub fn set_output_state(&self, state: OutputState) {
        let state_val = match state {
            OutputState::Priming => OUTPUT_STATE_PRIMING,
            OutputState::Running => OUTPUT_STATE_RUNNING,
            OutputState::Stopping => OUTPUT_STATE_STOPPING,
        };
        self.output_state.store(state_val, Ordering::SeqCst);
    }

    pub fn get_output_state(&self) -> OutputState {
        match self.output_state.load(Ordering::SeqCst) {
            OUTPUT_STATE_PRIMING => OutputState::Priming,
            OUTPUT_STATE_RUNNING => OutputState::Running,
            OUTPUT_STATE_STOPPING => OutputState::Stopping,
            _ => OutputState::Priming,
        }
    }

    pub fn set_decoder_eof(&self, eof: bool) {
        self.decoder_eof.store(eof, Ordering::SeqCst);
    }

    fn create_pa_simple_with_latency(
        device_name: Option<&str>,
        sample_rate: u32,
    ) -> Result<pa_simple::Simple, String> {
        let spec = Spec {
            format: Format::F32le,
            channels: 2,
            rate: sample_rate,
        };

        let latency_bytes = (sample_rate * 2 * 4 * TARGET_LATENCY_MS / 1000) as u32;
        let minreq_bytes = (sample_rate * 2 * 4 * TARGET_MINREQ_MS / 1000) as u32;

        let buffer_attr = BufferAttr {
            maxlength: u32::MAX,
            tlength: latency_bytes,
            prebuf: 0,
            minreq: minreq_bytes,
            fragsize: minreq_bytes / 2,
        };

        pa_simple::Simple::new(
            None,
            "loonix-tunes",
            Direction::Playback,
            device_name,
            "Music",
            &spec,
            None,
            Some(&buffer_attr),
        )
        .map_err(|e| format!("pa_simple::new failed: {}", e))
    }

    pub fn start(&mut self, consumer: HeapCons<f32>, clear_old: bool, buffer_capacity: usize) {
        if clear_old {
            if let Ok(mut xf) = self.old_track_consumer.lock() {
                *xf = None;
            }
            self.seek_fade_remaining.store(0, Ordering::SeqCst);
        }

        self.ring_buffer_capacity = buffer_capacity;

        let device_name = {
            let selected = self.selected_device_index.lock().ok();
            selected.and_then(|s| s.map(|_| None::<&str>)).flatten()
        };

        match Self::create_pa_simple_with_latency(device_name, self.sample_rate) {
            Ok(handle) => {
                self.should_stop.store(false, Ordering::SeqCst);
                self.is_running.store(true, Ordering::SeqCst);
                self.output_state
                    .store(OUTPUT_STATE_PRIMING, Ordering::SeqCst);
                self.decoder_eof.store(false, Ordering::SeqCst);
                self.empty_callback_count.store(0, Ordering::Relaxed);

                if let Ok(mut current) = self.current_device_name.lock() {
                    *current = device_name.map(|s| s.to_string());
                }

                let _ = self.command_tx.send(AudioCommand::Play {
                    handle,
                    consumer,
                    should_stop: self.should_stop.clone(),
                    seek_mode: self.seek_mode.clone(),
                    paused: self.paused.clone(),
                    flush_requested: self.flush_requested.clone(),
                    seek_fade_remaining: self.seek_fade_remaining.clone(),
                    volume_bits: self.volume_bits.clone(),
                    balance_bits: self.balance_bits.clone(),
                    mode: self.mode_shared.clone(),
                    dsp_chain: self.dsp_chain.clone(),
                    dsp_enabled: self.dsp_enabled.clone(),
                    normalizer_enabled: self.normalizer_enabled.clone(),
                    normalizer: self.normalizer.clone(),
                    samples_played: self.samples_played.clone(),
                    empty_callback_count: self.empty_callback_count.clone(),
                    device_name: device_name.map(|s| s.to_string()),
                    output_state: self.output_state.clone(),
                    decoder_eof: self.decoder_eof.clone(),
                    is_bluetooth_detected: self.is_bluetooth_detected.clone(),
                    reconnecting: self.reconnecting.clone(),
                    reconnect_attempts: self.reconnect_attempts.clone(),
                });

                self.is_started.store(true, Ordering::SeqCst);
            }
            Err(e) => {
                eprintln!("[AudioOutput] Failed to create PulseAudio stream: {}", e);
            }
        }
        // Explicitly ensure paused is false on start
        self.paused.store(false, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        self.should_stop.store(true, Ordering::SeqCst);
        self.seek_mode.store(false, Ordering::SeqCst);
        self.seek_fade_remaining.store(0, Ordering::SeqCst);
        self.resume_frame_counter.store(0, Ordering::SeqCst);
        self.paused.store(false, Ordering::SeqCst); // Reset paused on stop
        self.output_state
            .store(OUTPUT_STATE_STOPPING, Ordering::SeqCst);
        self.reset_dsp();
    }

    pub fn start_consumers(&self) {
        self.is_running.store(true, Ordering::SeqCst);
    }

    pub fn pause(&mut self) {
        self.paused.store(true, Ordering::SeqCst);
    }

    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }

    pub fn trigger_seek_fade(&self) {
        let fade_samples = (self.sample_rate as f32 * 0.015) as u32;
        self.seek_fade_remaining
            .store(fade_samples, Ordering::SeqCst);
    }

    pub fn set_seek_mode(&self, seeking: bool) {
        self.seek_mode.store(seeking, Ordering::SeqCst);
    }

    pub fn trigger_delayed_resume(&self) {
        self.resume_frame_counter.store(2, Ordering::SeqCst);
    }

    pub fn check_resume_counter(&self) -> bool {
        let remaining = self.resume_frame_counter.load(Ordering::SeqCst);
        if remaining > 0 {
            self.resume_frame_counter
                .store(remaining - 1, Ordering::SeqCst);
            return false;
        }
        true
    }

    pub fn resume(&mut self) {
        self.paused.store(false, Ordering::SeqCst);
        self.is_running.store(true, Ordering::SeqCst);
    }

    fn handle_write_error(read_buffer: &[f32], samples_per_write: usize) {
        let silence = unsafe {
            std::slice::from_raw_parts(read_buffer.as_ptr() as *const u8, samples_per_write * 4)
        };
        let _ = silence;
    }

    fn reconnect_device(
        device_name: Option<&str>,
        sample_rate: u32,
        max_retries: u32,
    ) -> Result<pa_simple::Simple, String> {
        for attempt in 0..max_retries {
            match Self::create_pa_simple_with_latency(device_name, sample_rate) {
                Ok(handle) => return Ok(handle),
                Err(e) => {
                    eprintln!(
                        "[AudioOutput] Reconnect attempt {} failed: {}",
                        attempt + 1,
                        e
                    );
                    if attempt < max_retries - 1 {
                        thread::sleep(Duration::from_millis(500));
                    }
                }
            }
        }
        Err(format!(
            "Failed to reconnect after {} attempts",
            max_retries
        ))
    }

    fn audio_thread_loop(rx: mpsc::Receiver<AudioCommand>) {
        let _ = set_current_thread_priority(ThreadPriority::Max);
        let current_handle: Option<pa_simple::Simple> = None;

        loop {
            match rx.recv() {
                Ok(AudioCommand::Exit) => break,
                Ok(AudioCommand::Play {
                    handle,
                    consumer,
                    should_stop,
                    seek_mode,
                    paused,
                    flush_requested: flush_req,
                    seek_fade_remaining,
                    volume_bits,
                    balance_bits,
                    mode,
                    dsp_chain,
                    dsp_enabled,
                    normalizer_enabled,
                    normalizer,
                    samples_played,
                    empty_callback_count,
                    device_name: _device_name,
                    output_state,
                    decoder_eof,
                    is_bluetooth_detected,
                    reconnecting,
                    reconnect_attempts,
                }) => {
                    Self::push_loop_owned(
                        handle,
                        consumer,
                        should_stop,
                        seek_mode,
                        paused,
                        flush_req,
                        seek_fade_remaining,
                        volume_bits,
                        balance_bits,
                        mode,
                        dsp_chain,
                        dsp_enabled,
                        normalizer_enabled,
                        normalizer,
                        samples_played,
                        empty_callback_count,
                        output_state,
                        decoder_eof,
                        is_bluetooth_detected,
                        reconnecting,
                        reconnect_attempts,
                    );
                }
                Ok(AudioCommand::Stop) => {}
                Ok(AudioCommand::Flush) => {
                    if let Some(ref handle) = current_handle {
                        let _ = handle.flush();
                    }
                }
                Ok(AudioCommand::ChangeDevice {
                    device_name,
                    result_tx,
                }) => {
                    let sample_rate = 48000;
                    let result =
                        Self::create_pa_simple_with_latency(device_name.as_deref(), sample_rate)
                            .map(|_| ());
                    let _ = result_tx.send(result);
                }

                Ok(AudioCommand::ReconnectDevice { device_name, retry_count }) => {
                    if let Err(e) = Self::reconnect_device(device_name.as_deref(), 48000, retry_count) {
                        eprintln!("[AudioOutput] Reconnect gagal: {}", e);
                    }
                }

                Err(_) => break,
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn push_loop_owned(
        handle: pa_simple::Simple,
        mut consumer: HeapCons<f32>,
        should_stop: Arc<AtomicBool>,
        seek_mode: Arc<AtomicBool>,
        paused: Arc<AtomicBool>,
        flush_flag: Arc<AtomicBool>,
        seek_fade_remaining: Arc<AtomicU32>,
        volume_bits: Arc<AtomicU32>,
        balance_bits: Arc<AtomicU32>,
        mode: Arc<Mutex<OutputMode>>,
        dsp_chain: DspChain,
        dsp_enabled: Arc<AtomicBool>,
        normalizer_enabled: Arc<AtomicBool>,
        normalizer: Arc<Mutex<crate::audio::dsp::normalizer::AudioNormalizer>>,
        samples_played: Arc<AtomicU64>,
        empty_callback_count: Arc<AtomicU32>,
        output_state: Arc<AtomicU32>,
        decoder_eof: Arc<AtomicBool>,
        is_bluetooth_detected: Arc<AtomicBool>,
        reconnecting: Arc<AtomicBool>,
        reconnect_attempts: Arc<AtomicU32>,
    ) {
        let channels = 2;
        let frames_per_write = 1024usize;
        let samples_per_write = frames_per_write * channels;

        let mut read_buffer = vec![0.0f32; samples_per_write];
        let mut processed_buffer = vec![0.0f32; samples_per_write];
        let mut norm_input = vec![0.0f32; samples_per_write];
        let mut norm_output = vec![0.0f32; samples_per_write];

        let bluetooth_detected = is_bluetooth_detected.load(Ordering::Relaxed);

        const MAX_RECONNECT_ATTEMPTS: u32 = 3;

        let current_mode = *mode.lock().unwrap_or_else(|e| e.into_inner());

        loop {
            if should_stop.load(Ordering::SeqCst) {
                break;
            }

            let current_detected = crate::core::services::wireless::isBluetoothDetected();
            if current_detected != bluetooth_detected {
                is_bluetooth_detected.store(current_detected, Ordering::Relaxed);
                eprintln!("[AudioOutput] Device berubah. Bluetooth: {}. Reconnecting...", current_detected);
                break;
            }

            if flush_flag.load(Ordering::SeqCst) {
                loop {
                    let drained = consumer.pop_slice(&mut read_buffer);
                    if drained == 0 {
                        break;
                    }
                }
                empty_callback_count.store(0, Ordering::Relaxed);
                flush_flag.store(false, Ordering::SeqCst);

                let _ = handle.flush();
            }

            let is_seeking = seek_mode.load(Ordering::SeqCst);
            let is_paused = paused.load(Ordering::SeqCst);
            let state = output_state.load(Ordering::SeqCst);
            let is_eof = decoder_eof.load(Ordering::SeqCst);

            if is_seeking {
                read_buffer.fill(0.0);
                let silence = unsafe {
                    std::slice::from_raw_parts(
                        read_buffer.as_ptr() as *const u8,
                        samples_per_write * 4,
                    )
                };
                match handle.write(silence) {
                    Ok(_) => {
                        reconnect_attempts.store(0, Ordering::Relaxed);
                    }
                    Err(e) => {
                        eprintln!("[AudioOutput] Write error during seek: {}", e);
                        Self::handle_write_error(&read_buffer, samples_per_write);
                    }
                }
                std::thread::sleep(Duration::from_millis(2));
                continue;
            }

            if is_paused {
                read_buffer.fill(0.0);
                let silence = unsafe {
                    std::slice::from_raw_parts(
                        read_buffer.as_ptr() as *const u8,
                        samples_per_write * 4,
                    )
                };
                match handle.write(silence) {
                    Ok(_) => {
                        reconnect_attempts.store(0, Ordering::Relaxed);
                    }
                    Err(e) => {
                        eprintln!("[AudioOutput] Write error during pause: {}", e);
                        Self::handle_write_error(&read_buffer, samples_per_write);
                    }
                }
                continue;
            }

            read_buffer.fill(0.0);
            let samples_read = consumer.pop_slice(&mut read_buffer);

            if samples_read == 0 {
                empty_callback_count.fetch_add(1, Ordering::Relaxed);

                // Clean exit condition: EOF AND buffer empty AND in RUNNING state
                if is_eof && state == OUTPUT_STATE_RUNNING {
                    break;
                }

                // During PRIMING state: keep writing silence, don't exit
                // Buffer starvation during startup is NORMAL, not an error
                read_buffer.fill(0.0);
                let silence = unsafe {
                    std::slice::from_raw_parts(
                        read_buffer.as_ptr() as *const u8,
                        samples_per_write * 4,
                    )
                };
                match handle.write(silence) {
                    Ok(_) => {
                        reconnect_attempts.store(0, Ordering::Relaxed);
                    }
                    Err(e) => {
                        eprintln!("[AudioOutput] Write error on empty: {}", e);
                        Self::handle_write_error(&read_buffer, samples_per_write);
                    }
                }
                continue;
            }

            // Transition from PRIMING to RUNNING once we have data
            if state == OUTPUT_STATE_PRIMING {
                output_state.store(OUTPUT_STATE_RUNNING, Ordering::SeqCst);
            }

            empty_callback_count.store(0, Ordering::Relaxed);
            reconnect_attempts.store(0, Ordering::Relaxed);

            samples_played.fetch_add(samples_read as u64, Ordering::SeqCst);

            let process_len = samples_read.min(read_buffer.len());
            if dsp_enabled.load(Ordering::SeqCst) {
                dsp_chain.process(
                    &read_buffer[..process_len],
                    &mut processed_buffer[..process_len],
                );
            } else {
                processed_buffer[..process_len].copy_from_slice(&read_buffer[..process_len]);
            }


            if normalizer_enabled.load(Ordering::SeqCst) {
                if let Ok(mut norm) = normalizer.try_lock() {
                    norm_input[..process_len].copy_from_slice(&processed_buffer[..process_len]);
                    norm.process(&norm_input[..process_len], &mut norm_output[..process_len]);
                    processed_buffer[..process_len].copy_from_slice(&norm_output[..process_len]);
                }
            }

            let vol = f32::from_bits(volume_bits.load(Ordering::Relaxed));
            let bal = f32::from_bits(balance_bits.load(Ordering::Relaxed));
            let left_gain = if bal > 0.0 { 1.0 - bal } else { 1.0 };
            let right_gain = if bal < 0.0 { 1.0 + bal } else { 1.0 };

            let num_frames = process_len / 2;
            let fade_samples = seek_fade_remaining.load(Ordering::Acquire);

            for frame in 0..num_frames {
                let mut left = processed_buffer[frame * 2];
                let mut right = processed_buffer[frame * 2 + 1];

                left *= left_gain;
                right *= right_gain;

                match current_mode {
                    OutputMode::Mono => {
                        let mono = (left + right) * 0.5;
                        left = mono;
                        right = mono;
                    }
                    OutputMode::Surround => {
                        let diff = (left - right) * 0.3;
                        left += diff;
                        right -= diff;
                    }
                    OutputMode::Stereo => {}
                }

                left *= vol;
                right *= vol;

                if fade_samples > 0 {
                    let frames_remaining = (fade_samples / 2) as usize;
                    let fade_this_frame = frame.min(frames_remaining.saturating_sub(1));
                    let fade_gain = if frames_remaining > 0 {
                        (fade_this_frame as f32 + 1.0) / (frames_remaining as f32 + 1.0)
                    } else {
                        1.0
                    };
                    let fade_factor = fade_gain.sqrt();
                    left *= fade_factor;
                    right *= fade_factor;
                }

                if !left.is_finite() {
                    left = 0.0;
                }
                if !right.is_finite() {
                    right = 0.0;
                }
                left = left.clamp(-0.99, 0.99);
                right = right.clamp(-0.99, 0.99);

                processed_buffer[frame * 2] = left;
                processed_buffer[frame * 2 + 1] = right;
            }

            if fade_samples > 0 {
                let frames_used = (num_frames as u32).min(fade_samples / 2);
                if frames_used > 0 {
                    seek_fade_remaining.fetch_sub(frames_used * 2, Ordering::SeqCst);
                }
            }

            let bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(processed_buffer.as_ptr() as *const u8, process_len * 4)
            };

            match handle.write(bytes) {
                Ok(_) => {
                    reconnect_attempts.store(0, Ordering::Relaxed);
                }
                Err(e) => {
                    let current_attempts = reconnect_attempts.fetch_add(1, Ordering::Relaxed) + 1;

                    eprintln!(
                        "[AudioOutput] Write error (attempt {}/{}): {:?}",
                        current_attempts, MAX_RECONNECT_ATTEMPTS, e
                    );

                    if current_attempts >= MAX_RECONNECT_ATTEMPTS {
                        eprintln!(
                            "[AudioOutput] Max reconnect attempts reached, stopping: {:?}",
                            e
                        );
                        reconnecting.store(true, Ordering::Relaxed);
                        output_state.store(OUTPUT_STATE_ERROR, Ordering::SeqCst);
                        break;
                    }
                    
                    // Exponential backoff before retry
                    let delay_ms = 100 * current_attempts;
                    eprintln!("[AudioOutput] Waiting {}ms before retry...", delay_ms);
                    std::thread::sleep(Duration::from_millis(delay_ms as u64));
                }
            }
        }
    }
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
        self.is_started.store(false, Ordering::SeqCst);

        let _ = self.command_tx.send(AudioCommand::Exit);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}
