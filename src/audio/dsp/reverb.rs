/* --- loonixtunesv2/src/audio/dsp/reverb.rs | Reverb --- */

use crate::audio::dsp::biquad::BiquadHpf;
use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static REVERB_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static REVERB_MODE: OnceLock<AtomicU32> = OnceLock::new();
static REVERB_AMOUNT: OnceLock<AtomicU32> = OnceLock::new();
static REVERB_ROOM_SIZE: OnceLock<AtomicU32> = OnceLock::new();
static REVERB_DAMP: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_reverb_enabled_arc() -> &'static AtomicBool {
    REVERB_ENABLED.get_or_init(|| AtomicBool::new(false))
}

pub fn get_reverb_mode_arc() -> &'static AtomicU32 {
    REVERB_MODE.get_or_init(|| AtomicU32::new(1))
}

pub fn get_reverb_amount_arc() -> &'static AtomicU32 {
    REVERB_AMOUNT.get_or_init(|| AtomicU32::new(30))
}

pub fn get_reverb_room_size_arc() -> &'static AtomicU32 {
    REVERB_ROOM_SIZE.get_or_init(|| AtomicU32::new(0.0_f32.to_bits()))
}

pub fn get_reverb_damp_arc() -> &'static AtomicU32 {
    REVERB_DAMP.get_or_init(|| AtomicU32::new(0.0_f32.to_bits()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReverbMode {
    Off = 0,
    Studio = 1,
    Stage = 2,
    Stadium = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct ReverbParams {
    pub room_size: f32,
    pub decay_time: f32,
    pub damping: f32,
    pub width: f32,
    pub wet: f32,
    pub dry: f32,
    pub predelay_ms: f32,
}

const BASE_PARAMS: [ReverbParams; 3] = [
    // MODE 1: STUDIO
    ReverbParams {
        room_size: 0.15,
        decay_time: 0.4,
        damping: 0.85,
        width: 0.5,
        wet: 0.30, // 30% at UI 100%
        dry: 1.0,
        predelay_ms: 12.0,
    },
    // MODE 2: STAGE
    ReverbParams {
        room_size: 0.45,
        decay_time: 1.5,
        damping: 0.50,
        width: 0.85,
        wet: 0.30, // Same as Studio
        dry: 1.0,
        predelay_ms: 25.0,
    },
    // MODE 3: STADIUM
    ReverbParams {
        room_size: 0.70,
        decay_time: 3.0,
        damping: 0.25,
        width: 1.0,
        wet: 0.30, // Same as Studio
        dry: 1.0,
        predelay_ms: 50.0,
    },
];

const COMB_DELAYS: [usize; 4] = [1557, 1617, 1491, 1422];
const ALLPASS_DELAYS: [usize; 2] = [225, 556];
const FIXED_GAIN: f32 = 0.12;
const SCALE_WET: f32 = 3.0;
const SCALE_DAMP: f32 = 0.4;
const SCALE_ROOM: f32 = 0.28;
const OFFSET_ROOM: f32 = 0.7;
const INITIAL_REVERB: f32 = 0.5;

struct CombFilter {
    buffer: Vec<f32>,
    idx: usize,
    feedback: f32,
    damp1: f32,
    damp2: f32,
    filterstore: f32,
}

impl CombFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            idx: 0,
            feedback: 0.0,
            damp1: 0.0,
            damp2: 0.0,
            filterstore: 0.0,
        }
    }

    #[inline(always)]
    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.idx];

        self.filterstore = output * self.damp2 + self.filterstore * self.damp1;
        self.filterstore = self.filterstore.abs().min(1.0) * self.filterstore.signum();

        self.buffer[self.idx] = input + self.filterstore * self.feedback;
        self.idx = (self.idx + 1) % self.buffer.len();

        output
    }

    fn set_feedback_and_damp(&mut self, room_size: f32, damp: f32) {
        self.feedback = room_size;
        self.damp1 = damp * 0.4;
        self.damp2 = 1.0 - damp * 0.4;
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.filterstore = 0.0;
    }
}

struct AllpassFilter {
    buffer: Vec<f32>,
    idx: usize,
    feedback: f32,
}

impl AllpassFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            idx: 0,
            feedback: 0.3, // Lower to reduce metallic sound
        }
    }

    #[inline(always)]
    fn process(&mut self, input: f32) -> f32 {
        let bufout = self.buffer[self.idx];
        let output = -input + bufout;
        self.buffer[self.idx] = input + bufout * self.feedback;
        self.idx = (self.idx + 1) % self.buffer.len();
        output
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
    }
}

