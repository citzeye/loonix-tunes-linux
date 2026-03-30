/* --- LOONIX-TUNES src/audio/dsp/stereowidth.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/// Constant-power Mid/Side stereo width control.
/// width = 0.0 -> summed mono (Mid only)
/// width = 1.0 -> full stereo (original)
pub struct StereoWidth {
    width_bits: Arc<AtomicU32>,
}

impl StereoWidth {
    pub fn new(width: f32) -> Self {
        Self {
            width_bits: Arc::new(AtomicU32::new(width.to_bits())),
        }
    }

    pub fn with_arc(arc: Arc<AtomicU32>) -> Self {
        Self { width_bits: arc }
    }

    #[inline(always)]
    fn get_width(&self) -> f32 {
        f32::from_bits(self.width_bits.load(Ordering::Relaxed))
    }

    pub fn set_width(&self, width: f32) {
        self.width_bits.store(width.to_bits(), Ordering::Relaxed);
    }
}

impl DspProcessor for StereoWidth {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let width = self.get_width();
        let len = input.len();

        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }
            let in_l = input[i];
            let in_r = input[i + 1];

            // Constant-power mono sum (-3 dB)
            let mono_signal = (in_l + in_r) * 0.707;

            // Width blend: 0.0 = mono, 1.0 = full stereo
            output[i] = (in_l * width) + (mono_signal * (1.0 - width));
            output[i + 1] = (in_r * width) + (mono_signal * (1.0 - width));
        }
    }

    fn reset(&mut self) {}

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
