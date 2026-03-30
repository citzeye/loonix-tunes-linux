/* --- LOONIX-TUNES src/audio/config.rs --- */
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
    // VST3 settings
    pub vst3_initial_scan_done: bool,
    pub vst3_paths: Vec<String>,
    pub vst3_loaded_plugins: Vec<String>,
    pub vst3_active_plugin_index: i32,
    pub mode: crate::audio::engine::OutputMode,
    pub last_track_path: String,
    // DSP settings
    pub dsp_enabled: bool,
    pub bass_enabled: bool,
    pub bass_gain: f32,
    #[serde(default = "default_bass_cutoff")]
    pub bass_cutoff: f32,
    pub crystal_enabled: bool,
    pub crystal_amount: f32,
    pub surround_enabled: bool,
    pub surround_width: f32,
    pub mono_enabled: bool,
    pub mono_width: f32,
    pub pitch_enabled: bool,
    pub pitch_semitones: f32,
    pub middle_enabled: bool,
    pub middle_amount: f32,
    pub reverb_preset: u32,
    pub compressor_enabled: bool,
    pub compressor_threshold: f32,
    pub stereo_enabled: bool,
    pub stereo_amount: f32,
    pub crossfeed_enabled: bool,
    pub crossfeed_amount: f32,
    pub eq_enabled: bool,
    pub eq_dry: f32,
    pub eq_wet: f32,
    pub highres_enabled: bool,
    pub dac_exclusive_mode: bool,
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
    pub user_preset_dry: [f32; 6],
    pub user_preset_wet: [f32; 6],
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
        // Load defaults from fxpreset.json
        let fx = Self::load_fx_config();

        let bass_gain = fx
            .as_ref()
            .map(|f| f.instant_fx.bass_booster.gain)
            .unwrap_or(6.0);
        let bass_cutoff = fx
            .as_ref()
            .map(|f| f.instant_fx.bass_booster.cutoff)
            .unwrap_or(180.0);
        let bass_enabled = fx
            .as_ref()
            .map(|f| f.instant_fx.bass_booster.enabled)
            .unwrap_or(false);
        let crystal_amount = fx
            .as_ref()
            .map(|f| f.instant_fx.crystalizer.amount)
            .unwrap_or(0.2);
        let crystal_enabled = fx
            .as_ref()
            .map(|f| f.instant_fx.crystalizer.enabled)
            .unwrap_or(false);
        let surround_width = fx
            .as_ref()
            .map(|f| f.instant_fx.surround.width)
            .unwrap_or(1.8);
        let surround_enabled = fx
            .as_ref()
            .map(|f| f.instant_fx.surround.enabled)
            .unwrap_or(false);

        let _reverb_enabled = fx
            .as_ref()
            .map(|f| f.master_fx.reverb.enabled)
            .unwrap_or(false);
        let compressor_enabled = fx
            .as_ref()
            .map(|f| f.master_fx.compressor.enabled)
            .unwrap_or(false);
        let compressor_threshold = fx
            .as_ref()
            .map(|f| f.master_fx.compressor.threshold)
            .unwrap_or(-18.0);
        let pitch_enabled = fx
            .as_ref()
            .map(|f| f.master_fx.pitch_shifter.enabled)
            .unwrap_or(false);
        let pitch_semitones = fx
            .as_ref()
            .map(|f| f.master_fx.pitch_shifter.semitones)
            .unwrap_or(0.0);
        let middle_enabled = fx
            .as_ref()
            .map(|f| f.master_fx.middle_clarity.enabled)
            .unwrap_or(false);
        let middle_amount = fx
            .as_ref()
            .map(|f| f.master_fx.middle_clarity.amount)
            .unwrap_or(0.0);
        let mono_enabled = fx
            .as_ref()
            .map(|f| f.master_fx.stereo_width.enabled)
            .unwrap_or(false);
        let mono_width = fx
            .as_ref()
            .map(|f| f.master_fx.stereo_width.amount)
            .unwrap_or(1.0);
        let stereo_enabled = fx
            .as_ref()
            .map(|f| f.master_fx.stereo_enhance.enabled)
            .unwrap_or(false);
        let stereo_amount = fx
            .as_ref()
            .map(|f| f.master_fx.stereo_enhance.amount)
            .unwrap_or(0.0);
        let crossfeed_enabled = fx
            .as_ref()
            .map(|f| f.master_fx.headphone_crossfeed.enabled)
            .unwrap_or(false);
        let crossfeed_amount = fx
            .as_ref()
            .map(|f| f.master_fx.headphone_crossfeed.amount)
            .unwrap_or(0.0);

        Self {
            eq_bands: [3.0, 8.0, -5.0, 0.0, -3.0, -1.0, -3.0, -1.0, 1.0, -5.0],
            volume: 0.2,
            balance: 0.0,
            shuffle: false,
            loop_playlist: false,
            custom_folders: vec![],
            favorites: vec![],
            locked_folders: vec![],
            vst3_initial_scan_done: false,
            vst3_paths: vec![],
            vst3_loaded_plugins: vec![],
            vst3_active_plugin_index: -1,
            mode: crate::audio::engine::OutputMode::Stereo,
            last_track_path: String::new(),
            dsp_enabled: true,
            bass_enabled,
            bass_gain,
            bass_cutoff,
            crystal_enabled,
            crystal_amount,
            surround_enabled,
            surround_width,
            mono_enabled,
            mono_width,
            pitch_enabled,
            pitch_semitones,
            middle_enabled,
            middle_amount,
            reverb_preset: 0,
            compressor_enabled,
            compressor_threshold,
            stereo_enabled,
            stereo_amount,
            crossfeed_enabled,
            crossfeed_amount,
            eq_enabled: true,
            eq_dry: 0.0,
            eq_wet: 100.0,
            highres_enabled: false,
            dac_exclusive_mode: false,
            normalizer_enabled: true,
            normalizer_target_lufs: -14.0,
            normalizer_true_peak_dbtp: -1.5,
            normalizer_max_gain_db: 12.0,
            normalizer_smoothing: 0.002,
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
            user_preset_dry: [100.0; 6],
            user_preset_wet: [100.0; 6],
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if let Some(path) = config_path {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        return config;
                    }
                }
            }
        }

        Self::default()
    }

    pub fn save(&self) {
        let config_path = Self::get_config_path();

        if let Some(path) = config_path {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            if let Ok(content) = serde_json::to_string_pretty(self) {
                let _ = fs::write(&path, content);
            }
        }
    }

    fn get_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("loonix-tunes").join("config.json"))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EqPreset {
    pub name: String,
    pub gains: [f32; 10],
}

