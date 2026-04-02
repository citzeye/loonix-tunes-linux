/* --- LOONIX-TUNES src/audio/audio_output.rs --- */
use crate::audio::dsp::crystalizer::get_crystalizer_amount_arc;
use crate::audio::dsp::{DspChain, DspProcessor};
use crate::audio::engine::OutputMode;
use crate::audio::highres::HighResProcessor;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::traits::{Consumer, Observer};
use ringbuf::HeapCons;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[cfg(target_os = "linux")]
use std::thread;

// Helper to convert f32 to u32 bits for atomic storage
fn f32_to_bits(f: f32) -> u32 {
    f.to_bits()
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct AudioOutput {
    stream: Option<cpal::Stream>,
    is_running: Arc<AtomicBool>,
    is_started: Arc<AtomicBool>,
    // FIX #1: Use AtomicU32 for volume/balance (lock-free)
    volume_bits: Arc<AtomicU32>,
    balance_bits: Arc<AtomicU32>,
    // Mode is rarely changed, keep Mutex but avoid locking in callback if possible
    // For now, we keep Mutex but minimize locking
    pub mode_shared: Arc<Mutex<OutputMode>>,
    pub mode: OutputMode,
    // DSP Chain (lock-free via AtomicPtr)
    dsp_chain: DspChain,
    // True bypass switch for DSP (Send/Return)
    pub dsp_enabled: Arc<AtomicBool>,
    // Sample counter for audio clock
    samples_played: Arc<AtomicU64>,
    sample_rate: u32,
    // Ring buffer capacity (known at creation)
    ring_buffer_capacity: usize,
    // Callback starvation counter for end-of-track detection
    empty_callback_count: Arc<AtomicU32>,
    // Flag to reset samples on loop
    loop_reset: Arc<AtomicBool>,
    // Consumer for buffer access (owned by callback, not shared)
    _consumer: Option<HeapCons<f32>>,
    // Clear request flag - set by engine, cleared by audio callback
    clear_request: Arc<AtomicBool>,
    // Crossfade state for seek (50ms = ~2400 samples at 48000Hz)
    crossfade_frames: Arc<AtomicU32>,
    // Seek mode - unconditional silence when true
    seek_mode: Arc<AtomicBool>,
    // Resume pending counter - frames to wait before unmuting
    resume_frame_counter: Arc<AtomicU32>,
    // Shadow Slot: shared handle to consumer for crossfade on track change
    shared_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
    // Crossfade consumer: holds old track's consumer during 50ms overlap
    crossfade_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
    // High-Res processor (f64 internal processing)
    highres: HighResProcessor,
    highres_enabled: Arc<AtomicBool>,
    // Exclusive mode (PipeWire bypass on Linux)
    exclusive_mode: Arc<AtomicBool>,
    // Normalizer enabled (EBU R128 loudness normalization)
    normalizer_enabled: Arc<AtomicBool>,
    // Normalizer processor (EBU R128) - wrapped in Arc<Mutex> for thread-safe mutable access
    normalizer: Arc<Mutex<crate::audio::dsp::AudioNormalizer>>,
    // PipeWire thread for exclusive mode on Linux
    #[cfg(target_os = "linux")]
    pw_thread: Option<std::thread::JoinHandle<()>>,
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        Self {
            stream: None,
            is_running: Arc::new(AtomicBool::new(false)),
            is_started: Arc::new(AtomicBool::new(false)),
            volume_bits: Arc::new(AtomicU32::new(f32_to_bits(1.0))),
            balance_bits: Arc::new(AtomicU32::new(f32_to_bits(0.0))),
            mode_shared: Arc::new(Mutex::new(OutputMode::Stereo)),
            mode: OutputMode::Stereo,
            dsp_chain: DspChain::default(),
            dsp_enabled: Arc::new(AtomicBool::new(true)),
            samples_played: Arc::new(AtomicU64::new(0)),
            sample_rate: 48000,
            ring_buffer_capacity: 0,
            empty_callback_count: Arc::new(AtomicU32::new(0)),
            loop_reset: Arc::new(AtomicBool::new(false)),
            _consumer: None,
            clear_request: Arc::new(AtomicBool::new(false)),
            crossfade_frames: Arc::new(AtomicU32::new(0)),
            seek_mode: Arc::new(AtomicBool::new(false)),
            resume_frame_counter: Arc::new(AtomicU32::new(0)),
            shared_consumer: Arc::new(Mutex::new(None)),
            crossfade_consumer: Arc::new(Mutex::new(None)),
            highres: HighResProcessor::new(),
            highres_enabled: Arc::new(AtomicBool::new(false)),
            exclusive_mode: Arc::new(AtomicBool::new(false)),
            normalizer_enabled: Arc::new(AtomicBool::new(false)),
            normalizer: Arc::new(Mutex::new(crate::audio::dsp::AudioNormalizer::new(
                true, -14.0,
            ))),
            #[cfg(target_os = "linux")]
            pw_thread: None,
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

    // Clear ring buffer - kuras habis secara SINKRON!
    pub fn clear_buffer(&self) {
        // 🔥 FIX MUTLAK: Kunci consumer dan buang isinya SEKARANG JUGA.
        // Ini memastikan buffer 100% kosong SEBELUM decoder memasukkan audio baru.
        if let Ok(mut cons_lock) = self.shared_consumer.lock() {
            if let Some(ref mut c) = *cons_lock {
                let mut discard_buf = [0.0f32; 2048];
                loop {
                    // Buang tanpa ampun, tanpa mencatat ke Master Clock
                    if c.pop_slice(&mut discard_buf) == 0 {
                        break;
                    }
                }
            }
        }
    }

    pub fn is_buffer_empty(&self) -> bool {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                return c.is_empty();
            }
        }
        true
    }

    // Legacy alias
    pub fn is_ring_buffer_empty(&self) -> bool {
        self.is_buffer_empty()
    }

    pub fn is_truly_buffer_empty(&self) -> bool {
        self.empty_callback_count.load(Ordering::Relaxed) >= 5
    }

    // Get ring buffer length (samples available to play)
    pub fn get_buffer_len(&self) -> usize {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                // Use known capacity + is_empty check
                // If not empty, assume full capacity (conservative estimate)
                // This is accurate enough for the hysteresis check
                if !c.is_empty() {
                    return self.ring_buffer_capacity;
                }
                return 0;
            }
        }
        0
    }

    // Check if ring buffer has data (for FIX 5 - buffer guarantee)
    pub fn is_ring_buffer_ready(&self) -> bool {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                return !c.is_empty();
            }
        }
        false
    }

    // Check if in seek mode
    pub fn is_seek_mode(&self) -> bool {
        self.seek_mode.load(Ordering::SeqCst)
    }

    // Reset DSP chain (clear effect tails)
    pub fn reset_dsp(&self) {
        self.dsp_chain.reset();
    }

    // Update DSP chain with new settings
    pub fn update_dsp(&mut self, settings: &crate::audio::dsp::DspSettings) {
        // Clone settings to possibly modify crystal_amount from atomic
        let mut current_settings = settings.clone();

        // Read current crystal amount from atomic (real-time from UI)
        if let Some(arc) = get_crystalizer_amount_arc() {
            let bits = arc.load(Ordering::Relaxed);
            let amount = bits_to_f32(bits);
            current_settings.crystal_amount = amount;
        }

        let rack = crate::audio::dsp::DspRack::build_chain(&current_settings);
        self.dsp_chain.swap_chain(rack);
    }

    // Fungsi sinkronisasi saat mode berubah di core.rs
    pub fn update_mode_internal(&self) {
        // Only lock when actually changing mode (rare)
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

    pub fn set_highres_enabled(&mut self, enabled: bool) {
        self.highres.toggle(enabled);
        self.highres_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn set_exclusive_mode(&mut self, enabled: bool) {
        self.exclusive_mode.store(enabled, Ordering::SeqCst);
        if enabled {
            println!("[AUDIO] Exclusive mode enabled - PipeWire hardware bypass");
        } else {
            println!("[AUDIO] Shared mode enabled - PipeWire mixer");
        }
    }

    pub fn set_normalizer_enabled(&mut self, enabled: bool) {
        self.normalizer_enabled.store(enabled, Ordering::SeqCst);
        println!("[NORMALIZER] Enabled: {}", enabled);
    }

    pub fn set_normalizer_gain(&self, gain: f32) {
        if let Ok(mut norm) = self.normalizer.lock() {
            norm.set_fixed_gain(gain);
            println!("[NORMALIZER] Fixed gain set to {:.3}", gain);
        }
    }

    pub fn get_normalizer_arc(&self) -> Arc<Mutex<crate::audio::dsp::AudioNormalizer>> {
        self.normalizer.clone()
    }

    pub fn start(&mut self, consumer: HeapCons<f32>, clear_old: bool, buffer_capacity: usize) {
        // Store buffer capacity for buffer level checks
        self.ring_buffer_capacity = buffer_capacity;

        // Clear old crossfade consumer if this is a fresh start (not track transition)
        if clear_old {
            if let Ok(mut xf) = self.crossfade_consumer.lock() {
                *xf = None;
            }
            self.crossfade_frames.store(0, Ordering::SeqCst);
        }

        let is_exclusive = self.exclusive_mode.load(Ordering::SeqCst);

        // HYBRID MODE: Linux + Exclusive = Native PipeWire
        #[cfg(target_os = "linux")]
        if is_exclusive {
            // Stop existing cpal stream if any
            self.stream = None;

            let sample_rate = self.sample_rate;
            let volume_bits = self.volume_bits.clone();
            let is_running = self.is_running.clone();

            // Spawn native PipeWire thread
            let handle = thread::spawn(move || {
                spawn_pipewire_exclusive(sample_rate, volume_bits, is_running);
            });

            self.pw_thread = Some(handle);
            self.is_running.store(true, Ordering::SeqCst);
            self.is_started.store(true, Ordering::SeqCst);
            return;
        }

        // If stream already exists (reuse for crossfade), just update consumer
        if self.stream.is_some() {
            // Update shared consumer - callback will pick up the new one
            if let Ok(mut cons) = self.shared_consumer.lock() {
                *cons = Some(consumer);
            }
            // Trigger 50ms crossfade (old consumer is in crossfade_consumer from stop())
            let crossfade_samples = (self.sample_rate as f32 * 0.05) as u32;
            self.crossfade_frames
                .store(crossfade_samples, Ordering::SeqCst);
            // Re-enable processing
            self.is_running.store(true, Ordering::SeqCst);
            return;
        }

        // Normal mode - use cpal with default device
        let device = {
            let host = cpal::default_host();
            match host.default_output_device() {
                Some(d) => d,
                None => {
                    eprintln!("[AudioOutput] No output device found, skipping playback");
                    return;
                }
            }
        };

        #[cfg(not(target_os = "linux"))]
        let _ = is_exclusive;

        // Force stereo (2 channels) and 48khz sample rate
        let channels = 2;
        self.sample_rate = 48000;

        // Use fixed buffer size to prevent underruns (kaset kusut/distortion)
        let config = cpal::StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(4096), // ~85ms at 48kHz — lebih aman untuk DSP berat (WSOLA)
        };

        // Wrap consumer in Arc<Mutex<Option>> for shared access (crossfade slot)
        let shared = Arc::new(Mutex::new(Some(consumer)));
        self.shared_consumer = shared.clone();
        let shared_for_callback = shared.clone();
        let crossfade_consumer_for_callback = self.crossfade_consumer.clone();
        let _clear_request = self.clear_request.clone();
        let seek_mode = self.seek_mode.clone();
        let resume_frame_counter = self.resume_frame_counter.clone();
        let is_running = self.is_running.clone();
        let volume_bits = self.volume_bits.clone();
        let balance_bits = self.balance_bits.clone();
        let mode_shared = self.mode_shared.clone();
        let dsp_chain = self.dsp_chain.clone();
        let dsp_enabled = self.dsp_enabled.clone();
        let highres_enabled = self.highres_enabled.clone();
        let normalizer_enabled = self.normalizer_enabled.clone();
        let normalizer = self.normalizer.clone();
        let samples_played = self.samples_played.clone();
        let crossfade_frames = self.crossfade_frames.clone();
        let sample_rate = self.sample_rate;
        let empty_count_clone = self.empty_callback_count.clone();

        is_running.store(true, Ordering::SeqCst);
        self.is_started.store(true, Ordering::SeqCst);
        self.update_mode_internal();

        // Reset samples counter on start (Master Clock)
        samples_played.store(0, Ordering::SeqCst);
        // Reset callback starvation counter
        self.empty_callback_count.store(0, Ordering::SeqCst);

        // FIX: Alokasi 16384 buat jaga-jaga kalau OS/Driver ngasih buffer lebih gede
        // 4096 frames * 2 channels = 8192 samples minimum
        let mut read_buffer = vec![0.0f32; 16384];
        let mut processed_buffer = vec![0.0f32; 16384];
        let mut crossfade_buffer = vec![0.0f32; 16384];

        let err_fn = |_err| {};

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // FIX #1: Zero-fill di awal callback (KRITIS)
                    data.fill(0.0);

                    // EBU R128 Normalizer buffers (zero-alloc inside closure)
                    let mut norm_input = vec![0.0f32; 16384];
                    let mut norm_output = vec![0.0f32; 16384];

                    if !is_running.load(Ordering::SeqCst) {
                        return;
                    }

                    // Lock primary consumer
                    let mut cons_lock = match shared_for_callback.lock() {
                        Ok(l) => l,
                        Err(_) => return,
                    };
                    let cons = match cons_lock.as_mut() {
                        Some(c) => c,
                        None => return,
                    };

                    // 🔥 SHADOW SLOT: crossfade from old consumer (track transition)
                    let cf_remaining = crossfade_frames.load(Ordering::SeqCst);
                    let samples_per_frame = 2;
                    let num_frames = data.len() / channels as usize;
                    let samples_needed = num_frames * samples_per_frame;

                    // Check if crossfade consumer has data (track transition vs seek fade-in)
                    let has_crossfade_consumer = {
                        let xf_lock = crossfade_consumer_for_callback.lock().unwrap();
                        xf_lock.is_some()
                    };

                    // Track samples actually read from consumer (for Master Clock)
                    let mut samples_read_from_consumer: usize = 0;
                    // Track crossfade samples read (for starvation detection)
                    let mut xfade_read: usize = 0;

                    if cf_remaining > 0 && has_crossfade_consumer {
                        // === TRACK TRANSITION CROSSFADE ===
                        // Pull primary (new track) samples
                        read_buffer.fill(0.0);
                        let safe_len = samples_needed.min(read_buffer.len());
                        let primary_read = cons.pop_slice(&mut read_buffer[..safe_len]);

                        // Mix with crossfade consumer (old track)
                        crossfade_buffer.fill(0.0);
                        let mut xfade_dropped = false;

                        // Scope the crossfade lock to minimize hold time
                        {
                            let mut xf_lock = crossfade_consumer_for_callback.lock().unwrap();
                            if let Some(ref mut xf_cons) = *xf_lock {
                                let xfade_safe_len = samples_needed.min(crossfade_buffer.len());
                                xfade_read =
                                    xf_cons.pop_slice(&mut crossfade_buffer[..xfade_safe_len]);
                                // Auto-cleanup: old consumer exhausted -> drop -> Rust Drop
                                if xfade_read == 0 {
                                    *xf_lock = None;
                                    xfade_dropped = true;
                                }
                            } else {
                                xfade_dropped = true;
                            }
                        }

                        if xfade_dropped && xfade_read == 0 {
                            // Crossfade ended, clear counter
                            crossfade_frames.store(0, Ordering::SeqCst);
                        } else {
                            // Mix: Outgoing * fade_out + Incoming * fade_in
                            let total_cf_samples = (sample_rate as f32 * 0.05) as u32;
                            let fade_out = cf_remaining as f32 / total_cf_samples as f32;
                            let fade_in = 1.0 - fade_out;
                            let loop_len = safe_len.min(xfade_read);
                            for i in 0..loop_len {
                                let primary_sample = if i < primary_read {
                                    read_buffer[i]
                                } else {
                                    0.0
                                };
                                let xfade_sample = if i < xfade_read {
                                    crossfade_buffer[i]
                                } else {
                                    0.0
                                };
                                read_buffer[i] = xfade_sample * fade_out + primary_sample * fade_in;
                            }

                            // Decrement crossfade counter by frames output
                            let frames_output = (primary_read.max(xfade_read) + samples_per_frame
                                - 1)
                                / samples_per_frame;
                            if frames_output > 0 {
                                let decrement = std::cmp::min(frames_output as u32, cf_remaining);
                                crossfade_frames.fetch_sub(decrement, Ordering::SeqCst);
                            }

                            // Count primary samples for Master Clock
                            samples_read_from_consumer = primary_read;
                        }
                    } else {
                        // === NORMAL PULL (no crossfade, or seek fade-in) ===
                        read_buffer.fill(0.0);
                        let safe_len = samples_needed.min(read_buffer.len());
                        samples_read_from_consumer = cons.pop_slice(&mut read_buffer[..safe_len]);
                    }

                    // 🔥 LOCK-FREE CLEAR: use swap to atomically clear and reset flag
                    // if clear_request.swap(false, Ordering::SeqCst) {
                    //     // Drain buffer (Buang sisa audio lama ke tong sampah)
                    //     // PERINGATAN: JANGAN hitung samples ini ke Master Clock!
                    //     // Karena mereka batal dikirim ke speaker.
                    //     let mut discard_buf = [0.0f32; 2048];
                    //     loop {
                    //         let n = cons.pop_slice(&mut discard_buf);
                    //         if n == 0 {
                    //             break;
                    //         }
                    //     }
                    // }

                    // 🔥 SEEK MODE: mute output but ALWAYS count consumed samples
                    // Consumer clock must never skip — samples were already popped from ring buffer
                    let in_seek_mode = seek_mode.load(Ordering::SeqCst);

                    // 🔥 RESUME DELAY: mute output for a few frames
                    let in_resume_delay = {
                        let remaining = resume_frame_counter.load(Ordering::SeqCst);
                        if remaining > 0 {
                            resume_frame_counter.fetch_sub(1, Ordering::SeqCst);
                            true
                        } else {
                            false
                        }
                    };

                    // Always increment consumer clock for consumed samples
                    if samples_read_from_consumer > 0 {
                        samples_played
                            .fetch_add(samples_read_from_consumer as u64, Ordering::SeqCst);
                    }

                    // If muted, output silence and skip DSP processing
                    if in_seek_mode || in_resume_delay {
                        data.fill(0.0);
                        return;
                    }

                    // Normal processing
                    let vol = bits_to_f32(volume_bits.load(Ordering::Relaxed));
                    let bal = bits_to_f32(balance_bits.load(Ordering::Relaxed));

                    let current_mode = match mode_shared.try_lock() {
                        Ok(m) => *m,
                        Err(_) => OutputMode::Stereo,
                    };

                    let left_gain = if bal > 0.0 { 1.0 - bal } else { 1.0 };
                    let right_gain = if bal < 0.0 { 1.0 + bal } else { 1.0 };

                    processed_buffer.fill(0.0);
                    let process_len = samples_needed.min(read_buffer.len());

                    // Check DSP bypass switch
                    let is_dsp_on = dsp_enabled.load(Ordering::SeqCst);
                    if is_dsp_on {
                        // Send/Return: DSP active
                        dsp_chain.process(
                            &read_buffer[..process_len],
                            &mut processed_buffer[..process_len],
                        );
                    } else {
                        // True bypass: copy raw audio directly
                        processed_buffer[..process_len]
                            .copy_from_slice(&read_buffer[..process_len]);
                    }

                    // High-Res processing (f64 internal depth)
                    if highres_enabled.load(Ordering::SeqCst) {
                        let mut temp = vec![0.0f32; process_len];
                        for (i, &sample) in processed_buffer[..process_len].iter().enumerate() {
                            temp[i] = (sample as f64 * 0.99999999) as f32;
                        }
                        processed_buffer[..process_len].copy_from_slice(&temp);
                    }

                    // EBU R128 Normalizer (Fixed gain per track)
                    if normalizer_enabled.load(Ordering::SeqCst) {
                        let safe_len = process_len.min(norm_input.len());
                        norm_input[..safe_len].copy_from_slice(&processed_buffer[..safe_len]);
                        if let Ok(mut norm) = normalizer.lock() {
                            norm.process(&norm_input[..safe_len], &mut norm_output[..safe_len]);
                            processed_buffer[..safe_len].copy_from_slice(&norm_output[..safe_len]);
                        }
                    }

                    let num_frames_to_write = (process_len / 2).min(num_frames);
                    for frame in 0..num_frames_to_write {
                        let left = processed_buffer[frame * 2];
                        let right = processed_buffer[frame * 2 + 1];

                        let mut out_left = left * vol * left_gain;
                        let mut out_right = right * vol * right_gain;

                        if !out_left.is_finite() {
                            out_left = 0.0;
                        }
                        if !out_right.is_finite() {
                            out_right = 0.0;
                        }

                        match current_mode {
                            OutputMode::Mono => {
                                let mono_mix = (out_left + out_right) * 0.5;
                                out_left = mono_mix;
                                out_right = mono_mix;
                            }
                            OutputMode::Surround => {
                                let diff = (out_left - out_right) * 0.3;
                                out_left += diff;
                                out_right -= diff;
                            }
                            OutputMode::Stereo => {}
                        }

                        let out_idx = frame * channels as usize;
                        if out_idx < data.len() {
                            // Safety clamp terakhir sebelum hardware (-0.1dB headroom)
                            data[out_idx] = out_left.clamp(-0.99, 0.99);
                            if channels > 1 && out_idx + 1 < data.len() {
                                data[out_idx + 1] = out_right.clamp(-0.99, 0.99);
                            }
                        }
                    }

                    // Callback starvation detection
                    if samples_read_from_consumer == 0 && xfade_read == 0 {
                        empty_count_clone.fetch_add(1, Ordering::Relaxed);
                    } else {
                        empty_count_clone.store(0, Ordering::Relaxed);
                    }
                },
                err_fn,
                None,
            )
            .expect("Failed to build stream");

        stream.play().expect("Failed to start stream");
        self.stream = Some(stream);
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        self.empty_callback_count.store(0, Ordering::SeqCst);

        // Note: PipeWire thread will exit on its own when is_running is false
        // No need to join here - it's handled by Drop

        // 🔥 SHADOW SLOT: move current consumer to crossfade slot
        // The callback will drain it with fade-out when next track starts
        if let Ok(mut cons) = self.shared_consumer.lock() {
            if let Some(c) = cons.take() {
                if let Ok(mut xf) = self.crossfade_consumer.lock() {
                    // Drop any existing crossfade consumer silently (rapid track switch)
                    *xf = Some(c);
                }
            }
        }

        // Reset state for clean transition
        self.seek_mode.store(false, Ordering::SeqCst);
        self.crossfade_frames.store(0, Ordering::SeqCst);
        self.resume_frame_counter.store(0, Ordering::SeqCst);

        // Reset filter state to prevent pop on next track
        self.reset_dsp();
    }

    pub fn start_consumers(&self) {
        // PHASE 3: Resume - restart audio processing
        self.is_running.store(true, Ordering::SeqCst);
    }

    pub fn pause(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn trigger_crossfade(&self) {
        // 50ms fade IN at current sample rate (~2200 samples stereo)
        let crossfade_samples = (self.sample_rate as f32 * 0.05) as u32;
        self.crossfade_frames
            .store(crossfade_samples, Ordering::SeqCst);
    }

    pub fn start_fade_out(&self) {
        // 50ms fade OUT before seek
        let fade_samples = (self.sample_rate as f32 * 0.05) as u32;
        self.crossfade_frames.store(fade_samples, Ordering::SeqCst);
    }

    pub fn set_seek_mode(&self, seeking: bool) {
        self.seek_mode.store(seeking, Ordering::SeqCst);
    }

    /// Trigger resume with delay - waits a few frames before unmuting
    pub fn trigger_delayed_resume(&self) {
        // Wait ~2 frames (~84ms at 24fps) before unmuting
        self.resume_frame_counter.store(2, Ordering::SeqCst);
    }

    /// Called from audio callback to check if we should unmute
    /// Returns true if seek mode should be disabled
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
        self.is_running.store(true, Ordering::SeqCst);
    }
}

