/* --- LOONIX-TUNES src/audio/scanner.rs | Pre-scan Loudness Scanner --- */

use ffmpeg::format::input;
use ffmpeg::media::Type;
use ffmpeg::util::frame::audio::Audio as AudioFrame;
use ffmpeg_next as ffmpeg;

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread::JoinHandle;

const MIN_GAIN: f32 = 0.01;

static GAIN_CACHE: OnceLock<Mutex<HashMap<String, f32>>> = OnceLock::new();

fn gain_cache() -> &'static Mutex<HashMap<String, f32>> {
    GAIN_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get_cached_gain(path: &str) -> Option<f32> {
    let guard = gain_cache().lock().ok()?;
    guard.get(path).copied()
}

fn store_cached_gain(path: &str, gain: f32) {
    if let Ok(mut guard) = gain_cache().lock() {
        guard.insert(path.to_string(), gain);
    }
}

pub fn clear_cache() {
    if let Ok(mut guard) = gain_cache().lock() {
        guard.clear();
    }
}

/// Parameters for the loudness scanner.
#[derive(Clone)]
pub struct ScanParams {
    pub target_lufs: f32,
    pub true_peak_dbtp: f32,
    pub max_gain_db: f32,
}

impl Default for ScanParams {
    fn default() -> Self {
        Self {
            target_lufs: -14.0,
            true_peak_dbtp: -1.5,
            max_gain_db: 12.0,
        }
    }
}

/// Calculate track gain multiplier by decoding the entire file at max speed.
/// Applies all 3 constraints: target_lufs, true_peak_ceiling, max_gain.
pub fn calculate_track_gain(path: &str, params: &ScanParams) -> f32 {
    // Check cache first
    if let Some(cached) = get_cached_gain(path) {
        return cached;
    }

    let gain = scan_loudness(path, params);
    store_cached_gain(path, gain);
    gain
}

/// Convenience overload with defaults.
pub fn calculate_track_gain_default(path: &str) -> f32 {
    calculate_track_gain(path, &ScanParams::default())
}

/// Spawn loudness scan in a background thread.
pub fn spawn_scan(
    path: String,
    params: ScanParams,
) -> (JoinHandle<f32>, std::sync::Arc<AtomicBool>) {
    let done = std::sync::Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();

    let handle = std::thread::Builder::new()
        .name("loudness-scanner".to_string())
        .spawn(move || {
            let gain = calculate_track_gain(&path, &params);
            done_clone.store(true, Ordering::SeqCst);
            gain
        })
        .expect("Failed to spawn loudness scanner thread");

    (handle, done)
}

/// Spawn scan and return gain via callback when done.
pub fn spawn_scan_with_callback(
    path: String,
    params: ScanParams,
    callback: impl FnOnce(f32) + Send + 'static,
) -> JoinHandle<()> {
    std::thread::Builder::new()
        .name("loudness-scanner".to_string())
        .spawn(move || {
            let gain = calculate_track_gain(&path, &params);
            callback(gain);
        })
        .expect("Failed to spawn loudness scanner thread")
}

