/* --- loonixtunesv2/src/core/dspconfig.rs | DSP Config --- */



use crate::audio::config::{AppConfig, DspConfig};
use crate::audio::dsp::DspSettings;
use std::sync::{Arc, Mutex};

pub struct DspConfigManager {
    pub(crate) saved_config: Option<Arc<Mutex<AppConfig>>>,
    pub(crate) is_dirty: bool,
}

impl Default for DspConfigManager {
    fn default() -> Self {
        Self {
            saved_config: None,
            is_dirty: false,
        }
    }
}

impl DspConfigManager {
    pub fn new(config: Option<Arc<Mutex<AppConfig>>>) -> Self {
        Self {
            saved_config: config,
            is_dirty: false,
        }
    }
}

impl DspConfigManager {
    pub fn apply_dsp_settings(
        &self,
        ffmpeg: &Arc<Mutex<crate::audio::engine::FfmpegEngine>>,
        settings: &DspSettings,
    ) {
        match ffmpeg.try_lock() {
            Ok(mut ff) => {
                ff.set_dsp_settings(settings.clone());
            }
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(1));
                if let Ok(mut ff) = ffmpeg.lock() {
                    ff.set_dsp_settings(settings.clone());
                }
            }
        }
    }

    pub fn get_current_dsp_settings(&self, state: &DspStateView) -> DspSettings {
        DspSettings {
            dsp_enabled: state.dsp_enabled,
            eq_enabled: state.eq_enabled,
            preamp_db: 0.0,
            bass_enabled: state.bass_active,
            bass_gain: state.bass_gain as f32,
            bass_cutoff: state.bass_cutoff as f32,
            bass_q: 0.7,
            crystal_enabled: state.crystal_active,
            crystal_amount: state.crystal_amount as f32,
            crystal_freq: 4000.0,
            surround_enabled: state.surround_active,
            surround_width: state.surround_width as f32,
            surround_room_size: 15.0,
            surround_bass_safe: true,
            mono_enabled: state.mono_active,
            mono_width: state.mono_width as f32,
            pitch_enabled: state.pitch_active,
            pitch_semitones: state.pitch_semitones as f32,
            middle_enabled: state.middle_active,
            middle_amount: state.middle_amount as f32,
            compressor_enabled: state.compressor_active,
            stereo_enabled: state.stereo_active,
            stereo_amount: state.stereo_amount as f32,
            crossfeed_enabled: state.crossfeed_active,
            crossfeed_amount: state.crossfeed_amount as f32,
            eq_bands: state.dsp_bands,
        }
    }

    pub fn save_dsp_config(&mut self, state: &DspStateView) {
        // Save user presets to dsp.json
        let dsp_config = DspConfig {
            active_preset_index: state.active_preset_index,
            user_preset_names: state.user_eq_names.clone(),
            user_preset_gains: state.user_eq_gains,
            user_preset_macro: state.user_eq_macro,
            user_fx_enabled: state.user_fx_enabled,
            user_fx_bass_enabled: state.user_fx_bass_enabled,
            user_fx_bass_gain: state.user_fx_bass_gain,
            user_fx_bass_cutoff: state.user_fx_bass_cutoff,
            user_fx_bass_mode: state.user_fx_bass_mode,
            user_fx_crystal_enabled: state.user_fx_crystal_enabled,
            user_fx_crystal_amount: state.user_fx_crystal_amount,
            user_fx_surround_enabled: state.user_fx_surround_enabled,
            user_fx_surround_width: state.user_fx_surround_width,
            user_fx_mono_enabled: state.user_fx_mono_enabled,
            user_fx_mono_width: state.user_fx_mono_width,
            user_fx_stereo_enabled: state.user_fx_stereo_enabled,
            user_fx_stereo_amount: state.user_fx_stereo_amount,
            user_fx_crossfeed_enabled: state.user_fx_crossfeed_enabled,
            user_fx_crossfeed_amount: state.user_fx_crossfeed_amount,
            user_fx_compressor_enabled: state.user_fx_compressor_enabled,
            user_fx_compressor_threshold: state.user_fx_compressor_threshold,
            user_fx_reverb_enabled: state.user_fx_reverb_enabled,
            user_fx_reverb_mode: state.user_fx_reverb_mode,
            user_fx_reverb_amount: state.user_fx_reverb_amount,
        };
        let _ = dsp_config.save();

        // DSP settings are saved ONLY to dsp.json - NOT to config.json
        // Config.json only contains app settings (volume, balance, window position, etc.)
        self.is_dirty = false;
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
}

