/* --- LOONIX-TUNES src/audio/dsp/reverb.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

static REVERB_ENABLED: OnceLock<AtomicBool> = OnceLock::new();
static REVERB_MODE: OnceLock<AtomicU32> = OnceLock::new();
static REVERB_AMOUNT: OnceLock<AtomicU32> = OnceLock::new();
static REVERB_ROOM_SIZE: OnceLock<AtomicU32> = OnceLock::new();
static REVERB_DAMP: OnceLock<AtomicU32> = OnceLock::new();

pub fn get_reverb_enabled_arc() -> &'static AtomicBool {
    REVERB_ENABLED.get_or_init(|| AtomicBool::new(true))
}

pub fn get_reverb_mode_arc() -> &'static AtomicU32 {
    REVERB_MODE.get_or_init(|| AtomicU32::new(1))
}

pub fn get_reverb_amount_arc() -> &'static AtomicU32 {
    REVERB_AMOUNT.get_or_init(|| AtomicU32::new(50))
}

pub fn get_reverb_room_size_arc() -> &'static AtomicU32 {
    REVERB_ROOM_SIZE.get_or_init(|| AtomicU32::new((0.5_f32).to_bits()))
}

pub fn get_reverb_damp_arc() -> &'static AtomicU32 {
    REVERB_DAMP.get_or_init(|| AtomicU32::new((0.5_f32).to_bits()))
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
    ReverbParams {
        room_size: 0.25,
        decay_time: 0.8,
        damping: 0.65,
        width: 1.1,
        wet: 0.18,
        dry: 1.0,
        predelay_ms: 8.0,
    },
    ReverbParams {
        room_size: 0.55,
        decay_time: 1.8,
        damping: 0.55,
        width: 1.3,
        wet: 0.28,
        dry: 1.0,
        predelay_ms: 18.0,
    },
    ReverbParams {
        room_size: 0.85,
        decay_time: 3.5,
        damping: 0.40,
        width: 1.5,
        wet: 0.38,
        dry: 1.0,
        predelay_ms: 35.0,
    },
];

const COMB_DELAYS: [usize; 4] = [1557, 1617, 1491, 1422];
const ALLPASS_DELAYS: [usize; 2] = [225, 556];
const FIXED_GAIN: f32 = 0.015;
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
            feedback: 0.5,
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
    fn process_stereo(&mut self, input_l: f32, input_r: f32, wet: f32, width: f32) -> (f32, f32) {
        let mono = (input_l + input_r) * FIXED_GAIN;

        self.predelay_l[self.predelay_idx] = mono;
        let delayed_mono = self.predelay_l[(self.predelay_idx + self.predelay_size) % 4096];
        self.predelay_idx = (self.predelay_idx + 1) % 4096;

        let mut out_l = 0.0f32;
        let mut out_r = 0.0f32;

        for i in 0..4 {
            out_l += self.comb_l[i].process(delayed_mono);
            out_r += self.comb_r[i].process(delayed_mono);
        }

        out_l = self.allpass_l[0].process(out_l);
        out_r = self.allpass_r[0].process(out_r);
        out_l = self.allpass_l[1].process(out_l);
        out_r = self.allpass_r[1].process(out_r);

        let wet1 = wet * 0.5;
        let wet2 = wet * 0.5 * width;

        let left = out_l * wet1 + out_r * wet2 + input_l;
        let right = out_r * wet1 + out_l * wet2 + input_r;

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

        let amount = f32::from_bits(get_reverb_amount_arc().load(Ordering::Relaxed));

        if amount < 0.5 {
            output.copy_from_slice(input);
            return;
        }

        let mode_idx = match mode {
            ReverbMode::Studio => 0,
            ReverbMode::Stage => 1,
            ReverbMode::Stadium => 2,
            ReverbMode::Off => return,
        };
        let wet = BASE_PARAMS[mode_idx].wet * (amount / 100.0);
        let width = BASE_PARAMS[mode_idx].width;

        let room = BASE_PARAMS[mode_idx].room_size * SCALE_ROOM + OFFSET_ROOM;
        let damp = BASE_PARAMS[mode_idx].damping * SCALE_DAMP;

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

            let in_l = input[i];
            let in_r = input[i + 1];

            self.predelay_l[self.predelay_idx] = (in_l + in_r) * FIXED_GAIN;
            let delayed_mono = self.predelay_l[(self.predelay_idx + self.predelay_size) % 4096];

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

            let wet1 = wet * 0.5;
            let wet2 = wet * 0.5 * width;

            let left = out_l * wet1 + out_r * wet2 + in_l;
            let right = out_r * wet1 + out_l * wet2 + in_r;

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
