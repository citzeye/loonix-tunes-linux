/* --- LOONIX-TUNES src/audio/config.rs --- */
use crate::audio::presets::{EQ_PRESETS, FX_PRESETS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CustomTheme {
    pub name: String,
    pub colors: HashMap<String, String>,
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
    pub user_preset_names: [String; 6],
    pub user_preset_gains: [[f32; 10]; 6],
    pub user_preset_macro: [f32; 6],
    // Theme settings
    pub theme: String,
    pub custom_themes: Vec<CustomTheme>,
    pub use_wallpaper_theme: bool,
    pub matugen_colors: HashMap<String, String>,
    pub wallpaper_path: String,
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
        Self {
            eq_bands: [3.0, 8.0, -5.0, 0.0, -3.0, -1.0, -3.0, -1.0, 1.0, -5.0],
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
            bass_enabled: true,
            bass_gain: 6.0,
            bass_cutoff: 180.0,
            bass_mode: 2,
            bass_q: 0.7,
            crystal_enabled: true,
            crystal_amount: 0.5,
            crystal_freq: 8000.0,
            surround_enabled: true,
            surround_width: 1.5,
            surround_room_size: 15.0,
            surround_bass_safe: true,
            mono_enabled: false,
            mono_width: 1.0,
            pitch_enabled: false,
            pitch_semitones: 0.0,
            middle_enabled: false,
            middle_amount: 0.0,
            reverb_enabled: false,
            reverb_mode: 1,    // Studio mode by default
            reverb_amount: 50, // 50% intensity by default
            compressor_enabled: true,
            compressor_threshold: -10.0,
            stereo_enabled: true,
            stereo_amount: 0.4,
            preamp_db: 0.0,
            crossfeed_enabled: false,
            crossfeed_amount: 0.0,
            eq_enabled: true,
            normalizer_enabled: true,
            normalizer_target_lufs: default_norm_target_lufs(),
            normalizer_true_peak_dbtp: default_norm_true_peak(),
            normalizer_max_gain_db: default_norm_max_gain(),
            normalizer_smoothing: default_norm_smoothing(),
            active_preset_index: 0,
            user_preset_names: [
                "User 1".to_string(),
                "User 2".to_string(),
                "User 3".to_string(),
                "User 4".to_string(),
                "User 5".to_string(),
                "User 6".to_string(),
            ],
            user_preset_gains: [[0.0; 10]; 6],
            user_preset_macro: [0.0; 6],
            theme: "Default".to_string(),
            custom_themes: vec![
                CustomTheme {
                    name: "Custom 1".to_string(),
                    colors: Self::default_theme_colors(),
                },
                CustomTheme {
                    name: "Custom 2".to_string(),
                    colors: Self::default_theme_colors(),
                },
                CustomTheme {
                    name: "Custom 3".to_string(),
                    colors: Self::default_theme_colors(),
                },
            ],
            use_wallpaper_theme: false,
            matugen_colors: HashMap::new(),
            wallpaper_path: String::new(),
        }
    }
}

impl AppConfig {
    pub fn default_theme_colors() -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("bgmain".to_string(), "#15151B".to_string());
        map.insert("bgoverlay".to_string(), "#201f2b".to_string());
        map.insert("graysolid".to_string(), "#6d6d6d".to_string());
        map.insert("contextmenubg".to_string(), "#2d2d2d".to_string());
        map.insert("overlay".to_string(), "#000000".to_string());
        map.insert("headerbg".to_string(), "#201f2b".to_string());
        map.insert("headericon".to_string(), "#6d6d6d".to_string());
        map.insert("headertext".to_string(), "#6d6d6d".to_string());
        map.insert("headerhover".to_string(), "#ff1ae0".to_string());
        map.insert("playertitle".to_string(), "#00ffa2".to_string());
        map.insert("playersubtext".to_string(), "#57caab".to_string());
        map.insert("playeraccent".to_string(), "#9442ff".to_string());
        map.insert("playerhover".to_string(), "#ff1ae0".to_string());
        map.insert("tabtext".to_string(), "#c6c6c6".to_string());
        map.insert("tabborder".to_string(), "#00ffa2".to_string());
        map.insert("tabhover".to_string(), "#ff1ae0".to_string());
        map.insert("playlisttext".to_string(), "#c6c6c6".to_string());
        map.insert("playlistfolder".to_string(), "#ff881a".to_string());
        map.insert("playlistactive".to_string(), "#00ffa2".to_string());
        map.insert("playlisticon".to_string(), "#ff881a".to_string());

        // DSP Panel
        map.insert("dspbg".to_string(), "#15151B".to_string()); // outer
        map.insert("dspborder".to_string(), "#6d6d6d".to_string());
        map.insert("dspgridbg".to_string(), "#111111".to_string());    


            // EQ Panel
            map.insert("dspeqbg".to_string(), "#151515".to_string()); // eq wrapper
            map.insert("dspeqslider".to_string(), "#ff1ae0".to_string());
            map.insert("dspeqsliderbg".to_string(), "#15151B".to_string());
            map.insert("dspeqhandle".to_string(), "#ff1ae0".to_string());
            map.insert("dspeqtext".to_string(), "#6d6d6d".to_string());
            map.insert("dspeqicon".to_string(), "#ff881a".to_string());
            map.insert("dspeqhover".to_string(), "#ff1ae0".to_string());
            map.insert("dspeqpresetactive".to_string(), "#00ffa2".to_string());

            map.insert("dspeampslider".to_string(), "#ff1ae0".to_string());
            map.insert("dspeamphandle".to_string(), "#9442ff".to_string());
            map.insert("dspeampbg".to_string(), "#111111".to_string());
            map.insert("dspeq10slider".to_string(), "#ff1ae0".to_string());
            map.insert("dspeq10handle".to_string(), "#9442ff".to_string());
            map.insert("dspeq10bg".to_string(), "#000000".to_string());
            map.insert("dspeqfaderslider".to_string(), "#ff1ae0".to_string());
            map.insert("dspeqfaderhandle".to_string(), "#9442ff".to_string());
            map.insert("dspeqfaderbg".to_string(), "#111111".to_string());

            // FX Panel
            map.insert("dspfxbg".to_string(), "#151515".to_string()); // fx wrapper
            map.insert("dspfxborder".to_string(), "#00ffa2".to_string());
            map.insert("dspfxtext".to_string(), "#6d6d6d".to_string());
            map.insert("dspfxicon".to_string(), "#ff881a".to_string());
            map.insert("dspfxhover".to_string(), "#ff1ae0".to_string());
            map.insert("dspfxactive".to_string(), "#9442ff".to_string());
            map.insert("dspfxslider".to_string(), "#ff1ae0".to_string());
            map.insert("dspfxsliderbg".to_string(), "#15151B".to_string());
            map.insert("dspfxhandle".to_string(), "#9442ff".to_string());

        map
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
