/* --- loonixtunesv2/src/ui/dsp.rs | DSP Controller --- */

#![allow(non_snake_case)]

use crate::audio::config::{AppConfig, DspConfig, EqPreset, FxPreset};
use crate::audio::dsp::crystalizer::get_crystal_amount_arc;
use crate::audio::dsp::pitchshifter::{get_pitch_enabled_arc, get_pitch_ratio_arc};
use crate::audio::dsp::DspSettings;
use crate::core::dspconfig::DspConfigManager;
use qmetaobject::prelude::*;
use qmetaobject::{QString, QVariant, QVariantList};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
enum PresetSource {
    Factory(usize), // Load from internal presets.rs
    User(usize),    // Load from JSON (user preset data)
    Preserve,       // Keep current FX settings (for user presets)
}

pub struct DspController {
    pub ffmpeg: Arc<Mutex<crate::audio::engine::FfmpegEngine>>,
    pub config_manager: DspConfigManager,

    pub reverb_active: bool,
    pub reverb_mode: i32,
    pub reverb_amount: i32,
    pub reverb_room_size: f64,
    pub reverb_damp: f64,
    pub reverb_preset: u32,

    pub bass_active: bool,
    pub bass_gain: f64,
    pub bass_cutoff: f64,
    pub bass_mode: i32,

    pub surround_active: bool,
    pub surround_width: f64,

    pub crystal_active: bool,
    pub crystal_amount: f64,
    pub crystal_freq: f64,

    pub compressor_active: bool,
    pub compressor_threshold: f64,

    pub dsp_enabled: bool,

    pub mono_active: bool,
    pub mono_width: f64,

    pub middle_active: bool,
    pub middle_amount: f64,

    pub stereo_active: bool,
    pub stereo_amount: f64,

    pub crossfeed_active: bool,
    pub crossfeed_amount: f64,

    pub pitch_active: bool,
    pub pitch_semitones: f64,

    pub eq_enabled: bool,
    pub eq_bands: [f32; 10],
    pub fader_offset: f64,

    pub active_preset_index: i32,
    pub user_eq_names: [String; 6],
    pub user_eq_gains: [[f32; 10]; 6],
    pub user_eq_macro: [f32; 6],
    // User FX presets (User 1-6)
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

    pub preamp_active: bool,

    pub limiter_active: bool,

    pub normalizer_enabled: bool,
    pub normalizer_target_lufs: f64,
    pub normalizer_true_peak_dbtp: f64,
    pub normalizer_max_gain_db: f64,
    pub normalizer_smoothing: f64,

    pub eq_presets: Vec<EqPreset>,
    pub fx_presets: Vec<FxPreset>,
    pub default_fx_snapshot: Option<FxPreset>,
}

impl Default for DspController {
    fn default() -> Self {
        Self {
            ffmpeg: Arc::new(Mutex::new(crate::audio::engine::FfmpegEngine::new())),
            config_manager: DspConfigManager::default(),
            reverb_active: false,
            reverb_mode: 0,
            reverb_amount: 50,
            reverb_room_size: 0.5,
            reverb_damp: 0.5,
            reverb_preset: 0,
            bass_active: false,
            bass_gain: 0.0,
            bass_cutoff: 180.0,
            bass_mode: 0,
            surround_active: false,
            surround_width: 1.8,
            crystal_active: false,
            crystal_amount: 0.0,
            crystal_freq: 4000.0,
            compressor_active: false,
            compressor_threshold: 1.0,
            dsp_enabled: true,
            mono_active: false,
            mono_width: 1.0,
            middle_active: false,
            middle_amount: 0.0,
            stereo_active: false,
            stereo_amount: 0.0,
            crossfeed_active: false,
            crossfeed_amount: 0.0,
            pitch_active: false,
            pitch_semitones: 0.0,
            eq_enabled: false,
            eq_bands: [0.0; 10],
            fader_offset: 0.0,
            active_preset_index: -1,
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
            preamp_active: false,
            limiter_active: false,
            normalizer_enabled: false,
            normalizer_target_lufs: -14.0,
            normalizer_true_peak_dbtp: -1.0,
            normalizer_max_gain_db: 6.0,
            normalizer_smoothing: 0.005,
            eq_presets: Vec::new(),
            fx_presets: Vec::new(),
            default_fx_snapshot: None,
        }
    }
}

impl DspController {
    pub fn new(
        ffmpeg: Arc<Mutex<crate::audio::engine::FfmpegEngine>>,
        saved_config: Option<Arc<Mutex<AppConfig>>>,
    ) -> Self {
        let mut controller = Self::default();
        controller.ffmpeg = ffmpeg;
        controller.config_manager = DspConfigManager::new(saved_config);
        controller.eq_presets = AppConfig::get_eq_presets();
        controller.fx_presets = AppConfig::get_fx_presets();

        controller
    }

