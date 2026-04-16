/* --- LOONIX-TUNES src/audio/dsp/biquad.rs --- */

use std::f32::consts::PI;

/// Biquad High-Pass Filter using Audio EQ Cookbook formulas (Robert Bristow-Johnson)
pub struct BiquadHpf {
    // Filter coefficients (normalized)
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    // State / Memory (delay lines for previous input and output)
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadHpf {
    pub fn new() -> Self {
        Self {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Recalculate coefficients.
    /// Call this ONLY when sample rate changes or when you want to change cutoff frequency.
    pub fn update_coefficients(&mut self, sample_rate: f32, cutoff_freq: f32, q_factor: f32) {
        // Angular frequency
        let w0 = 2.0 * PI * cutoff_freq / sample_rate;
        let alpha = w0.sin() / (2.0 * q_factor);
        let cos_w0 = w0.cos();

        // High-Pass Filter formula (Audio EQ Cookbook)
        let b0_raw = (1.0 + cos_w0) / 2.0;
        let b1_raw = -(1.0 + cos_w0);
        let b2_raw = (1.0 + cos_w0) / 2.0;
        let a0_raw = 1.0 + alpha;
        let a1_raw = -2.0 * cos_w0;
        let a2_raw = 1.0 - alpha;

        // Normalize by a0.
        // This trick DSP so that in process_sample() we don't need division operations (division is CPU heavy).
        self.b0 = b0_raw / a0_raw;
        self.b1 = b1_raw / a0_raw;
        self.b2 = b2_raw / a0_raw;
        self.a1 = a1_raw / a0_raw;
        self.a2 = a2_raw / a0_raw;
    }

    /// Process audio per sample.
    /// This function is called 44100 times per second (depending on sample rate), so must be super lightweight.
    #[inline(always)]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        // Biquad difference equation (Direct Form I)
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        // Shift memory (update state for next sample)
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    /// Reset filter state (clear delay lines)
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// Biquad Low-Shelf Filter using Audio EQ Cookbook formulas (Robert Bristow-Johnson)
pub struct BiquadLowShelf {
    // Filter coefficients (normalized)
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    // State / Memory (delay lines for previous input and output)
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadLowShelf {
    pub fn new() -> Self {
        Self {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Recalculate coefficients for low-shelf filter.
    /// gain_db: Boost/cut in dB (positive = boost, negative = cut)
    pub fn update_coefficients(
        &mut self,
        sample_rate: f32,
        cutoff_freq: f32,
        gain_db: f32,
        q_factor: f32,
    ) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * cutoff_freq / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q_factor);

        let sq_a = a.sqrt();
        let a_plus_1 = a + 1.0;
        let a_minus_1 = a - 1.0;

        // Low-Shelf formula (Audio EQ Cookbook)
        let b0_raw = a * (a_plus_1 - a_minus_1 * cos_w0 + 2.0 * sq_a * alpha);
        let b1_raw = 2.0 * a * (a_minus_1 - a_plus_1 * cos_w0);
        let b2_raw = a * (a_plus_1 - a_minus_1 * cos_w0 - 2.0 * sq_a * alpha);
        let a0_raw = a_plus_1 + a_minus_1 * cos_w0 + 2.0 * sq_a * alpha;
        let a1_raw = -2.0 * (a_minus_1 + a_plus_1 * cos_w0);
        let a2_raw = a_plus_1 + a_minus_1 * cos_w0 - 2.0 * sq_a * alpha;

        // Normalize by a0
        self.b0 = b0_raw / a0_raw;
        self.b1 = b1_raw / a0_raw;
        self.b2 = b2_raw / a0_raw;
        self.a1 = a1_raw / a0_raw;
        self.a2 = a2_raw / a0_raw;
    }

    /// Process audio per sample.
    #[inline(always)]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        // Shift memory
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    /// Reset filter state (clear delay lines)
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}
