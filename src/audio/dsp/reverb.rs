/* --- LOONIX-TUNES src/audio/dsp/reverb.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};

// Global ARC buat nyimpen Preset ID: 0=Off, 1=Stage, 2=Hall, 3=Stadium
static REVERB_PRESET_ARC: OnceLock<Arc<AtomicU32>> = OnceLock::new();

pub fn get_reverb_preset_arc() -> Arc<AtomicU32> {
    REVERB_PRESET_ARC
        .get_or_init(|| Arc::new(AtomicU32::new(0)))
        .clone()
}

static REVERB_ROOM_SIZE_ARC: OnceLock<Arc<AtomicU32>> = OnceLock::new();

pub fn get_reverb_room_size_arc() -> Arc<AtomicU32> {
    REVERB_ROOM_SIZE_ARC
        .get_or_init(|| Arc::new(AtomicU32::new(0.5_f32.to_bits())))
        .clone()
}

static REVERB_DAMP_ARC: OnceLock<Arc<AtomicU32>> = OnceLock::new();

pub fn get_reverb_damp_arc() -> Arc<AtomicU32> {
    REVERB_DAMP_ARC
        .get_or_init(|| Arc::new(AtomicU32::new(0.3_f32.to_bits())))
        .clone()
}

/// Ghost Reverb: transparent, subtle, "I Miss It" effect
pub struct Reverb {
    room_size: f32,
    damp: f32,
    wet: f32,
    dry: f32,
    makeup: f32,
    #[allow(dead_code)]
    sample_rate: f32,

    // Input filter state (low-cut / high-pass)
    last_low_cut: f32,

    // Comb filters (prime sizes scaled for 48kHz)
    comb1: Vec<f32>,
    comb2: Vec<f32>,
    comb3: Vec<f32>,
    comb4: Vec<f32>,
    comb_idx1: usize,
    comb_idx2: usize,
    comb_idx3: usize,
    comb_idx4: usize,

    // Allpass filters (prime sizes for density)
    allpass1: Vec<f32>,
    allpass2: Vec<f32>,
    allpass_idx1: usize,
    _allpass_idx2: usize,
}

impl Reverb {
    pub fn new() -> Self {
        Self {
            room_size: 0.5,
            damp: 0.3,
            wet: 0.0,
            dry: 1.0,
            makeup: 1.0,
            sample_rate: 48000.0,
            last_low_cut: 0.0,
            // Buffer Prime tetap dipakai biar gema-nya jernih (gak metallic)
            comb1: vec![0.0; 3607],
            comb2: vec![0.0; 3701],
            comb3: vec![0.0; 4001],
            comb4: vec![0.0; 4217],
            comb_idx1: 0,
            comb_idx2: 0,
            comb_idx3: 0,
            comb_idx4: 0,
            allpass1: vec![0.0; 1051],
            allpass2: vec![0.0; 941],
            allpass_idx1: 0,
            _allpass_idx2: 0,
        }
    }

    pub fn set_room_size(&mut self, size: f32) {
        self.room_size = size.clamp(0.0, 1.0);
    }

    pub fn set_damp(&mut self, damp: f32) {
        self.damp = damp.clamp(0.0, 1.0);
    }

    pub fn set_wet(&mut self, wet: f32) {
        self.wet = wet.clamp(0.0, 1.0);
    }

    fn apply_preset(&mut self, preset_id: u32) {
        match preset_id {
            1 => {
                // STAGE
                self.dry = 0.8;
                self.wet = 0.2;
                self.room_size = 0.55;
                self.damp = 0.5;
                self.makeup = 1.15; // Kompensasi drop 20%
            }
            2 => {
                // HALL
                self.dry = 0.65;
                self.wet = 0.35;
                self.room_size = 0.78;
                self.damp = 0.35;
                self.makeup = 1.35; // Kompensasi drop 35%
            }
            3 => {
                // STADIUM
                self.dry = 0.5;
                self.wet = 0.5;
                self.room_size = 0.92;
                self.damp = 0.4;
                self.makeup = 1.6; // Kompensasi drop 50%
            }
            _ => {
                self.wet = 0.0;
                self.dry = 1.0;
                self.makeup = 1.0;
            }
        }
    }
}

impl DspProcessor for Reverb {
    #[inline(always)]
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let preset_id = get_reverb_preset_arc().load(Ordering::Relaxed);

        if preset_id == 0 {
            output.copy_from_slice(input);
            return;
        }

        self.apply_preset(preset_id);

        // Override room_size and damp from atomics (UI adjustable)
        let room_size_arc = get_reverb_room_size_arc();
        let damp_arc = get_reverb_damp_arc();
        self.room_size = f32::from_bits(room_size_arc.load(Ordering::Relaxed)).clamp(0.0, 1.0);
        self.damp = f32::from_bits(damp_arc.load(Ordering::Relaxed)).clamp(0.0, 1.0);

        let feedback = self.room_size;
        let damp_val = self.damp * 0.2; // Damp halus
        let filter_alpha = 0.15; // Low-cut tipis biar gak gemuruh

        for (i, &sample) in input.iter().enumerate() {
            // 1. GENTLE LOW CUT (High Pass)
            let lp = sample * filter_alpha + self.last_low_cut * (1.0 - filter_alpha);
            self.last_low_cut = lp;
            let reverb_input = sample - lp;

            // 2. COMB FILTERS (Parallel)
            let out1 = self.comb1[self.comb_idx1];
            self.comb1[self.comb_idx1] = reverb_input + (out1 * (feedback - damp_val));
            self.comb_idx1 = (self.comb_idx1 + 1) % self.comb1.len();

            let out2 = self.comb2[self.comb_idx2];
            self.comb2[self.comb_idx2] = reverb_input + (out2 * (feedback - damp_val));
            self.comb_idx2 = (self.comb_idx2 + 1) % self.comb2.len();

            let out3 = self.comb3[self.comb_idx3];
            self.comb3[self.comb_idx3] = reverb_input + (out3 * (feedback - damp_val));
            self.comb_idx3 = (self.comb_idx3 + 1) % self.comb3.len();

            let out4 = self.comb4[self.comb_idx4];
            self.comb4[self.comb_idx4] = reverb_input + (out4 * (feedback - damp_val));
            self.comb_idx4 = (self.comb_idx4 + 1) % self.comb4.len();

            let mut reverb_out = (out1 + out2 + out3 + out4) * 0.25;

            // 3. ALLPASS (Density)
            let ap1 = self.allpass1[self.allpass_idx1];
            let ap_out1 = -reverb_out + ap1;
            self.allpass1[self.allpass_idx1] = reverb_out + (ap1 * 0.5);
            reverb_out = ap_out1;
            self.allpass_idx1 = (self.allpass_idx1 + 1) % self.allpass1.len();

            // 4. FINAL MIX DENGAN MAKEUP GAIN
            let mixed = (sample * self.dry) + (reverb_out * self.wet);
            output[i] = mixed * self.makeup;
        }
    }

    fn reset(&mut self) {
        self.comb1.fill(0.0);
        self.comb2.fill(0.0);
        self.comb3.fill(0.0);
        self.comb4.fill(0.0);
        self.allpass1.fill(0.0);
        self.allpass2.fill(0.0);
        self.last_low_cut = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
