/* --- LOONIX-TUNES src/audio/dsp/vst3processor.rs --- */

use crate::audio::dsp::DspProcessor;
use crate::audio::vsthost::{Vst3Error, Vst3Plugin};
use parking_lot::RwLock;
use std::any::Any;
use std::sync::Arc;
use vst3_sys::vst::{AudioBusBuffers, IAudioProcessor, ProcessData};
use vst3_sys::VstPtr;

const MAX_BUFFERS: usize = 8;
const MAX_SAMPLES: usize = 8192;

pub struct Vst3Processor {
    plugin: Arc<RwLock<Option<Vst3Plugin>>>,
    sample_rate: f64,
    block_size: i32,
    input_buffers: [*mut std::ffi::c_void; MAX_BUFFERS],
    output_buffers: [*mut std::ffi::c_void; MAX_BUFFERS],
    input_bus: AudioBusBuffers,
    output_bus: AudioBusBuffers,
    input_deinterleaved: Vec<f32>,
    output_deinterleaved: Vec<f32>,
}

unsafe impl Send for Vst3Processor {}
unsafe impl Sync for Vst3Processor {}

impl Clone for Vst3Processor {
    fn clone(&self) -> Self {
        let mut input_buffers = [std::ptr::null_mut(); MAX_BUFFERS];
        let mut output_buffers = [std::ptr::null_mut(); MAX_BUFFERS];

        let mut input_deinterleaved = vec![0.0f32; MAX_SAMPLES * 2];
        let mut output_deinterleaved = vec![0.0f32; MAX_SAMPLES * 2];

        let input_ptrs = input_deinterleaved.as_mut_ptr();
        let output_ptrs = output_deinterleaved.as_mut_ptr();

        for i in 0..2 {
            input_buffers[i] = unsafe {
                std::mem::transmute::<*mut f32, *mut std::ffi::c_void>(
                    input_ptrs.add(i * MAX_SAMPLES),
                )
            };
            output_buffers[i] = unsafe {
                std::mem::transmute::<*mut f32, *mut std::ffi::c_void>(
                    output_ptrs.add(i * MAX_SAMPLES),
                )
            };
        }

        let mut input_bus: AudioBusBuffers = unsafe { std::mem::zeroed() };
        input_bus.num_channels = 2;
        input_bus.silence_flags = 0;
        input_bus.buffers = input_buffers.as_mut_ptr();

        let mut output_bus: AudioBusBuffers = unsafe { std::mem::zeroed() };
        output_bus.num_channels = 2;
        output_bus.silence_flags = 0;
        output_bus.buffers = output_buffers.as_mut_ptr();

        Self {
            plugin: Arc::new(RwLock::new(None)),
            sample_rate: self.sample_rate,
            block_size: self.block_size,
            input_buffers,
            output_buffers,
            input_bus,
            output_bus,
            input_deinterleaved,
            output_deinterleaved,
        }
    }
}

impl Vst3Processor {
    pub fn new() -> Self {
        let mut input_buffers = [std::ptr::null_mut(); MAX_BUFFERS];
        let mut output_buffers = [std::ptr::null_mut(); MAX_BUFFERS];

        let mut input_deinterleaved = vec![0.0f32; MAX_SAMPLES * 2];
        let mut output_deinterleaved = vec![0.0f32; MAX_SAMPLES * 2];

        let input_ptrs = input_deinterleaved.as_mut_ptr();
        let output_ptrs = output_deinterleaved.as_mut_ptr();

        for i in 0..2 {
            input_buffers[i] = unsafe {
                std::mem::transmute::<*mut f32, *mut std::ffi::c_void>(
                    input_ptrs.add(i * MAX_SAMPLES),
                )
            };
            output_buffers[i] = unsafe {
                std::mem::transmute::<*mut f32, *mut std::ffi::c_void>(
                    output_ptrs.add(i * MAX_SAMPLES),
                )
            };
        }

        let mut input_bus: AudioBusBuffers = unsafe { std::mem::zeroed() };
        input_bus.num_channels = 2;
        input_bus.silence_flags = 0;
        input_bus.buffers = input_buffers.as_mut_ptr();

        let mut output_bus: AudioBusBuffers = unsafe { std::mem::zeroed() };
        output_bus.num_channels = 2;
        output_bus.silence_flags = 0;
        output_bus.buffers = output_buffers.as_mut_ptr();

        Self {
            plugin: Arc::new(RwLock::new(None)),
            sample_rate: 48000.0,
            block_size: 512,
            input_buffers,
            output_buffers,
            input_bus,
            output_bus,
            input_deinterleaved,
            output_deinterleaved,
        }
    }

