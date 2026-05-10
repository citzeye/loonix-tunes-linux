/* --- loonixtunesv2/src/audio/dsp/surround.rs | surround --- */

use crate::audio::dsp::DspProcessor;
use crate::audio::samplerate; // Import sample rate module
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static SURROUND_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static SURROUND_WIDTH: OnceLock<AtomicU32> = OnceLock::new();
static SURROUND_BASS_SAFE: OnceLock<AtomicU32> = OnceLock::new();
static SURROUND_MAGIC_MODE: OnceLock<AtomicBool> = OnceLock::new();

pub fn get_surround_enabled_arc() -> &'static AtomicBool {
    SURROUND_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_surround_width_arc() -> &'static AtomicU32 {
    SURROUND_WIDTH.get_or_init(|| AtomicU32::new((1.0_f32).to_bits()))
}

pub fn get_surround_bass_safe_arc() -> &'static AtomicU32 {
    SURROUND_BASS_SAFE.get_or_init(|| AtomicU32::new((1.0_f32).to_bits()))
}

pub fn get_surround_magic_mode_arc() -> &'static AtomicBool {
    SURROUND_MAGIC_MODE.get_or_init(|| AtomicBool::new(false))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct SurroundProcessor {
    current_width: f32,
    hp_prev_in: f32,
    hp_prev_out: f32,
    hp_coeff: f32,
}

impl SurroundProcessor {
    pub fn new() -> Self {
        Self {
            current_width: 1.0,
            hp_prev_in: 0.0,
            hp_prev_out: 0.0,
            hp_coeff: 0.0,
        }
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
        let is_on = get_surround_enabled_arc().load(Ordering::Relaxed);
        let raw_width = bits_to_f32(get_surround_width_arc().load(Ordering::Relaxed));
        let bass_safe = bits_to_f32(get_surround_bass_safe_arc().load(Ordering::Relaxed));

        if !is_on {
            // FIX: Pastikan copy benar dengan bounds check
            if output.len() >= input.len() {
                output[..input.len()].copy_from_slice(input);
            }
            return;
        }

        // Get current rate directly
        let current_rate = samplerate::get_rate();

        // Hitung hp_coeff hanya jika rate valid (> 0.0)
        // Dan hanya jika rate berubah ATAU coeff belum pernah dihitung (0.0)
        if current_rate > 0.0 && (samplerate::consume_rate_changed() || self.hp_coeff == 0.0) {
            // Bass protection cutoff (preserve drum thump - 60Hz allows fundamental bass frequencies)
            let hp_cutoff = 60.0;
            let rc = 1.0 / (2.0 * std::f32::consts::PI * hp_cutoff);
            let dt = 1.0 / current_rate;
            self.hp_coeff = rc / (rc + dt);
        }

        // Update width once per buffer (not per sample!)
        if (raw_width - self.current_width).abs() > 0.01 {
            self.current_width = raw_width;
        }

        let len = input.len();

        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }

            let left = input[i];
            let right = input[i + 1];

            // 1. Get difference (Side signal)
            let mut side = left - right;

            // 2. Bass-safe: Filter side only to preserve bass in center
            // Only apply if hp_coeff was calculated (valid)
            if bass_safe > 0.5 && self.hp_coeff > 0.0 {
                side = self.high_pass(side);
            }

            // 3. Calculate Injection Gain (Range 1.0 - 2.0)
            // If current_width = 1.0, then injection_gain = 0.0 (Bypass!)
            // If current_width = 2.0, then injection_gain = 0.5 (Max wide)
            let injection_gain = (self.current_width - 1.0) * 0.5;

            // 4. SIDE INJECTION (Original signals untouched!)
            let l = left + (side * injection_gain);
            let r = right - (side * injection_gain);

            // 5. Hard clamp as safety net
            output[i] = l.clamp(-1.0, 1.0);
            output[i + 1] = r.clamp(-1.0, 1.0);
        }
    }

    fn reset(&mut self) {
        self.hp_prev_in = 0.0;
        self.hp_prev_out = 0.0;
        self.current_width = 1.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
