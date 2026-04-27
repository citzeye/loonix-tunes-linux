/* --- loonixtunesv2/src/audio/dsp/surround.rs | surround --- */

use crate::audio::dsp::DspProcessor;
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
    SURROUND_WIDTH.get_or_init(|| AtomicU32::new(1.3_f32.to_bits()))
}

pub fn get_surround_bass_safe_arc() -> &'static AtomicU32 {
    SURROUND_BASS_SAFE.get_or_init(|| AtomicU32::new(1.0_f32.to_bits()))
}

pub fn get_surround_magic_mode_arc() -> &'static AtomicBool {
    SURROUND_MAGIC_MODE.get_or_init(|| AtomicBool::new(false))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct SurroundProcessor {
    current_width: f32,
    current_bass_safe: f32,
    hp_prev_in: f32,
    hp_prev_out: f32,
    hp_coeff: f32,
}

impl SurroundProcessor {
    pub fn new() -> Self {
        let hp_cutoff = 250.0;
        let sample_rate = 48000.0;
        let rc = 1.0 / (2.0 * std::f32::consts::PI * hp_cutoff);
        let dt = 1.0 / sample_rate;
        let hp_coeff = rc / (rc + dt);
        Self {
            current_width: 1.0,
            current_bass_safe: 1.0,
            hp_prev_in: 0.0,
            hp_prev_out: 0.0,
            hp_coeff,
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
        let target_width = bits_to_f32(get_surround_width_arc().load(Ordering::Relaxed));
        let bass_safe_val = bits_to_f32(get_surround_bass_safe_arc().load(Ordering::Relaxed));

        self.current_width = target_width * 2.0;
        self.current_bass_safe = bass_safe_val;

        if !is_on || (target_width - 0.5).abs() < 0.01 {
            output.copy_from_slice(input);
            return;
        }

        let len = input.len();
        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }

            let left_in = input[i];
            let right_in = input[i + 1];

            let mid = (left_in + right_in) * 0.5;
            let side = (left_in - right_in) * 0.5;

            let side_filtered = if self.current_bass_safe > 0.5 {
                self.high_pass(side)
            } else {
                side
            };

            let widened_side = side_filtered * self.current_width;

            output[i] = (mid + widened_side).clamp(-1.0, 1.0);
            output[i + 1] = (mid - widened_side).clamp(-1.0, 1.0);
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