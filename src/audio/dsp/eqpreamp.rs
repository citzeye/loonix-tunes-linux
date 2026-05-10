/* --- loonixtunesv2/src/audio/dsp/eqpreamp.rs | eqpreamp --- */

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

/// Safety clipper to prevent overshoot after preamp gain.
#[inline(always)]
fn soft_clip(sample: f32) -> f32 {
    let threshold = 0.99;
    let abs_s = sample.abs();
    if abs_s <= threshold {
        sample // Bit-perfect transparency for transients
    } else {
        // Cubic clipping (more transparent than tanh for drums)
        let sign = sample.signum();
        let normalized = (abs_s - threshold) / (1.0 - threshold);
        let clipped = threshold + (1.0 - threshold) * (normalized - (normalized.powi(3) / 3.0));
        sign * clipped.clamp(0.0, 0.99)
    }
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

        // Apply gain, then soft-clip if exceeds threshold
        // No headroom restriction - let user control gain freely
        for (i, &sample) in input.iter().enumerate() {
            let amplified = sample * gain;
            // Soft clip at 0.95 to prevent harsh digital clipping
            output[i] = if amplified.abs() > 0.95 {
                soft_clip(amplified)
            } else {
                amplified
            };
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
