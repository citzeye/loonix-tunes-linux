/* --- LOONIX-TUNES src/audio/dsp/stereoenhance.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

static STEREO_ENHANCE_AMOUNT_ARC: OnceLock<Mutex<Option<Arc<AtomicU32>>>> = OnceLock::new();

pub fn get_stereo_enhance_arc() -> Option<Arc<AtomicU32>> {
    let guard = STEREO_ENHANCE_AMOUNT_ARC.get_or_init(|| Mutex::new(None));
    guard.lock().unwrap().clone()
}

pub fn set_stereo_enhance_arc(arc: Arc<AtomicU32>) {
    let guard = STEREO_ENHANCE_AMOUNT_ARC.get_or_init(|| Mutex::new(None));
    *guard.lock().unwrap() = Some(arc);
}

pub struct StereoEnhance {
    amount_bits: Arc<AtomicU32>,
}

impl StereoEnhance {
    pub fn new(amount: f32) -> Self {
        let arc = Arc::new(AtomicU32::new(amount.to_bits()));
        set_stereo_enhance_arc(arc.clone());
        Self { amount_bits: arc }
    }
}

impl DspProcessor for StereoEnhance {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let amount = f32::from_bits(self.amount_bits.load(Ordering::Relaxed));

        if amount <= 0.001 {
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

            output[i] = new_left.clamp(-1.0, 1.0);
            output[i + 1] = new_right.clamp(-1.0, 1.0);
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
