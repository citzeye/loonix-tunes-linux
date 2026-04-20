/* --- LOONIX-TUNES src/audio/config.rs --- */
use crate::audio::presets::{EQ_PRESETS, FX_PRESETS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_APP_NAME: &str = "loonix-tunes";

#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    ParseError(String),
    IoError(String),
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::NotFound {
            ConfigError::NotFound
        } else {
            ConfigError::IoError(e.to_string())
        }
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
        ConfigError::ParseError(e.to_string())
    }
}

pub fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join(CONFIG_APP_NAME))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DspConfig {
    pub user_preset_names: [String; 6],
    pub user_preset_gains: [[f32; 10]; 6],
    pub user_preset_macro: [f32; 6],
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

impl DspConfig {
    pub fn load() -> Self {
        match Self::load_dsp_config() {
            Ok(cfg) => cfg,
            Err(_) => Self::dsp_user_template(),
        }
    }

    fn load_dsp_config() -> Result<Self, ConfigError> {
        let path = Self::dsp_path().ok_or(ConfigError::NotFound)?;
        let content = fs::read_to_string(&path)?;
        let config: DspConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn dsp_user_template() -> Self {
        Self {
            user_preset_names: [
                "User 1".into(),
                "User 2".into(),
                "User 3".into(),
                "User 4".into(),
                "User 5".into(),
                "User 6".into(),
            ],
            user_preset_gains: [[0.0; 10]; 6],
            user_preset_macro: [0.0; 6],
            user_fx_enabled: [false; 6],
            user_fx_bass_enabled: [false; 6],
            user_fx_bass_gain: [0.0; 6],
            user_fx_bass_cutoff: [0.0; 6],
            user_fx_bass_mode: [0; 6],
            user_fx_crystal_enabled: [false; 6],
            user_fx_crystal_amount: [0.0; 6],
            user_fx_surround_enabled: [false; 6],
            user_fx_surround_width: [0.0; 6],
            user_fx_mono_enabled: [false; 6],
            user_fx_mono_width: [0.0; 6],
            user_fx_stereo_enabled: [false; 6],
            user_fx_stereo_amount: [0.0; 6],
            user_fx_crossfeed_enabled: [false; 6],
            user_fx_crossfeed_amount: [0.0; 6],
            user_fx_compressor_enabled: [false; 6],
            user_fx_compressor_threshold: [0.0; 6],
            user_fx_reverb_enabled: [false; 6],
            user_fx_reverb_mode: [0; 6],
            user_fx_reverb_amount: [0; 6],
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::dsp_path().ok_or(ConfigError::IoError("Invalid path".into()))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp_path = path.with_extension("tmp");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&temp_path, json)?;
        fs::rename(&temp_path, &path)?;
        Ok(())
    }

    fn dsp_path() -> Option<PathBuf> {
        config_dir().map(|p| p.join("dsp.json"))
    }
}

impl Default for DspConfig {
    fn default() -> Self {
        Self {
            user_preset_names: [
                "User 1".into(),
                "User 2".into(),
                "User 3".into(),
                "User 4".into(),
                "User 5".into(),
                "User 6".into(),
            ],
            user_preset_gains: [[0.0; 10]; 6],
            user_preset_macro: [0.0; 6],
            user_fx_enabled: [false; 6],
            user_fx_bass_enabled: [false; 6],
            user_fx_bass_gain: [6.0; 6],
            user_fx_bass_cutoff: [180.0; 6],
            user_fx_bass_mode: [0; 6],
            user_fx_crystal_enabled: [false; 6],
            user_fx_crystal_amount: [0.20; 6],
            user_fx_surround_enabled: [false; 6],
            user_fx_surround_width: [1.8; 6],
            user_fx_mono_enabled: [false; 6],
            user_fx_mono_width: [1.0; 6],
            user_fx_stereo_enabled: [false; 6],
            user_fx_stereo_amount: [0.0; 6],
            user_fx_crossfeed_enabled: [false; 6],
            user_fx_crossfeed_amount: [0.0; 6],
            user_fx_compressor_enabled: [true; 6],
            user_fx_compressor_threshold: [-6.0; 6],
            user_fx_reverb_enabled: [false; 6],
            user_fx_reverb_mode: [0; 6],
            user_fx_reverb_amount: [0; 6],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub eq_bands: [f32; 10],
    pub volume: f64,
    pub balance: f64,
    pub shuffle: bool,
    pub loop_playlist: bool,
    pub custom_folders: Vec<(String, String)>,
    pub favorites: Vec<(String, String)>,
    pub locked_folders: Vec<i32>,
    pub mode: crate::audio::engine::OutputMode,
    pub last_track_path: String,
    // Window position
    pub window_x: i32,
    pub window_y: i32,
    pub window_width: i32,
    pub window_height: i32,
    // DSP settings
    pub dsp_enabled: bool,
    pub bass_enabled: bool,
    pub bass_gain: f32,
    #[serde(default = "default_bass_cutoff")]
    pub bass_cutoff: f32,
    #[serde(default)]
    pub bass_mode: i32,
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
    pub reverb_enabled: bool,
    pub reverb_mode: i32,   // 0=Off, 1=Studio, 2=Stage, 3=Stadium
    pub reverb_amount: i32, // 0-100 percentage
    pub compressor_enabled: bool,
    pub compressor_threshold: f32,
    pub stereo_enabled: bool,
    pub stereo_amount: f32,
    pub preamp_db: f32,
    pub crossfeed_enabled: bool,
    pub crossfeed_amount: f32,
    pub eq_enabled: bool,
    pub normalizer_enabled: bool,
    #[serde(default = "default_norm_target_lufs")]
    pub normalizer_target_lufs: f32,
    #[serde(default = "default_norm_true_peak")]
    pub normalizer_true_peak_dbtp: f32,
    #[serde(default = "default_norm_max_gain")]
    pub normalizer_max_gain_db: f32,
    #[serde(default = "default_norm_smoothing")]
    pub normalizer_smoothing: f32,
    pub active_preset_index: i32,
}

fn default_bass_cutoff() -> f32 {
    180.0
}
fn default_norm_target_lufs() -> f32 {
    -14.0
}
fn default_norm_true_peak() -> f32 {
    -1.5
}
fn default_norm_max_gain() -> f32 {
    12.0
}
fn default_norm_smoothing() -> f32 {
    0.002
}

impl Default for AppConfig {
    fn default() -> Self {
        let p = &FX_PRESETS[0]; // LOONIX - auto from presets.rs
        Self {
            eq_bands: EQ_PRESETS[0].gains,
            volume: 0.2,
            balance: 0.0,
            shuffle: false,
            loop_playlist: false,
            custom_folders: vec![],
            favorites: vec![],
            locked_folders: vec![],
            mode: crate::audio::engine::OutputMode::Stereo,
            last_track_path: String::new(),
            window_x: -1,
            window_y: -1,
            window_width: 350,
            window_height: 700,
            dsp_enabled: true,
            bass_enabled: p.bass_enabled,
            bass_gain: p.bass_gain,
            bass_cutoff: p.bass_cutoff,
            bass_mode: p.bass_mode,
            bass_q: 0.7,
            crystal_enabled: p.crystal_enabled,
            crystal_amount: p.crystal_amount,
            crystal_freq: p.crystal_freq,
            surround_enabled: p.surround_enabled,
            surround_width: p.surround_width,
            surround_room_size: 15.0,
            surround_bass_safe: true,
            mono_enabled: p.mono_enabled,
            mono_width: p.mono_width,
            pitch_enabled: p.pitch_enabled,
            pitch_semitones: p.pitch_semitones,
            middle_enabled: p.middle_enabled,
            middle_amount: p.middle_amount,
            reverb_enabled: p.reverb_enabled,
            reverb_mode: p.reverb_mode,
            reverb_amount: p.reverb_amount,
            compressor_enabled: p.compressor_enabled,
            compressor_threshold: p.compressor_threshold,
            stereo_enabled: p.stereo_enabled,
            stereo_amount: p.stereo_amount,
            preamp_db: 0.0,
            crossfeed_enabled: p.crossfeed_enabled,
            crossfeed_amount: p.crossfeed_amount,
            eq_enabled: true,
            normalizer_enabled: true,
            normalizer_target_lufs: default_norm_target_lufs(),
            normalizer_true_peak_dbtp: default_norm_true_peak(),
            normalizer_max_gain_db: default_norm_max_gain(),
            normalizer_smoothing: default_norm_smoothing(),
            active_preset_index: 0,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        match Self::load_user_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("[Config] Using defaults: {:?}", e);
                Self::default()
            }
        }
    }

    fn load_user_config() -> Result<Self, ConfigError> {
        let path = Self::config_path().ok_or(ConfigError::NotFound)?;

        let content = fs::read_to_string(&path)?;
        let config: AppConfig = serde_json::from_str(&content)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::config_path().ok_or(ConfigError::IoError("Invalid path".into()))?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let temp_path = path.with_extension("tmp");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&temp_path, json)?;
        fs::rename(&temp_path, &path)?; // Atomic on POSIX

        Ok(())
    }

    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("loonix-tunes").join("config.json"))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EqPreset {
    pub name: String,
    pub gains: [f32; 10],
    pub preamp: f32,
    pub macro_val: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FxPreset {
    pub name: String,
    pub bass_enabled: bool,
    pub bass_gain: f32,
    pub bass_cutoff: f32,
    pub bass_mode: i32,
    pub crystal_enabled: bool,
    pub crystal_amount: f32,
    pub crystal_freq: f32,
    pub surround_enabled: bool,
    pub surround_width: f32,
    pub mono_enabled: bool,
    pub mono_width: f32,
    pub pitch_enabled: bool,
    pub pitch_semitones: f32,
    pub middle_enabled: bool,
    pub middle_amount: f32,
    pub stereo_enabled: bool,
    pub stereo_amount: f32,
    pub crossfeed_enabled: bool,
    pub crossfeed_amount: f32,
    pub compressor_enabled: bool,
    pub compressor_threshold: f32,
    pub reverb_enabled: bool,
    pub reverb_mode: i32,
    pub reverb_amount: i32,
}

impl Default for FxPreset {
    fn default() -> Self {
        Self {
            name: "OFF".into(),
            bass_enabled: false,
            bass_gain: 6.0,
            bass_cutoff: 180.0,
            bass_mode: 0,
            crystal_enabled: false,
            crystal_amount: 0.20,
            crystal_freq: 8000.0,
            surround_enabled: false,
            surround_width: 1.8,
            mono_enabled: false,
            mono_width: 1.0,
            pitch_enabled: false,
            pitch_semitones: 0.0,
            middle_enabled: false,
            middle_amount: 0.0,
            stereo_enabled: false,
            stereo_amount: 0.0,
            crossfeed_enabled: false,
            crossfeed_amount: 0.0,
            compressor_enabled: true,
            compressor_threshold: -6.0,
            reverb_enabled: false,
            reverb_mode: 0,
            reverb_amount: 0,
        }
    }
}

impl AppConfig {
    pub fn get_eq_presets() -> Vec<EqPreset> {
        EQ_PRESETS
            .iter()
            .map(|p| EqPreset {
                name: p.name.to_string(),
                gains: p.gains,
                preamp: p.preamp,
                macro_val: p.macro_val,
            })
            .collect()
    }

    pub fn get_fx_presets() -> Vec<FxPreset> {
        FX_PRESETS
            .iter()
            .map(|p| FxPreset {
                name: p.name.to_string(),
                bass_enabled: p.bass_enabled,
                bass_gain: p.bass_gain,
                bass_cutoff: p.bass_cutoff,
                bass_mode: p.bass_mode,
                crystal_enabled: p.crystal_enabled,
                crystal_amount: p.crystal_amount,
                crystal_freq: p.crystal_freq,
                surround_enabled: p.surround_enabled,
                surround_width: p.surround_width,
                mono_enabled: p.mono_enabled,
                mono_width: p.mono_width,
                pitch_enabled: p.pitch_enabled,
                pitch_semitones: p.pitch_semitones,
                middle_enabled: p.middle_enabled,
                middle_amount: p.middle_amount,
                stereo_enabled: p.stereo_enabled,
                stereo_amount: p.stereo_amount,
                crossfeed_enabled: p.crossfeed_enabled,
                crossfeed_amount: p.crossfeed_amount,
                compressor_enabled: p.compressor_enabled,
                compressor_threshold: p.compressor_threshold,
                reverb_enabled: p.reverb_enabled,
                reverb_mode: p.reverb_mode,
                reverb_amount: p.reverb_amount,
            })
            .collect()
    }
}