pub struct Reverb {
    sample_rate: f32,
    comb_l: [CombFilter; 4],
    comb_r: [CombFilter; 4],
    allpass_l: [AllpassFilter; 2],
    allpass_r: [AllpassFilter; 2],
    predelay_l: Vec<f32>,
    predelay_r: Vec<f32>,
    predelay_idx: usize,
    predelay_size: usize,
    stereo_spread: f32,
    hpf: BiquadHpf,
    current_mode: ReverbMode,
}

impl Reverb {
    pub fn new() -> Self {
        let comb_l: [CombFilter; 4] = [
            CombFilter::new(COMB_DELAYS[0]),
            CombFilter::new(COMB_DELAYS[1]),
            CombFilter::new(COMB_DELAYS[2]),
            CombFilter::new(COMB_DELAYS[3]),
        ];
        let comb_r: [CombFilter; 4] = [
            CombFilter::new(COMB_DELAYS[0] + COMB_DELAYS[0] / 2),
            CombFilter::new(COMB_DELAYS[1] + COMB_DELAYS[1] / 2),
            CombFilter::new(COMB_DELAYS[2] + COMB_DELAYS[2] / 2),
            CombFilter::new(COMB_DELAYS[3] + COMB_DELAYS[3] / 2),
        ];
        let allpass_l: [AllpassFilter; 2] = [
            AllpassFilter::new(ALLPASS_DELAYS[0]),
            AllpassFilter::new(ALLPASS_DELAYS[1]),
        ];
        let allpass_r: [AllpassFilter; 2] = [
            AllpassFilter::new(ALLPASS_DELAYS[0] + ALLPASS_DELAYS[0] / 2),
            AllpassFilter::new(ALLPASS_DELAYS[1] + ALLPASS_DELAYS[1] / 2),
        ];

        Self {
            sample_rate: 48000.0,
            comb_l,
            comb_r,
            allpass_l,
            allpass_r,
            predelay_l: vec![0.0; 4096],
            predelay_r: vec![0.0; 4096],
            predelay_idx: 0,
            predelay_size: 512,
            stereo_spread: 23.0,
            hpf: {
                let mut hpf = BiquadHpf::new();
                hpf.update_coefficients(48000.0, 250.0, 0.707);
                hpf
            },
            current_mode: ReverbMode::Off,
        }
    }

    fn update_params(&mut self, mode: ReverbMode, amount: f32) {
        let mode_idx = match mode {
            ReverbMode::Studio => 0,
            ReverbMode::Stage => 1,
            ReverbMode::Stadium => 2,
            ReverbMode::Off => return,
        };

        let base = BASE_PARAMS[mode_idx];
        let room = base.room_size * SCALE_ROOM + OFFSET_ROOM;
        let damp = base.damping * SCALE_DAMP;
        let _wet = base.wet * SCALE_WET * (amount / 100.0);

        for i in 0..4 {
            self.comb_l[i].set_feedback_and_damp(room, damp);
            self.comb_r[i].set_feedback_and_damp(room - self.stereo_spread / 100.0 * 0.28, damp);
        }

        self.predelay_size = (base.predelay_ms * self.sample_rate / 1000.0) as usize;
        self.predelay_size = self.predelay_size.min(4096);

        self.allpass_l[0].feedback = INITIAL_REVERB;
        self.allpass_r[0].feedback = INITIAL_REVERB - self.stereo_spread / 200.0;
        self.allpass_l[1].feedback = INITIAL_REVERB;
        self.allpass_r[1].feedback = INITIAL_REVERB - self.stereo_spread / 200.0;
    }

    #[inline(always)]
    fn process_stereo(&mut self, input_l: f32, input_r: f32, wet: f32, _width: f32) -> (f32, f32) {
        // 🔥 TRUE STEREO: Each channel has its own predelay buffer
        self.predelay_l[self.predelay_idx] = input_l * FIXED_GAIN;

        let _delayed_l = self.predelay_l[(self.predelay_idx + 4096 - self.predelay_size) % 4096];
        self.predelay_idx = (self.predelay_idx + 1) % 4096;

        let mut out_l = 0.0f32;
        let mut out_r = 0.0f32;

        // 🔥 TRUE STEREO: Left processes left, Right processes right
        for i in 0..4 {
            out_l += self.comb_l[i].process(input_l);
            out_r += self.comb_r[i].process(input_r);
        }

        out_l = self.allpass_l[0].process(out_l);
        out_r = self.allpass_r[0].process(out_r);
        out_l = self.allpass_l[1].process(out_l);
        out_r = self.allpass_r[1].process(out_r);

        // Simple wet/dry mix
        let left = input_l + out_l * wet;
        let right = input_r + out_r * wet;

        (left, right)
    }
}

impl DspProcessor for Reverb {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let enabled = get_reverb_enabled_arc().load(Ordering::Relaxed);
        if !enabled {
            output.copy_from_slice(input);
            return;
        }