pub struct DspStateView {
    pub dsp_enabled: bool,
    pub dsp_bands: [f32; 10],
    pub eq_enabled: bool,
    pub active_preset_index: i32,
    pub bass_active: bool,
    pub bass_gain: f64,
    pub bass_cutoff: f64,
    pub crystal_active: bool,
    pub crystal_amount: f64,
    pub crystal_frdsp: f64,
    pub surround_active: bool,
    pub surround_width: f64,
    pub mono_active: bool,
    pub mono_width: f64,
    pub pitch_active: bool,
    pub pitch_semitones: f64,
    pub middle_active: bool,
    pub middle_amount: f64,
    pub reverb_mode: u32,
    pub reverb_amount: u32,
    pub compressor_active: bool,
    pub stereo_active: bool,
    pub stereo_amount: f64,
    pub crossfeed_active: bool,
    pub crossfeed_amount: f64,
    pub user_eq_names: [String; 6],
    pub user_eq_gains: [[f32; 10]; 6],
    pub user_eq_macro: [f32; 6],
    // User FX presets saved state
    pub user_fx_enabled: [bool; 6],
    pub user_fx_bass_enabled: [bool; 6],
    pub user_fx_bass_gain: [f32; 6],
    pub user_fx_bass_cutoff: [f32; 6],
    pub user_fx_bass_mode: [i32; 6],
    pub user_fx_crystal_enabled: [bool; 6],
    pub user_fx_crystal_amount: [f32; 6],
    pub user_fx_surround_enabled: [bool; 6],
    pub user_fx_surround_width: [f32; 6],
    pub user_fx_mono_enabled: [bool; 6],
    pub user_fx_mono_width: [f32; 6],
    pub user_fx_stereo_enabled: [bool; 6],
    pub user_fx_stereo_amount: [f32; 6],
    pub user_fx_crossfeed_enabled: [bool; 6],
    pub user_fx_crossfeed_amount: [f32; 6],
    pub user_fx_compressor_enabled: [bool; 6],
    pub user_fx_compressor_threshold: [f32; 6],
    pub user_fx_reverb_enabled: [bool; 6],
    pub user_fx_reverb_mode: [i32; 6],
    pub user_fx_reverb_amount: [i32; 6],
}

impl Default for DspStateView {
    fn default() -> Self {
        Self {
            dsp_enabled: true,
            dsp_bands: [0.0; 10],
            eq_enabled: false,
            active_preset_index: -1,
            bass_active: false,
            bass_gain: 0.0,
            bass_cutoff: 180.0,
            crystal_active: false,
            crystal_amount: 0.0,
            crystal_frdsp: 4000.0,
            surround_active: false,
            surround_width: 1.8,
            mono_active: false,
            mono_width: 1.0,
            pitch_active: false,
            pitch_semitones: 0.0,
            middle_active: false,
            middle_amount: 0.0,
            reverb_mode: 0,
            reverb_amount: 30, // 30% ceiling
            compressor_active: false,
            stereo_active: false,
            stereo_amount: 0.0,
            crossfeed_active: false,
            crossfeed_amount: 0.0,
            user_eq_names: [const { String::new() }; 6],
            user_eq_gains: [[0.0; 10]; 6],
            user_eq_macro: [0.0; 6],
            user_fx_enabled: [false; 6],
            user_fx_bass_enabled: [false; 6],
            user_fx_bass_gain: [0.0; 6],
            user_fx_bass_cutoff: [180.0; 6],
            user_fx_bass_mode: [0; 6],
            user_fx_crystal_enabled: [false; 6],
            user_fx_crystal_amount: [0.0; 6],
            user_fx_surround_enabled: [false; 6],
            user_fx_surround_width: [1.5; 6],
            user_fx_mono_enabled: [false; 6],
            user_fx_mono_width: [1.0; 6],
            user_fx_stereo_enabled: [false; 6],
            user_fx_stereo_amount: [0.0; 6],
            user_fx_crossfeed_enabled: [false; 6],
            user_fx_crossfeed_amount: [0.0; 6],
            user_fx_compressor_enabled: [false; 6],
            user_fx_compressor_threshold: [-10.0; 6],
            user_fx_reverb_enabled: [false; 6],
            user_fx_reverb_mode: [1; 6],
            user_fx_reverb_amount: [50; 6],
        }
    }
}

