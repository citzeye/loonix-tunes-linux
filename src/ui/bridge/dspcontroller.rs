/* --- loonixtunesv2/src/ui/bridge/dspcontroller.rs | dspcontroller --- */

use crate::audio::config::{AppConfig, DspConfig, EqPreset, FxPreset};
// removed duplicate import
use crate::core::config::DspConfigManager;
use qmetaobject::prelude::*;
use qmetaobject::{QObject, QString, QVariant, QVariantList};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
enum PresetSource {
    Factory(usize),
    User(usize),
}

#[derive(QObject, Default)]
pub struct DspController {
    base: qt_base_class!(trait QObject),
    pub ffmpeg: Arc<Mutex<crate::audio::engine::FfmpegEngine>>,
    pub config_manager: DspConfigManager,

    pub reverb_preset: u32,
    pub eq_presets: Vec<EqPreset>,
    pub fx_presets: Vec<FxPreset>,
    pub default_fx_snapshot: Option<FxPreset>,

    pub dsp_enabled: qt_property!(bool; NOTIFY dsp_changed),
    pub dsp_changed: qt_signal!(),
    pub reverb_active: qt_property!(bool; NOTIFY reverb_active_changed),
    pub reverb_active_changed: qt_signal!(),
    pub reverb_mode: qt_property!(i32; NOTIFY reverb_mode_changed),
    pub reverb_mode_changed: qt_signal!(),
    pub reverb_amount: qt_property!(f64; NOTIFY reverb_amount_changed),
    pub reverb_amount_changed: qt_signal!(),
    pub reverb_room_size: qt_property!(f64; NOTIFY reverb_room_size_changed),
    pub reverb_room_size_changed: qt_signal!(),
    pub reverb_damp: qt_property!(f64; NOTIFY reverb_damp_changed),
    pub reverb_damp_changed: qt_signal!(),

    pub bass_active: qt_property!(bool; NOTIFY bass_active_changed),
    pub bass_active_changed: qt_signal!(),
    pub bass_gain: qt_property!(f64; NOTIFY bass_gain_changed),
    pub bass_gain_changed: qt_signal!(),
    pub bass_cutoff: qt_property!(f64; NOTIFY bass_cutoff_changed),
    pub bass_cutoff_changed: qt_signal!(),
    pub bass_mode: qt_property!(i32; NOTIFY bass_mode_changed),
    pub bass_mode_changed: qt_signal!(),

    pub surround_active: qt_property!(bool; NOTIFY surround_active_changed),
    pub surround_active_changed: qt_signal!(),
    pub surround_width: qt_property!(f64; NOTIFY surround_width_changed),
    pub surround_width_changed: qt_signal!(),

    pub crystal_active: qt_property!(bool; NOTIFY crystal_active_changed),
    pub crystal_active_changed: qt_signal!(),
    pub crystal_amount: qt_property!(f64; NOTIFY crystal_amount_changed),
    pub crystal_amount_changed: qt_signal!(),
    pub crystal_freq: qt_property!(f64; NOTIFY crystal_freq_changed),
    pub crystal_freq_changed: qt_signal!(),

    pub compressor_active: qt_property!(bool; NOTIFY compressor_active_changed),
    pub compressor_active_changed: qt_signal!(),
    pub compressor_threshold: qt_property!(f64; NOTIFY compressor_threshold_changed),
    pub compressor_threshold_changed: qt_signal!(),
    pub compressor_makeup: qt_property!(f64; NOTIFY compressor_makeup_changed),
    pub compressor_makeup_changed: qt_signal!(),

    pub mono_active: qt_property!(bool; NOTIFY mono_changed),
    pub mono_changed: qt_signal!(),
    pub mono_width: qt_property!(f64; NOTIFY mono_width_changed),
    pub mono_width_changed: qt_signal!(),

    pub middle_active: qt_property!(bool; NOTIFY middle_changed),
    pub middle_changed: qt_signal!(),
    pub middle_amount: qt_property!(f64; NOTIFY middle_amount_changed),
    pub middle_amount_changed: qt_signal!(),

    pub stereo_active: qt_property!(bool; NOTIFY stereo_changed),
    pub stereo_changed: qt_signal!(),
    pub stereo_amount: qt_property!(f64; NOTIFY stereo_amount_changed),
    pub stereo_amount_changed: qt_signal!(),

    pub crossfeed_active: qt_property!(bool; NOTIFY crossfeed_changed),
    pub crossfeed_changed: qt_signal!(),
    pub crossfeed_amount: qt_property!(f64; NOTIFY crossfeed_amount_changed),
    pub crossfeed_amount_changed: qt_signal!(),

    pub eq_enabled: qt_property!(bool; NOTIFY eq_enabled_changed),
    pub eq_enabled_changed: qt_signal!(),
    pub eq_bands: qt_property!(QVariantList; NOTIFY eq_bands_changed),
    pub eq_bands_raw: [f32; 10],
    pub eq_bands_changed: qt_signal!(),
    pub fader_offset: qt_property!(f64; NOTIFY fader_offset_changed),
    pub fader_offset_changed: qt_signal!(),

    pub pitch_active: qt_property!(bool; NOTIFY pitch_changed),
    pub pitch_changed: qt_signal!(),
    pub pitch_semitones: qt_property!(f64; NOTIFY pitch_changed),

    pub preamp_active: qt_property!(bool; NOTIFY preamp_changed),
    pub preamp_changed: qt_signal!(),
    pub limiter_active: qt_property!(bool; NOTIFY limiter_changed),
    pub limiter_changed: qt_signal!(),
    pub normalizer_enabled: qt_property!(bool; NOTIFY normalizer_changed),
    pub normalizer_changed: qt_signal!(),

    pub active_preset_index: qt_property!(i32; NOTIFY active_preset_index_changed),
    pub active_preset_index_changed: qt_signal!(),

    pub user_preset_names: qt_property!(QVariantList; NOTIFY user_preset_names_changed),
    pub user_preset_names_changed: qt_signal!(),

    pub user_eq_names: [String; 6],
    pub user_eq_gains: [[f32; 10]; 6],
    pub eq_bands_internal: [f32; 10],
    pub user_eq_macro: [f32; 6],
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

    pub normalizer_target_lufs: f64,
    pub normalizer_true_peak_dbtp: f64,
    pub normalizer_max_gain_db: f64,
    pub normalizer_smoothing: f64,