        let mode = match get_reverb_mode_arc().load(Ordering::Relaxed) {
            1 => ReverbMode::Studio,
            2 => ReverbMode::Stage,
            3 => ReverbMode::Stadium,
            _ => {
                output.copy_from_slice(input);
                return;
            }
        };

        // Read amount as integer (0-100), fallback to 30 if corrupted/uninitialized
        let amount_u = get_reverb_amount_arc().load(Ordering::Relaxed);
        let amount_u = if amount_u == 0 { 30 } else { amount_u };

        // Convert to 0.0-1.0, then multiply by 0.3 (30% ceiling)
        let amount_f = (amount_u as f32 / 100.0) * 0.3;

        // Auto-bypass if amount is near zero
        if amount_f < 0.001 {
            output.copy_from_slice(input);
            return;
        }

        let mut _new_mode = ReverbMode::Off;
        let mode_idx = match mode {
            ReverbMode::Studio => {
                _new_mode = ReverbMode::Studio;
                0
            }
            ReverbMode::Stage => {
                _new_mode = ReverbMode::Stage;
                1
            }
            ReverbMode::Stadium => {
                _new_mode = ReverbMode::Stadium;
                2
            }
            ReverbMode::Off => {
                _new_mode = ReverbMode::Off;
                return;
            }
        };

        // Only reset when mode actually changes
        if _new_mode != self.current_mode {
            self.reset();
            self.current_mode = _new_mode;
        }

        // Get mode parameters
        let base = BASE_PARAMS[mode_idx];

        // Final Wet Gain = base.wet * amount_f * SCALE_WET
        // base.wet: karakter mode (Studio/Stage/Stadium)
        // amount_f: user selection (0-30% dari 0.3 ceiling)
        // SCALE_WET: algorithm boost (3.0)
        let final_wet_gain = base.wet * amount_f * SCALE_WET;
        let width = base.width;

        let room = base.room_size * SCALE_ROOM + OFFSET_ROOM;
        let damp = base.damping * SCALE_DAMP;

        self.predelay_size = (base.predelay_ms * self.sample_rate / 1000.0) as usize;
        self.predelay_size = self.predelay_size.min(4096);

        for i in 0..4 {
            self.comb_l[i].set_feedback_and_damp(room, damp);
            self.comb_r[i].set_feedback_and_damp(room - self.stereo_spread / 100.0 * 0.28, damp);
        }

        let len = input.len();

        for i in (0..len).step_by(2) {
            if i + 1 >= len {
                output[i] = input[i];
                break;
            }

            // 1. DRY: Sinyal asli dibiarkan 100% utuh
            let in_l = input[i];
            let in_r = input[i + 1];

            // 2. WET PATH: Mono + HPF (buang bass)
            let mono_input = (in_l + in_r) * FIXED_GAIN;
            let filtered_mono = self.hpf.process_sample(mono_input);

            // 3. PREDELAY
            self.predelay_l[self.predelay_idx] = filtered_mono;
            let delayed_mono =
                self.predelay_l[(self.predelay_idx + 4096 - self.predelay_size) % 4096];
            self.predelay_idx = (self.predelay_idx + 1) % 4096;

            // 4. COMB + ALLPASS
            let mut out_l = 0.0f32;
            let mut out_r = 0.0f32;
            for c in 0..4 {
                out_l += self.comb_l[c].process(delayed_mono);
                out_r += self.comb_r[c].process(delayed_mono);
            }
            out_l = self.allpass_l[0].process(out_l);
            out_r = self.allpass_r[0].process(out_r);
            out_l = self.allpass_l[1].process(out_l);
            out_r = self.allpass_r[1].process(out_r);

            // 5. STEREO MATRIX (using final_wet_gain)
            let wet1 = final_wet_gain * (width / 2.0 + 0.5);
            let wet2 = final_wet_gain * ((1.0 - width) / 2.0);
            let wet_final_l = out_l * wet1 + out_r * wet2;
            let wet_final_r = out_r * wet1 + out_l * wet2;

            // 6. FINAL MIX: Dry + Wet
            let left = in_l + wet_final_l;
            let right = in_r + wet_final_r;

            // Soft clipper
            let left = left.abs().min(1.0) * left.signum();
            let right = right.abs().min(1.0) * right.signum();

            output[i] = left;
            output[i + 1] = right;
        }
    }

    fn reset(&mut self) {
        for c in 0..4 {
            self.comb_l[c].reset();
            self.comb_r[c].reset();
        }
        for a in 0..2 {
            self.allpass_l[a].reset();
            self.allpass_r[a].reset();
        }
        self.predelay_l.fill(0.0);
        self.predelay_r.fill(0.0);
        self.predelay_idx = 0;
        self.hpf.reset();
        self.current_mode = ReverbMode::Off;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for Reverb {
    fn default() -> Self {
        Self::new()
    }
}
