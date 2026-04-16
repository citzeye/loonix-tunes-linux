/* --- LOONIX-TUNES src/audio/dsp/stdeq.rs --- */

/* --- LOONIX-TUNES src/audio/dsp/eq.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};

static EQ_PREAMP_ARC: OnceLock<AtomicU32> = OnceLock::new();
static EQ_BANDS_ARC: OnceLock<Arc<[AtomicU32; 10]>> = OnceLock::new();
static EQ_ENABLED_ARC: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_eq_preamp_arc() -> &'static AtomicU32 {
    EQ_PREAMP_ARC.get_or_init(|| AtomicU32::new(0_f32.to_bits()))
}

pub fn get_eq_bands_arc() -> &'static Arc<[AtomicU32; 10]> {
    EQ_BANDS_ARC.get_or_init(|| {
        Arc::new([
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        ])
    })
}

pub fn get_eq_enabled_arc() -> &'static AtomicU32 {
    EQ_ENABLED_ARC.get_or_init(|| AtomicU32::new(1))
}

pub fn get_eq_band_arc(band_index: i32) -> Option<&'static AtomicU32> {
    if band_index >= 0 && band_index < 10 {
        let bands = get_eq_bands_arc();
        Some(&bands[band_index as usize])
    } else {
        None
    }
}

/// Biquad IIR Filter coefficients
pub struct BiquadCoeffs {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
}

impl BiquadCoeffs {
    fn new() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        }
    }

    fn set_lowshelf(&mut self, freq: f32, q: f32, gain_db: f32, sample_rate: f32) {
        let omega = 2.0 * std::f64::consts::PI * (freq as f64) / (sample_rate as f64);
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let a = 10.0f64.powf((gain_db as f64) / 40.0);
        let alpha = sin_omega / (2.0 * (q as f64));
        let beta = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_omega + beta);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_omega - beta);
        let a0 = (a + 1.0) + (a - 1.0) * cos_omega + beta;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) + (a - 1.0) * cos_omega - beta;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    fn set_highshelf(&mut self, freq: f32, q: f32, gain_db: f32, sample_rate: f32) {
        let omega = 2.0 * std::f64::consts::PI * (freq as f64) / (sample_rate as f64);
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let a = 10.0f64.powf((gain_db as f64) / 40.0);
        let alpha = sin_omega / (2.0 * (q as f64));
        let beta = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_omega + beta);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_omega);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_omega - beta);
        let a0 = (a + 1.0) - (a - 1.0) * cos_omega + beta;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_omega);
        let a2 = (a + 1.0) - (a - 1.0) * cos_omega - beta;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    fn set_peak(&mut self, freq: f32, q: f32, gain_db: f32, sample_rate: f32) {
        let omega = 2.0 * std::f64::consts::PI * (freq as f64) / (sample_rate as f64);
        let alpha = omega.sin() / (2.0 * (q as f64));
        let a = 10.0f64.powf((gain_db as f64) / 40.0);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * omega.cos();
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * omega.cos();
        let a2 = 1.0 - alpha / a;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }
}

pub struct BiquadFilter {
    coeffs: BiquadCoeffs,
    s1: [f64; 2],
    s2: [f64; 2],
}

impl BiquadFilter {
    fn new() -> Self {
        Self {
            coeffs: BiquadCoeffs::new(),
            s1: [0.0; 2],
            s2: [0.0; 2],
        }
    }

    #[inline(always)]
    pub fn process(&mut self, input: f64, ch: usize) -> f64 {
        let x = input + 1e-18; // Anti-denormal
        let y = self.coeffs.b0 * x + self.s1[ch];
        self.s1[ch] = self.coeffs.b1 * x - self.coeffs.a1 * y + self.s2[ch];
        self.s2[ch] = self.coeffs.b2 * x - self.coeffs.a2 * y;

        if !y.is_finite() {
            self.s1[ch] = 0.0;
            self.s2[ch] = 0.0;
            return input;
        }
        y
    }

    pub fn reset(&mut self) {
        self.s1 = [0.0; 2];
        self.s2 = [0.0; 2];
    }
}

pub struct EqProcessor {
    filters: Vec<BiquadFilter>,
    frequencies: [f32; 10],
    gains: [f32; 10],
    target_gains: [f32; 10],
    q_factors: [f32; 10],
    sample_rate: f32,
    is_flat: bool,
}

impl EqProcessor {
    pub fn new() -> Self {
        let frequencies = [
            31.0, 62.0, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0,
        ];
        let q_factors = [0.5, 0.5, 1.414, 1.414, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5];

        let mut filters = Vec::with_capacity(10);
        for _ in 0..10 {
            filters.push(BiquadFilter::new());
        }

        let mut eq = Self {
            filters,
            frequencies,
            gains: [0.0; 10],
            target_gains: [0.0; 10],
            q_factors,
            sample_rate: 48000.0,
            is_flat: true,
        };

        eq.update_all_filters();
        eq
    }

    pub fn with_bands(gains: [f32; 10]) -> Self {
        let mut eq = Self::new();
        for i in 0..10 {
            eq.gains[i] = gains[i];
            eq.target_gains[i] = gains[i];
        }
        eq.update_all_filters();
        eq
    }

    fn update_all_filters(&mut self) {
        for i in 0..10 {
            self.update_filter(i);
        }
    }

    fn update_filter(&mut self, index: usize) {
        let freq = self.frequencies[index];
        let q = self.q_factors[index];
        let gain = self.gains[index];
        let filter = &mut self.filters[index];

        if index <= 1 {
            filter.coeffs.set_lowshelf(freq, q, gain, self.sample_rate);
        } else if index >= 8 {
            filter.coeffs.set_highshelf(freq, q, gain, self.sample_rate);
        } else {
            filter.coeffs.set_peak(freq, q, gain, self.sample_rate);
        }
    }

    pub fn sync_from_atomics(&mut self) {
        let arc = get_eq_bands_arc();
        let mut flat_check = true;

        for i in 0..10 {
            let bits = arc[i].load(Ordering::Relaxed);
            let target = f32::from_bits(bits);
            if target.abs() > 0.1 {
                flat_check = false;
            }

            if (target - self.gains[i]).abs() > 0.001 {
                self.gains[i] = target;
                self.update_filter(i);
            }
        }
        self.is_flat = flat_check;
    }
}

impl DspProcessor for EqProcessor {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let enabled = get_eq_enabled_arc().load(Ordering::Relaxed) != 0;
        let preamp_bits = get_eq_preamp_arc().load(Ordering::Relaxed);
        let preamp_db = f32::from_bits(preamp_bits);

        let preamp_linear = 10.0f32.powf(preamp_db / 20.0);

        self.sync_from_atomics();

        if !enabled || (self.is_flat && preamp_db.abs() < 0.1) {
            output.copy_from_slice(input);
            return;
        }

        for (i, &sample) in input.iter().enumerate() {
            let ch = i % 2;

            let mut result = (sample * preamp_linear) as f64;

            for filter in self.filters.iter_mut() {
                result = filter.process(result, ch);
            }

            output[i] = result as f32;
        }
    }

    fn reset(&mut self) {
        for filter in self.filters.iter_mut() {
            filter.reset();
        }
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