    // DSP wrapper methods for QML
    pub set_reverb_mode: qt_method!(fn(&mut self, mode: i32)),
    pub set_reverb_amount: qt_method!(fn(&mut self, amount: f64)),
    pub set_reverb_room_size: qt_method!(fn(&mut self, val: f64)),
    pub set_reverb_damp: qt_method!(fn(&mut self, val: f64)),
    pub toggle_reverb: qt_method!(fn(&mut self)),
    pub toggle_bass_booster: qt_method!(fn(&mut self)),
    pub set_bass_gain: qt_method!(fn(&mut self, val: f64)),
    pub set_bass_cutoff: qt_method!(fn(&mut self, val: f64)),
    pub set_bass_mode: qt_method!(fn(&mut self, mode: i32)),
    pub toggle_surround: qt_method!(fn(&mut self)),
    pub set_surround_width: qt_method!(fn(&mut self, val: f64)),
    pub toggle_crystalizer: qt_method!(fn(&mut self)),
    pub set_crystalizer_amount: qt_method!(fn(&mut self, val: f64)),
    pub toggle_compressor: qt_method!(fn(&mut self)),
    pub get_compressor_threshold: qt_method!(fn(&self) -> f64),
    pub set_compressor_threshold: qt_method!(fn(&mut self, val: f64)),
    pub get_compressor_makeup: qt_method!(fn(&self) -> f64),
    pub set_compressor_makeup: qt_method!(fn(&mut self, val: f64)),
    pub toggle_pitch: qt_method!(fn(&mut self)),
    pub set_pitch_semitones: qt_method!(fn(&mut self, val: f64)),
    pub toggle_middle_clarity: qt_method!(fn(&mut self)),
    pub set_middle_clarity_amount: qt_method!(fn(&mut self, val: f64)),
    pub toggle_stereo_width: qt_method!(fn(&mut self)),
    pub set_stereo_width_amount: qt_method!(fn(&mut self, val: f64)),
    pub toggle_stereo_enhance: qt_method!(fn(&mut self)),
    pub set_stereo_enhance_amount: qt_method!(fn(&mut self, val: f64)),
    pub toggle_crossfeed: qt_method!(fn(&mut self)),
    pub set_crossfeed_amount: qt_method!(fn(&mut self, val: f64)),
    pub toggle_dsp: qt_method!(fn(&mut self)),
    pub reset_all: qt_method!(fn(&mut self)),
    pub toggle_preamp: qt_method!(fn(&mut self)),
    pub toggle_limiter: qt_method!(fn(&mut self)),
    pub toggle_normalizer: qt_method!(fn(&mut self)),
    pub set_eq_band: qt_method!(fn(&mut self, index: i32, gain: f64)),
    pub set_fader: qt_method!(fn(&mut self, offset: f64)),
    pub set_eq_enabled: qt_method!(fn(&mut self, enabled: bool)),
    pub set_eq_instant_apply: qt_method!(fn(&mut self)),
    pub get_preamp_gain: qt_method!(fn(&self) -> f64),
    pub set_preamp_gain: qt_method!(fn(&mut self, gain: f64)),
    pub save_user_eq: qt_method!(fn(&mut self, preset: i32, name: String, macro_val: f64)),
    pub save_user_preset: qt_method!(fn(&mut self, slot: usize, name: String) -> i32),
    pub get_eq_preset_count: qt_method!(fn(&self) -> i32),
    pub get_eq_preset_name: qt_method!(fn(&self, index: i32) -> QString),
    pub get_eq_preset_gains: qt_method!(fn(&self, index: i32) -> QVariantList),
    pub get_fx_preset_count: qt_method!(fn(&self) -> i32),
    pub get_fx_preset_name: qt_method!(fn(&self, index: i32) -> QString),
    pub load_preset: qt_method!(fn(&mut self, index: i32)),
    pub load_eq_preset: qt_method!(fn(&mut self, index: i32)),
    pub load_fx_preset: qt_method!(fn(&mut self, index: i32)),
    pub set_active_preset_index: qt_method!(fn(&mut self, index: i32)),
    pub get_active_preset_index: qt_method!(fn(&self) -> i32),
    pub get_user_eq_gains: qt_method!(fn(&self, preset: i32) -> QVariantList),
    pub get_user_eq_macro: qt_method!(fn(&self, preset: i32) -> f64),
    pub get_user_preset_name: qt_method!(fn(&self, preset: i32) -> QString),
    pub reset_compressor: qt_method!(fn(&mut self)),
    pub reset_surround: qt_method!(fn(&mut self)),
    pub reset_stereo_width: qt_method!(fn(&mut self)),
    pub reset_middle_clarity: qt_method!(fn(&mut self)),
    pub reset_stereo_enhance: qt_method!(fn(&mut self)),
    pub reset_crossfeed: qt_method!(fn(&mut self)),
    pub reset_crystalizer: qt_method!(fn(&mut self)),
    pub reset_bass: qt_method!(fn(&mut self)),
    pub reset_reverb: qt_method!(fn(&mut self)),
    pub reset_pitch: qt_method!(fn(&mut self)),
}

impl DspController {
    pub fn new(
        ffmpeg: Arc<Mutex<crate::audio::engine::FfmpegEngine>>,
    ) -> Self {
        let mut controller = Self::default();
        controller.eq_bands_raw = [0.0; 10];
        controller.eq_bands = QVariantList::default();
        controller.ffmpeg = ffmpeg;
        controller.config_manager = crate::core::config::DspConfigManager::new();
        controller.eq_presets = AppConfig::get_eq_presets();
        controller.fx_presets = AppConfig::get_fx_presets();

        controller
    }

