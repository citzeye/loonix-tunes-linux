/* --- loonixtunesv2/src/audio/dsp/mod.rs | DSP Module --- */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

static PRO_UNLOCKED: OnceLock<AtomicBool> = OnceLock::new();
static DSP_BYPASS_ENABLED: OnceLock<AtomicBool> = OnceLock::new();

pub fn get_pro_unlocked_arc() -> &'static AtomicBool {
    PRO_UNLOCKED.get_or_init(|| AtomicBool::new(false))
}

pub fn is_pro_active() -> bool {
    get_pro_unlocked_arc().load(Ordering::Relaxed)
}

pub fn get_dsp_bypass_arc() -> &'static AtomicBool {
    DSP_BYPASS_ENABLED.get_or_init(|| AtomicBool::new(true))
}

pub fn is_dsp_bypass() -> bool {
    get_dsp_bypass_arc().load(Ordering::Relaxed)
}

pub mod bassbooster;
pub mod biquad;
pub mod chain;
pub mod compressor;
pub mod crossfeed;
pub mod crystalizer;
pub mod eq;
pub mod eqpreamp;
pub mod limiter;
pub mod middleclarity;
pub mod normalizer;
pub mod pitchshifter;
pub mod preamp;
pub mod rack;
pub mod reverb;
pub mod rubberbandffi;
pub mod stereoenhance;
pub mod stereowidth;
pub mod surround;

pub use self::chain::DspChain;
pub use self::rack::DspRack;

// Re-exports with  prefix only
pub use self::bassbooster::{
    get_bass_enabled_arc, get_bass_freq_arc, get_bass_gain_arc, get_bass_q_arc, BassBooster,
};
pub use self::compressor::{get_compressor_enabled_arc, get_compressor_threshold_arc, Compressor};
pub use self::crossfeed::{get_crossfeed_amount_arc, get_crossfeed_enabled_arc, Crossfeed};
pub use self::crystalizer::{
    get_crystal_amount_arc, get_crystal_enabled_arc, get_crystal_freq_arc, Crystalizer,
};
pub use self::eq::{get_eq_band_arc, get_eq_bands_arc, get_eq_enabled_arc, EqProcessor};
pub use self::eqpreamp::{get_preamp_enabled_arc, get_preamp_gain_arc, EqPreamp};
pub use self::limiter::{get_limiter_enabled_arc, Limiter};
pub use self::middleclarity::{get_middle_amount_arc, get_middle_enabled_arc, MiddleClarity};
pub use self::normalizer::{
    get_normalizer_gain_arc, get_normalizer_smoothing_arc, AudioNormalizer,
};
pub use self::pitchshifter::{get_pitch_enabled_arc, get_pitch_ratio_arc, PitchShifter};
pub use self::reverb::{
    get_reverb_amount_arc, get_reverb_damp_arc, get_reverb_enabled_arc, get_reverb_mode_arc,
    get_reverb_room_size_arc, Reverb,
};
pub use self::stereoenhance::{get_stereo_amount_arc, get_stereo_enabled_arc, StereoEnhance};
pub use self::stereowidth::{get_mono_enabled_arc, get_mono_width_arc, StereoWidth};
pub use self::surround::{get_surround_enabled_arc, get_surround_width_arc, SurroundProcessor};

pub trait DspProcessor: Send + Sync {
    fn process(&mut self, input: &[f32], output: &mut [f32]);
    fn reset(&mut self);
    fn as_any(&mut self) -> &mut dyn std::any::Any;
    fn as_any_ref(&self) -> &dyn std::any::Any;
}

#[derive(Clone)]
pub struct DspSettings {
    pub dsp_enabled: bool,
    pub eq_enabled: bool,
    pub preamp_db: f32,
    pub bass_enabled: bool,
    pub bass_gain: f32,
    pub bass_cutoff: f32,
    pub bass_q: f32,
    pub crystal_enabled: bool,
    pub crystal_amount: f32,
    pub crystal_freq: f32,
    pub surround_enabled: bool,
    pub surround_width: f32,
    pub surround_room_size: f32,
    pub surround_bass_safe: bool,
    pub mono_enabled: bool,
    pub mono_width: f32,
    pub pitch_enabled: bool,
    pub pitch_semitones: f32,
    pub middle_enabled: bool,
    pub middle_amount: f32,
    pub compressor_enabled: bool,
    pub stereo_enabled: bool,
    pub stereo_amount: f32,
    pub crossfeed_enabled: bool,
    pub crossfeed_amount: f32,
    pub eq_bands: [f32; 10],
}

impl Default for DspSettings {
    fn default() -> Self {
        Self {
            dsp_enabled: true,
            eq_enabled: true,
            preamp_db: 0.0,
            bass_enabled: false,
            bass_gain: 6.0,
            bass_cutoff: 80.0,
            bass_q: 0.7,
            crystal_enabled: false,
            crystal_amount: 0.0,
            crystal_freq: 4000.0,
            surround_enabled: false,
            surround_width: 1.3,
            surround_room_size: 15.0,
            surround_bass_safe: true,
            mono_enabled: false,
            mono_width: 1.0,
            pitch_enabled: false,
            pitch_semitones: 0.0,
            middle_enabled: false,
            middle_amount: 0.5,
            compressor_enabled: false,
            stereo_enabled: false,
            stereo_amount: 1.0,
            crossfeed_enabled: false,
            crossfeed_amount: 0.5,
            eq_bands: [0.0; 10],
        }
    }
}

pub struct DspManager {
    dsp_rack: DspRack,
}

impl DspManager {
    pub fn new() -> Self {
        Self {
            dsp_rack: DspRack::new(),
        }
    }

    pub fn build_rack(&mut self, _is_pro: bool) {
        let settings = DspSettings::default();
        self.dsp_rack.processors = DspRack::build_processors(&settings);
    }

    pub fn update_settings(&mut self, _settings: &DspSettings) {}

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        self.dsp_rack.process(input, output);
    }
}

impl Default for DspManager {
    fn default() -> Self {
        Self::new()
    }
}