    pub fn init_from_config(&mut self, config: &AppConfig) {
        // 1. Load Normalizer Settings from config.json
        self.normalizer_enabled = config.normalizer_enabled;
        self.normalizer_target_lufs = config.normalizer_target_lufs as f64;
        self.normalizer_true_peak_dbtp = config.normalizer_true_peak_dbtp as f64;
        self.normalizer_max_gain_db = config.normalizer_max_gain_db as f64;
        self.normalizer_smoothing = config.normalizer_smoothing as f64;

        crate::audio::dsp::normalizer::get_normalizer_smoothing_arc().store(
            config.normalizer_smoothing.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        // 2. Load all DSP settings from dsp.json or defaults
        let dsp_settings = crate::audio::dsp::DspSettings::default();
        self.dsp_enabled = dsp_settings.dsp_enabled;
        self.eq_enabled = dsp_settings.eq_enabled;

        // 2. Load Preset Definitions into Memory
        self.eq_presets = AppConfig::get_eq_presets();
        self.fx_presets = AppConfig::get_fx_presets();

        // Load user presets from dsp.json
        let dsp_config = DspConfig::load();
        self.user_eq_names = dsp_config.user_preset_names.clone();
        self.user_eq_gains = dsp_config.user_preset_gains;
        self.user_eq_macro = dsp_config.user_preset_macro;
        self.user_fx_enabled = dsp_config.user_fx_enabled;
        self.user_fx_bass_enabled = dsp_config.user_fx_bass_enabled;
        self.user_fx_bass_gain = dsp_config.user_fx_bass_gain;
        self.user_fx_bass_cutoff = dsp_config.user_fx_bass_cutoff;
        self.user_fx_bass_mode = dsp_config.user_fx_bass_mode;
        self.user_fx_crystal_enabled = dsp_config.user_fx_crystal_enabled;
        self.user_fx_crystal_amount = dsp_config.user_fx_crystal_amount;
        self.user_fx_surround_enabled = dsp_config.user_fx_surround_enabled;
        self.user_fx_surround_width = dsp_config.user_fx_surround_width;
        self.user_fx_mono_enabled = dsp_config.user_fx_mono_enabled;
        self.user_fx_mono_width = dsp_config.user_fx_mono_width;
        self.user_fx_stereo_enabled = dsp_config.user_fx_stereo_enabled;
        self.user_fx_stereo_amount = dsp_config.user_fx_stereo_amount;
        self.user_fx_crossfeed_enabled = dsp_config.user_fx_crossfeed_enabled;
        self.user_fx_crossfeed_amount = dsp_config.user_fx_crossfeed_amount;
        self.user_fx_compressor_enabled = dsp_config.user_fx_compressor_enabled;
        self.user_fx_compressor_threshold = dsp_config.user_fx_compressor_threshold;
        self.user_fx_reverb_enabled = dsp_config.user_fx_reverb_enabled;
        self.user_fx_reverb_mode = dsp_config.user_fx_reverb_mode;
        self.user_fx_reverb_amount = dsp_config.user_fx_reverb_amount;

        // 3. THE MASTER BOOT ACTION (0 to 11)
        // Read preset index from dsp.json (not config.json)
        let preset_index = dsp_config.active_preset_index.clamp(0, 11);
        self.load_preset(preset_index);
    }

    fn applyBassMode(&mut self, mode: i32) {
        // Frekuensi disesuaikan biar selaras sama preset (Punch dapet 180.0 biar tebel!)
        // 0=Deep(80Hz), 1=Soft(120Hz), 2=Punch(180Hz), 3=Warm(220Hz)
        let freqs: [f32; 4] = [80.0, 120.0, 180.0, 220.0]; 
        let q_vals: [f32; 4] = [0.5, 0.6, 0.7, 0.8];

        self.bass_cutoff = freqs[mode as usize] as f64;
        
        crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
            freqs[mode as usize].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::bassbooster::get_bass_q_arc().store(
            q_vals[mode as usize].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        
        // Panggil ini biar UI (kalau nanti ada slider cutoff) ikut update
        self.bass_cutoff_changed(); 
    }

    pub fn save_config(&mut self) {
        let state = self.get_state_view();
        self.config_manager.save_dsp_config(&state);
    }

    pub fn get_state_view(&self) -> crate::core::dspconfig::DspStateView {
        crate::core::dspconfig::DspStateView {
            dsp_enabled: self.dsp_enabled,
            dsp_bands: self.eq_bands,
            eq_enabled: self.eq_enabled,
            active_preset_index: self.active_preset_index,
            bass_active: self.bass_active,
            bass_gain: self.bass_gain,
            bass_cutoff: self.bass_cutoff,
            crystal_active: self.crystal_active,
            crystal_amount: self.crystal_amount,
            crystal_frdsp: self.crystal_freq,
            surround_active: self.surround_active,
            surround_width: self.surround_width,
            mono_active: self.mono_active,
            mono_width: self.mono_width,
            pitch_active: self.pitch_active,
            pitch_semitones: self.pitch_semitones,
            middle_active: self.middle_active,
            middle_amount: self.middle_amount,
            reverb_mode: self.reverb_mode as u32,
            reverb_amount: self.reverb_amount as u32,
            compressor_active: self.compressor_active,
            stereo_active: self.stereo_active,
            stereo_amount: self.stereo_amount,
            crossfeed_active: self.crossfeed_active,
            crossfeed_amount: self.crossfeed_amount,
            user_eq_names: self.user_eq_names.clone(),
            user_eq_gains: self.user_eq_gains,
            user_eq_macro: self.user_eq_macro,
            user_fx_enabled: self.user_fx_enabled,
            user_fx_bass_enabled: self.user_fx_bass_enabled,
            user_fx_bass_gain: self.user_fx_bass_gain,
            user_fx_bass_cutoff: self.user_fx_bass_cutoff,
            user_fx_bass_mode: self.user_fx_bass_mode,
            user_fx_crystal_enabled: self.user_fx_crystal_enabled,
            user_fx_crystal_amount: self.user_fx_crystal_amount,
            user_fx_surround_enabled: self.user_fx_surround_enabled,
            user_fx_surround_width: self.user_fx_surround_width,
            user_fx_mono_enabled: self.user_fx_mono_enabled,
            user_fx_mono_width: self.user_fx_mono_width,
            user_fx_stereo_enabled: self.user_fx_stereo_enabled,
            user_fx_stereo_amount: self.user_fx_stereo_amount,
            user_fx_crossfeed_enabled: self.user_fx_crossfeed_enabled,
            user_fx_crossfeed_amount: self.user_fx_crossfeed_amount,
            user_fx_compressor_enabled: self.user_fx_compressor_enabled,
            user_fx_compressor_threshold: self.user_fx_compressor_threshold,
            user_fx_reverb_enabled: self.user_fx_reverb_enabled,
            user_fx_reverb_mode: self.user_fx_reverb_mode,
            user_fx_reverb_amount: self.user_fx_reverb_amount,
        }
    }

    pub fn sync_eq_bands(&mut self) -> QVariantList {
        let mut list = QVariantList::default();
        for &gain in &self.eq_bands {
            let effective = (gain as f64 + self.fader_offset).clamp(-20.0, 20.0);
            list.push(QVariant::from(effective));
        }
        list
    }

    pub fn emit_all_signals(&mut self) {
        self.bass_active_changed();
        self.bass_mode_changed();
        self.surround_active_changed();
        self.surround_width_changed();
        self.crystal_active_changed();
        self.compressor_active_changed();
        self.mono_changed();
        self.mono_width_changed();
        self.pitch_changed();
        self.middle_changed();
        self.middle_amount_changed();
        self.stereo_changed();
        self.stereo_amount_changed();
        self.crossfeed_changed();
        self.crossfeed_amount_changed();
        self.active_preset_index_changed();
        self.eqBandsChanged();
        self.faderOffsetChanged();
        self.dsp_changed();
        self.eq_enabled_changed();
        self.normalizer_changed();
        self.normalizer_params_changed();
        self.preamp_changed();
        self.limiter_changed();
    }

    // QML signal emitters
    pub fn reverb_changed(&self) {}
    pub fn reverb_active_changed(&self) {}
    pub fn reverb_mode_changed(&self) {}
    pub fn reverb_amount_changed(&self) {}
    pub fn bass_active_changed(&self) {}
    pub fn bass_gain_changed(&self) {}
    pub fn bass_cutoff_changed(&self) {}
    pub fn bass_mode_changed(&self) {}
    pub fn bass_params_changed(&self) {}
    pub fn surround_width_changed(&self) {}
    pub fn surround_active_changed(&self) {}
    pub fn crystal_active_changed(&self) {}
    pub fn crystal_amount_changed(&self) {}
    pub fn crystal_freq_changed(&self) {}
    pub fn compressor_active_changed(&self) {}
    pub fn compressor_threshold_changed(&self) {}
    pub fn dsp_changed(&self) {}
    pub fn mono_changed(&self) {}
    pub fn mono_width_changed(&self) {}
    pub fn middle_changed(&self) {}
    pub fn middle_amount_changed(&self) {}
    pub fn stereo_changed(&self) {}
    pub fn stereo_amount_changed(&self) {}
    pub fn crossfeed_changed(&self) {}
    pub fn crossfeed_amount_changed(&self) {}
    pub fn eq_enabled_changed(&self) {}
    pub fn eqBandsChanged(&self) {}
    pub fn faderOffsetChanged(&self) {}
    pub fn active_preset_index_changed(&self) {}
    pub fn normalizer_changed(&self) {}
    pub fn normalizer_params_changed(&self) {}
    pub fn preamp_changed(&self) {}
    pub fn limiter_changed(&self) {}
    pub fn pitch_changed(&self) {}

    // --- REVERB METHODS ---
    pub fn set_reverb_mode(&mut self, mode: i32) {
        let mode = mode.clamp(0, 3) as u32;
        self.reverb_preset = mode;
        self.reverb_mode = mode as i32;
        self.reverb_active = true;

        // Send ALL reverb values to Rust atomics
        crate::audio::dsp::reverb::get_reverb_enabled_arc()
            .store(true, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::reverb::get_reverb_mode_arc()
            .store(mode, std::sync::atomic::Ordering::Relaxed);
        let current_amount = self.reverb_amount as u32;
        crate::audio::dsp::reverb::get_reverb_amount_arc()
            .store(current_amount, std::sync::atomic::Ordering::Relaxed);

        self.reverb_active_changed();
        self.reverb_mode_changed();
        self.reverb_amount_changed();
        self.reverb_changed();
        self.save_config();
    }

    pub fn set_reverb_amount(&mut self, amount: i32) {
        let amount = amount.clamp(0, 100) as u32;
        self.reverb_amount = amount as i32;
        // Sync amount to Rust - use raw u32 (not float bits)
        crate::audio::dsp::reverb::get_reverb_amount_arc()
            .store(amount, std::sync::atomic::Ordering::Relaxed);
        self.reverb_amount_changed();
        self.save_config();
    }

    pub fn set_reverb_room_size(&mut self, val: f64) {
        let val = val.clamp(0.0, 1.0);
        self.reverb_room_size = val;
        crate::audio::dsp::reverb::get_reverb_room_size_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.save_config();
    }

    pub fn set_reverb_damp(&mut self, val: f64) {
        let val = val.clamp(0.0, 1.0);
        self.reverb_damp = val;
        crate::audio::dsp::reverb::get_reverb_damp_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.save_config();
    }

    pub fn toggle_reverb(&mut self) {
        self.reverb_active = !self.reverb_active;
        self.reverb_active_changed();

        // Toggle only enables/disables - do NOT change mode
        crate::audio::dsp::reverb::get_reverb_enabled_arc()
            .store(self.reverb_active, std::sync::atomic::Ordering::Relaxed);

        // Sync amount to prevent "Ghost Slider"
        let current_amount = self.reverb_amount as u32;
        crate::audio::dsp::reverb::get_reverb_amount_arc()
            .store(current_amount, std::sync::atomic::Ordering::Relaxed);

        self.reverb_changed();
        self.save_config();
    }

    pub fn set_reverb(&mut self, reverb: String) {
        let p_str = reverb.to_lowercase();
        let preset_id = match p_str.as_str() {
            "studio" => 1,
            "stage" => 2,
            "stadium" => 3,
            _ => 0,
        };

        crate::audio::dsp::reverb::get_reverb_mode_arc()
            .store(preset_id, std::sync::atomic::Ordering::Relaxed);

        // Sync amount to prevent "Ghost Slider" - always send current amount
        let current_amount = self.reverb_amount as u32;
        crate::audio::dsp::reverb::get_reverb_amount_arc()
            .store(current_amount, std::sync::atomic::Ordering::Relaxed);

        self.reverb_preset = preset_id;
        self.reverb_mode = preset_id as i32;
        self.reverb_active = preset_id > 0;
        self.reverb_active_changed();
        self.reverb_mode_changed();
        self.reverb_amount_changed();
        self.reverb_changed();
        self.save_config();
    }

    // --- BASS METHODS ---
    pub fn toggle_bass(&mut self) {
        self.bass_active = !self.bass_active;
        self.bass_active_changed();

        // Murni cuma ngasih tau engine buat bypass/enable, tanpa ngotak-ngatik parameter slider!
        crate::audio::dsp::bassbooster::get_bass_enabled_arc()
            .store(self.bass_active, std::sync::atomic::Ordering::Relaxed);
            
        self.save_config();
    }

    pub fn set_bass_mode(&mut self, mode: i32) {
        let mode = mode.clamp(0, 3);
        self.bass_mode = mode;
        self.bass_mode_changed();

        if self.bass_active {
            self.applyBassMode(mode);
        }

        self.bass_params_changed();
        self.save_config();
    }

    pub fn set_bass_gain(&mut self, val: f64) {
        self.bass_gain = val.clamp(0.0, 12.0);
        self.bass_gain_changed(); 
        self.bass_params_changed();

        if self.bass_active {
            crate::audio::dsp::bassbooster::get_bass_gain_arc().store(
                (self.bass_gain as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        self.save_config();
    }

    pub fn set_bass_cutoff(&mut self, val: f64) {
        self.bass_cutoff = val.clamp(20.0, 500.0);
        self.bass_params_changed();

        if self.bass_active {
            crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
                (self.bass_cutoff as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        self.save_config();
    }

    // --- SURROUND METHODS ---
    pub fn toggle_surround(&mut self) {
        self.surround_active = !self.surround_active;
        self.surround_active_changed();

        // Toggle only bypasses - do NOT reset width values
        crate::audio::dsp::surround::get_surround_enabled_arc()
            .store(self.surround_active, std::sync::atomic::Ordering::Relaxed);

        self.save_config();
    }

    pub fn set_surround_width(&mut self, val: f64) {
        let val = val.max(0.0).min(2.0);
        self.surround_width = val;

        if !self.surround_active {
            self.surround_active = true;
            self.surround_active_changed();
            crate::audio::dsp::surround::get_surround_enabled_arc()
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }

        crate::audio::dsp::surround::get_surround_width_arc().store(
            (self.surround_width as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        self.surround_width_changed();
        self.save_config();
    }

    // --- CRYSTALIZER METHODS ---
    pub fn toggle_crystalizer(&mut self) {
        self.crystal_active = !self.crystal_active;
        self.crystal_active_changed();

        if self.crystal_active {
            if self.crystal_amount <= 0.0 {
                self.crystal_amount = 0.5;
            }
            get_crystal_amount_arc().store(
                (self.crystal_amount as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        } else {
            get_crystal_amount_arc().store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        crate::audio::dsp::crystalizer::get_crystal_enabled_arc()
            .store(self.crystal_active, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_crystalizer_amount(&mut self, amount: f64) {
        let amount = amount.max(0.0).min(1.0);
        self.crystal_amount = amount;
        self.crystal_active_changed();

        get_crystal_amount_arc().store(
            (amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_config();
    }

    // --- COMPRESSOR METHODS ---
    pub fn toggle_compressor(&mut self) {
        self.compressor_active = !self.compressor_active;
        self.compressor_active_changed();

        crate::audio::dsp::compressor::get_compressor_enabled_arc()
            .store(self.compressor_active, std::sync::atomic::Ordering::Relaxed);

        self.save_config();
    }

    pub fn set_compressor_threshold(&mut self, val: f64) {
        let clamped = val.clamp(-60.0, 0.0);
        self.compressor_threshold = clamped;     
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            (clamped as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.compressor_threshold_changed();    
        self.save_config();
    }

    pub fn get_compressor_threshold(&self) -> f64 {
        let bits = crate::audio::dsp::compressor::get_compressor_threshold_arc()
            .load(std::sync::atomic::Ordering::Relaxed);
        f32::from_bits(bits) as f64
    }

    // --- PITCH METHODS ---
    pub fn toggle_pitch(&mut self) {
        self.pitch_active = !self.pitch_active;
        self.pitch_changed();

        get_pitch_enabled_arc().store(self.pitch_active, std::sync::atomic::Ordering::Relaxed);

        self.save_config();
    }

    pub fn set_pitch_semitones(&mut self, val: f64) {
        let raw = val.max(-12.0).min(12.0);
        let semitones = if raw.abs() < 0.5 { 0.0 } else { raw };
        self.pitch_semitones = semitones;
        self.pitch_changed();

        let ratio = 2.0_f32.powf((semitones as f32) / 12.0);
        get_pitch_ratio_arc().store(ratio.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.save_config();
    }

    // --- MIDDLE CLARITY METHODS ---
    pub fn toggle_middle_clarity(&mut self) {
        self.middle_active = !self.middle_active;
        self.middle_changed();

        crate::audio::dsp::middleclarity::get_middle_enabled_arc()
            .store(self.middle_active, std::sync::atomic::Ordering::Relaxed);

        self.save_config();
    }

    pub fn set_middle_clarity_amount(&mut self, val: f64) {
        self.middle_amount = val.max(0.0).min(1.0);
        self.middle_amount_changed();
        crate::audio::dsp::middleclarity::get_middle_amount_arc().store(
            (self.middle_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_config();
    }

    // --- STEREO WIDTH METHODS ---
    pub fn toggle_stereo_width(&mut self) {
        self.mono_active = !self.mono_active;
        self.mono_changed();

        crate::audio::dsp::stereowidth::get_mono_enabled_arc()
            .store(self.mono_active, std::sync::atomic::Ordering::Relaxed);

        self.save_config();
    }

    pub fn set_stereo_width_amount(&mut self, val: f64) {
        self.mono_width = val.max(0.0).min(1.0);
        self.mono_width_changed();
        crate::audio::dsp::stereowidth::get_mono_width_arc().store(
            (self.mono_width as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_config();
    }

    // --- STEREO ENHANCE METHODS ---
    pub fn toggle_stereo_enhance(&mut self) {
        self.stereo_active = !self.stereo_active;
        self.stereo_changed();

        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
            .store(self.stereo_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc().store(
            (self.stereo_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        self.save_config();
    }

    pub fn set_stereo_enhance_amount(&mut self, val: f64) {
        self.stereo_amount = val.max(0.0).min(1.0);
        self.stereo_amount_changed();
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc().store(
            (self.stereo_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_config();
    }

    // --- CROSSFEED METHODS ---
    pub fn toggle_crossfeed(&mut self) {
        self.crossfeed_active = !self.crossfeed_active;
        self.crossfeed_changed();

        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc()
            .store(self.crossfeed_active, std::sync::atomic::Ordering::Relaxed);

        self.save_config();
    }

    pub fn set_crossfeed_amount(&mut self, val: f64) {
        self.crossfeed_amount = val.max(0.0).min(1.0);
        self.crossfeed_amount_changed();
        crate::audio::dsp::crossfeed::get_crossfeed_amount_arc().store(
            (self.crossfeed_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_config();
    }

    // --- PREAMP METHODS ---
    pub fn toggle_preamp(&mut self) {
        self.preamp_active = !self.preamp_active;
        self.preamp_changed();
        crate::audio::dsp::preamp::get_preamp_enabled_arc()
            .store(self.preamp_active, std::sync::atomic::Ordering::Relaxed);
        self.save_config();
    }

    // --- LIMITER METHODS ---
    pub fn toggle_limiter(&mut self) {
        self.limiter_active = !self.limiter_active;
        self.limiter_changed();
        crate::audio::dsp::get_limiter_enabled_arc()
            .store(self.limiter_active, std::sync::atomic::Ordering::Relaxed);
        self.save_config();
    }

    // --- NORMALIZER METHODS ---
    pub fn toggle_normalizer(&mut self) {
        self.normalizer_enabled = !self.normalizer_enabled;
        self.normalizer_changed();
        self.save_config();
    }

    pub fn set_normalizer_target_lufs(&mut self, val: f64) {
        let clamped = val.clamp(-24.0, -10.0);
        self.normalizer_target_lufs = clamped;
        self.normalizer_params_changed();

        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_normalizer_params(
                clamped as f32,
                self.normalizer_true_peak_dbtp as f32,
                self.normalizer_max_gain_db as f32,
            );
        }
        self.save_config();
    }

    pub fn set_normalizer_true_peak_dbtp(&mut self, val: f64) {
        let clamped = val.clamp(-3.0, 0.0);
        self.normalizer_true_peak_dbtp = clamped;
        self.normalizer_params_changed();

        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_normalizer_params(
                self.normalizer_target_lufs as f32,
                clamped as f32,
                self.normalizer_max_gain_db as f32,
            );
        }
        self.save_config();
    }

    pub fn set_normalizer_max_gain_db(&mut self, val: f64) {
        let clamped = val.clamp(0.0, 12.0);
        self.normalizer_max_gain_db = clamped;
        self.normalizer_params_changed();

        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_normalizer_params(
                self.normalizer_target_lufs as f32,
                self.normalizer_true_peak_dbtp as f32,
                clamped as f32,
            );
        }
        self.save_config();
    }

    pub fn set_normalizer_smoothing(&mut self, val: f64) {
        let clamped = val.clamp(0.0005, 0.01);
        self.normalizer_smoothing = clamped;
        self.normalizer_params_changed();

        crate::audio::dsp::normalizer::get_normalizer_smoothing_arc().store(
            (clamped as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_config();
    }

    pub fn get_normalizer_smoothing_label(&self) -> QString {
        let label = crate::audio::dsp::normalizer::SmoothingPreset::from_factor(
            self.normalizer_smoothing as f32,
        );
        QString::from(label)
    }

    // --- DSP MASTER METHODS ---
    pub fn toggle_dsp(&mut self) {
        self.dsp_enabled = !self.dsp_enabled;
        self.dsp_changed();    
        self.emit_all_signals();     

        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_dsp_enabled(self.dsp_enabled);
        }

        if !self.dsp_enabled {
            if let Ok(mut ff) = self.ffmpeg.lock() {
                ff.reset_dsp();
            }
        }
        self.save_config();
    }

    // --- EQ METHODS ---
    pub fn set_eq_enabled(&mut self, enabled: bool) {
        self.eq_enabled = enabled;
        crate::audio::dsp::eq::get_eq_enabled_arc().store(
            if enabled { 1 } else { 0 },
            std::sync::atomic::Ordering::Relaxed,
        );
        self.eq_enabled_changed();
        self.save_config();
    }

    pub fn set_eq_band(&mut self, index: i32, gain: f64) {
        let band = index as usize;
        if band < 10 {
            self.eq_bands[band] = gain as f32;
            self.active_preset_index = -1;
            self.active_preset_index_changed();

            let arc = crate::audio::dsp::eq::get_eq_bands_arc();
            arc[band].store(
                (gain as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );

            self.eqBandsChanged();
            self.save_config();
        }
    }

    pub fn set_fader(&mut self, offset: f64) {
        let offset = offset.clamp(-20.0, 20.0);
        self.fader_offset = offset;
        self.faderOffsetChanged();

        let arc = crate::audio::dsp::eq::get_eq_bands_arc();
        for i in 0..10 {
            let effective = (self.eq_bands[i] as f64 + offset).clamp(-20.0, 20.0) as f32;
            arc[i].store(effective.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        self.eqBandsChanged();
        self.save_config();
    }

    pub fn set_active_preset_index(&mut self, index: i32) {
        if index >= 0 && index < 6 {
            self.load_preset(index);
        }
    }

    pub fn get_preamp_gain(&self) -> f64 {
        let bits =
            crate::audio::dsp::eq::get_eq_preamp_arc().load(std::sync::atomic::Ordering::Relaxed);
        f32::from_bits(bits) as f64
    }

    pub fn set_preamp_gain(&mut self, gain: f64) {
        let clamped = gain.clamp(-20.0, 20.0);
        crate::audio::dsp::eq::get_eq_preamp_arc().store(
            (clamped as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_config();
    }

    pub fn save_user_eq(&mut self, preset: i32, name: String, macro_val: f64) {
        if preset >= 0 && preset < 6 {
            let idx = preset as usize;

            let mut trimmed_name = name.trim().to_string();
            trimmed_name.truncate(10);
            if trimmed_name.is_empty() {
                trimmed_name = format!("User {}", preset + 1);
            }

            self.user_eq_names[idx] = trimmed_name.to_uppercase();
            self.user_eq_gains[idx] = self.eq_bands;
            self.user_eq_macro[idx] = macro_val as f32;

            self.save_config();
        }
    }

    pub fn save_user_preset(&mut self, name: String) -> i32 {
        let mut trimmed_name = name.trim().to_string();
        trimmed_name.truncate(10);
        if trimmed_name.is_empty() {
            return -1;
        }
        let name_upper = trimmed_name.to_uppercase();

        // Find slot: first empty OR matching name
        let mut found_idx: Option<usize> = None;

        // First try to find matching name
        for idx in 0..6 {
            if self.user_eq_names[idx].trim().to_uppercase() == name_upper {
                found_idx = Some(idx);
                break;
            }
        }

        // If no matching name, find first empty slot
        if found_idx.is_none() {
            for idx in 0..6 {
                if self.user_eq_names[idx].trim().is_empty() {
                    found_idx = Some(idx);
                    break;
                }
            }
        }

        // No empty slot and no matching name
        if found_idx.is_none() {
            return -2; // No slot available
        }

        let idx = found_idx.unwrap();

        // Save ALL DSP (EQ + FX)
        self.user_eq_names[idx] = name_upper;
        self.user_eq_gains[idx] = self.eq_bands;
        self.user_eq_macro[idx] = self.fader_offset as f32;

        // FX settings
        self.user_fx_enabled[idx] = self.bass_active
            || self.crystal_active
            || self.surround_active
            || self.compressor_active;
        self.user_fx_bass_enabled[idx] = self.bass_active;
        self.user_fx_bass_gain[idx] = self.bass_gain as f32;
        self.user_fx_bass_cutoff[idx] = self.bass_cutoff as f32;
        self.user_fx_bass_mode[idx] = self.bass_mode;
        self.user_fx_crystal_enabled[idx] = self.crystal_active;
        self.user_fx_crystal_amount[idx] = self.crystal_amount as f32;
        self.user_fx_surround_enabled[idx] = self.surround_active;
        self.user_fx_surround_width[idx] = self.surround_width as f32;
        self.user_fx_mono_enabled[idx] = self.mono_active;
        self.user_fx_mono_width[idx] = self.mono_width as f32;
        self.user_fx_stereo_enabled[idx] = self.stereo_active;
        self.user_fx_stereo_amount[idx] = self.stereo_amount as f32;
        self.user_fx_crossfeed_enabled[idx] = self.crossfeed_active;
        self.user_fx_crossfeed_amount[idx] = self.crossfeed_amount as f32;
        self.user_fx_compressor_enabled[idx] = self.compressor_active;
        self.user_fx_compressor_threshold[idx] = self.compressor_threshold as f32;
        self.user_fx_reverb_enabled[idx] = self.reverb_active;
        self.user_fx_reverb_mode[idx] = self.reverb_mode;
        self.user_fx_reverb_amount[idx] = self.reverb_amount;

        self.save_config();
        idx as i32
    }

    pub fn get_eq_preset_count(&self) -> i32 {
        self.eq_presets.len() as i32
    }

    pub fn get_eq_preset_name(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.eq_presets.len() {
            QString::from(self.eq_presets[index as usize].name.clone())
        } else {
            QString::default()
        }
    }

    pub fn get_eq_preset_gains(&self, index: i32) -> QVariantList {
        let mut list = QVariantList::default();
        if index >= 0 && (index as usize) < self.eq_presets.len() {
            for &gain in &self.eq_presets[index as usize].gains {
                list.push(QVariant::from(gain as f64));
            }
        }
        list
    }

    pub fn get_fx_preset_count(&self) -> i32 {
        self.fx_presets.len() as i32
    }

    pub fn get_fx_preset_name(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.fx_presets.len() {
            QString::from(self.fx_presets[index as usize].name.clone())
        } else {
            QString::default()
        }
    }

    pub fn get_user_eq_gains(&self, preset: i32) -> QVariantList {
        let mut list = QVariantList::default();
        if preset >= 0 && preset < 6 {
            for &gain in &self.user_eq_gains[preset as usize] {
                list.push(QVariant::from(gain as f64));
            }
        }
        list
    }

    pub fn get_user_eq_macro(&self, preset: i32) -> f64 {
        if preset >= 0 && preset < 6 {
            self.user_eq_macro[preset as usize] as f64
        } else {
            0.0
        }
    }

    pub fn get_user_preset_name(&self, preset: i32) -> QString {
        if preset >= 0 && preset < 6 {
            QString::from(self.user_eq_names[preset as usize].clone())
        } else {
            QString::default()
        }
    }

    // --- PRESET LOADING METHODS ---
    pub fn load_eq_preset(&mut self, index: i32) {
        if index < 0 || (index as usize) >= self.eq_presets.len() {
            return;
        }

        let preset = &self.eq_presets[index as usize];

        for (i, &gain) in preset.gains.iter().enumerate() {
            self.eq_bands[i] = gain;
            let arc = crate::audio::dsp::eq::get_eq_bands_arc();
            arc[i].store(gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        self.eqBandsChanged();
        self.active_preset_index = index;
        self.active_preset_index_changed();
        self.save_config();
    }

    pub fn load_preset(&mut self, index: i32) {
        if index < 0 || index > 11 {
            return;
        }

        if index == self.active_preset_index {
            return;
        }

        let (eq_source, fx_source, use_factory_fx) = if index < 6 {
            // FACTORY PRESET (0-5): Load ALL from internal code (presets.rs)
            // JSON is completely IGNORED for these indices
            (
                PresetSource::Factory(index as usize),
                PresetSource::Factory(index as usize),
                true,
            )
        } else {
            // USER PRESET (6-11)
            let user_idx = (index - 6) as usize;
            if self.user_eq_names[user_idx].trim().is_empty() {
                // INVALID/EMPTY: FALLBACK to LOONIX (factory preset 0)
                (PresetSource::Factory(0), PresetSource::Factory(0), true)
            } else {
                // VALID: Load user saved EQ + FX
                self.load_user_fx_preset(user_idx);
                (
                    PresetSource::User(user_idx),
                    PresetSource::User(user_idx),
                    true,
                )
            }
        };

        // Load EQ bands from source
        match eq_source {
            PresetSource::Factory(idx) => {
                self.fader_offset = 0.0;
                if idx < self.eq_presets.len() {
                    let eq_preset = &self.eq_presets[idx];
                    for (i, &gain) in eq_preset.gains.iter().enumerate() {
                        self.eq_bands[i] = gain;
                        let arc = crate::audio::dsp::eq::get_eq_bands_arc();
                        arc[i].store(gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
                    }
                }
            }
            PresetSource::User(idx) => {
                self.fader_offset = self.user_eq_macro[idx] as f64;
                for i in 0..10 {
                    let gain = self.user_eq_gains[idx][i];
                    self.eq_bands[i] = gain;
                    let effective = (gain as f64 + self.fader_offset).clamp(-20.0, 20.0) as f32;
                    let arc = crate::audio::dsp::eq::get_eq_bands_arc();
                    arc[i].store(effective.to_bits(), std::sync::atomic::Ordering::Relaxed);
                }
            }
            PresetSource::Preserve => {}
        }

        // Load FX from source (only for factory presets)
        if use_factory_fx {
            match fx_source {
                PresetSource::Factory(idx) => {
                    if idx < self.fx_presets.len() {
                        self.load_fx_preset(idx as i32);
                    }
                }
                _ => {}
            }
        }

        // Sinkronisasi Signal ke QML UI
        self.faderOffsetChanged();
        self.eqBandsChanged();

        self.active_preset_index = index;
        self.active_preset_index_changed();

        // Only save untuk user preset (6-11), bukan default preset (0-5)
        if index >= 6 {
            self.save_config();
        }
    }

    pub fn load_fx_preset(&mut self, index: i32) {
        if index < 0 || (index as usize) >= self.fx_presets.len() {
            return;
        }

        let preset = self.fx_presets[index as usize].clone();

        self.bass_active = preset.bass_enabled; 
        self.bass_gain = preset.bass_gain as f64;
        self.bass_cutoff = preset.bass_cutoff as f64;
        self.bass_mode = preset.bass_mode as i32;
        
        crate::audio::dsp::bassbooster::get_bass_enabled_arc()
            .store(self.bass_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::bassbooster::get_bass_gain_arc().store(
            preset.bass_gain.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
            preset.bass_cutoff.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        
        self.bass_active_changed();
        self.bass_gain_changed();
        self.bass_cutoff_changed();
        self.bass_mode_changed();

        self.crystal_active = preset.crystal_enabled || preset.crystal_amount > 0.0;
        self.crystal_amount = preset.crystal_amount as f64;
        self.crystal_freq = preset.crystal_freq as f64;
        crate::audio::dsp::crystalizer::get_crystal_enabled_arc()
            .store(preset.crystal_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::crystalizer::get_crystal_amount_arc().store(
            preset.crystal_amount.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crystal_active_changed();
        self.crystal_amount_changed();
        self.crystal_freq_changed();

        self.surround_active = preset.surround_enabled || preset.surround_width > 0.0;
        self.surround_width = preset.surround_width.clamp(0.0, 2.0) as f64;
        crate::audio::dsp::surround::get_surround_enabled_arc().store(
            preset.surround_enabled,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::surround::get_surround_width_arc().store(
            preset.surround_width.clamp(0.0, 2.0).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.surround_active_changed();
        self.surround_width_changed();

        self.mono_active = preset.mono_enabled;
        self.mono_width = preset.mono_width as f64;
        crate::audio::dsp::stereowidth::get_mono_enabled_arc()
            .store(preset.mono_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::stereowidth::get_mono_width_arc().store(
            preset.mono_width.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.mono_changed();
        self.mono_width_changed();

        self.pitch_active = preset.pitch_enabled;
        self.pitch_semitones = preset.pitch_semitones as f64;
        crate::audio::dsp::pitchshifter::get_pitch_enabled_arc()
            .store(preset.pitch_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::pitchshifter::get_pitch_ratio_arc().store(
            2.0_f32.powf(preset.pitch_semitones / 12.0).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.pitch_changed();

        self.middle_active = preset.middle_enabled;
        self.middle_amount = preset.middle_amount as f64;
        crate::audio::dsp::middleclarity::get_middle_enabled_arc()
            .store(preset.middle_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::middleclarity::get_middle_amount_arc().store(
            preset.middle_amount.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.middle_changed();
        self.middle_amount_changed();

        self.stereo_active = preset.stereo_enabled || preset.stereo_amount > 0.0;
        self.stereo_amount = preset.stereo_amount as f64;
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
            .store(self.stereo_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc().store(
            preset.stereo_amount.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.stereo_changed();
        self.stereo_amount_changed();

        self.crossfeed_active = preset.crossfeed_enabled;
        self.crossfeed_amount = preset.crossfeed_amount as f64;
        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc().store(
            preset.crossfeed_enabled,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::crossfeed::get_crossfeed_amount_arc().store(
            preset.crossfeed_amount.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crossfeed_changed();
        self.crossfeed_amount_changed();

        self.compressor_active = preset.compressor_enabled;
        self.compressor_threshold = preset.compressor_threshold.clamp(-60.0, 0.0) as f64;
        crate::audio::dsp::compressor::get_compressor_enabled_arc()
            .store(self.compressor_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            (self.compressor_threshold as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.compressor_active_changed();
        self.compressor_threshold_changed();

        // Reverb - Murni snapshot
        self.reverb_active = preset.reverb_enabled;
        self.reverb_mode = preset.reverb_mode as i32;
        self.reverb_amount = preset.reverb_amount as i32;

        crate::audio::dsp::reverb::get_reverb_enabled_arc()
            .store(self.reverb_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::reverb::get_reverb_mode_arc().store(
            self.reverb_mode as u32,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::reverb::get_reverb_amount_arc().store(
            self.reverb_amount as u32,
            std::sync::atomic::Ordering::Relaxed,
        );

        self.reverb_active_changed();
        self.reverb_mode_changed();
        self.reverb_amount_changed(); 
        self.reverb_changed();

        self.default_fx_snapshot = Some(preset.clone());
        self.save_config();
    }

    pub fn load_user_fx_preset(&mut self, idx: usize) {
        // Load user saved FX settings
        if !self.user_fx_enabled[idx] {
            return;
        }

        // Bass
        self.bass_active = self.user_fx_bass_enabled[idx];
        self.bass_gain = self.user_fx_bass_gain[idx] as f64;
        self.bass_cutoff = self.user_fx_bass_cutoff[idx] as f64;
        self.bass_mode = self.user_fx_bass_mode[idx];
        
        crate::audio::dsp::bassbooster::get_bass_enabled_arc().store(
            self.user_fx_bass_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::bassbooster::get_bass_gain_arc().store(
            self.user_fx_bass_gain[idx].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
            self.user_fx_bass_cutoff[idx].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        
        self.bass_active_changed();
        self.bass_gain_changed();
        self.bass_cutoff_changed();
        self.bass_mode_changed();

        // Crystalizer
        self.crystal_active = self.user_fx_crystal_enabled[idx];
        self.crystal_amount = self.user_fx_crystal_amount[idx] as f64;
        crate::audio::dsp::crystalizer::get_crystal_enabled_arc().store(
            self.user_fx_crystal_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::crystalizer::get_crystal_amount_arc().store(
            self.user_fx_crystal_amount[idx].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crystal_active_changed();

        // Surround
        self.surround_active = self.user_fx_surround_enabled[idx];
        self.surround_width = self.user_fx_surround_width[idx] as f64;
        crate::audio::dsp::surround::get_surround_enabled_arc().store(
            self.user_fx_surround_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::surround::get_surround_width_arc().store(
            self.user_fx_surround_width[idx].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.surround_active_changed();
        self.surround_width_changed();

        // Mono
        self.mono_active = self.user_fx_mono_enabled[idx];
        self.mono_width = self.user_fx_mono_width[idx] as f64;
        crate::audio::dsp::stereowidth::get_mono_enabled_arc().store(
            self.user_fx_mono_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        self.mono_changed();
        self.mono_width_changed();

        // Stereo
        self.stereo_active = self.user_fx_stereo_enabled[idx];
        self.stereo_amount = self.user_fx_stereo_amount[idx] as f64;
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc().store(
            self.user_fx_stereo_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        self.stereo_changed();
        self.stereo_amount_changed();

        // Crossfeed
        self.crossfeed_active = self.user_fx_crossfeed_enabled[idx];
        self.crossfeed_amount = self.user_fx_crossfeed_amount[idx] as f64;
        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc().store(
            self.user_fx_crossfeed_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crossfeed_changed();
        self.crossfeed_amount_changed();

        // Compressor
        self.compressor_active = self.user_fx_compressor_enabled[idx];
        self.compressor_threshold = self.user_fx_compressor_threshold[idx].clamp(-60.0, 0.0) as f64;
        crate::audio::dsp::compressor::get_compressor_enabled_arc().store(
            self.user_fx_compressor_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::compressor::get_compressor_threshold_arc()
            .store((self.compressor_threshold as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.compressor_active_changed();
        self.compressor_threshold_changed();

        // Reverb - Murni dari array user
        self.reverb_active = self.user_fx_reverb_enabled[idx];
        self.reverb_mode = self.user_fx_reverb_mode[idx];
        self.reverb_amount = self.user_fx_reverb_amount[idx];

        crate::audio::dsp::reverb::get_reverb_enabled_arc().store(
            self.reverb_active,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::reverb::get_reverb_mode_arc().store(
            self.reverb_mode as u32,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::reverb::get_reverb_amount_arc().store(
            self.reverb_amount as u32,
            std::sync::atomic::Ordering::Relaxed,
        );

        self.reverb_active_changed();
        self.reverb_mode_changed();
        self.reverb_amount_changed();
        self.reverb_changed();

        self.default_fx_snapshot = Some(FxPreset {
            name: String::from("User"),
            bass_enabled: self.user_fx_bass_enabled[idx],
            bass_gain: self.user_fx_bass_gain[idx],
            bass_cutoff: self.user_fx_bass_cutoff[idx],
            bass_mode: self.user_fx_bass_mode[idx],
            crystal_enabled: self.user_fx_crystal_enabled[idx],
            crystal_amount: self.user_fx_crystal_amount[idx],
            crystal_freq: 4000.0,
            surround_enabled: self.user_fx_surround_enabled[idx],
            surround_width: self.user_fx_surround_width[idx],
            mono_enabled: self.user_fx_mono_enabled[idx],
            mono_width: self.user_fx_mono_width[idx],
            pitch_enabled: false,
            pitch_semitones: 0.0,
            middle_enabled: false,
            middle_amount: 0.0,
            stereo_enabled: self.user_fx_stereo_enabled[idx],
            stereo_amount: self.user_fx_stereo_amount[idx],
            crossfeed_enabled: self.user_fx_crossfeed_enabled[idx],
            crossfeed_amount: self.user_fx_crossfeed_amount[idx],
            compressor_enabled: self.user_fx_compressor_enabled[idx],
            compressor_threshold: self.user_fx_compressor_threshold[idx],
            reverb_enabled: self.user_fx_reverb_enabled[idx],
            reverb_mode: self.user_fx_reverb_mode[idx],
            reverb_amount: self.user_fx_reverb_amount[idx],
        });
    }

    // --- RESET METHODS ---
    pub fn compressor_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            let db = default.compressor_threshold.clamp(-60.0, 0.0);
            self.compressor_threshold = db as f64;
            self.compressor_active = default.compressor_enabled;
            crate::audio::dsp::compressor::get_compressor_enabled_arc()
                .store(self.compressor_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
                (db as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.compressor_active_changed();
            self.compressor_threshold_changed();
        }
    }

    pub fn surround_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.surround_active = default.surround_enabled || default.surround_width > 0.0;
            self.surround_width = default.surround_width.clamp(0.0, 2.0) as f64;
            crate::audio::dsp::surround::get_surround_enabled_arc()
                .store(self.surround_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::surround::get_surround_width_arc().store(
                default.surround_width.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.surround_active_changed();
            self.surround_width_changed();
        }
    }

    pub fn stereo_width_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.mono_active = default.mono_enabled;
            self.mono_width = default.mono_width.clamp(0.0, 2.0) as f64;
            crate::audio::dsp::stereowidth::get_mono_enabled_arc()
                .store(self.mono_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::stereowidth::get_mono_width_arc().store(
                default.mono_width.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.mono_changed();
            self.mono_width_changed();
        }
    }

    pub fn middle_clarity_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.middle_active = default.middle_enabled || default.middle_amount > 0.0;
            self.middle_amount = default.middle_amount as f64;
            crate::audio::dsp::middleclarity::get_middle_enabled_arc()
                .store(self.middle_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::middleclarity::get_middle_amount_arc().store(
                default.middle_amount.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.middle_changed();
            self.middle_amount_changed();
        }
    }

    pub fn stereo_enhance_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.stereo_active = default.stereo_enabled || default.stereo_amount > 0.0;
            self.stereo_amount = default.stereo_amount.clamp(0.0, 1.0) as f64;
            crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
                .store(self.stereo_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::stereoenhance::get_stereo_amount_arc().store(
                default.stereo_amount.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.stereo_changed();
            self.stereo_amount_changed();
        }
    }

    pub fn crossfeed_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.crossfeed_active = default.crossfeed_enabled || default.crossfeed_amount > 0.0;
            self.crossfeed_amount = default.crossfeed_amount.clamp(0.0, 1.0) as f64;
            crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc()
                .store(self.crossfeed_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::crossfeed::get_crossfeed_amount_arc().store(
                default.crossfeed_amount.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.crossfeed_changed();
            self.crossfeed_amount_changed();
        }
    }

    pub fn crystalizer_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.crystal_active = default.crystal_enabled || default.crystal_amount > 0.0;
            self.crystal_amount = default.crystal_amount.clamp(0.0, 1.0) as f64;
            self.crystal_freq = default.crystal_freq as f64;
            crate::audio::dsp::crystalizer::get_crystal_enabled_arc()
                .store(self.crystal_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::crystalizer::get_crystal_amount_arc().store(
                default.crystal_amount.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.crystal_active_changed();
            self.crystal_amount_changed();
            self.crystal_freq_changed();
        }
    }

    pub fn bass_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.bass_active = default.bass_enabled;
            self.bass_gain = default.bass_gain as f64;
            self.bass_cutoff = default.bass_cutoff as f64;
            self.bass_mode = default.bass_mode as i32;
            
            crate::audio::dsp::bassbooster::get_bass_enabled_arc()
                .store(self.bass_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::bassbooster::get_bass_gain_arc().store(
                default.bass_gain.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
                default.bass_cutoff.to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            
            self.bass_active_changed();
            self.bass_gain_changed();
            self.bass_cutoff_changed();
            self.bass_mode_changed();
        }
    }

    pub fn reverb_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            // Pola BASS: Pakai data asli snapshot, buang logika "|| amount > 0"
            self.reverb_active = default.reverb_enabled;
            self.reverb_mode = default.reverb_mode;
            self.reverb_amount = default.reverb_amount;

            crate::audio::dsp::reverb::get_reverb_enabled_arc()
                .store(self.reverb_active, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::reverb::get_reverb_mode_arc().store(
                self.reverb_mode as u32,
                std::sync::atomic::Ordering::Relaxed,
            );
            crate::audio::dsp::reverb::get_reverb_amount_arc().store(
                self.reverb_amount as u32,
                std::sync::atomic::Ordering::Relaxed,
            );

            // Trigger UI Update (Sama persis polanya ama Bass Reset)
            self.reverb_active_changed(); 
            self.reverb_mode_changed();
            self.reverb_amount_changed();
            self.reverb_changed();
        }
    }

    pub fn pitch_indie_reset(&mut self) {
        if let Some(default) = &self.default_fx_snapshot {
            self.pitch_semitones = default.pitch_semitones as f64;
            self.pitch_active = default.pitch_enabled;
            crate::audio::dsp::pitchshifter::get_pitch_enabled_arc()
                .store(default.pitch_enabled, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::pitchshifter::get_pitch_ratio_arc().store(
                2.0_f32.powf(default.pitch_semitones / 12.0).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
            self.pitch_changed();
        }
    }

    // --- EQ INSTANT APPLY ---
    pub fn set_eq_instant_apply(&mut self) {
        // No-op: eq.rs already does instant updates
    }

    // --- RESET ALL TO FLAT ---
    pub fn reset_all(&mut self) {
        // Reset EQ bands to flat (all zeros)
        for band in self.eq_bands.iter_mut() {
            *band = 0.0;
        }
        // Sync EQ bands to audio
        self.sync_eq_bands();

        // Reset preamp and fader
        self.set_preamp_gain(0.0);
        self.set_fader(0.0);

        // Turn off all FX and reset amounts
        if self.compressor_active {
            self.toggle_compressor();
        }
        self.set_compressor_threshold(1.0);

        if self.surround_active {
            self.toggle_surround();
        }
        self.set_surround_width(1.8);

        if self.mono_active {
            self.toggle_stereo_width();
        }
        self.set_stereo_width_amount(1.0);

        if self.middle_active {
            self.toggle_middle_clarity();
        }
        self.set_middle_clarity_amount(0.0);

        if self.stereo_active {
            self.toggle_stereo_enhance();
        }
        self.set_stereo_enhance_amount(0.0);

        if self.crossfeed_active {
            self.toggle_crossfeed();
        }
        self.set_crossfeed_amount(0.0);

        if self.crystal_active {
            self.toggle_crystalizer();
        }
        self.set_crystalizer_amount(0.0);

        if self.bass_active {
            self.toggle_bass();
        }
        self.set_bass_gain(0.0);
        self.set_bass_cutoff(180.0);
        self.set_bass_mode(0);

        if self.reverb_active {
            self.toggle_reverb();
        }
        self.set_reverb_amount(50);
        self.set_reverb_mode(1);

        if self.pitch_active {
            self.toggle_pitch();
        }
        self.set_pitch_semitones(0.0);

        self.active_preset_index = 0;
    }
}