    pub fn load_plugin<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Vst3Error> {
        let plugin = Vst3Plugin::new(path)?;
        let mut lock = self.plugin.write();
        *lock = Some(plugin);
        Ok(())
    }

    pub fn unload(&self) {
        let mut lock = self.plugin.write();
        *lock = None;
    }

    pub fn is_loaded(&self) -> bool {
        self.plugin.read().is_some()
    }

    pub fn get_loaded_path(&self) -> Option<String> {
        self.plugin.read().as_ref().map(|p| p.path.clone())
    }

    #[cfg(target_os = "linux")]
    pub fn open_editor(&self, window_id: u32) {
        let plugin_lock = self.plugin.read();
        if let Some(ref plugin) = *plugin_lock {
            if let Err(e) = plugin.open_editor(window_id) {
                eprintln!("[VST3] Gagal buka UI: {}", e);
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn open_editor(&self, _window_id: u32) {
        eprintln!("[VST3] Editor not supported on this platform");
    }

    pub fn set_sample_rate(&mut self, rate: f64) {
        self.sample_rate = rate;
        let mut lock = self.plugin.write();
        if let Some(ref mut plugin) = *lock {
            let _ = plugin.set_sample_rate(rate);
        }
    }

    fn deinterleave(&mut self, interleaved: &[f32], num_frames: usize) {
        let num_channels = 2usize;
        for frame in 0..num_frames {
            let src_idx = frame * num_channels;
            if src_idx < interleaved.len() && src_idx + 1 < interleaved.len() {
                self.input_deinterleaved[frame] = interleaved[src_idx];
                self.input_deinterleaved[MAX_SAMPLES + frame] = interleaved[src_idx + 1];
            }
        }
    }

    fn interleave(&mut self, output: &mut [f32], num_frames: usize) {
        let num_channels = 2usize;
        for frame in 0..num_frames {
            let dst_idx = frame * num_channels;
            if dst_idx < output.len() && dst_idx + 1 < output.len() {
                output[dst_idx] = self.output_deinterleaved[frame];
                output[dst_idx + 1] = self.output_deinterleaved[MAX_SAMPLES + frame];
            }
        }
    }

    fn process_internal(&mut self, input: &[f32], output: &mut [f32], num_frames: usize) {
        let (ap, needs_copy) = {
            let plugin_guard = match self.plugin.try_read() {
                Some(guard) => guard,
                None => return,
            };

            let plugin = match plugin_guard.as_ref() {
                Some(p) => p,
                None => return,
            };

            match &plugin.audio_processor {
                Some(audio_processor) => (Some(audio_processor.clone()), false),
                None => (None, true),
            }
        };

        if needs_copy {
            output[..input.len()].copy_from_slice(input);
            return;
        }

        let ap = match ap {
            Some(a) => a,
            None => {
                output[..input.len()].copy_from_slice(input);
                return;
            }
        };

        self.deinterleave(input, num_frames);

        let mut process_data: ProcessData = unsafe { std::mem::zeroed() };
        process_data.process_mode = 0;
        process_data.symbolic_sample_size = 0;
        process_data.num_samples = num_frames as i32;
        process_data.num_inputs = 1;
        process_data.num_outputs = 1;
        process_data.inputs = &mut self.input_bus;
        process_data.outputs = &mut self.output_bus;

        let result = unsafe { ap.process(&mut process_data) };
        if result != 0 {
            eprintln!("[VST3] Process error: {}", result);
        }

        self.interleave(output, num_frames);
    }
}

impl DspProcessor for Vst3Processor {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let num_frames = input.len() / 2;

        if num_frames == 0 {
            return;
        }

        if num_frames > MAX_SAMPLES {
            output[..input.len()].copy_from_slice(input);
            return;
        }

        self.process_internal(input, output, num_frames);
    }

    fn reset(&mut self) {
        for sample in self.input_deinterleaved.iter_mut() {
            *sample = 0.0;
        }
        for sample in self.output_deinterleaved.iter_mut() {
            *sample = 0.0;
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

impl Default for Vst3Processor {
    fn default() -> Self {
        Self::new()
    }
}
