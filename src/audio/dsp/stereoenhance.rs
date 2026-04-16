/* --- LOONIX-TUNES src/audio/dsp/std/stdstereoenhance.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static STEREO_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static STEREO_AMOUNT: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_stereo_enabled_arc() -> &'static AtomicBool {
    STEREO_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_stereo_amount_arc() -> &'static AtomicU32 {
    STEREO_AMOUNT.get_or_init(|| AtomicU32::new(0.0_f32.to_bits()))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct StereoEnhance {}

impl StereoEnhance {
    pub fn new() -> Self {
        Self {}
    }
}

impl DspProcessor for StereoEnhance {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_stereo_enabled_arc().load(Ordering::Relaxed);
        let amount = bits_to_f32(get_stereo_amount_arc().load(Ordering::Relaxed));

        // Auto-Bypass
        if !is_on || amount < 0.01 {
            output.copy_from_slice(input);
            return;
        }

        let side_boost = 1.0 + (amount * 1.5);
        let len = input.len();
        let safe_len = len - (len % 2);

        for i in (0..safe_len).step_by(2) {
            let left = input[i];
            let right = input[i + 1];

            let mid = (left + right) * 0.5;
            let side = (left - right) * 0.5;
            let widened_side = side * side_boost;
            let new_left = mid + widened_side;
            let new_right = mid - widened_side;

            output[i] = new_left;
            output[i + 1] = new_right;
        }

        if len % 2 != 0 {
            output[safe_len] = input[safe_len];
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