#[cfg(target_os = "linux")]
fn spawn_pipewire_exclusive(
    sample_rate: u32,
    _volume_bits: Arc<AtomicU32>,
    is_running: Arc<AtomicBool>,
) {
    use pipewire as pw;
    use pw::properties::properties;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    pw::init();

    let mainloop = match pw::main_loop::MainLoopRc::new(None) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[PIPEWIRE] Failed to create main loop: {}", e);
            return;
        }
    };

    let context = match pw::context::ContextRc::new(&mainloop, None) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[PIPEWIRE] Failed to create context: {}", e);
            return;
        }
    };

    let core = match context.connect_rc(None) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[PIPEWIRE] Failed to connect to core: {}", e);
            return;
        }
    };

    let props = properties! {
        *pw::keys::MEDIA_TYPE => "Audio",
        *pw::keys::MEDIA_CATEGORY => "Playback",
        *pw::keys::MEDIA_ROLE => "Music",
        *pw::keys::NODE_NAME => "LoonixTunesExclusive",
        "node.exclusive" => "true",
    };

    let _stream = match pw::stream::StreamBox::new(&core, "loonix-tunes-exclusive", props) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[PIPEWIRE] Failed to create stream: {}", e);
            return;
        }
    };

    println!(
        "[PIPEWIRE] Exclusive mode ACTIVE - direct hardware access at {}Hz",
        sample_rate
    );
    println!("[PIPEWIRE] Note: Full audio pipeline integration pending");

    while is_running.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(10));
    }

    println!("[PIPEWIRE] Exclusive stream stopped");
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        // Stop processing first
        self.is_running.store(false, Ordering::SeqCst);
        self.is_started.store(false, Ordering::SeqCst);

        // Explicitly drop the stream to release audio device resources
        let _ = self.stream.take();

        // Join PipeWire thread to ensure clean shutdown
        #[cfg(target_os = "linux")]
        {
            if let Some(handle) = self.pw_thread.take() {
                let _ = handle.join();
            }
        }
    }
}
