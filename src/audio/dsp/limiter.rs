/* --- LOONIX-TUNES src/audio/dsp/std/stdlimiter.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

// Global atomics for real-time UI control (lock-free, like compressor pattern)
static LIMITER_ENABLED: OnceLock<Arc<AtomicBool>> = OnceLock::new();

pub fn get_limiter_enabled_arc() -> Arc<AtomicBool> {
    LIMITER_ENABLED
        .get_or_init(|| Arc::new(AtomicBool::new(false)))
        .clone()
}

pub struct Limiter {
    threshold_lin: f32,
    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,
}

impl Limiter {
    pub fn new() -> Self {
        let sample_rate = 48000.0;
        let attack_ms = 2.0;
        let release_ms = 50.0;

        Self {
            threshold_lin: 10.0_f32.powf(-0.5 / 20.0),
            envelope: 0.0,
            attack_coeff: (-1.0_f32 / (attack_ms * 0.001 * sample_rate)).exp(),
            release_coeff: (-1.0_f32 / (release_ms * 0.001 * sample_rate)).exp(),
        }
    }
}

impl DspProcessor for Limiter {
    #[inline(always)]
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        // Check enabled state from atomic (set by UI toggle)
        let enabled = get_limiter_enabled_arc().load(Ordering::Relaxed);
        if !enabled {
            output.copy_from_slice(input);
            return;
        }

        let safe_len = input.len() - (input.len() % 2);

        for i in (0..safe_len).step_by(2) {
            let l = input[i];
            let r = input[i + 1];

            // 1. STEREO LINKING: Cari peak paling kenceng antara Kiri & Kanan
            let peak = l.abs().max(r.abs());

            // 2. ENVELOPE FOLLOWER (Mencegah Bass Pecah)
            if peak > self.envelope {
                // Attack: Envelope naik mengejar peak
                self.envelope = peak + self.attack_coeff * (self.envelope - peak);
            } else {
                // Release: Envelope turun pelan-pelan
                self.envelope = peak + self.release_coeff * (self.envelope - peak);
            }

            // 3. CALCULATE GAIN REDUCTION
            let mut gain = 1.0;
            if self.envelope > self.threshold_lin {
                gain = self.threshold_lin / self.envelope;
            }

            // 4. APPLY GAIN with SOFT CLIPPING
            let l_limited = l * gain;
            let r_limited = r * gain;

            // Soft limiter sederhana (Hard limit di 1.0, tapi kasih margin di 0.95)
            output[i] = l_limited.clamp(-1.0, 1.0);
            output[i + 1] = r_limited.clamp(-1.0, 1.0);
        }
    }

    fn reset(&mut self) {
        self.envelope = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