/// Internal: decode file, compute integrated RMS and true peak, apply constraints.
fn scan_loudness(path: &str, params: &ScanParams) -> f32 {
    ffmpeg::init().ok();

    let mut ictx = match input(&path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("[SCANNER] Failed to open {}: {}", path, e);
            return 1.0;
        }
    };

    let input_stream = match ictx.streams().best(Type::Audio) {
        Some(s) => s,
        None => {
            eprintln!("[SCANNER] No audio stream in {}", path);
            return 1.0;
        }
    };
    let stream_idx = input_stream.index();

    let context = match ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())
    {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!(
                "[SCANNER] Failed to create codec context for {}: {}",
                path, e
            );
            return 1.0;
        }
    };
    let mut decoder = match context.decoder().audio() {
        Ok(dec) => dec,
        Err(e) => {
            eprintln!("[SCANNER] No audio decoder found for {}: {}", path, e);
            return 1.0;
        }
    };

    let input_format = decoder.format();
    let input_layout = if decoder.channel_layout().bits() != 0 {
        decoder.channel_layout()
    } else {
        ffmpeg::util::channel_layout::ChannelLayout::default(decoder.channels() as i32)
    };
    let input_rate = decoder.rate();

    let mut format_converter = match ffmpeg::software::resampling::Context::get(
        input_format,
        input_layout,
        input_rate,
        ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed),
        ffmpeg::util::channel_layout::ChannelLayout::STEREO,
        input_rate,
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[SCANNER] Failed to create converter for {}: {}", path, e);
            return 1.0;
        }
    };

    let mut frame = AudioFrame::empty();
    let mut interleaved_frame = AudioFrame::empty();

    // Accumulate mean square for RMS
    let mut sum_squares: f64 = 0.0;
    let mut total_samples: u64 = 0;
    // Track true peak (max absolute sample value)
    let mut true_peak: f32 = 0.0;

    // Decode every packet as fast as possible - no sleep, no clock
    for (stream, packet) in ictx.packets() {
        if stream.index() != stream_idx {
            continue;
        }

        let _ = decoder.send_packet(&packet);

        while decoder.receive_frame(&mut frame).is_ok() {
            if format_converter.run(&frame, &mut interleaved_frame).is_ok() {
                let data = interleaved_frame.data(0);
                if data.is_empty() {
                    continue;
                }

                let samples = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const f32,
                        data.len() / std::mem::size_of::<f32>(),
                    )
                };

                for &s in samples {
                    sum_squares += (s as f64) * (s as f64);
                    total_samples += 1;
                    let abs_s = s.abs();
                    if abs_s > true_peak {
                        true_peak = abs_s;
                    }
                }
            }
        }
    }

    // Flush decoder
    let _ = decoder.send_eof();
    let mut draining_frame = AudioFrame::empty();
    while decoder.receive_frame(&mut draining_frame).is_ok() {
        if draining_frame.samples() == 0 {
            continue;
        }
        if format_converter
            .run(&draining_frame, &mut interleaved_frame)
            .is_ok()
        {
            let data = interleaved_frame.data(0);
            if data.is_empty() {
                continue;
            }
            let samples = unsafe {
                std::slice::from_raw_parts(
                    data.as_ptr() as *const f32,
                    data.len() / std::mem::size_of::<f32>(),
                )
            };
            for &s in samples {
                sum_squares += (s as f64) * (s as f64);
                total_samples += 1;
                let abs_s = s.abs();
                if abs_s > true_peak {
                    true_peak = abs_s;
                }
            }
        }
    }

    if total_samples == 0 {
        eprintln!("[SCANNER] No samples decoded from {}", path);
        return 1.0;
    }

    // --- Calculate RMS loudness in dBFS ---
    let mean_square = sum_squares / total_samples as f64;
    let rms_linear = mean_square.sqrt() as f32;

    let measured_loudness_db = if rms_linear > 1e-10 {
        20.0 * rms_linear.log10()
    } else {
        -100.0
    };

    // --- Gain from target_lufs ---
    let diff = params.target_lufs - measured_loudness_db;
    let mut gain = 10.0_f32.powf(diff / 20.0);

    // --- Constraint 1: Max Gain / Pre-Amp cap ---
    let max_gain_linear = 10.0_f32.powf(params.max_gain_db / 20.0);
    if gain > max_gain_linear {
        gain = max_gain_linear;
    }

    // --- Constraint 2: True Peak Ceiling ---
    // If boosted peak would exceed ceiling, reduce gain
    let peak_ceiling_linear = 10.0_f32.powf(params.true_peak_dbtp / 20.0);
    if true_peak > 0.0 {
        let projected_peak = true_peak * gain;
        if projected_peak > peak_ceiling_linear {
            gain = peak_ceiling_linear / true_peak;
        }
    }

    // Final safety clamp
    gain = gain.clamp(MIN_GAIN, max_gain_linear);

    println!(
        "[SCANNER] {} -> loudness={:.1} dBFS, peak={:.3}, gain={:.3} ({} samples)",
        path, measured_loudness_db, true_peak, gain, total_samples
    );

    gain
}
