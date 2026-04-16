/* --- LOONIX-TUNES src/audio/dsp/crossfeed.rs --- */
use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static CROSSFEED_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static CROSSFEED_AMOUNT: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_crossfeed_enabled_arc() -> &'static AtomicBool {
    CROSSFEED_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_crossfeed_amount_arc() -> &'static AtomicU32 {
    CROSSFEED_AMOUNT.get_or_init(|| AtomicU32::new(0.0_f32.to_bits()))
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub struct Crossfeed {
    delay_l: [f32; 32],
    delay_r: [f32; 32],
    write_pos: usize,
    lp_l: f32,
    lp_r: f32,
}

impl Crossfeed {
    pub fn new() -> Self {
        Self {
            delay_l: [0.0; 32],
            delay_r: [0.0; 32],
            write_pos: 0,
            lp_l: 0.0,
            lp_r: 0.0,
        }
    }
}

impl DspProcessor for Crossfeed {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let is_on = get_crossfeed_enabled_arc().load(Ordering::Relaxed);
        let amount = bits_to_f32(get_crossfeed_amount_arc().load(Ordering::Relaxed));

        // Auto-Bypass
        if !is_on || amount < 0.01 {
            output.copy_from_slice(input);
            return;
        }

        let delay_samples = 14;
        let filter_coeff = 0.15;
        let gain_reduction = 1.0 - (amount * 0.15);

        for i in (0..input.len()).step_by(2) {
            let in_l = input[i];
            let in_r = input[i + 1];

            self.delay_l[self.write_pos] = in_l;
            self.delay_r[self.write_pos] = in_r;

            let read_pos = (self.write_pos + 32 - delay_samples) % 32;
            let delayed_l = self.delay_l[read_pos];
            let delayed_r = self.delay_r[read_pos];

            self.lp_l += filter_coeff * (delayed_l - self.lp_l);
            self.lp_r += filter_coeff * (delayed_r - self.lp_r);

            output[i] = (in_l + (self.lp_r * amount * 0.8)) * gain_reduction;
            output[i + 1] = (in_r + (self.lp_l * amount * 0.8)) * gain_reduction;

            self.write_pos = (self.write_pos + 1) % 32;
        }
    }

    fn reset(&mut self) {
        self.delay_l.fill(0.0);
        self.delay_r.fill(0.0);
        self.lp_l = 0.0;
        self.lp_r = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
