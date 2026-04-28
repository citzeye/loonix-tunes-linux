/* --- loonixtunesv2/src/audio/io/decoder.rs | decoder --- */

use ffmpeg::format::input;
use ffmpeg::media::Type;
use ffmpeg::util::frame::audio::Audio as AudioFrame;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::sys as ff;
use ffmpeg_next::Rescale;

use ringbuf::HeapProd;

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

use super::resample;

pub const SEEK_STATE_IDLE: u8 = 0;
pub const SEEK_STATE_DECODING: u8 = 1;
pub const SEEK_STATE_READY: u8 = 2;

pub enum DecoderEvent {
    SeekComplete,
    BufferReady,
    InitialBufferReady,
    EndOfTrack { total_samples: u64 },
}

pub struct DecoderHandle {
    pub control: Arc<DecoderControl>,
    thread_handle: Option<JoinHandle<()>>,
}

impl DecoderHandle {
    pub fn stop(&self) {
        self.control.should_stop.store(true, Ordering::SeqCst);
    }

    pub fn join(mut self) {
        self.stop();
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for DecoderHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

pub struct DecoderControl {
    pub should_stop: AtomicBool,
    pub seek_request: AtomicU64,
    pub is_seeking: AtomicBool,
    pub seeking_state: AtomicU8,
    pub buffer_ready: AtomicBool,
    pub min_buffer_samples: AtomicU64,
    pub duration_ms: AtomicU64,
    pub event_tx: Arc<Mutex<Option<mpsc::Sender<DecoderEvent>>>>,
    pub loop_callback: Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>>,
    pub duration_callback: Arc<Mutex<Option<Box<dyn Fn(u64) + Send + Sync>>>>,
    pub output_samples: AtomicU64,
}

impl DecoderControl {
    pub fn new() -> Self {
        Self {
            should_stop: AtomicBool::new(false),
            seek_request: AtomicU64::new(0),
            is_seeking: AtomicBool::new(false),
            seeking_state: AtomicU8::new(SEEK_STATE_IDLE),
            buffer_ready: AtomicBool::new(false),
            min_buffer_samples: AtomicU64::new(8192 * 2),
            duration_ms: AtomicU64::new(0),
            event_tx: Arc::new(Mutex::new(None)),
            loop_callback: Arc::new(Mutex::new(None)),
            duration_callback: Arc::new(Mutex::new(None)),
            output_samples: AtomicU64::new(0),
        }
    }

    pub fn set_event_sender(&self, tx: mpsc::Sender<DecoderEvent>) {
        if let Ok(mut lock) = self.event_tx.lock() {
            *lock = Some(tx);
        }
    }

    pub fn request_seek(&self, position_ms: u64) {
        self.seek_request.store(position_ms, Ordering::SeqCst);
        self.is_seeking.store(true, Ordering::SeqCst);
        self.seeking_state.store(SEEK_STATE_IDLE, Ordering::SeqCst);
    }

    pub fn clear_seek(&self) {
        self.seek_request.store(0, Ordering::SeqCst);
        self.is_seeking.store(false, Ordering::SeqCst);
        self.seeking_state.store(SEEK_STATE_IDLE, Ordering::SeqCst);
        self.buffer_ready.store(false, Ordering::SeqCst);
    }

    pub fn send_event(&self, event: DecoderEvent) {
        if let Ok(tx_lock) = self.event_tx.lock() {
            if let Some(ref tx) = *tx_lock {
                let _ = tx.send(event);
            }
        }
    }

    pub fn set_duration(&self, duration_ms: u64) {
        self.duration_ms.store(duration_ms, Ordering::SeqCst);
        if let Ok(cb) = self.duration_callback.lock() {
            if let Some(callback) = cb.as_ref() {
                callback(duration_ms);
            }
        }
    }

    pub fn get_duration(&self) -> u64 {
        self.duration_ms.load(Ordering::SeqCst)
    }
}

pub fn spawn_decoder(
    path: String,
    producer: HeapProd<f32>,
    control: Arc<DecoderControl>,
    abrepeat: Arc<Mutex<crate::audio::engine::abrepeat::ABRepeat>>,
) -> DecoderHandle {
    spawn_decoder_with_sample_rate(path, producer, control, 48000, abrepeat)
}

pub fn spawn_decoder_with_sample_rate(
    path: String,
    producer: HeapProd<f32>,
    control: Arc<DecoderControl>,
    output_sample_rate: u32,
    abrepeat: Arc<Mutex<crate::audio::engine::abrepeat::ABRepeat>>,
) -> DecoderHandle {
    let control_clone = control.clone();
    let thread_handle = std::thread::Builder::new()
        .name("decoder".to_string())
        .spawn(move || decoder_loop(path, producer, control_clone, output_sample_rate, abrepeat))
        .ok();

    DecoderHandle {
        control,
        thread_handle,
    }
}

fn decoder_loop(
    path: String,
    mut producer: HeapProd<f32>,
    control: Arc<DecoderControl>,
    output_sample_rate: u32,
    abrepeat: Arc<Mutex<crate::audio::engine::abrepeat::ABRepeat>>,
) {
    loop {
        if control.should_stop.load(Ordering::SeqCst) {
            break;
        }

        ffmpeg::init().ok();
        ffmpeg::log::set_level(ffmpeg::log::Level::Error);

        let mut ictx = match input(&path) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Decoder: Failed to open: {} - {}", path, e);
                return;
            }
        };

        let duration = ictx.duration();
        let duration_ms = if duration > 0 {
            (duration as f64 / 1000.0) as u64
        } else {
            0
        };
        control.set_duration(duration_ms);

        let input_stream = match ictx.streams().best(Type::Audio) {
            Some(s) => s,
            None => {
                eprintln!("Decoder: No audio stream found in: {}", path);
                return;
            }
        };
        let stream_idx = input_stream.index();
        let time_base = input_stream.time_base();

        let context =
            match ffmpeg::codec::context::Context::from_parameters(input_stream.parameters()) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Decoder: Failed to get codec context: {}", e);
                    return;
                }
            };

        let mut decoder = match context.decoder().audio() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Decoder: Failed to create audio decoder: {}", e);
                return;
            }
        };

        unsafe {
            (*decoder.as_mut_ptr()).flags2 |= 1 << 29;
        }

        let input_rate_u32 = decoder.rate();
        let input_rate = input_rate_u32 as f64;
        let output_rate = output_sample_rate as f64;

        let input_format = decoder.format();
        let input_layout = if decoder.channel_layout().bits() != 0 {
            decoder.channel_layout()
        } else {
            ffmpeg::util::channel_layout::ChannelLayout::default(decoder.channels() as i32)
        };

        let mut format_converter = match ffmpeg::software::resampling::Context::get(
            input_format,
            input_layout,
            input_rate_u32,
            ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed),
            ffmpeg::util::channel_layout::ChannelLayout::STEREO,
            input_rate_u32,
        ) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Decoder: Failed to create resampling context: {}", e);
                return;
            }
        };

        let mut resampler = match resample::create_resampler(input_rate, output_rate) {
            Some(r) => r,
            None => {
                eprintln!("Decoder: Failed to create resampler, exiting");
                return;
            }
        };

        let mut frame = AudioFrame::empty();
        let mut interleaved_frame = AudioFrame::empty();

        let mut total_decoded_samples: u64 = 0;
        let mut eof_reached = false;
        let mut last_reported_samples: u64 = 0;

        // PRE-BUFFER: Fill ring buffer before signaling play
        // This ensures audio thread has data ready when is_playing = true
        {
            let min_buffer = control.min_buffer_samples.load(Ordering::SeqCst);
            let mut buffered: u64 = 0;

            while buffered < min_buffer {
                if control.should_stop.load(Ordering::SeqCst) {
                    return;
                }

                if control.is_seeking.load(Ordering::Acquire) {
                    break;
                }

                if let Some((stream, packet)) = ictx.packets().next() {
                    if stream.index() != stream_idx {
                        continue;
                    }

                    if let Err(_) = decoder.send_packet(&packet) {
                        continue;
                    }

                    while decoder.receive_frame(&mut frame).is_ok() {
                        if control.should_stop.load(Ordering::SeqCst) {
                            return;
                        }
                        if control.is_seeking.load(Ordering::Acquire) {
                            break;
                        }
                        if format_converter.run(&frame, &mut interleaved_frame).is_ok() {
                            let data = interleaved_frame.data(0);
                            if !data.is_empty() {
                                resample::process_frame(
                                    data,
                                    &mut resampler,
                                    &mut producer,
                                    &mut total_decoded_samples,
                                );
                                buffered = total_decoded_samples;
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            if !control.is_seeking.load(Ordering::Acquire)
                && !control.should_stop.load(Ordering::SeqCst)
            {
                control
                    .output_samples
                    .store(total_decoded_samples, Ordering::SeqCst);
                control.send_event(DecoderEvent::InitialBufferReady);
            }
        }

        loop {
            if control.should_stop.load(Ordering::SeqCst) {
                break;
            }

            if total_decoded_samples != last_reported_samples {
                control
                    .output_samples
                    .store(total_decoded_samples, Ordering::SeqCst);
                last_reported_samples = total_decoded_samples;
            }

            // Check A-B Repeat
            let current_pos_secs = total_decoded_samples as f64 / (output_sample_rate as f64 * 2.0);
            if let Ok(ab) = abrepeat.lock() {
                if let Some(seek_to) = ab.check_loop(current_pos_secs) {
                    let seek_ms = (seek_to * 1000.0) as i64;
                    control.is_seeking.store(true, Ordering::Release);
                    control.seek_request.store(seek_ms as u64, Ordering::Relaxed);
                }
            }
            

            let seek_state = control.seeking_state.load(Ordering::SeqCst);

            if control.is_seeking.load(Ordering::Acquire) && seek_state == SEEK_STATE_IDLE {
                let target_ms = control.seek_request.load(Ordering::Relaxed);

                control
                    .seeking_state
                    .store(SEEK_STATE_DECODING, Ordering::SeqCst);

                let target_samples_absolute =
                    (target_ms as f64 * output_sample_rate as f64 * 2.0 / 1000.0) as u64;
                total_decoded_samples = target_samples_absolute;

                let seek_pos = (target_ms as i64).rescale((1, 1000), time_base);

                unsafe {
                    decoder.flush();
                    ff::av_seek_frame(
                        ictx.as_mut_ptr(),
                        stream_idx as i32,
                        seek_pos,
                        ff::AVSEEK_FLAG_BACKWARD as i32,
                    );
                    decoder.flush();
                }

                resampler = match resample::create_resampler(input_rate, output_rate) {
                    Some(r) => r,
                    None => {
                        eprintln!("Decoder: Failed to create resampler on seek, exiting");
                        return;
                    }
                };

                let mut buffered = 0;
                let min = control.min_buffer_samples.load(Ordering::SeqCst);
                let mut first_packet = true;
                let seek_target_ms = target_ms;

                while buffered < min {
                    // GUARD: Stop jika thread di-stop
                    if control.should_stop.load(Ordering::SeqCst) {
                        break;
                    }

                    // GUARD: Stop jika ada seek request baru (spam seek protection)
                    let current_request = control.seek_request.load(Ordering::SeqCst);
                    if current_request != seek_target_ms {
                        break;
                    }

                    if let Some((stream, packet)) = ictx.packets().next() {
                        if stream.index() != stream_idx {
                            continue;
                        }

                        if first_packet {
                            first_packet = false;
                            if let Some(pts) = packet.pts() {
                                let actual_ms = (pts as f64 * 1000.0 * time_base.numerator() as f64
                                    / time_base.denominator() as f64)
                                    as u64;
                                let exact_samples =
                                    (actual_ms as f64 * output_sample_rate as f64 * 2.0 / 1000.0)
                                        as u64;

                                total_decoded_samples = exact_samples;
                                control.seek_request.store(actual_ms, Ordering::SeqCst);
                            }
                        }

                        while decoder.receive_frame(&mut frame).is_ok() {
                            if format_converter.run(&frame, &mut interleaved_frame).is_ok() {
                                let data = interleaved_frame.data(0);
                                if !data.is_empty() {
                                    resample::process_frame_buffered(
                                        data,
                                        &mut resampler,
                                        &mut producer,
                                        &mut total_decoded_samples,
                                        &mut buffered,
                                    );
                                }
                            }
                        }

                        let _ = decoder.send_packet(&packet);

                        while decoder.receive_frame(&mut frame).is_ok() {
                            if format_converter.run(&frame, &mut interleaved_frame).is_ok() {
                                let data = interleaved_frame.data(0);
                                if !data.is_empty() {
                                    resample::process_frame_buffered(
                                        data,
                                        &mut resampler,
                                        &mut producer,
                                        &mut total_decoded_samples,
                                        &mut buffered,
                                    );
                                }
                            }
                        }
                    }
                }

                control
                    .seeking_state
                    .store(SEEK_STATE_READY, Ordering::SeqCst);

                // TIDAK PERNAH turunkan is_seeking di sini!
                // Decoder hanya worker - ENGINE yang decide kapan audio boleh resume
                // Engine.on_buffer_ready() akan:
                //   1. Set samples_played exact
                //   2. Reset DSP
                //   3. Set seek_mode(false)
                //   4. Clear seek flags
                //   5. Set state = Playing

                control.send_event(DecoderEvent::BufferReady);
            }

            // Safety check: Jika masih dalam proses decoding/buffering untuk seek, skip loop utama
            if control.is_seeking.load(Ordering::SeqCst) {
                continue;
            }

            match ictx.packets().next() {
                Some((stream, packet)) => {
                    if stream.index() != stream_idx {
                        continue;
                    }

                    if control.should_stop.load(Ordering::Relaxed) {
                        break;
                    }

                    while decoder.receive_frame(&mut frame).is_ok() {
                        if control.should_stop.load(Ordering::Relaxed) {
                            break;
                        }
                        if format_converter.run(&frame, &mut interleaved_frame).is_ok() {
                            let data = interleaved_frame.data(0);
                            if !data.is_empty() {
                                resample::process_frame(
                                    data,
                                    &mut resampler,
                                    &mut producer,
                                    &mut total_decoded_samples,
                                );
                            }
                        }
                    }

                    if let Err(_) = decoder.send_packet(&packet) {
                        continue;
                    }

                    while decoder.receive_frame(&mut frame).is_ok() {
                        if control.should_stop.load(Ordering::Relaxed) {
                            break;
                        }
                        if format_converter.run(&frame, &mut interleaved_frame).is_ok() {
                            let data = interleaved_frame.data(0);
                            if !data.is_empty() {
                                resample::process_frame(
                                    data,
                                    &mut resampler,
                                    &mut producer,
                                    &mut total_decoded_samples,
                                );
                            }
                        }
                    }
                }
                None => {
                    eprintln!(
                        "[DECODER] True EOF reached: decoded_samples={}, duration_ms={}",
                        total_decoded_samples, duration_ms
                    );

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
                            if !data.is_empty() {
                                resample::process_frame(
                                    data,
                                    &mut resampler,
                                    &mut producer,
                                    &mut total_decoded_samples,
                                );
                            }
                        }
                    }

                    resample::drain(
                        &mut resampler,
                        &mut producer,
                        &mut total_decoded_samples,
                        &control.should_stop,
                    );

                    eof_reached = true;
                }
            }

            if eof_reached {
                let actual_output_samples = total_decoded_samples;

                println!(
                    "[DEBUG] total_decoded_samples={}, expected_ms_calc={}",
                    actual_output_samples,
                    (actual_output_samples as f64 * 1000.0) / (output_sample_rate as f64 * 2.0)
                );

                control
                    .output_samples
                    .store(actual_output_samples, Ordering::SeqCst);

                control.send_event(DecoderEvent::EndOfTrack {
                    total_samples: actual_output_samples,
                });

                return;
            }
        }
    }
}
