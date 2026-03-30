/* --- LOONIX-TUNES src/audio/dsp/bassbooster.rs --- */

use crate::audio::dsp::biquad::BiquadLowShelf;
use crate::audio::dsp::DspProcessor;

pub struct BassBooster {
    left_filter: BiquadLowShelf,
    right_filter: BiquadLowShelf,
    sample_rate: f32,
    cutoff_hz: f32,
    gain_db: f32,
    q_factor: f32,
}

impl BassBooster {
    pub fn new() -> Self {
        Self::with_params(48000.0, 180.0, 6.0, 0.707)
    }

    pub fn with_params(sample_rate: f32, cutoff_hz: f32, gain_db: f32, q_factor: f32) -> Self {
        let mut booster = Self {
            left_filter: BiquadLowShelf::new(),
            right_filter: BiquadLowShelf::new(),
            sample_rate,
            cutoff_hz,
            gain_db,
            q_factor,
        };
        booster.update_filters();
        booster
    }

    fn update_filters(&mut self) {
        self.left_filter.update_coefficients(
            self.sample_rate,
            self.cutoff_hz,
            self.gain_db,
            self.q_factor,
        );
        self.right_filter.update_coefficients(
            self.sample_rate,
            self.cutoff_hz,
            self.gain_db,
            self.q_factor,
        );
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        if (sample_rate - self.sample_rate).abs() > f32::EPSILON {
            self.sample_rate = sample_rate;
            self.update_filters();
        }
    }

    pub fn set_cutoff(&mut self, cutoff_hz: f32) {
        self.cutoff_hz = cutoff_hz.max(20.0).min(500.0);
        self.update_filters();
    }

    pub fn set_gain(&mut self, gain_db: f32) {
        self.gain_db = gain_db.max(-24.0).min(24.0);
        self.update_filters();
    }
}

impl DspProcessor for BassBooster {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        // Compensation gain: reduce output to prevent clipping from bass boost
        let compensation = 10.0_f32.powf(-3.0 / 20.0); // -3 dB headroom

        let len = input.len();
        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }
            let left = input[i];
            let right = input[i + 1];

            let boosted_left = self.left_filter.process_sample(left);
            let boosted_right = self.right_filter.process_sample(right);

            // Apply compensation to preserve headroom
            output[i] = boosted_left * compensation;
            output[i + 1] = boosted_right * compensation;
        }
    }

    fn reset(&mut self) {
        self.left_filter.reset();
        self.right_filter.reset();
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