impl DspStateView {
    pub fn from_config(_config: &AppConfig) -> Self {
        let dsp_config = DspConfig::load();
        let defaults = crate::audio::dsp::DspSettings::default();
        Self {
            dsp_enabled: defaults.dsp_enabled,
            dsp_bands: defaults.eq_bands,
            eq_enabled: defaults.eq_enabled,
            active_preset_index: dsp_config.active_preset_index,
            bass_active: defaults.bass_enabled,
            bass_gain: defaults.bass_gain as f64,
            bass_cutoff: defaults.bass_cutoff as f64,
            crystal_active: defaults.crystal_enabled,
            crystal_amount: defaults.crystal_amount as f64,
            crystal_frdsp: defaults.crystal_freq as f64,
            surround_active: defaults.surround_enabled,
            surround_width: defaults.surround_width as f64,
            mono_active: defaults.mono_enabled,
            mono_width: defaults.mono_width as f64,
            pitch_active: defaults.pitch_enabled,
            pitch_semitones: defaults.pitch_semitones as f64,
            middle_active: defaults.middle_enabled,
            middle_amount: defaults.middle_amount as f64,
            reverb_mode: 0,
            reverb_amount: 0,
            compressor_active: defaults.compressor_enabled,
            stereo_active: defaults.stereo_enabled,
            stereo_amount: defaults.stereo_amount as f64,
            crossfeed_active: defaults.crossfeed_enabled,
            crossfeed_amount: defaults.crossfeed_amount as f64,
            user_eq_names: dsp_config.user_preset_names.clone(),
            user_eq_gains: dsp_config.user_preset_gains,
            user_eq_macro: dsp_config.user_preset_macro,
            user_fx_enabled: dsp_config.user_fx_enabled,
            user_fx_bass_enabled: dsp_config.user_fx_bass_enabled,
            user_fx_bass_gain: dsp_config.user_fx_bass_gain,
            user_fx_bass_cutoff: dsp_config.user_fx_bass_cutoff,
            user_fx_bass_mode: dsp_config.user_fx_bass_mode,
            user_fx_crystal_enabled: dsp_config.user_fx_crystal_enabled,
            user_fx_crystal_amount: dsp_config.user_fx_crystal_amount,
            user_fx_surround_enabled: dsp_config.user_fx_surround_enabled,
            user_fx_surround_width: dsp_config.user_fx_surround_width,
            user_fx_mono_enabled: dsp_config.user_fx_mono_enabled,
            user_fx_mono_width: dsp_config.user_fx_mono_width,
            user_fx_stereo_enabled: dsp_config.user_fx_stereo_enabled,
            user_fx_stereo_amount: dsp_config.user_fx_stereo_amount,
            user_fx_crossfeed_enabled: dsp_config.user_fx_crossfeed_enabled,
            user_fx_crossfeed_amount: dsp_config.user_fx_crossfeed_amount,
            user_fx_compressor_enabled: dsp_config.user_fx_compressor_enabled,
            user_fx_compressor_threshold: dsp_config.user_fx_compressor_threshold,
            user_fx_reverb_enabled: dsp_config.user_fx_reverb_enabled,
            user_fx_reverb_mode: dsp_config.user_fx_reverb_mode,
            user_fx_reverb_amount: dsp_config.user_fx_reverb_amount,
        }
    }
}





