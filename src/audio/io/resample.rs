/* --- loonixtunesv2/src/audio/resample.rs | Resampler --- */

use ringbuf::traits::Producer;
use ringbuf::HeapProd;
use soxr::{format::Interleaved, Soxr};

use std::sync::atomic::{AtomicBool, Ordering};

pub type StereoResampler = Soxr<Interleaved<f32, 2>>;

pub fn create_resampler(input_rate: f64, output_rate: f64) -> Option<StereoResampler> {
    match Soxr::new(input_rate, output_rate) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Failed to create Soxr resampler: {}", e);
            None
        }
    }
}

pub fn process_frame(
    raw_data: &[u8],
    resampler: &mut StereoResampler,
    producer: &mut HeapProd<f32>,
    total_decoded_samples: &mut u64,
) {
    if raw_data.is_empty() || raw_data.len() % 4 != 0 {
        return;
    }

    let sample_count = raw_data.len() / 4;
    let input_flat: &[f32] =
        unsafe { std::slice::from_raw_parts(raw_data.as_ptr() as *const f32, sample_count) };

    let input_frames = input_flat.len() / 2;
    let input_stereo: &[[f32; 2]] =
        unsafe { std::slice::from_raw_parts(input_flat.as_ptr() as *const [f32; 2], input_frames) };

    let max_output_frames = ((input_frames as f64 * 1.5) as usize) + 32;
    let mut output_stereo: Vec<[f32; 2]> = vec![[0.0; 2]; max_output_frames];

    if let Ok(processed) = resampler.process(input_stereo, &mut output_stereo) {
        if processed.output_frames > 0 {
            push_output(
                &output_stereo,
                processed.output_frames,
                producer,
                total_decoded_samples,
            );
        }
    }
}

pub fn process_frame_buffered(
    raw_data: &[u8],
    resampler: &mut StereoResampler,
    producer: &mut HeapProd<f32>,
    total_decoded_samples: &mut u64,
    buffered: &mut u64,
) {
    if raw_data.is_empty() || raw_data.len() % 4 != 0 {
        return;
    }

    let sample_count = raw_data.len() / 4;
    let input_flat: &[f32] =
        unsafe { std::slice::from_raw_parts(raw_data.as_ptr() as *const f32, sample_count) };

    let input_frames = input_flat.len() / 2;
    let input_stereo: &[[f32; 2]] =
        unsafe { std::slice::from_raw_parts(input_flat.as_ptr() as *const [f32; 2], input_frames) };

    let max_output_frames = ((input_frames as f64 * 1.5) as usize) + 32;
    let mut output_stereo: Vec<[f32; 2]> = vec![[0.0; 2]; max_output_frames];

    if let Ok(processed) = resampler.process(input_stereo, &mut output_stereo) {
        if processed.output_frames > 0 {
            let output_flat = &output_stereo[..processed.output_frames];
            let flat_len = output_flat.len() * 2;

            *buffered += flat_len as u64;

            push_output(
                output_stereo.as_slice(),
                processed.output_frames,
                producer,
                total_decoded_samples,
            );
        }
    }
}

pub fn drain(
    resampler: &mut StereoResampler,
    producer: &mut HeapProd<f32>,
    total_decoded_samples: &mut u64,
    should_stop: &AtomicBool,
) {
    let mut output_stereo: Vec<[f32; 2]> = vec![[0.0; 2]; 4096];

    loop {
        let output_frames = match resampler.drain(&mut output_stereo) {
            Ok(frames) => frames,
            Err(e) => {
                eprintln!("Resampler drain error: {}", e);
                break;
            }
        };

        if output_frames == 0 {
            break;
        }

        if should_stop.load(Ordering::Relaxed) {
            break;
        }

        push_output(
            &output_stereo,
            output_frames,
            producer,
            total_decoded_samples,
        );
    }
}

fn push_output(
    output_stereo: &[[f32; 2]],
    output_frames: usize,
    producer: &mut HeapProd<f32>,
    total_decoded_samples: &mut u64,
) {
    let output_flat: &[f32] = unsafe {
        std::slice::from_raw_parts(output_stereo.as_ptr() as *const f32, output_frames * 2)
    };

    let mut pushed = 0;
    while pushed < output_flat.len() {
        match producer.push_slice(&output_flat[pushed..]) {
            n if n > 0 => {
                pushed += n;
                *total_decoded_samples += n as u64;
            }
            _ => std::thread::yield_now(),
        }
    }
}
