/* --- LOONIX-TUNES src/audio/dsp/middleclarity.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static MIDDLE_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static MIDDLE_AMOUNT: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_middle_enabled_arc() -> &'static AtomicBool {
    MIDDLE_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_middle_amount_arc() -> &'static AtomicU32 {
    MIDDLE_AMOUNT.get_or_init(|| AtomicU32::new(0.0_f32.to_bits()))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct MiddleClarity {
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
    pub fn new() -> Self {
        Self {
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
        }
    }

    fn update_coefficients(&mut self, amount: f32) {
        let gain_db = amount * 4.0;
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * self.corner_freq / self.sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * self.q_factor);

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
        let is_on = get_middle_enabled_arc().load(Ordering::Relaxed);
        let amount = bits_to_f32(get_middle_amount_arc().load(Ordering::Relaxed));

        // Auto-Bypass
        if !is_on || amount < 0.01 {
            output.copy_from_slice(input);
            return;
        }

        self.update_coefficients(amount);

        let len = input.len();
        for i in 0..len {
            let x = input[i];
            let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
                - self.a1 * self.y1
                - self.a2 * self.y2;

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
