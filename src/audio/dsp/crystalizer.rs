/* --- loonixtunesv2/src/audio/dsp/crystalizer.rs | crystalizer --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static CRYSTAL_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static CRYSTAL_AMOUNT: OnceLock<AtomicU32> = OnceLock::new();
static CRYSTAL_FREQ: OnceLock<AtomicU32> = OnceLock::new();
static CRYSTAL_MAGIC_MODE: OnceLock<AtomicBool> = OnceLock::new();

pub fn get_crystal_enabled_arc() -> &'static AtomicBool {
    CRYSTAL_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_crystal_amount_arc() -> &'static AtomicU32 {
    CRYSTAL_AMOUNT.get_or_init(|| AtomicU32::new(0.30_f32.to_bits()))
}

pub fn get_crystal_freq_arc() -> &'static AtomicU32 {
    CRYSTAL_FREQ.get_or_init(|| AtomicU32::new(4000.0_f32.to_bits()))
}

pub fn get_crystal_magic_mode_arc() -> &'static AtomicBool {
    CRYSTAL_MAGIC_MODE.get_or_init(|| AtomicBool::new(false))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// Crystalizer: JetAudio+ Style Wide-Band Exciter
pub struct Crystalizer {
    current_amount: f32,
    current_freq: f32,
    sample_rate: f32,
    last_x_left: f32,
    last_y_left: f32,
    last_x_right: f32,
    last_y_right: f32,
}

impl Crystalizer {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            current_amount: 0.0,
            current_freq: 4000.0,
            sample_rate,
            last_x_left: 0.0,
            last_y_left: 0.0,
            last_x_right: 0.0,
            last_y_right: 0.0,
        }
    }
}

impl DspProcessor for Crystalizer {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_crystal_enabled_arc().load(Ordering::Relaxed);
        let crystal_raw = get_crystal_amount_arc().load(Ordering::Relaxed);
        let amount = bits_to_f32(crystal_raw);
        let freq = bits_to_f32(get_crystal_freq_arc().load(Ordering::Relaxed));

        if !is_on || amount < 0.01 {
            output.copy_from_slice(input);
            return;
        }

        if (self.current_freq - freq).abs() > 1.0 {
            self.current_freq = freq;
        }

        self.current_amount = amount;

        let len = input.len();
        let drive = 3.0;

        // Fixed alpha formula: alpha = fs / (fs + 2*PI*fc)
        let two_pi = 2.0 * std::f32::consts::PI;
        let alpha = self.sample_rate / (self.sample_rate + two_pi * self.current_freq);

        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }

            let left_in = input[i];
            let right_in = input[i + 1];

            let hp_l = alpha * (self.last_y_left + left_in - self.last_x_left);
            self.last_x_left = left_in;
            self.last_y_left = hp_l;

            let hp_r = alpha * (self.last_y_right + right_in - self.last_x_right);
            self.last_x_right = right_in;
            self.last_y_right = hp_r;

            let crystal_l = (hp_l * drive).tanh() * self.current_amount;
            let crystal_r = (hp_r * drive).tanh() * self.current_amount;

            output[i] = left_in + crystal_l;
            output[i + 1] = right_in + crystal_r;
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
