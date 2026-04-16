/* --- LOONIX-TUNES src/audio/dsp/stdeqpreamp.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static PREAMP_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static PREAMP_GAIN: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_preamp_enabled_arc() -> &'static AtomicBool {
    PREAMP_ENABLED.get_or_init(|| AtomicBool::new(true))
}

pub fn get_preamp_gain_arc() -> &'static AtomicU32 {
    PREAMP_GAIN.get_or_init(|| AtomicU32::new(1.0_f32.to_bits()))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct EqPreamp {}

impl EqPreamp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EqPreamp {
    fn default() -> Self {
        Self::new()
    }
}

impl DspProcessor for EqPreamp {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_preamp_enabled_arc().load(Ordering::Relaxed);
        let gain = bits_to_f32(get_preamp_gain_arc().load(Ordering::Relaxed));

        // Auto-Bypass
        if !is_on || (gain - 1.0).abs() < f32::EPSILON {
            output.copy_from_slice(input);
            return;
        }

        for (i, &sample) in input.iter().enumerate() {
            output[i] = sample * gain;
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
