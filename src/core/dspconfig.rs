/* --- LOONIX-TUNES src/core/dspconfig.rs --- */

#![allow(non_snake_case)]

use crate::audio::config::AppConfig;
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
            preamp_db: 0.0,
            bass_enabled: state.bass_magic_active,
            bass_gain: state.bass_gain as f32,
            bass_cutoff: state.bass_cutoff as f32,
            bass_q: 0.7,
            crystal_enabled: state.crystal_magic_active,
            crystal_amount: state.crystal_amount as f32,
            crystal_freq: 4000.0,
            surround_enabled: state.surround_magic_active,
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
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.dsp_enabled = state.dsp_enabled;
                cfg.eq_bands = state.dsp_bands;
                cfg.eq_enabled = state.eq_enabled;
                cfg.active_preset_index = state.active_preset_index;
                cfg.bass_enabled = state.bass_magic_active;
                cfg.bass_gain = state.bass_gain as f32;
                cfg.bass_cutoff = state.bass_cutoff as f32;
                cfg.crystal_enabled = state.crystal_magic_active;
                cfg.crystal_amount = state.crystal_amount as f32;
                cfg.crystal_freq = state.crystal_frdsp as f32;
                cfg.surround_enabled = state.surround_magic_active;
                cfg.surround_width = state.surround_width as f32;
                cfg.mono_enabled = state.mono_active;
                cfg.mono_width = state.mono_width as f32;
                cfg.pitch_enabled = state.pitch_active;
                cfg.pitch_semitones = state.pitch_semitones as f32;
                cfg.middle_enabled = state.middle_active;
                cfg.middle_amount = state.middle_amount as f32;
                cfg.reverb_preset = state.reverb_preset;
                cfg.compressor_enabled = state.compressor_active;
                let threshold_bits = crate::audio::dsp::compressor::get_compressor_threshold_arc()
                    .load(std::sync::atomic::Ordering::Relaxed);
                cfg.compressor_threshold = f32::from_bits(threshold_bits);
                cfg.stereo_enabled = state.stereo_active;
                cfg.stereo_amount = state.stereo_amount as f32;
                cfg.crossfeed_enabled = state.crossfeed_active;
                cfg.crossfeed_amount = state.crossfeed_amount as f32;
                cfg.user_preset_names = state.user_eq_names.clone();
                cfg.user_preset_gains = state.user_eq_gains;
                cfg.user_preset_macro = state.user_eq_macro;
                let _ = cfg.save();
            }
        }
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
    pub bass_magic_active: bool,
    pub bass_gain: f64,
    pub bass_cutoff: f64,
    pub crystal_magic_active: bool,
    pub crystal_amount: f64,
    pub crystal_frdsp: f64,
    pub surround_magic_active: bool,
    pub surround_width: f64,
    pub mono_active: bool,
    pub mono_width: f64,
    pub pitch_active: bool,
    pub pitch_semitones: f64,
    pub middle_active: bool,
    pub middle_amount: f64,
    pub reverb_preset: u32,
    pub compressor_active: bool,
    pub stereo_active: bool,
    pub stereo_amount: f64,
    pub crossfeed_active: bool,
    pub crossfeed_amount: f64,
    pub user_eq_names: [String; 6],
    pub user_eq_gains: [[f32; 10]; 6],
    pub user_eq_macro: [f32; 6],
}

impl Default for DspStateView {
    fn default() -> Self {
        Self {
            dsp_enabled: true,
            dsp_bands: [0.0; 10],
            eq_enabled: false,
            active_preset_index: 0,
            bass_magic_active: false,
            bass_gain: 0.0,
            bass_cutoff: 180.0,
            crystal_magic_active: false,
            crystal_amount: 0.0,
            crystal_frdsp: 4000.0,
            surround_magic_active: false,
            surround_width: 1.8,
            mono_active: false,
            mono_width: 1.0,
            pitch_active: false,
            pitch_semitones: 0.0,
            middle_active: false,
            middle_amount: 0.0,
            reverb_preset: 0,
            compressor_active: false,
            stereo_active: false,
            stereo_amount: 0.0,
            crossfeed_active: false,
            crossfeed_amount: 0.0,
            user_eq_names: [const { String::new() }; 6],
            user_eq_gains: [[0.0; 10]; 6],
            user_eq_macro: [0.0; 6],
        }
    }
}

impl DspStateView {
    pub fn from_config(config: &AppConfig) -> Self {
        Self {
            dsp_enabled: config.dsp_enabled,
            dsp_bands: config.eq_bands,
            eq_enabled: config.eq_enabled,
            active_preset_index: config.active_preset_index,
            bass_magic_active: config.bass_enabled,
            bass_gain: config.bass_gain as f64,
            bass_cutoff: config.bass_cutoff as f64,
            crystal_magic_active: config.crystal_enabled,
            crystal_amount: config.crystal_amount as f64,
            crystal_frdsp: config.crystal_freq as f64,
            surround_magic_active: config.surround_enabled,
            surround_width: config.surround_width as f64,
            mono_active: config.mono_enabled,
            mono_width: config.mono_width as f64,
            pitch_active: config.pitch_enabled,
            pitch_semitones: config.pitch_semitones as f64,
            middle_active: config.middle_enabled,
            middle_amount: config.middle_amount as f64,
            reverb_preset: config.reverb_preset,
            compressor_active: config.compressor_enabled,
            stereo_active: config.stereo_enabled,
            stereo_amount: config.stereo_amount as f64,
            crossfeed_active: config.crossfeed_enabled,
            crossfeed_amount: config.crossfeed_amount as f64,
            user_eq_names: config.user_preset_names.clone(),
            user_eq_gains: config.user_preset_gains,
            user_eq_macro: config.user_preset_macro,
        }
    }
}