    pub fn init_from_config(&mut self, config: &AppConfig) {
        // eprintln!("[DSP] init_from_config called - START");
        
        self.normalizer_enabled = config.normalizer_enabled;
        self.normalizer_target_lufs = config.normalizer_target_lufs as f64;
        self.normalizer_true_peak_dbtp = config.normalizer_true_peak_dbtp as f64;
        self.normalizer_max_gain_db = config.normalizer_max_gain_db as f64;
        self.normalizer_smoothing = config.normalizer_smoothing as f64;

        crate::audio::dsp::normalizer::get_normalizer_smoothing_arc().store(
            config.normalizer_smoothing.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        let dsp_settings = crate::audio::dsp::DspSettings::default();
        self.dsp_enabled = dsp_settings.dsp_enabled;
        self.eq_enabled = dsp_settings.eq_enabled;

        self.eq_presets = AppConfig::get_eq_presets();
        self.fx_presets = AppConfig::get_fx_presets();

        // Flow A/B: Check if dsp.json exists
        let dsp_path = DspConfig::dsp_path();
        // eprintln!("[DSP] dsp_path: {:?}", dsp_path);
        let is_fresh_install = dsp_path.map(|p| !p.exists()).unwrap_or(true);
        // eprintln!("[DSP] is_fresh_install: {}", is_fresh_install);

        if is_fresh_install {
            // Flow A: Fresh install - dsp.json does not exist
            // eprintln!("[DSP] Flow A: Creating fresh dsp.json");
            let mut new_config = DspConfig::dsp_user_template();
            new_config.active_preset_index = 0;
            new_config.dsp_enabled = true;
            if let Err(save_err) = new_config.save() {
                eprintln!("Error saving dsp config: {:?}", save_err);
            }
            // eprintln!("[DSP] Flow A: save_result: {:?}", save_result);

            // Initialize user preset names
            self.user_eq_names = new_config.user_preset_names.clone();
            self.user_preset_names = self.get_user_preset_names_list();
            self.user_preset_names_changed();

            // Set DSP enabled state AND store to Audio Engine (Atomic)
            self.dsp_enabled = true;
            crate::audio::dsp::get_dsp_bypass_arc()
                .store(false, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::preamp::get_preamp_enabled_arc()
                .store(true, std::sync::atomic::Ordering::Relaxed);
            self.dsp_changed();

            // Load factory preset 0 (Loonix Default)
            // Reset to -1 to force load (default is 0 which causes early return)
            self.active_preset_index = -1;
            self.load_preset(0);
            self.active_preset_index_changed();
        } else {
            // Flow B: Routine reload - dsp.json exists
            let dsp_config = DspConfig::load();

            // Restore dsp_enabled state from JSON AND store to Audio Engine
            self.dsp_enabled = dsp_config.dsp_enabled;
            crate::audio::dsp::get_dsp_bypass_arc()
                .store(!self.dsp_enabled, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::eqpreamp::get_preamp_enabled_arc()
                .store(true, std::sync::atomic::Ordering::Relaxed); // Preamp always ON
            // Convert dB from config to Linear for engine
            let linear_gain = if dsp_config.preamp_db != 0.0 {
                10.0_f32.powf(dsp_config.preamp_db / 20.0)
            } else {
                1.0_f32
            };
            crate::audio::dsp::eqpreamp::get_preamp_gain_arc()
                .store(linear_gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
            self.dsp_changed();

            // Load user preset data from JSON
            self.user_eq_names = dsp_config.user_preset_names.clone();
            self.user_preset_names = self.get_user_preset_names_list();
            self.user_preset_names_changed();
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

            // Determine preset source
            let preset_index = dsp_config.active_preset_index.clamp(0, 11);
            // Reset to -1 to force load (avoid early return when same index)
            self.active_preset_index = -1;  
            let actual_index = preset_index;
            if preset_index <= 5 {
                // Flow B: Factory preset (SSoT from preset.rs)
                self.load_preset(actual_index);
            } else {
                // Flow B: User preset (from dsp.json)
                self.load_preset(actual_index);
            }
            self.active_preset_index_changed();
        }
    }

    fn apply_bass_mode(&mut self, mode: i32) {
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

        self.bass_cutoff_changed();
    }

    pub fn save_config(&mut self) {
        let state = self.get_state_view();
        self.config_manager.save_dsp_config(&state);
    }

    pub fn get_state_view(&self) -> crate::core::config::DspStateView {
        crate::core::config::DspStateView {
            dsp_enabled: self.dsp_enabled,
            dsp_bands: self.eq_bands_internal,
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

    // Ganti fungsi sync_eq_bands lu jadi begini:
    pub fn sync_eq_bands(&mut self) -> QVariantList {
        let mut list = QVariantList::default();
        for &gain in &self.eq_bands_internal {
            let effective = (gain as f64 + self.fader_offset).clamp(-20.0, 20.0);
            list.push(QVariant::from(effective));
        }
        list
    }

    pub fn emit_all_signals(&mut self) {
        // EQ & DSP
        self.dsp_changed();
        self.eq_enabled_changed();
        self.eq_bands_changed();
        self.fader_offset_changed();
        // Bass
        self.bass_active_changed();
        self.bass_gain_changed();
        self.bass_cutoff_changed();
        self.bass_mode_changed();
        // Crystalizer
        self.crystal_active_changed();
        self.crystal_amount_changed();
        self.crystal_freq_changed();
        // Surround
        self.surround_active_changed();
        self.surround_width_changed();
        // Stereo Width
        self.mono_changed();
        self.mono_width_changed();
        // Middle Clarity
        self.middle_changed();
        self.middle_amount_changed();
        // Stereo Enhance
        self.stereo_changed();
        self.stereo_amount_changed();
        // Crossfeed
        self.crossfeed_changed();
        self.crossfeed_amount_changed();
        // Compressor
        self.compressor_active_changed();
        self.compressor_threshold_changed();
        // Reverb
        self.reverb_active_changed();
        self.reverb_mode_changed();
        self.reverb_amount_changed();
        self.reverb_damp_changed();
        self.reverb_room_size_changed();
        // Pitch
        self.pitch_changed();
        // Preamp & Limiter
        self.preamp_changed();
        self.limiter_changed();
        // Normalizer
        self.normalizer_changed();
        // Presets
        self.active_preset_index_changed();
        self.user_preset_names_changed();
    }

    pub fn set_reverb_mode(&mut self, mode: i32) {
        let mode = mode.clamp(0, 3) as u32;
        self.reverb_preset = mode;
        self.reverb_mode = mode as i32;
        self.reverb_active = true;

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
        self.reverb_damp_changed();
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

        let current_amount = self.reverb_amount as u32;
        crate::audio::dsp::reverb::get_reverb_amount_arc()
            .store(current_amount, std::sync::atomic::Ordering::Relaxed);

        self.reverb_preset = preset_id;
        self.reverb_mode = preset_id as i32;
        self.reverb_active = preset_id > 0;
        self.reverb_active_changed();
        self.reverb_mode_changed();
        self.reverb_amount_changed();
        self.reverb_damp_changed();
    }

    pub fn reset_pitch(&mut self) {
        let dsp_default = crate::audio::dsp::DspSettings::default();

        if let Some(ref snapshot) = self.default_fx_snapshot {
            self.pitch_active = snapshot.pitch_enabled;
            self.pitch_semitones = snapshot.pitch_semitones as f64;
            crate::audio::dsp::pitchshifter::get_pitch_enabled_arc()
                .store(snapshot.pitch_enabled, std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::pitchshifter::get_pitch_ratio_arc().store(
                2.0_f32.powf(snapshot.pitch_semitones / 12.0).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        } else {
            self.pitch_active = dsp_default.pitch_enabled;
            self.pitch_semitones = dsp_default.pitch_semitones as f64;
            crate::audio::dsp::pitchshifter::get_pitch_enabled_arc().store(
                dsp_default.pitch_enabled,
                std::sync::atomic::Ordering::Relaxed,
            );
            crate::audio::dsp::pitchshifter::get_pitch_ratio_arc().store(
                2.0_f32.powf(dsp_default.pitch_semitones / 12.0).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        self.pitch_changed();
    }

    pub fn reset_all(&mut self) {
        let dsp_settings = crate::audio::dsp::DspSettings::default();

        self.dsp_enabled = dsp_settings.dsp_enabled;
        self.eq_enabled = dsp_settings.eq_enabled;

        self.eq_bands_internal = [0.0; 10];
        self.fader_offset = 0.0;

        let arc_eq = crate::audio::dsp::eq::get_eq_bands_arc();
        for i in 0..10 {
            arc_eq[i].store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        self.bass_active = false;
        crate::audio::dsp::bassbooster::get_bass_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.surround_active = false;
        crate::audio::dsp::surround::get_surround_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.crystal_active = false;
        crate::audio::dsp::crystalizer::get_crystal_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::crystalizer::get_crystal_amount_arc()
            .store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.compressor_active = false;
        crate::audio::dsp::compressor::get_compressor_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.reverb_active = false;
        crate::audio::dsp::reverb::get_reverb_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.pitch_active = false;
        crate::audio::dsp::pitchshifter::get_pitch_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.middle_active = false;
        crate::audio::dsp::middleclarity::get_middle_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.mono_active = false;
        crate::audio::dsp::stereowidth::get_mono_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.stereo_active = false;
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.crossfeed_active = false;
        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc()
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.bass_gain = 0.0;
        self.bass_cutoff = 80.0;
        self.bass_mode = 0;
        crate::audio::dsp::bassbooster::get_bass_gain_arc()
            .store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::bassbooster::get_bass_freq_arc()
            .store(80.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.surround_width = 1.8;
        crate::audio::dsp::surround::get_surround_width_arc()
            .store(1.8_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.crystal_amount = 0.0;

        self.compressor_threshold = -14.0; // Protocol IV: -14.0 dB for clean slate
        crate::audio::dsp::compressor::get_compressor_threshold_arc()
            .store((-14.0_f32).to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.middle_amount = 0.0;
        crate::audio::dsp::middleclarity::get_middle_amount_arc()
            .store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.stereo_amount = 0.0;
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc()
            .store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.crossfeed_amount = 0.0;
        crate::audio::dsp::crossfeed::get_crossfeed_amount_arc()
            .store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.eq_bands = self.sync_eq_bands();
        self.eq_bands_changed();

        self.user_preset_names = self.get_user_preset_names_list();
        self.user_preset_names_changed();

        self.active_preset_index = 0;
        self.active_preset_index_changed();

        // Protocol IV: Update snapshot to clean state
        self.default_fx_snapshot = Some(crate::audio::config::FxPreset {
            name: "Clean Slate".to_string(),
            bass_enabled: false,
            bass_gain: 0.0,
            bass_cutoff: 80.0,
            bass_mode: 0,
            crystal_enabled: false,
            crystal_amount: 0.0,
            crystal_freq: 4000.0,
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
            compressor_enabled: false,
            compressor_threshold: -14.0,
            reverb_enabled: false,
            reverb_mode: 0,
            reverb_amount: 0,
        });

        self.emit_all_signals();
    }

    pub fn get_active_preset_index(&self) -> i32 {
        self.active_preset_index
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

    pub fn get_user_preset_names_list(&self) -> QVariantList {
        let mut list = QVariantList::default();
        for i in 0..6 {
            let name = if self.user_eq_names[i].is_empty() {
                format!("User {}", i + 1)
            } else {
                self.user_eq_names[i].clone()
            };
            list.push(QString::from(name).into());
        }
        list
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

    pub fn load_eq_preset(&mut self, index: i32) {
        if index < 0 || (index as usize) >= self.eq_presets.len() {
            return;
        }

        let preset = &self.eq_presets[index as usize];

        for (i, &gain) in preset.gains.iter().enumerate() {
            self.eq_bands_internal[i] = gain;
            let arc = crate::audio::dsp::eq::get_eq_bands_arc();
            arc[i].store(gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        self.eq_bands = self.sync_eq_bands();
        self.eq_bands_changed();
        self.active_preset_index = index;
        self.active_preset_index_changed();
    }

    pub fn load_preset(&mut self, index: i32) {
        // eprintln!("[DSP] load_preset START - index: {}, current active_preset_index: {}", index, self.active_preset_index);
        
        if index < 0 || index > 11 {
            // eprintln!("[DSP] load_preset: index out of range");
            return;
        }

        if index == self.active_preset_index {
            // eprintln!("[DSP] load_preset: SAME INDEX - returning early!");
            return;
        }

        // eprintln!("[DSP] load_preset called with index: {}", index);

        let (eq_source, fx_source, use_factory_fx) = if index < 6 {
            (
                PresetSource::Factory(index as usize),
                PresetSource::Factory(index as usize),
                true,
            )
        } else {
            let user_idx = (index - 6) as usize;
            if self.user_eq_names[user_idx].trim().is_empty() {
                (PresetSource::Factory(0), PresetSource::Factory(0), true)
            } else {
                self.load_user_fx_preset(user_idx);
                (
                    PresetSource::User(user_idx),
                    PresetSource::User(user_idx),
                    true,
                )
            }
        };

        match eq_source {
            PresetSource::Factory(idx) => {
                self.fader_offset = 0.0;
                if idx < self.eq_presets.len() {
                    let eq_preset = &self.eq_presets[idx];
                    for (i, &gain) in eq_preset.gains.iter().enumerate() {
                        self.eq_bands_internal[i] = gain;
                        let arc = crate::audio::dsp::eq::get_eq_bands_arc();
                        arc[i].store(gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
                    }
                }
            }
            PresetSource::User(idx) => {
                self.fader_offset = self.user_eq_macro[idx] as f64;
                for i in 0..10 {
                    let gain = self.user_eq_gains[idx][i];
                    self.eq_bands_internal[i] = gain;
                    let effective = (gain as f64 + self.fader_offset).clamp(-20.0, 20.0) as f32;
                    let arc = crate::audio::dsp::eq::get_eq_bands_arc();
                    arc[i].store(effective.to_bits(), std::sync::atomic::Ordering::Relaxed);
                }
            }
        }

        self.eq_bands = self.sync_eq_bands();
        self.eq_bands_changed();

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

        self.fader_offset_changed();
        self.eq_bands_changed();

        self.active_preset_index = index;
        self.active_preset_index_changed();

        // =====================================================
        // AUTO-SAVE HANYA INDEX KE dsp.json (Hybrid Persistence)
        // =====================================================
        let mut dsp_config = crate::audio::config::DspConfig::load();
        dsp_config.active_preset_index = index;
        if let Err(e) = dsp_config.save() {
            eprintln!("Error saving dsp config: {:?}", e);
        } else {
            // eprintln!("[DSP] Auto-saved active_preset_index: {}", index);
        }
    }

    pub fn set_active_preset_index(&mut self, index: i32) {
        if index >= 0 && index < 6 {
            self.load_preset(index);
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
        self.surround_width = preset.surround_width.clamp(0.0, 1.0) as f64;
        crate::audio::dsp::surround::get_surround_enabled_arc().store(
            preset.surround_enabled,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::surround::get_surround_width_arc().store(
            preset.surround_width.clamp(0.0, 1.0).to_bits(),
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

        self.reverb_active = preset.reverb_enabled;
        self.reverb_mode = preset.reverb_mode as i32;
        self.reverb_amount = preset.reverb_amount as f64;

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
        self.reverb_damp_changed();

        self.default_fx_snapshot = Some(preset.clone());
    }

    pub fn load_user_fx_preset(&mut self, idx: usize) {
        let temp_preset = crate::audio::config::FxPreset {
            name: "".to_string(),
            bass_enabled: self.user_fx_bass_enabled[idx],
            bass_gain: self.user_fx_bass_gain[idx],
            bass_cutoff: self.user_fx_bass_cutoff[idx],
            bass_mode: self.user_fx_bass_mode[idx],
            crystal_enabled: self.user_fx_crystal_enabled[idx],
            crystal_amount: self.user_fx_crystal_amount[idx],
            crystal_freq: 4000.0,
            pitch_enabled: false,
            pitch_semitones: 0.0,
            middle_enabled: false,
            middle_amount: 0.5,
            surround_enabled: self.user_fx_surround_enabled[idx],
            surround_width: self.user_fx_surround_width[idx],
            mono_enabled: self.user_fx_mono_enabled[idx],
            mono_width: self.user_fx_mono_width[idx],
            stereo_enabled: self.user_fx_stereo_enabled[idx],
            stereo_amount: self.user_fx_stereo_amount[idx],
            crossfeed_enabled: self.user_fx_crossfeed_enabled[idx],
            crossfeed_amount: self.user_fx_crossfeed_amount[idx],
            compressor_enabled: self.user_fx_compressor_enabled[idx],
            compressor_threshold: self.user_fx_compressor_threshold[idx],
            reverb_enabled: self.user_fx_reverb_enabled[idx],
            reverb_mode: self.user_fx_reverb_mode[idx],
            reverb_amount: self.user_fx_reverb_amount[idx],
        };

        if !self.user_fx_enabled[idx] {
            self.default_fx_snapshot = Some(temp_preset);
            return;
        }

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
        self.crystal_amount_changed();

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

        self.mono_active = self.user_fx_mono_enabled[idx];
        self.mono_width = self.user_fx_mono_width[idx] as f64;
        crate::audio::dsp::stereowidth::get_mono_enabled_arc().store(
            self.user_fx_mono_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        self.mono_changed();
        self.mono_width_changed();

        self.stereo_active = self.user_fx_stereo_enabled[idx];
        self.stereo_amount = self.user_fx_stereo_amount[idx] as f64;
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc().store(
            self.user_fx_stereo_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        self.stereo_changed();
        self.stereo_amount_changed();

        self.crossfeed_active = self.user_fx_crossfeed_enabled[idx];
        self.crossfeed_amount = self.user_fx_crossfeed_amount[idx] as f64;
        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc().store(
            self.user_fx_crossfeed_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::crossfeed::get_crossfeed_amount_arc().store(
            self.user_fx_crossfeed_amount[idx].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crossfeed_changed();
        self.crossfeed_amount_changed();

        self.compressor_active = self.user_fx_compressor_enabled[idx];
        self.compressor_threshold = self.user_fx_compressor_threshold[idx] as f64;
        crate::audio::dsp::compressor::get_compressor_enabled_arc().store(
            self.user_fx_compressor_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            self.user_fx_compressor_threshold[idx].to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.compressor_active_changed();
        self.compressor_threshold_changed();

        self.reverb_active = self.user_fx_reverb_enabled[idx];
        self.reverb_mode = self.user_fx_reverb_mode[idx];
        self.reverb_amount = self.user_fx_reverb_amount[idx] as f64;
        crate::audio::dsp::reverb::get_reverb_enabled_arc().store(
            self.user_fx_reverb_enabled[idx],
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::reverb::get_reverb_mode_arc().store(
            self.user_fx_reverb_mode[idx] as u32,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::reverb::get_reverb_amount_arc().store(
            self.user_fx_reverb_amount[idx] as u32,
            std::sync::atomic::Ordering::Relaxed,
        );
        self.reverb_active_changed();
        self.reverb_mode_changed();
        self.reverb_amount_changed();
        self.reverb_damp_changed();

        // Create a temporary preset object to store as snapshot
        let temp_preset = crate::audio::config::FxPreset {
            name: "".to_string(),
            bass_enabled: self.bass_active,
            bass_gain: self.bass_gain as f32,
            bass_cutoff: self.bass_cutoff as f32,
            bass_mode: self.bass_mode as i32,
            crystal_enabled: self.crystal_active,
            crystal_amount: self.crystal_amount as f32,
            crystal_freq: self.crystal_freq as f32,
            pitch_enabled: self.pitch_active,
            pitch_semitones: self.pitch_semitones as f32,
            middle_enabled: self.middle_active,
            middle_amount: self.middle_amount as f32,
            surround_enabled: self.surround_active,
            surround_width: self.surround_width as f32,
            mono_enabled: self.mono_active,
            mono_width: self.mono_width as f32,
            stereo_enabled: self.stereo_active,
            stereo_amount: self.stereo_amount as f32,
            crossfeed_enabled: self.crossfeed_active,
            crossfeed_amount: self.crossfeed_amount as f32,
            compressor_enabled: self.compressor_active,
            compressor_threshold: self.compressor_threshold as f32,
            reverb_enabled: self.reverb_active,
            reverb_mode: self.reverb_mode as i32,
            reverb_amount: self.reverb_amount as i32,
        };

        self.default_fx_snapshot = Some(temp_preset);

        self.pitch_changed();

        self.eq_bands = self.sync_eq_bands();
        self.eq_bands_changed();
    }

    // ==========================================
    // 1. MASTER & POWER CONTROLS
    // ==========================================
    pub fn toggle_dsp(&mut self) {
        self.dsp_enabled = !self.dsp_enabled;
        
        // Store to DSP Module Atomic (for Rack bypass)
        // INVERTED: dsp_bypass = true means DSP is OFF (bypass cosmetic chain)
        crate::audio::dsp::get_dsp_bypass_arc()
            .store(!self.dsp_enabled, std::sync::atomic::Ordering::Relaxed);
        
        // Push to FfmpegEngine for real-time audio processing
        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_dsp_enabled(self.dsp_enabled);
        }
        
        self.dsp_changed();
    }

    pub fn toggle_preamp(&mut self) {
        self.preamp_active = !self.preamp_active;
        crate::audio::dsp::eqpreamp::get_preamp_enabled_arc()
            .store(self.preamp_active, std::sync::atomic::Ordering::Relaxed);
        self.preamp_changed();
    }

    pub fn get_preamp_gain(&self) -> f64 {
        let linear = f32::from_bits(
            crate::audio::dsp::eqpreamp::get_preamp_gain_arc()
                .load(std::sync::atomic::Ordering::Relaxed),
        );
        // Convert Linear back to dB for UI display
        // 1.0 → 0 dB, 2.0 → +6 dB, 0.5 → -6 dB
        if linear > 0.0 {
            (20.0_f32 * linear.log10()) as f64
        } else {
            0.0
        }
    }

    pub fn set_preamp_gain(&mut self, gain: f64) {
        let clamped_db = gain.clamp(-20.0, 20.0) as f32;
        
        // Convert dB to Linear gain before sending to engine
        // 0 dB → 1.0 linear, +6 dB → 2.0 linear, -6 dB → 0.5 linear
        let linear_gain = 10.0_f32.powf(clamped_db / 20.0);
        
        crate::audio::dsp::eqpreamp::get_preamp_gain_arc()
            .store(linear_gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
        
        // Save dB value to config for UI display
        let mut dsp_cfg = crate::audio::config::DspConfig::load();
        dsp_cfg.preamp_db = clamped_db;
        let _ = dsp_cfg.save();
        
        self.preamp_changed();
    }

    pub fn toggle_limiter(&mut self) {
        self.limiter_active = !self.limiter_active;
        crate::audio::dsp::limiter::get_limiter_enabled_arc()
            .store(self.limiter_active, std::sync::atomic::Ordering::Relaxed);
        self.limiter_changed();
    }

    pub fn toggle_normalizer(&mut self) {
        self.normalizer_enabled = !self.normalizer_enabled;
        // Internal normalizer logic
        self.normalizer_changed();
    }

    // ==========================================
    // 2. EQUALIZER CONTROLS
    // ==========================================
    pub fn set_eq_enabled(&mut self, enabled: bool) {
        self.eq_enabled = enabled;
        self.eq_enabled_changed();
    }

    pub fn set_eq_band(&mut self, index: i32, gain: f64) {
        if index >= 0 && index < 10 {
            let clamped_gain = gain.clamp(-20.0, 20.0) as f32;
            self.eq_bands_internal[index as usize] = clamped_gain;

            let effective = (clamped_gain + self.fader_offset as f32).clamp(-20.0, 20.0);
            let arc = crate::audio::dsp::eq::get_eq_bands_arc();
            arc[index as usize].store(effective.to_bits(), std::sync::atomic::Ordering::Relaxed);

            self.eq_bands = self.sync_eq_bands();
            self.eq_bands_changed();
        }
    }

    pub fn set_fader(&mut self, offset: f64) {
        self.fader_offset = offset.clamp(-20.0, 20.0);
        let arc = crate::audio::dsp::eq::get_eq_bands_arc();

        for i in 0..10 {
            let base_gain = self.eq_bands_internal[i];
            let effective = (base_gain + self.fader_offset as f32).clamp(-20.0, 20.0);
            arc[i].store(effective.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        self.eq_bands = self.sync_eq_bands();
        self.fader_offset_changed();
        self.eq_bands_changed();
    }

    pub fn set_eq_instant_apply(&mut self) {
        // Dummy trigger for QML if needed, actual atomics are handled in set_eq_band
        self.eq_bands_changed();
    }

    // ==========================================
    // 3. BASS & FREQUENCY FX
    // ==========================================
    pub fn toggle_bass_booster(&mut self) {
        self.bass_active = !self.bass_active;
        crate::audio::dsp::bassbooster::get_bass_enabled_arc()
            .store(self.bass_active, std::sync::atomic::Ordering::Relaxed);
        self.bass_active_changed();
    }

    pub fn set_bass_gain(&mut self, val: f64) {
        self.bass_gain = val;
        crate::audio::dsp::bassbooster::get_bass_gain_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.bass_gain_changed();
    }

    pub fn set_bass_cutoff(&mut self, val: f64) {
        self.bass_cutoff = val;
        crate::audio::dsp::bassbooster::get_bass_freq_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.bass_cutoff_changed();
    }

    pub fn set_bass_mode(&mut self, mode: i32) {
        let clamped = mode.clamp(0, 3);
        self.bass_mode = clamped;
        self.apply_bass_mode(clamped);
        self.bass_mode_changed();
    }

    pub fn toggle_crystalizer(&mut self) {
        self.crystal_active = !self.crystal_active;
        crate::audio::dsp::crystalizer::get_crystal_enabled_arc()
            .store(self.crystal_active, std::sync::atomic::Ordering::Relaxed);
        self.crystal_active_changed();
    }

    pub fn set_crystalizer_amount(&mut self, val: f64) {
        self.crystal_amount = val;
        crate::audio::dsp::crystalizer::get_crystal_amount_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.crystal_amount_changed();
    }

    pub fn toggle_middle_clarity(&mut self) {
        self.middle_active = !self.middle_active;
        crate::audio::dsp::middleclarity::get_middle_enabled_arc()
            .store(self.middle_active, std::sync::atomic::Ordering::Relaxed);
        self.middle_changed();
    }

    pub fn set_middle_clarity_amount(&mut self, val: f64) {
        self.middle_amount = val;
        crate::audio::dsp::middleclarity::get_middle_amount_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.middle_amount_changed();
    }

    // ==========================================
    // 4. SPATIAL & STEREO FX
    // ==========================================
    pub fn toggle_surround(&mut self) {
        self.surround_active = !self.surround_active;
        crate::audio::dsp::surround::get_surround_enabled_arc()
            .store(self.surround_active, std::sync::atomic::Ordering::Relaxed);
        self.surround_active_changed();
    }

    pub fn set_surround_width(&mut self, val: f64) {
        let actual_width = val * 2.0;
        self.surround_width = actual_width;
        crate::audio::dsp::surround::get_surround_width_arc()
            .store((actual_width as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.surround_width_changed();
    }

    pub fn toggle_stereo_width(&mut self) {
        self.mono_active = !self.mono_active;
        crate::audio::dsp::stereowidth::get_mono_enabled_arc()
            .store(self.mono_active, std::sync::atomic::Ordering::Relaxed);
        self.mono_changed();
    }

    pub fn set_stereo_width_amount(&mut self, val: f64) {
        self.mono_width = val;
        crate::audio::dsp::stereowidth::get_mono_width_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.mono_width_changed();
    }

    pub fn toggle_stereo_enhance(&mut self) {
        self.stereo_active = !self.stereo_active;
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
            .store(self.stereo_active, std::sync::atomic::Ordering::Relaxed);
        self.stereo_changed();
    }

    pub fn set_stereo_enhance_amount(&mut self, val: f64) {
        self.stereo_amount = val;
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.stereo_amount_changed();
    }

    pub fn toggle_crossfeed(&mut self) {
        self.crossfeed_active = !self.crossfeed_active;
        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc()
            .store(self.crossfeed_active, std::sync::atomic::Ordering::Relaxed);
        self.crossfeed_changed();
    }

    pub fn set_crossfeed_amount(&mut self, val: f64) {
        self.crossfeed_amount = val;
        crate::audio::dsp::crossfeed::get_crossfeed_amount_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.crossfeed_amount_changed();
    }

    // ==========================================
    // 5. COMPRESSOR & REVERB & PITCH
    // ==========================================
    pub fn toggle_compressor(&mut self) {
        self.compressor_active = !self.compressor_active;
        crate::audio::dsp::compressor::get_compressor_enabled_arc()
            .store(self.compressor_active, std::sync::atomic::Ordering::Relaxed);
        self.compressor_active_changed();
    }

    pub fn get_compressor_threshold(&self) -> f64 {
        self.compressor_threshold
    }

    pub fn set_compressor_threshold(&mut self, val: f64) {
        self.compressor_threshold = val.clamp(-60.0, 0.0);
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            (self.compressor_threshold as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.compressor_threshold_changed();
    }

    pub fn get_compressor_makeup(&self) -> f64 {
        f32::from_bits(
            crate::audio::dsp::compressor::get_compressor_makeup_arc()
                .load(std::sync::atomic::Ordering::Relaxed),
        ) as f64
    }

    pub fn set_compressor_makeup(&mut self, val: f64) {
        let clamped = val.clamp(0.0, 24.0) as f32;
        crate::audio::dsp::compressor::get_compressor_makeup_arc()
            .store(clamped.to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.compressor_makeup_changed();
    }

    pub fn toggle_reverb(&mut self) {
        self.reverb_active = !self.reverb_active;
        crate::audio::dsp::reverb::get_reverb_enabled_arc()
            .store(self.reverb_active, std::sync::atomic::Ordering::Relaxed);
        self.reverb_active_changed();
    }

    pub fn set_reverb_room_size(&mut self, val: f64) {
        self.reverb_room_size = val;
        // Add atomic store if your reverb DSP supports live room size adjustments
        self.reverb_room_size_changed();
    }

    pub fn set_reverb_damp(&mut self, val: f64) {
        self.reverb_damp = val;
        // Add atomic store if your reverb DSP supports live damp adjustments
        self.reverb_damp_changed();
    }

    pub fn toggle_pitch(&mut self) {
        self.pitch_active = !self.pitch_active;
        crate::audio::dsp::pitchshifter::get_pitch_enabled_arc()
            .store(self.pitch_active, std::sync::atomic::Ordering::Relaxed);
        self.pitch_changed();
    }

    pub fn set_pitch_semitones(&mut self, val: f64) {
        self.pitch_semitones = val;
        crate::audio::dsp::pitchshifter::get_pitch_ratio_arc().store(
            2.0_f32.powf((val as f32) / 12.0).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.pitch_changed();
    }

    // ==========================================
    // 6. SAVE PROTOCOLS (The only place where save_config is allowed)
    // ==========================================
    pub fn save_user_eq(&mut self, preset: i32, name: String, macro_val: f64) {
        if preset >= 0 && preset < 6 {
            self.user_eq_names[preset as usize] = name;
            self.user_eq_macro[preset as usize] = macro_val as f32;
            for i in 0..10 {
                self.user_eq_gains[preset as usize][i] = self.eq_bands_internal[i];
            }
            self.user_preset_names = self.get_user_preset_names_list();
            self.user_preset_names_changed();

            // This is explicit save action, so save_config is allowed.
            self.save_config();
        }
    }

    pub fn save_user_preset(&mut self, slot: usize, name: String) -> i32 {
        if slot >= 6 {
            return -1;
        }

        self.user_eq_names[slot] = name.clone();
        for i in 0..10 {
            self.user_eq_gains[slot][i] = self.eq_bands_internal[i];
        }

        self.user_fx_enabled[slot] = true;
        self.user_fx_bass_enabled[slot] = self.bass_active;
        self.user_fx_bass_gain[slot] = self.bass_gain as f32;
        self.user_fx_bass_cutoff[slot] = self.bass_cutoff as f32;
        self.user_fx_bass_mode[slot] = self.bass_mode;

        self.user_fx_crystal_enabled[slot] = self.crystal_active;
        self.user_fx_crystal_amount[slot] = self.crystal_amount as f32;

        self.user_fx_surround_enabled[slot] = self.surround_active;
        self.user_fx_surround_width[slot] = self.surround_width as f32;

        self.user_fx_mono_enabled[slot] = self.mono_active;
        self.user_fx_mono_width[slot] = self.mono_width as f32;

        self.user_fx_stereo_enabled[slot] = self.stereo_active;
        self.user_fx_stereo_amount[slot] = self.stereo_amount as f32;

        self.user_fx_crossfeed_enabled[slot] = self.crossfeed_active;
        self.user_fx_crossfeed_amount[slot] = self.crossfeed_amount as f32;

        self.user_fx_compressor_enabled[slot] = self.compressor_active;
        self.user_fx_compressor_threshold[slot] = self.compressor_threshold as f32;

        self.user_fx_reverb_enabled[slot] = self.reverb_active;
        self.user_fx_reverb_mode[slot] = self.reverb_mode;
        self.user_fx_reverb_amount[slot] = self.reverb_amount as i32;

        self.user_preset_names = self.get_user_preset_names_list();
        self.user_preset_names_changed();

        // Manual save allowed here
        self.save_config();

        slot as i32
    }

    // ==========================================
    // 7. MISSING SETTERS & INDIE RESETS (SNAPSHOT PROTOCOL)
    // ==========================================
    pub fn set_reverb_amount(&mut self, amount: f64) {
        let rounded = amount.round() as i32;
        self.reverb_amount = amount;
        crate::audio::dsp::reverb::get_reverb_amount_arc()
            .store(rounded as u32, std::sync::atomic::Ordering::Relaxed);
        self.reverb_amount_changed();
    }

    pub fn reset_bass(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.bass_active = snap.bass_enabled;
            self.bass_gain = snap.bass_gain as f64;
            self.bass_cutoff = snap.bass_cutoff as f64;
            self.bass_mode = snap.bass_mode as i32;
        } else {
            self.bass_active = false;
            self.bass_gain = 0.0;
            self.bass_cutoff = 80.0;
            self.bass_mode = 0;
        }
        crate::audio::dsp::bassbooster::get_bass_enabled_arc()
            .store(self.bass_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::bassbooster::get_bass_gain_arc().store(
            (self.bass_gain as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
            (self.bass_cutoff as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.bass_active_changed();
        self.bass_gain_changed();
        self.bass_cutoff_changed();
        self.bass_mode_changed();
    }

    pub fn reset_crystalizer(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.crystal_active = snap.crystal_enabled;
            self.crystal_amount = snap.crystal_amount as f64;
        } else {
            self.crystal_active = false;
            self.crystal_amount = 0.0;
        }
        crate::audio::dsp::crystalizer::get_crystal_enabled_arc()
            .store(self.crystal_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::crystalizer::get_crystal_amount_arc().store(
            (self.crystal_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crystal_active_changed();
        self.crystal_amount_changed();
    }

    pub fn reset_surround(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.surround_active = snap.surround_enabled;
            self.surround_width = snap.surround_width as f64;
        } else {
            self.surround_active = false;
            self.surround_width = 1.8;
        }
        crate::audio::dsp::surround::get_surround_enabled_arc()
            .store(self.surround_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::surround::get_surround_width_arc().store(
            (self.surround_width as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.surround_active_changed();
        self.surround_width_changed();
    }

    pub fn reset_stereo_width(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.mono_active = snap.mono_enabled;
            self.mono_width = snap.mono_width as f64;
        } else {
            self.mono_active = false;
            self.mono_width = 1.0;
        }
        crate::audio::dsp::stereowidth::get_mono_enabled_arc()
            .store(self.mono_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::stereowidth::get_mono_width_arc().store(
            (self.mono_width as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.mono_changed();
        self.mono_width_changed();
    }

    pub fn reset_stereo_enhance(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.stereo_active = snap.stereo_enabled;
            self.stereo_amount = snap.stereo_amount as f64;
        } else {
            self.stereo_active = false;
            self.stereo_amount = 0.0;
        }
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
            .store(self.stereo_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc().store(
            (self.stereo_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.stereo_changed();
        self.stereo_amount_changed();
    }

    pub fn reset_middle_clarity(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.middle_active = snap.middle_enabled;
            self.middle_amount = snap.middle_amount as f64;
        } else {
            self.middle_active = false;
            self.middle_amount = 0.0;
        }
        crate::audio::dsp::middleclarity::get_middle_enabled_arc()
            .store(self.middle_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::middleclarity::get_middle_amount_arc().store(
            (self.middle_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.middle_changed();
        self.middle_amount_changed();
    }

    pub fn reset_crossfeed(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.crossfeed_active = snap.crossfeed_enabled;
            self.crossfeed_amount = snap.crossfeed_amount as f64;
        } else {
            self.crossfeed_active = false;
            self.crossfeed_amount = 0.0;
        }
        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc()
            .store(self.crossfeed_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::crossfeed::get_crossfeed_amount_arc().store(
            (self.crossfeed_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crossfeed_changed();
        self.crossfeed_amount_changed();
    }

    pub fn reset_compressor(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.compressor_active = snap.compressor_enabled;
            self.compressor_threshold = snap.compressor_threshold as f64;
        } else {
            self.compressor_active = false;
            self.compressor_threshold = -14.0;
        }
        crate::audio::dsp::compressor::get_compressor_enabled_arc()
            .store(self.compressor_active, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            (self.compressor_threshold as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.compressor_active_changed();
        self.compressor_threshold_changed();
    }

    pub fn reset_reverb(&mut self) {
        if let Some(ref snap) = self.default_fx_snapshot {
            self.reverb_active = snap.reverb_enabled;
            self.reverb_mode = snap.reverb_mode as i32;
            self.reverb_amount = snap.reverb_amount as f64;
        } else {
            self.reverb_active = false;
            self.reverb_mode = 0;
            self.reverb_amount = 0.0;
        }
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
    }
}
