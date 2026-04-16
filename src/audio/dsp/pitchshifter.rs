/* --- LOONIX-TUNES src/audio/dsp/fx/fxpitchshifter.rs --- */
use crate::audio::dsp::rubberbandffi::{
    rubberband_available, rubberband_delete, rubberband_new, rubberband_process, rubberband_reset,
    rubberband_retrieve, rubberband_set_pitch_scale, RubberBandState, RB_OPTION_FORMANT_PRESERVED,
    RB_OPTION_PITCH_HIGH_QUALITY, RB_OPTION_PROCESS_REALTIME,
};
use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static PITCH_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static PITCH_RATIO: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_pitch_enabled_arc() -> &'static AtomicBool {
    PITCH_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_pitch_ratio_arc() -> &'static AtomicU32 {
    PITCH_RATIO.get_or_init(|| AtomicU32::new(1.0_f32.to_bits()))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct PitchShifter {
    handle: RubberBandState,
    out_fifo: Vec<f32>,
    l_in: Vec<f32>,
    r_in: Vec<f32>,
    l_out: Vec<f32>,
    r_out: Vec<f32>,
    l_out_ptr: Vec<*mut f32>,
    channels: usize,
}

unsafe impl Send for PitchShifter {}
unsafe impl Sync for PitchShifter {}

impl PitchShifter {
    pub fn new() -> Self {
        let options =
            RB_OPTION_PROCESS_REALTIME | RB_OPTION_PITCH_HIGH_QUALITY | RB_OPTION_FORMANT_PRESERVED;
        let handle = unsafe { rubberband_new(48000, 2, options, 1.0, 1.0) };

        Self {
            handle,
            out_fifo: Vec::with_capacity(16384),
            l_in: Vec::with_capacity(4096),
            r_in: Vec::with_capacity(4096),
            l_out: Vec::with_capacity(4096),
            r_out: Vec::with_capacity(4096),
            l_out_ptr: vec![std::ptr::null_mut(); 2],
            channels: 2,
        }
    }
}

impl Drop for PitchShifter {
    fn drop(&mut self) {
        unsafe { rubberband_delete(self.handle) };
    }
}

impl DspProcessor for PitchShifter {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_pitch_enabled_arc().load(Ordering::Relaxed);
        let ratio = bits_to_f32(get_pitch_ratio_arc().load(Ordering::Relaxed));

        unsafe {
            rubberband_set_pitch_scale(self.handle, ratio as f64);
        }

        // Auto-Bypass
        if !is_on || (ratio - 1.0).abs() < 0.005 {
            output.copy_from_slice(input);
            self.out_fifo.clear();
            return;
        }

        let frames = input.len() / self.channels;
        if frames == 0 {
            output.fill(0.0);
            return;
        }

        self.l_in.clear();
        self.r_in.clear();
        for chunk in input.chunks_exact(self.channels) {
            self.l_in.push(chunk[0]);
            self.r_in.push(chunk[1]);
        }

        let in_ptrs: [*const f32; 2] = [self.l_in.as_ptr(), self.r_in.as_ptr()];

        unsafe {
            rubberband_process(self.handle, in_ptrs.as_ptr(), frames as u32, 0);

            let avail = rubberband_available(self.handle) as usize;
            if avail > 0 {
                self.l_out.clear();
                self.r_out.clear();
                self.l_out.resize(avail, 0.0);
                self.r_out.resize(avail, 0.0);
                self.l_out_ptr[0] = self.l_out.as_mut_ptr();
                self.l_out_ptr[1] = self.r_out.as_mut_ptr();

                rubberband_retrieve(self.handle, self.l_out_ptr.as_mut_ptr(), avail as u32);

                for i in 0..avail {
                    self.out_fifo.push(self.l_out[i]);
                    self.out_fifo.push(self.r_out[i]);
                }
            }
        }

        let out_len = output.len();
        if self.out_fifo.len() >= out_len {
            for (i, val) in self.out_fifo.drain(0..out_len).enumerate() {
                output[i] = val;
            }
        } else {
            output.fill(0.0);
        }
    }

    fn reset(&mut self) {
        unsafe { rubberband_reset(self.handle) };
        self.out_fifo.clear();
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
