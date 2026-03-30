/* --- LOONIX-TUNES src/audio/dsp/middleclarity.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/// Presence enhancer using High-Shelf filter for elegant "air" presence.
/// Corner frequency 3.2 kHz, variable gain (0-4 dB boost).
pub struct MiddleClarity {
    amount_bits: Arc<AtomicU32>,
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    sample_rate: f32,
    corner_freq: f32,
    q_factor: f32,
}

impl MiddleClarity {
    pub fn new(amount: f32) -> Self {
        let mut mc = Self {
            amount_bits: Arc::new(AtomicU32::new(amount.to_bits())),
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
            sample_rate: 48000.0,
            corner_freq: 3200.0,
            q_factor: 0.707,
        };
        mc.update_coefficients();
        mc
    }

    #[inline(always)]
    fn get_amount(&self) -> f32 {
        f32::from_bits(self.amount_bits.load(Ordering::Relaxed))
    }

    pub fn set_amount(&self, amount: f32) {
        self.amount_bits.store(amount.to_bits(), Ordering::Relaxed);
        // Note: need to call update_coefficients() after setting amount,
        // but we cannot call it here because we only have &self.
        // The caller should ensure coefficients are updated.
        // In practice, we can update coefficients in process() each time amount changes.
    }

    fn update_coefficients(&mut self) {
        let amount = self.get_amount();
        let gain_db = amount * 4.0; // 0 to 4 dB boost (tame, elegant)
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * self.corner_freq / self.sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * self.q_factor);

        // High-Shelf filter (Audio EQ Cookbook)
        let a_plus_1 = a + 1.0;
        let a_minus_1 = a - 1.0;
        let sqrt_a_2_alpha = 2.0 * a.sqrt() * alpha;

        let b0_raw = a * (a_plus_1 + a_minus_1 * cos_w0 + sqrt_a_2_alpha);
        let b1_raw = -2.0 * a * (a_minus_1 + a_plus_1 * cos_w0);
        let b2_raw = a * (a_plus_1 + a_minus_1 * cos_w0 - sqrt_a_2_alpha);
        let a0_raw = a_plus_1 - a_minus_1 * cos_w0 + sqrt_a_2_alpha;
        let a1_raw = 2.0 * (a_minus_1 - a_plus_1 * cos_w0);
        let a2_raw = a_plus_1 - a_minus_1 * cos_w0 - sqrt_a_2_alpha;

        self.b0 = b0_raw / a0_raw;
        self.b1 = b1_raw / a0_raw;
        self.b2 = b2_raw / a0_raw;
        self.a1 = a1_raw / a0_raw;
        self.a2 = a2_raw / a0_raw;
    }
}

impl DspProcessor for MiddleClarity {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        // Update coefficients if amount changed (simple check: recompute every buffer)
        self.update_coefficients();

        let len = input.len();
        for i in 0..len {
            let x = input[i];
            let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
                - self.a1 * self.y1
                - self.a2 * self.y2;

            // Shift state
            self.x2 = self.x1;
            self.x1 = x;
            self.y2 = self.y1;
            self.y1 = y;

            output[i] = y;
        }
    }

    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
