/* --- LOONIX-TUNES src/audio/dsp/std/stdstereowidth.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static MONO_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static MONO_WIDTH: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_mono_enabled_arc() -> &'static AtomicBool {
    MONO_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_mono_width_arc() -> &'static AtomicU32 {
    MONO_WIDTH.get_or_init(|| AtomicU32::new(1.0_f32.to_bits()))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct StereoWidth {}

impl StereoWidth {
    pub fn new() -> Self {
        Self {}
    }
}

impl DspProcessor for StereoWidth {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_mono_enabled_arc().load(Ordering::Relaxed);
        let width = bits_to_f32(get_mono_width_arc().load(Ordering::Relaxed));

        // Auto-Bypass - if disabled or width = 1.0 (normal stereo)
        if !is_on || (width - 1.0).abs() < 0.01 {
            output.copy_from_slice(input);
            return;
        }

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
