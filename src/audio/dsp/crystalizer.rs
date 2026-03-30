/* --- LOONIX-TUNES src/audio/dsp/crystalizer.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

static CRYSTALIZER_AMOUNT_ARC: OnceLock<Mutex<Option<Arc<AtomicU32>>>> = OnceLock::new();

/// Get the global Arc<AtomicU32> for crystalizer amount (if initialized)
pub fn get_crystalizer_amount_arc() -> Option<Arc<AtomicU32>> {
    let guard = CRYSTALIZER_AMOUNT_ARC.get_or_init(|| Mutex::new(None));
    guard.lock().unwrap().clone()
}

/// Set the global Arc<AtomicU32> for crystalizer amount
pub fn set_crystalizer_amount_arc(arc: Arc<AtomicU32>) {
    let guard = CRYSTALIZER_AMOUNT_ARC.get_or_init(|| Mutex::new(None));
    *guard.lock().unwrap() = Some(arc);
}

/// Helper functions for atomic f32 storage
fn f32_to_bits(f: f32) -> u32 {
    f.to_bits()
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// Crystalizer: JetAudio+ Style Wide-Band Exciter
///
/// Algorithm:
/// 1. High-pass filter at ~2.5kHz to extract upper-mids and highs
/// 2. Harmonic synthesis using full-wave rectification
/// 3. Soft-clipper (tanh) to tame peaks
/// 4. Mix back with dry signal (40% max)
pub struct Crystalizer {
    amount_bits: Arc<AtomicU32>,
    // State for High-Pass Filter (crossover ~2.5kHz)
    last_x_left: f32,
    last_y_left: f32,
    last_x_right: f32,
    last_y_right: f32,
}

impl Crystalizer {
    pub fn new(amount: f32) -> Self {
        // Get pointer from Global ARC so UI and DSP connect to the same memory
        let shared_arc = {
            let mut guard = CRYSTALIZER_AMOUNT_ARC
                .get_or_init(|| Mutex::new(None))
                .lock()
                .unwrap();
            if let Some(existing_arc) = guard.as_ref() {
                existing_arc.store(f32_to_bits(amount.max(0.0).min(1.0)), Ordering::Relaxed);
                existing_arc.clone()
            } else {
                let new_arc = Arc::new(AtomicU32::new(f32_to_bits(amount.max(0.0).min(1.0))));
                *guard = Some(new_arc.clone());
                new_arc
            }
        };

        Self {
            amount_bits: shared_arc,
            last_x_left: 0.0,
            last_y_left: 0.0,
            last_x_right: 0.0,
            last_y_right: 0.0,
        }
    }

    /// Get a handle to the atomic amount parameter for lock-free updates from GUI thread
    pub fn get_amount_handle(&self) -> Arc<AtomicU32> {
        self.amount_bits.clone()
    }

    /// Set amount directly (for use when building DSP chain)
    pub fn set_amount(&mut self, amount: f32) {
        self.amount_bits
            .store(f32_to_bits(amount.max(0.0).min(1.0)), Ordering::Relaxed);
    }

    /// Simple 1-pole high-pass filter
    #[inline(always)]
    fn process_high_pass_left(&mut self, sample: f32) -> f32 {
        // Crossover di ~2.5kHz (Area "Tang" Kendang & Guitar Bite)
        // Alpha 0.65 di 48kHz kira-kira di area Upper-Mids
        let alpha = 0.55;
        let out = alpha * (self.last_y_left + sample - self.last_x_left);
        self.last_x_left = sample;
        self.last_y_left = out;
        out
    }

    /// Simple 1-pole high-pass filter (right channel)
    #[inline(always)]
    fn process_high_pass_right(&mut self, sample: f32) -> f32 {
        let alpha = 0.55;
        let out = alpha * (self.last_y_right + sample - self.last_x_right);
        self.last_x_right = sample;
        self.last_y_right = out;
        out
    }
}

impl DspProcessor for Crystalizer {
    #[inline(always)]
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let amount = bits_to_f32(self.amount_bits.load(Ordering::Relaxed));

        if amount <= 0.001 {
            output.copy_from_slice(input);
            return;
        }

        let len = input.len();
        // PRO TRICK: Drive multiplier biar harmoniknya beneran keluar (nggak mendem)
        let drive = 3.0;

        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }

            let left_in = input[i];
            let right_in = input[i + 1];

            // 1. Ambil sinyal Mids ke atas (2.5kHz+)
            let mids_highs_l = self.process_high_pass_left(left_in);
            let mids_highs_r = self.process_high_pass_right(right_in);

            // 2. EXCITE & LIMIT (JetAudio Style)
            // Kita kalikan drive, lalu soft-clip. Ini ngasilin "sparkle" yang stabil.
            let crystal_l = (mids_highs_l * drive).tanh() * amount;
            let crystal_r = (mids_highs_r * drive).tanh() * amount;

            // 3. MIXING
            // Sekarang kendang, gitar, dan simbal bakal kerasa "Crispy" dan loncat ke depan.
            output[i] = left_in + (crystal_l * 0.5);
            output[i + 1] = right_in + (crystal_r * 0.5);
        }
    }

    fn reset(&mut self) {
        self.last_x_left = 0.0;
        self.last_y_left = 0.0;
        self.last_x_right = 0.0;
        self.last_y_right = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