#[derive(Serialize, Deserialize)]
pub struct EqPresetFile {
    pub presets: Vec<EqPreset>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FxPreset {
    pub name: String,
    pub bass_enabled: bool,
    pub bass_gain: f32,
    #[serde(default = "default_bass_cutoff")]
    pub bass_cutoff: f32,
    pub crystal_enabled: bool,
    pub crystal_amount: f32,
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
}

impl Default for FxPreset {
    fn default() -> Self {
        Self {
            name: "OFF".into(),
            bass_enabled: false,
            bass_gain: 6.0,
            bass_cutoff: 180.0,
            crystal_enabled: false,
            crystal_amount: 0.20,
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
        }
    }
}

// === fxpreset.json structs ===

#[derive(Serialize, Deserialize, Clone)]
pub struct BassBoosterConfig {
    pub enabled: bool,
    pub gain: f32,
    pub cutoff: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SurroundConfig {
    pub enabled: bool,
    pub width: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CrystalizerConfig {
    pub enabled: bool,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InstantFx {
    pub bass_booster: BassBoosterConfig,
    pub surround: SurroundConfig,
    pub crystalizer: CrystalizerConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReverbConfig {
    pub enabled: bool,
    pub mode: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompressorConfig {
    pub enabled: bool,
    pub threshold: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PitchShifterConfig {
    pub enabled: bool,
    pub semitones: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MiddleClarityConfig {
    pub enabled: bool,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StereoWidthConfig {
    pub enabled: bool,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StereoEnhanceConfig {
    pub enabled: bool,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HeadphoneCrossfeedConfig {
    pub enabled: bool,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MasterFx {
    pub reverb: ReverbConfig,
    pub compressor: CompressorConfig,
    pub pitch_shifter: PitchShifterConfig,
    pub middle_clarity: MiddleClarityConfig,
    pub stereo_width: StereoWidthConfig,
    pub stereo_enhance: StereoEnhanceConfig,
    pub headphone_crossfeed: HeadphoneCrossfeedConfig,
}

#[derive(Serialize, Deserialize)]
pub struct FxPresetFile {
    pub instant_fx: InstantFx,
    pub master_fx: MasterFx,
}

// Factory presets hardcoded (anti-corrupt fallback)
impl AppConfig {
    pub fn factory_eq_presets() -> Vec<EqPreset> {
        vec![
            EqPreset {
                name: "CITZ".into(),
                gains: [0.0; 10],
            },
            EqPreset {
                name: "BASS".into(),
                gains: [6.0, 8.0, 5.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            EqPreset {
                name: "ROCK".into(),
                gains: [5.0, 4.0, 2.0, -2.0, -4.0, -2.0, 1.0, 3.0, 5.0, 6.0],
            },
            EqPreset {
                name: "POP".into(),
                gains: [-2.0, -1.0, 0.0, 2.0, 4.0, 4.0, 2.0, 0.0, -1.0, -2.0],
            },
            EqPreset {
                name: "METAL".into(),
                gains: [6.0, 5.0, 0.0, -4.0, -6.0, -4.0, 0.0, 3.0, 5.0, 6.0],
            },
            EqPreset {
                name: "JAZZ".into(),
                gains: [3.0, 2.0, 1.0, 2.0, -1.0, -1.0, 0.0, 1.0, 2.0, 3.0],
            },
        ]
    }

    pub fn factory_fx_presets() -> Vec<FxPreset> {
        vec![
            FxPreset {
                name: "OFF".into(),
                ..Default::default()
            },
            FxPreset {
                name: "BASS BOOST".into(),
                bass_enabled: true,
                bass_gain: 12.0,
                ..Default::default()
            },
            FxPreset {
                name: "CLARITY".into(),
                crystal_enabled: true,
                crystal_amount: 0.40,
                ..Default::default()
            },
            FxPreset {
                name: "WIDE".into(),
                surround_enabled: true,
                surround_width: 2.0,
                stereo_enabled: true,
                stereo_amount: 0.5,
                ..Default::default()
            },
            FxPreset {
                name: "HEADPHONE".into(),
                surround_enabled: true,
                surround_width: 1.5,
                crossfeed_enabled: true,
                crossfeed_amount: 0.3,
                ..Default::default()
            },
            FxPreset {
                name: "FULL".into(),
                bass_enabled: true,
                bass_gain: 10.0,
                crystal_enabled: true,
                crystal_amount: 0.30,
                stereo_enabled: true,
                stereo_amount: 0.2,
                compressor_enabled: true,
                ..Default::default()
            },
        ]
    }

    pub fn load_eq_presets() -> Vec<EqPreset> {
        let path = Self::get_assets_path().join("eqpreset.json");
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(file) = serde_json::from_str::<EqPresetFile>(&content) {
                return file.presets;
            }
        }
        // Safe fallback: use hardcoded factory presets
        Self::factory_eq_presets()
    }

    pub fn load_fx_presets() -> Vec<FxPreset> {
        Self::factory_fx_presets()
    }

    pub fn load_fx_config() -> Option<FxPresetFile> {
        let path = Self::get_assets_path().join("fxpreset.json");
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(file) = serde_json::from_str::<FxPresetFile>(&content) {
                return Some(file);
            }
        }
        None
    }

    fn get_assets_path() -> PathBuf {
        // Try to find assets relative to executable, then fallback to project dir
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let assets = parent.join("assets");
                if assets.exists() {
                    return assets;
                }
            }
        }
        PathBuf::from("assets")
    }
}
