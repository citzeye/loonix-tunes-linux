/* --- LOONIX-TUNES src/audio/dsp/surround.rs --- */

use crate::audio::dsp::DspProcessor;

pub struct SurroundProcessor {
    width_factor: f32,
    sample_rate: f32,
    hp_cutoff: f32,
    // High-pass filter state for side channel
    hp_prev_in: f32,
    hp_prev_out: f32,
    hp_coeff: f32,
}

impl SurroundProcessor {
    pub fn new() -> Self {
        let mut p = Self {
            width_factor: 1.8,
            sample_rate: 48000.0,
            hp_cutoff: 250.0,
            hp_prev_in: 0.0,
            hp_prev_out: 0.0,
            hp_coeff: 0.0,
        };
        p.calc_hp_coeff();
        p
    }

    pub fn with_width(width: f32) -> Self {
        let mut p = Self {
            width_factor: width.max(0.0).min(2.0),
            sample_rate: 48000.0,
            hp_cutoff: 250.0,
            hp_prev_in: 0.0,
            hp_prev_out: 0.0,
            hp_coeff: 0.0,
        };
        p.calc_hp_coeff();
        p
    }

    fn calc_hp_coeff(&mut self) {
        let rc = 1.0 / (2.0 * std::f32::consts::PI * self.hp_cutoff);
        let dt = 1.0 / self.sample_rate;
        self.hp_coeff = rc / (rc + dt);
    }

    fn high_pass(&mut self, sample: f32) -> f32 {
        let out = self.hp_coeff * (self.hp_prev_out + sample - self.hp_prev_in);
        self.hp_prev_in = sample;
        self.hp_prev_out = out;
        out
    }
}

impl DspProcessor for SurroundProcessor {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let len = input.len();
        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }
            let left_in = input[i];
            let right_in = input[i + 1];

            // Mid/Side processing
            let mid = (left_in + right_in) * 0.5;
            let side = (left_in - right_in) * 0.5;

            // High-pass side channel (only widen mid-high freqs > 250Hz)
            let side_hp = self.high_pass(side);

            // Widen stereo image (only on filtered side)
            let widened_side = side_hp * self.width_factor;

            // Recombine
            output[i] = mid + widened_side;
            output[i + 1] = mid - widened_side;
        }
    }

    fn reset(&mut self) {
        self.hp_prev_in = 0.0;
        self.hp_prev_out = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
