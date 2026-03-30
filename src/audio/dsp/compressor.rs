/* --- LOONIX-TUNES src/audio/dsp/compressor.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};

// Global atomics for real-time UI control (lock-free, like crystalizer pattern)
static COMPRESSOR_ENABLED: OnceLock<Arc<AtomicBool>> = OnceLock::new();
static COMPRESSOR_THRESHOLD: OnceLock<Arc<AtomicU32>> = OnceLock::new();

pub fn get_compressor_enabled_arc() -> Arc<AtomicBool> {
    COMPRESSOR_ENABLED
        .get_or_init(|| Arc::new(AtomicBool::new(true)))
        .clone()
}

pub fn get_compressor_threshold_arc() -> Arc<AtomicU32> {
    COMPRESSOR_THRESHOLD
        .get_or_init(|| Arc::new(AtomicU32::new((-6.0_f32).to_bits())))
        .clone()
}

fn bits_to_f32(b: u32) -> f32 {
    f32::from_bits(b)
}

/// Compressor with threshold, ratio, attack, release, makeup gain
pub struct Compressor {
    threshold_db: f32,
    ratio: f32,
    attack_ms: f32,
    release_ms: f32,
    makeup_gain_db: f32,
    sample_rate: f32,
    envelope: f32,
    env_coef_attack: f32,
    env_coef_release: f32,
}

impl Compressor {
    pub fn new() -> Self {
        let mut comp = Self {
            threshold_db: -18.0,
            ratio: 4.0,
            attack_ms: 10.0,
            release_ms: 100.0,
            makeup_gain_db: 0.0,
            sample_rate: 48000.0,
            envelope: 0.0,
            env_coef_attack: 0.0,
            env_coef_release: 0.0,
        };
        comp.update_coefficients();
        comp
    }

    fn update_coefficients(&mut self) {
        self.env_coef_attack = (-1.0 / (self.attack_ms * 0.001 * self.sample_rate)).exp();
        self.env_coef_release = (-1.0 / (self.release_ms * 0.001 * self.sample_rate)).exp();
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold_db = threshold_db;
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.max(1.0);
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_ms = attack_ms.max(0.1);
        self.update_coefficients();
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.release_ms = release_ms.max(1.0);
        self.update_coefficients();
    }

    pub fn set_makeup_gain(&mut self, gain_db: f32) {
        self.makeup_gain_db = gain_db;
    }
}

impl DspProcessor for Compressor {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        // Check enabled state from atomic (set by UI toggle)
        let enabled = get_compressor_enabled_arc().load(Ordering::Relaxed);
        if !enabled {
            output.copy_from_slice(input);
            return;
        }

        // Read threshold from atomic (set by UI slider)
        let bits = get_compressor_threshold_arc().load(Ordering::Relaxed);
        self.threshold_db = bits_to_f32(bits);

        let threshold_lin = 10.0_f32.powf(self.threshold_db / 20.0);
        let makeup_lin = 10.0_f32.powf(self.makeup_gain_db / 20.0);

        for (i, &sample) in input.iter().enumerate() {
            let abs_sample = sample.abs();

            // Peak detector (envelope follower)
            if abs_sample > self.envelope {
                self.envelope = abs_sample + self.env_coef_attack * (self.envelope - abs_sample);
            } else {
                self.envelope = abs_sample + self.env_coef_release * (self.envelope - abs_sample);
            }

            // Calculate gain reduction
            let mut gain = 1.0;
            if self.envelope > threshold_lin {
                let over_threshold = self.envelope / threshold_lin;
                let gain_db = -20.0 * (1.0 - 1.0 / self.ratio) * (over_threshold.log10());
                gain = 10.0_f32.powf(gain_db / 20.0);
            }

            output[i] = sample * gain * makeup_lin;
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
