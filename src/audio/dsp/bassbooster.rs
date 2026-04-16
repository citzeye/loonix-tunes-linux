/* --- LOONIX-TUNES src/audio/dsp/std/stdbassbooster.rs --- */

use crate::audio::dsp::biquad::BiquadLowShelf;
use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static BASS_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static BASS_GAIN: OnceLock<AtomicU32> = OnceLock::new();
static BASS_FREQ: OnceLock<AtomicU32> = OnceLock::new();
static BASS_Q: OnceLock<AtomicU32> = OnceLock::new();
static BASS_MAGIC_MODE: OnceLock<AtomicBool> = OnceLock::new();

pub fn get_bass_enabled_arc() -> &'static AtomicBool {
    BASS_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_bass_gain_arc() -> &'static AtomicU32 {
    BASS_GAIN.get_or_init(|| AtomicU32::new(6.0_f32.to_bits()))
}

pub fn get_bass_freq_arc() -> &'static AtomicU32 {
    BASS_FREQ.get_or_init(|| AtomicU32::new(80.0_f32.to_bits()))
}

pub fn get_bass_q_arc() -> &'static AtomicU32 {
    BASS_Q.get_or_init(|| AtomicU32::new(0.707_f32.to_bits()))
}

pub fn get_bass_magic_mode_arc() -> &'static AtomicBool {
    BASS_MAGIC_MODE.get_or_init(|| AtomicBool::new(false))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct BassBooster {
    current_gain: f32,
    current_freq: f32,
    current_q: f32,
    left_filter: BiquadLowShelf,
    right_filter: BiquadLowShelf,
    sample_rate: f32,
}

impl BassBooster {
    pub fn new() -> Self {
        Self {
            current_gain: -1.0,
            current_freq: 0.0,
            current_q: 0.0,
            left_filter: BiquadLowShelf::new(),
            right_filter: BiquadLowShelf::new(),
            sample_rate: 48000.0,
        }
    }

    fn update_filters(&mut self, freq: f32, q: f32, gain: f32) {
        self.left_filter
            .update_coefficients(self.sample_rate, freq, gain, q);
        self.right_filter
            .update_coefficients(self.sample_rate, freq, gain, q);
    }
}

impl DspProcessor for BassBooster {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_bass_enabled_arc().load(Ordering::Relaxed);
        let gain = bits_to_f32(get_bass_gain_arc().load(Ordering::Relaxed));
        let freq = bits_to_f32(get_bass_freq_arc().load(Ordering::Relaxed));
        let q = bits_to_f32(get_bass_q_arc().load(Ordering::Relaxed));

        if !is_on || gain < 0.1 {
            output.copy_from_slice(input);
            return;
        }

        // Lazy Update: Recalculate filter only when slider moves
        if (self.current_gain - gain).abs() > 0.01
            || (self.current_freq - freq).abs() > 0.5
            || (self.current_q - q).abs() > 0.01
        {
            self.current_gain = gain;
            self.current_freq = freq;
            self.current_q = q;
            self.update_filters(freq, q, gain);
        }

        let len = input.len();

        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }
            let left = input[i];
            let right = input[i + 1];

            output[i] = self.left_filter.process_sample(left);
            output[i + 1] = self.right_filter.process_sample(right);
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
