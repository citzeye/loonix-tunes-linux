/* --- LOONIX-TUNES src/audio/dsp/eq.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};

static EQ_BANDS_ARC: OnceLock<Arc<[AtomicU32; 10]>> = OnceLock::new();
static EQ_ENABLED_ARC: OnceLock<AtomicU32> = OnceLock::new();

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

static EQ_DRY_ARC: OnceLock<Arc<AtomicU32>> = OnceLock::new();
static EQ_WET_ARC: OnceLock<Arc<AtomicU32>> = OnceLock::new();

pub fn get_eq_dry_arc() -> &'static AtomicU32 {
    EQ_DRY_ARC.get_or_init(|| Arc::new(AtomicU32::new(0_f32.to_bits())))
}

pub fn get_eq_wet_arc() -> &'static AtomicU32 {
    EQ_WET_ARC.get_or_init(|| Arc::new(AtomicU32::new(100_f32.to_bits())))
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
        let a = 10.0f64.powf((gain_db as f64) / 20.0);

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

/// Biquad DF2T Architecture for absolute stability during live modulation
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
        // Anti-denormal offset
        let x = input + 1e-18;

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

        // PRO FIX: SURGICAL Q & BAXANDALL
        // Q = 0.5 (Critically Damped) untuk ujung (Lows & Highs) - Baxandall musikal
        // Q = 1.414 untuk 125Hz/250Hz - Surgical peak, tight punch, anti-mud
        let q_factors = [
            0.5,   // 31Hz   (Baxandall Low Shelf - Deep Sub)
            0.5,   // 62Hz   (Baxandall Low Shelf - Bass Body)
            1.414, // 125Hz  (Surgical Peak - TIGHT PUNCH, Anti-Pecah/Mud)
            1.414, // 250Hz  (Surgical Peak - Mud control)
            1.0,   // 500Hz  (Peaking)
            1.0,   // 1kHz   (Peaking)
            1.0,   // 2kHz   (Peaking)
            1.0,   // 4kHz   (Peaking)
            0.5,   // 8kHz   (Baxandall High Shelf)
            0.5,   // 16kHz  (Baxandall High Shelf)
        ];

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

        // Memakai Shelf untuk 2 band pertama dan 2 band terakhir
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

            // Lakukan instant update (tanpa interpolasi yang memberatkan)
            // DF2T aman untuk pergantian parameter instan
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

        let dry_bits = get_eq_dry_arc().load(Ordering::Relaxed);
        let wet_bits = get_eq_wet_arc().load(Ordering::Relaxed);
        let dry_pct = f32::from_bits(dry_bits);
        let wet_pct = f32::from_bits(wet_bits);

        let dry_f = dry_pct / 100.0;
        let wet_f = wet_pct / 100.0;

        self.sync_from_atomics();

        let bypass =
            !enabled || (self.is_flat && (dry_f).abs() < 0.001 && (wet_f - 1.0).abs() < 0.001);

        if bypass {
            output.copy_from_slice(input);
            return;
        }

        for (i, &sample) in input.iter().enumerate() {
            let ch = i % 2;
            let mut result = sample as f64;

            for (idx, filter) in self.filters.iter_mut().enumerate() {
                if self.gains[idx].abs() > 0.05 {
                    result = filter.process(result, ch);
                }
            }

            let abs_res = result.abs();
            if abs_res > 0.95 {
                let excess = abs_res - 0.95;
                let compressed_excess = excess / (1.0 + excess * 20.0);
                result = result.signum() * (0.95 + compressed_excess);
            }

            let eq_sample = result as f32;
            output[i] = (sample * dry_f) + (eq_sample * wet_f);
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
