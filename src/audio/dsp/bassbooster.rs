/* --- loonixtunesv2/src/audio/dsp/bassbooster.rs | bassbooster --- */

use crate::audio::dsp::biquad::BiquadLowShelf;
use crate::audio::dsp::DspProcessor;
use crate::audio::samplerate; // Import sample rate module
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
}

impl BassBooster {
    pub fn new() -> Self {
        Self {
            current_gain: -1.0,
            current_freq: 0.0,
            current_q: 0.0,
            left_filter: BiquadLowShelf::new(),
            right_filter: BiquadLowShelf::new(),
        }
    }

    fn update_filters(&mut self, rate: f32, freq: f32, q: f32, gain: f32) {
        self.left_filter
            .update_coefficients(rate, freq, gain, q);
        self.right_filter
            .update_coefficients(rate, freq, gain, q);
    }
}

impl DspProcessor for BassBooster {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_bass_enabled_arc().load(Ordering::Relaxed);
        let gain = bits_to_f32(get_bass_gain_arc().load(Ordering::Relaxed));
        let freq = bits_to_f32(get_bass_freq_arc().load(Ordering::Relaxed));
        let q = bits_to_f32(get_bass_q_arc().load(Ordering::Relaxed));

        // 1. Check if sample rate changed from outside
        let rate_changed = samplerate::consume_rate_changed();

        if !is_on || gain < 0.1 {
            output.copy_from_slice(input);
            return;
        }

        // 2. Recalculate coefficients ONLY if rate changed OR sliders moved
        if rate_changed
            || (self.current_gain - gain).abs() > 0.01
            || (self.current_freq - freq).abs() > 0.5
            || (self.current_q - q).abs() > 0.01
        {
            self.current_gain = gain;
            self.current_freq = freq;
            self.current_q = q;

            // Get the latest f32 rate (safe and instant from AtomicU32)
            let current_rate = samplerate::get_rate();

            // Recalculate Biquad with new rate!
            self.update_filters(current_rate, freq, q, gain);
        }

        let len = input.len();

        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }

            // Apply low shelf filter directly - biquad already applies the gain
            output[i] = self.left_filter.process_sample(input[i]);
            output[i + 1] = self.right_filter.process_sample(input[i + 1]);
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
