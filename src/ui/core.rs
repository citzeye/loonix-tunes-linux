/* --- LOONIX-TUNES src/ui/core.rs | The Bridge (QML)--- */

#![allow(non_snake_case)]

use crate::audio::audiooutput::AudioOutput;
use crate::audio::engine::{is_audio_file, AudioState, FfmpegEngine, MusicItem};

use crate::audio::dsp::abrepeat::ABRepeat;
use crate::audio::dsp::crystalizer::get_crystal_amount_arc;
use crate::audio::dsp::pitchshifter::{get_pitch_enabled_arc, get_pitch_ratio_arc};
use crate::audio::dsp::DspSettings;
use crate::core::dspconfig::{DspConfigManager, DspStateView};
use crate::core::library::LibraryManager;
use crate::core::playback::PlaybackController;
use dirs;
use qmetaobject::prelude::*;
use qmetaobject::QAbstractListModel;
use qmetaobject::QStringList;
use qmetaobject::QVariantList;
use qmetaobject::QVariantMap;

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

#[cfg(any(target_os = "linux", target_os = "windows"))]
use trash;

// Global storage for command line arguments
static COMMAND_LINE_FILES: OnceLock<Vec<String>> = OnceLock::new();

pub fn set_command_line_files(files: Vec<String>) {
    COMMAND_LINE_FILES.set(files).ok();
}

pub fn get_command_line_files() -> &'static Vec<String> {
    COMMAND_LINE_FILES.get_or_init(Vec::new)
}

/// Clean QML file:// URLs into proper OS paths
fn clean_qml_path(path: &str) -> String {
    // Handle raw file:// URLs
    if path.starts_with("file://") {
        if let Ok(url) = url::Url::parse(path) {
            if let Ok(p) = url.to_file_path() {
                return p.to_string_lossy().to_string();
            }
        }
    }
    // Handle QML-stripped paths (e.g. /C:/Users/... from file:///C:/Users/...)
    if path.len() > 3
        && path.starts_with('/')
        && path.as_bytes()[1].is_ascii_alphabetic()
        && path.as_bytes()[2] == b':'
    {
        return path[1..].to_string();
    }
    path.to_string()
}

#[derive(QObject, Default)]
pub struct MusicModel {
    base: qt_base_class!(trait QAbstractListModel),

    pub(crate) folders: HashMap<String, Vec<MusicItem>>,
    pub(crate) all_items: Vec<MusicItem>,
    pub(crate) display_list: Vec<MusicItem>,
    pub(crate) expanded_folders: HashSet<String>,
    #[allow(dead_code)]
    pub(crate) custom_folders: Vec<(String, String)>,
    #[allow(dead_code)]
    pub(crate) favorites: Vec<(String, String)>,
    pub(crate) external_files: Vec<MusicItem>,

    #[allow(dead_code)]
    pub(crate) current_folder: String,
    #[allow(dead_code)]
    pub(crate) current_folder_path: String,

    pub current_folder_qml: qt_property!(QString; NOTIFY current_folder_changed),
    pub current_folder_changed: qt_signal!(),

    pub(crate) ffmpeg: Arc<Mutex<FfmpegEngine>>,
    #[allow(dead_code)]
    pub(crate) audio: Arc<Mutex<AudioState>>,

    pub(crate) shuffle_active: bool,
    pub(crate) shuffle_queue: Vec<i32>,
    #[allow(dead_code)]
    pub(crate) queue_index: usize,
    pub(crate) loop_active: bool,
    pub(crate) abrepeat: ABRepeat,
    pub(crate) tick_counter: u32,

    pub(crate) output: AudioOutput,
    pub(crate) dsp_config: DspConfigManager,
    pub(crate) playback: PlaybackController,
    pub(crate) library: LibraryManager,

    pub is_playing: qt_property!(bool; NOTIFY playing_changed),
    pub playing_changed: qt_signal!(),

    pub current_title: qt_property!(QString; NOTIFY title_changed),
    pub title_changed: qt_signal!(),

    pub current_index: qt_property!(i32; NOTIFY current_index_changed),
    pub current_index_changed: qt_signal!(),

    pub position: qt_property!(i32; NOTIFY position_changed),
    pub position_changed: qt_signal!(),

    pub duration: qt_property!(i32; NOTIFY duration_changed),
    pub duration_changed: qt_signal!(),

    pub volume: qt_property!(f64; NOTIFY volume_changed),
    pub volume_changed: qt_signal!(),

    pub muted: qt_property!(bool; NOTIFY mute_changed),
    pub mute_changed: qt_signal!(),

    pub shuffle: qt_property!(bool; NOTIFY shuffle_changed),
    pub shuffle_changed: qt_signal!(),

    pub loop_playlist: qt_property!(bool; NOTIFY loop_changed),
    pub loop_changed: qt_signal!(),

    // --- DSP SECTION:
    // PANNING section Ui.qml
    pub balance: qt_property!(f64; NOTIFY balance_changed),
    pub balance_changed: qt_signal!(),

    // AB REPEAT section Ui.qml
    pub ab_state: qt_property!(i32; NOTIFY ab_state_changed),
    pub ab_state_changed: qt_signal!(),
    pub ab_point_a: qt_property!(i32; NOTIFY ab_point_a_changed),
    pub ab_point_a_changed: qt_signal!(),
    pub ab_point_b: qt_property!(i32; NOTIFY ab_point_b_changed),
    pub ab_point_b_changed: qt_signal!(),

    // MAGIC BUTTON section Ui.qml ---
    pub current_reverb: qt_property!(QString; NOTIFY reverb_changed),
    pub reverb_changed: qt_signal!(),
    pub reverb_active: qt_property!(bool; NOTIFY reverb_active_changed),
    pub reverb_active_changed: qt_signal!(),
    pub reverb_room_size: qt_property!(f64; NOTIFY reverb_params_changed),
    pub reverb_damp: qt_property!(f64; NOTIFY reverb_params_changed),
    pub reverb_params_changed: qt_signal!(),
    pub setStdReverbRoomSize: qt_method!(fn(&mut self, val: f64)),
    pub setStdReverbDamp: qt_method!(fn(&mut self, val: f64)),

    // BASSBOOSTER
    pub bass_gain: qt_property!(f64; NOTIFY bass_params_changed),
    pub bass_cutoff: qt_property!(f64; NOTIFY bass_params_changed),
    pub bass_mode: qt_property!(i32; NOTIFY bass_mode_changed),
    pub bass_params_changed: qt_signal!(),
    pub bass_mode_changed: qt_signal!(),
    pub setStdBassGain: qt_method!(fn(&mut self, val: f64)),
    pub setStdBassCutoff: qt_method!(fn(&mut self, val: f64)),
    pub set_bass_mode: qt_method!(fn(&mut self, mode: i32)),
    pub bass_magic_active: qt_property!(bool; NOTIFY bass_magic_changed),
    pub bass_magic_changed: qt_signal!(),
    pub toggleStdBassBooster: qt_method!(fn(&mut self)),

    // SURROUND
    pub surround_width: qt_property!(f64; NOTIFY surround_magic_changed),
    pub surround_magic_active: qt_property!(bool; NOTIFY surround_magic_changed),
    pub surround_magic_changed: qt_signal!(),
    pub toggleStdSurround: qt_method!(fn(&mut self)),
    pub setStdSurroundWidth: qt_method!(fn(&mut self, val: f64)),

    // CRYSTALIZER
    pub crystal_amount: qt_property!(f64; NOTIFY crystal_magic_changed),
    pub crystal_freq: qt_property!(f64; NOTIFY crystal_magic_changed),
    pub crystal_magic_active: qt_property!(bool; NOTIFY crystal_magic_changed),
    pub crystal_magic_changed: qt_signal!(),
    pub toggleStdCrystalizer: qt_method!(fn(&mut self)),
    pub set_crystalizer_amount: qt_method!(fn(&mut self, amount: f64)),

    // FX.qml
    pub compressor_active: qt_property!(bool; NOTIFY compressor_changed),
    pub compressor_changed: qt_signal!(),
    pub compressor_threshold: qt_property!(f64; NOTIFY compressor_changed),
    pub reset_std_compressor: qt_method!(fn(&mut self)),
    pub reset_std_surround: qt_method!(fn(&mut self)),
    pub reset_std_stereo_width: qt_method!(fn(&mut self)),
    pub reset_std_middle_clarity: qt_method!(fn(&mut self)),
    pub reset_std_stereo_enhance: qt_method!(fn(&mut self)),
    pub reset_std_crossfeed: qt_method!(fn(&mut self)),
    pub reset_std_crystalizer: qt_method!(fn(&mut self)),
    pub reset_std_bass: qt_method!(fn(&mut self)),
    pub dsp_enabled: qt_property!(bool; NOTIFY dsp_changed),
    pub dsp_changed: qt_signal!(),
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

    // EQ.qml
    eq_bands: [f32; 10],
    pub eq_enabled: qt_property!(bool; NOTIFY eq_enabled_changed),
    pub eq_enabled_changed: qt_signal!(),

    // Individual EQ band reactive properties with NOTIFY signals
    pub eq_band_0: qt_property!(f64; NOTIFY eq_band_0_changed),
    pub eq_band_0_changed: qt_signal!(),
    pub eq_band_1: qt_property!(f64; NOTIFY eq_band_1_changed),
    pub eq_band_1_changed: qt_signal!(),
    pub eq_band_2: qt_property!(f64; NOTIFY eq_band_2_changed),
    pub eq_band_2_changed: qt_signal!(),
    pub eq_band_3: qt_property!(f64; NOTIFY eq_band_3_changed),
    pub eq_band_3_changed: qt_signal!(),
    pub eq_band_4: qt_property!(f64; NOTIFY eq_band_4_changed),
    pub eq_band_4_changed: qt_signal!(),
    pub eq_band_5: qt_property!(f64; NOTIFY eq_band_5_changed),
    pub eq_band_5_changed: qt_signal!(),
    pub eq_band_6: qt_property!(f64; NOTIFY eq_band_6_changed),
    pub eq_band_6_changed: qt_signal!(),
    pub eq_band_7: qt_property!(f64; NOTIFY eq_band_7_changed),
    pub eq_band_7_changed: qt_signal!(),
    pub eq_band_8: qt_property!(f64; NOTIFY eq_band_8_changed),
    pub eq_band_8_changed: qt_signal!(),
    pub eq_band_9: qt_property!(f64; NOTIFY eq_band_9_changed),
    pub eq_band_9_changed: qt_signal!(),

    // Reactive property setters for individual bands
    pub set_eq_band_0: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_1: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_2: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_3: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_4: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_5: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_6: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_7: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_8: qt_method!(fn(&mut self, val: f64)),
    pub set_eq_band_9: qt_method!(fn(&mut self, val: f64)),

    // Combined preset loading methods
    pub load_eq_preset: qt_method!(fn(&mut self, index: i32)),
    pub load_fx_preset: qt_method!(fn(&mut self, index: i32)),

    // --- TRACK INFO & PRESETS ---
    pub track_info_visible: qt_property!(bool; NOTIFY track_info_visible_changed),
    pub track_info_visible_changed: qt_signal!(),
    pub track_info_title: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_artist: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_album: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_year: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_genre: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_duration: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_bitrate: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_sample_rate: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_channels: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_codec: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_file_size: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_file_path: qt_property!(QString; NOTIFY track_info_changed),
    pub track_info_changed: qt_signal!(),
    pub active_preset_index: qt_property!(i32; NOTIFY active_preset_index_changed),
    pub active_preset_index_changed: qt_signal!(),
    pub set_active_preset_index: qt_method!(fn(&mut self, index: i32)),
    pub get_active_preset_index: qt_method!(fn(&self) -> i32),
    user_eq_names: [String; 6],
    user_eq_gains: [[f32; 10]; 6],
    user_eq_macro: [f32; 6],
    eq_presets: Vec<crate::audio::config::EqPreset>,
    fx_presets: Vec<crate::audio::config::FxPreset>,
    reverb_preset: u32,

    // Saved config for persistence (shared with ThemeManager)
    pub(crate) saved_config:
        Option<std::sync::Arc<std::sync::Mutex<crate::audio::config::AppConfig>>>,

    pub scan_music: qt_method!(fn(&mut self)),
    pub scan_folder: qt_method!(fn(&mut self, path: String)),
    pub play_at: qt_method!(fn(&mut self, index: i32)),
    pub stop_playback: qt_method!(fn(&mut self)),
    pub play_next: qt_method!(fn(&mut self)),
    pub play_prev: qt_method!(fn(&mut self)),
    pub play_previous: qt_method!(fn(&mut self)),
    pub toggle_shuffle: qt_method!(fn(&mut self)),
    pub toggle_repeat: qt_method!(fn(&mut self)),
    pub seek_to: qt_method!(fn(&mut self, position: i32)),
    pub format_time: qt_method!(fn(&self, ms: i32) -> QString),
    pub set_volume: qt_method!(fn(&mut self, vol: f64)),
    pub set_balance: qt_method!(fn(&mut self, balance: f64)),
    pub set_reverb: qt_method!(fn(&mut self, reverb: QString)),
    pub toggleStdReverb: qt_method!(fn(&mut self)),
    pub toggle_mute: qt_method!(fn(&mut self)),
    pub toggle_abrepeat: qt_method!(fn(&mut self)),
    pub toggle_play: qt_method!(fn(&mut self)),
    pub toggleStdCompressor: qt_method!(fn(&mut self)),
    pub setStdCompressorThreshold: qt_method!(fn(&mut self, val: f64)),
    pub getStdCompressorThreshold: qt_method!(fn(&self) -> f64),
    pub toggleStdMiddleClarity: qt_method!(fn(&mut self)),
    pub setStdMiddleClarityAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleStdStereoWidth: qt_method!(fn(&mut self)),
    pub setStdStereoWidthAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleStdStereoEnhance: qt_method!(fn(&mut self)),
    pub setStdStereoEnhanceAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleStdCrossfeed: qt_method!(fn(&mut self)),
    pub setStdCrossfeedAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleStdPitch: qt_method!(fn(&mut self)),
    pub setStdPitchSemitones: qt_method!(fn(&mut self, val: f64)),
    pub toggleStdDsp: qt_method!(fn(&mut self)),
    pub preamp_active: qt_property!(bool; NOTIFY preamp_changed),
    pub preamp_changed: qt_signal!(),
    pub toggleStdPreamp: qt_method!(fn(&mut self)),
    pub limiter_active: qt_property!(bool; NOTIFY limiter_changed),
    pub limiter_changed: qt_signal!(),
    pub toggleStdLimiter: qt_method!(fn(&mut self)),
    pub normalizer_enabled: qt_property!(bool; NOTIFY normalizer_changed),
    pub normalizer_changed: qt_signal!(),
    pub toggle_normalizer: qt_method!(fn(&mut self)),
    pub normalizer_target_lufs: qt_property!(f64; NOTIFY normalizer_params_changed),
    pub normalizer_true_peak_dbtp: qt_property!(f64; NOTIFY normalizer_params_changed),
    pub normalizer_max_gain_db: qt_property!(f64; NOTIFY normalizer_params_changed),
    pub normalizer_smoothing: qt_property!(f64; NOTIFY normalizer_params_changed),
    pub normalizer_params_changed: qt_signal!(),
    pub set_normalizer_target_lufs: qt_method!(fn(&mut self, val: f64)),
    pub set_normalizer_true_peak_dbtp: qt_method!(fn(&mut self, val: f64)),
    pub set_normalizer_max_gain_db: qt_method!(fn(&mut self, val: f64)),
    pub set_normalizer_smoothing: qt_method!(fn(&mut self, val: f64)),
    pub get_normalizer_smoothing_label: qt_method!(fn(&self) -> QString),
    pub get_output_devices: qt_method!(fn(&self) -> QVariantList),
    pub set_output_device: qt_method!(fn(&mut self, index: i32)),
    pub set_eq_band: qt_method!(fn(&mut self, index: i32, gain: f64)),
    pub set_eq_enabled: qt_method!(fn(&mut self, enabled: bool)),
    pub set_eq_instant_apply: qt_method!(fn(&mut self)),
    pub get_eq_band_value: qt_method!(fn(&self, index: i32) -> f64),
    pub get_preamp_gain: qt_method!(fn(&self) -> f64),
    pub set_preamp_gain: qt_method!(fn(&mut self, gain: f64)),
    pub user_presets_changed: qt_signal!(),
    pub save_user_eq: qt_method!(fn(&mut self, preset: i32, name: String, macro_val: f64)),
    pub get_user_eq_gains: qt_method!(fn(&self, preset: i32) -> QVariantList),
    pub get_user_eq_macro: qt_method!(fn(&self, preset: i32) -> f64),
    pub get_user_preset_name: qt_method!(fn(&self, preset: i32) -> QString),
    pub get_eq_preset_count: qt_method!(fn(&self) -> i32),
    pub get_eq_preset_name: qt_method!(fn(&self, index: i32) -> QString),
    pub get_eq_preset_gains: qt_method!(fn(&self, index: i32) -> QVariantList),
    pub get_fx_preset_count: qt_method!(fn(&self) -> i32),
    pub get_fx_preset_name: qt_method!(fn(&self, index: i32) -> QString),
    pub toggle_folder: qt_method!(fn(&mut self, folder_name: String)),
    pub load_track_info: qt_method!(fn(&mut self, path: String)),
    pub close_track_info: qt_method!(fn(&mut self)),
    pub update_tick: qt_method!(fn(&mut self)),
    pub start_update_loop: qt_method!(fn(&mut self)),
    pub save_state: qt_method!(fn(&mut self)),
    pub save_window_position: qt_method!(fn(&mut self, x: i32, y: i32, width: i32, height: i32)),
    pub get_window_config: qt_method!(fn(&self) -> QVariantMap),
    pub add_folder_tab: qt_method!(fn(&mut self, path: String)),
    pub add_song: qt_method!(fn(&mut self, path: String)),
    pub add_temporary_folder: qt_method!(fn(&mut self, path: String)),
    pub remove_song: qt_method!(fn(&mut self, index: i32)),
    pub delete_item: qt_method!(fn(&mut self, path: String, is_folder: bool)),
    pub add_to_queue: qt_method!(fn(&mut self, path: String, name: String)),
    pub remove_from_queue: qt_method!(fn(&mut self, index: i32)),
    pub clear_queue: qt_method!(fn(&mut self)),
    pub get_queue_item: qt_method!(fn(&self, index: i32) -> QVariantMap),
    pub switch_to_queue: qt_method!(fn(&mut self)),
    pub queue_count: qt_property!(i32; NOTIFY queue_changed),
    pub queue_changed: qt_signal!(),
    pub user_queue: Vec<MusicItem>,
    pub switch_to_folder: qt_method!(fn(&mut self, folder_path: String)),
    pub save_custom_folders: qt_method!(fn(&mut self)),
    pub change_folder: qt_method!(fn(&mut self, index: i32, new_path: String)),
    pub remove_custom_folder: qt_method!(fn(&mut self, index: i32)),
    pub rename_folder: qt_method!(fn(&mut self, index: i32, new_name: String)),
    pub get_current_rename_name: qt_method!(fn(&self, index: i32) -> QString),
    pub get_custom_folder_name: qt_method!(fn(&self, index: i32) -> QString),
    pub get_custom_folder_path: qt_method!(fn(&self, index: i32) -> QString),
    pub get_custom_folder_count: qt_method!(fn(&self) -> i32),
    pub is_folder_locked: qt_method!(fn(&self, index: i32) -> bool),
    pub toggle_folder_lock: qt_method!(fn(&mut self, index: i32)),
    pub show_tab_context_menu: qt_method!(fn(&mut self, index: i32)),
    pub is_folder_expanded: qt_method!(fn(&self, folder_name: QString) -> bool),
    pub custom_folder_count: qt_property!(i32; NOTIFY custom_folders_changed),
    pub custom_folders_changed: qt_signal!(),
    pub folder_lock_changed: qt_signal!(),
    pub folder_lock_version: qt_property!(i32; NOTIFY folder_lock_changed),
    pub pitch_changed: qt_signal!(),
    pub pitch_active: qt_property!(bool; NOTIFY pitch_changed),
    pub pitch_semitones: qt_property!(f64; NOTIFY pitch_changed),

    // Theme sync - allow ThemeManager to update MusicModel's saved config
    pub sync_theme_to_config:
        qt_method!(fn(&mut self, theme_name: QString, custom_themes_json: QString)),

    // External files support
    pub external_files_count: qt_property!(i32; NOTIFY external_files_changed),
    pub external_files_changed: qt_signal!(),
    pub add_external_file: qt_method!(fn(&mut self, path: String)),
    pub switch_to_external_files: qt_method!(fn(&mut self)),
    pub clear_external_files: qt_method!(fn(&mut self)),
    pub process_command_line_files: qt_method!(fn(&mut self)),

    // Update checker
    pub update_status: qt_property!(QString; NOTIFY update_status_changed),
    pub update_available: qt_property!(bool; NOTIFY update_status_changed),
    pub update_status_changed: qt_signal!(),
    pub check_for_updates: qt_method!(fn(&mut self)),
    pub poll_update_result: qt_method!(fn(&mut self)),
    update_rx: Option<std::sync::mpsc::Receiver<String>>,

    // Device selector
    pub device_list: qt_property!(QStringList; NOTIFY device_list_changed),
    pub selected_device: qt_property!(QString; NOTIFY device_list_changed),
    pub device_list_changed: qt_signal!(),
    pub bluetooth_detected: qt_property!(bool; NOTIFY device_status_changed),
    pub systemMuted: qt_property!(bool; NOTIFY systemMutedChanged),
    pub systemMutedChanged: qt_signal!(),
    pub device_status_changed: qt_signal!(),
    pub refreshDeviceList: qt_method!(fn(&mut self)),
    pub selectDevice: qt_method!(fn(&mut self, deviceName: String)),

    // Favorites support
    pub favorites_count: qt_property!(i32; NOTIFY favorites_changed),
    pub favorites_changed: qt_signal!(),
    pub add_favorite: qt_method!(fn(&mut self, path: String, name: String)),
    pub remove_favorite: qt_method!(fn(&mut self, path: String)),
    pub is_favorite: qt_method!(fn(&self, path: String) -> bool),
    pub toggle_favorite: qt_method!(fn(&mut self, path: String, name: String)),
    pub switch_to_favorites: qt_method!(fn(&mut self)),
    pub switch_to_music: qt_method!(fn(&mut self)),
}

impl QAbstractListModel for MusicModel {
    fn row_count(&self) -> i32 {
        self.display_list.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let row = index.row() as usize;

        if row >= self.display_list.len() {
            return QVariant::default();
        }

        let item = &self.display_list[row];

        match role {
            256 => QString::from(item.name.clone()).into(),
            257 => item.is_folder.into(),
            258 => QString::from(item.path.clone()).into(),
            259 => QString::from(item.parent_folder.clone().unwrap_or_default()).into(),
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();

        map.insert(256, QByteArray::from("name"));
        map.insert(257, QByteArray::from("is_folder"));
        map.insert(258, QByteArray::from("path"));
        map.insert(259, QByteArray::from("parent_folder"));

        map
    }
}

impl MusicModel {
    pub fn new() -> Self {
        let saved_config = crate::audio::config::AppConfig::load();

        // Clone vectors first before moving saved_config
        let custom_folders = saved_config.custom_folders.clone();
        let favorites = saved_config.favorites.clone();

        let mut model = Self {
            ffmpeg: Arc::new(Mutex::new(FfmpegEngine::new())),
            audio: Arc::new(Mutex::new(AudioState::default())),
            output: AudioOutput::default(),
            volume: saved_config.volume as f64,
            current_index: -1,
            balance: saved_config.balance as f64,
            custom_folders: saved_config.custom_folders.clone(),
            custom_folder_count: saved_config.custom_folders.len() as i32,
            favorites: saved_config.favorites.clone(),
            favorites_count: saved_config.favorites.len() as i32,
            external_files: Vec::new(),
            external_files_count: 0,
            user_queue: Vec::new(),
            queue_count: 0,
            bass_magic_active: saved_config.bass_enabled,
            bass_gain: saved_config.bass_gain as f64,
            bass_cutoff: saved_config.bass_cutoff as f64,
            surround_magic_active: saved_config.surround_enabled,
            crystal_magic_active: saved_config.crystal_enabled,
            crystal_amount: saved_config.crystal_amount as f64,
            crystal_freq: saved_config.crystal_freq as f64,
            surround_width: saved_config.surround_width as f64,
            mono_active: saved_config.mono_enabled,
            mono_width: saved_config.mono_width as f64,
            pitch_active: saved_config.pitch_enabled,
            pitch_semitones: saved_config.pitch_semitones as f64,
            middle_active: saved_config.middle_enabled,
            middle_amount: saved_config.middle_amount as f64,
            stereo_active: saved_config.stereo_enabled,
            stereo_amount: saved_config.stereo_amount as f64,
            crossfeed_active: saved_config.crossfeed_enabled,
            crossfeed_amount: saved_config.crossfeed_amount as f64,
            reverb_preset: saved_config.reverb_preset,
            reverb_room_size: 0.55,
            reverb_damp: 0.5,
            eq_enabled: saved_config.eq_enabled,
            active_preset_index: saved_config.active_preset_index,
            user_eq_names: saved_config.user_preset_names.clone(),
            user_eq_gains: saved_config.user_preset_gains,
            user_eq_macro: saved_config.user_preset_macro,
            eq_presets: crate::audio::config::AppConfig::get_eq_presets(),
            fx_presets: crate::audio::config::AppConfig::get_fx_presets(),
            compressor_active: saved_config.compressor_enabled,
            eq_bands: saved_config.eq_bands,
            ..Default::default()
        };

        // Initialize compressor atomics from saved config
        crate::audio::dsp::compressor::get_compressor_enabled_arc().store(
            saved_config.compressor_enabled,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            saved_config.compressor_threshold.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        // Initialize pitch ratio atomic from saved config
        let pitch_ratio = 2.0_f32.powf(saved_config.pitch_semitones / 12.0);
        crate::audio::dsp::pitchshifter::get_pitch_ratio_arc()
            .store(pitch_ratio.to_bits(), std::sync::atomic::Ordering::Relaxed);

        // Initialize reverb preset ARC from saved config
        let reverb_arc = crate::audio::dsp::reverb::get_reverb_preset_arc();
        reverb_arc.store(
            saved_config.reverb_preset,
            std::sync::atomic::Ordering::Relaxed,
        );

        // Set current_reverb property based on preset
        model.current_reverb = QString::from(match saved_config.reverb_preset {
            1 => "stage",
            2 => "hall",
            3 => "stadium",
            _ => "off",
        });
        model.reverb_active = saved_config.reverb_preset > 0;
        model.reverb_changed();
        model.reverb_active_changed();
        model.dsp_enabled = saved_config.dsp_enabled;
        model.output.set_dsp_enabled(saved_config.dsp_enabled);
        model
            .output
            .set_normalizer_enabled(saved_config.normalizer_enabled);
        model.normalizer_enabled = saved_config.normalizer_enabled;

        // Initialize normalizer params from saved config
        model.normalizer_target_lufs = saved_config.normalizer_target_lufs as f64;
        model.normalizer_true_peak_dbtp = saved_config.normalizer_true_peak_dbtp as f64;
        model.normalizer_max_gain_db = saved_config.normalizer_max_gain_db as f64;
        model.normalizer_smoothing = saved_config.normalizer_smoothing as f64;

        // Push normalizer params to engine
        if let Ok(mut ff) = model.ffmpeg.lock() {
            ff.set_normalizer_params(
                saved_config.normalizer_target_lufs,
                saved_config.normalizer_true_peak_dbtp,
                saved_config.normalizer_max_gain_db,
            );
            ff.set_normalizer_smoothing(saved_config.normalizer_smoothing);
        }

        // Initialize normalizer smoothing atomic
        crate::audio::dsp::normalizer::get_normalizer_smoothing_arc().store(
            saved_config.normalizer_smoothing.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        // Initialize EQ enabled atomic from saved config
        crate::audio::dsp::eq::get_eq_enabled_arc().store(
            if saved_config.eq_enabled { 1 } else { 0 },
            std::sync::atomic::Ordering::Relaxed,
        );

        // Initialize EQ bands atomics from saved config
        let eq_arc = crate::audio::dsp::eq::get_eq_bands_arc();
        for i in 0..10 {
            eq_arc[i].store(
                saved_config.eq_bands[i].to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        }

        if let Ok(mut ff) = model.ffmpeg.lock() {
            ff.set_dsp_enabled(saved_config.dsp_enabled);
        }

        model.output.mode = saved_config.mode;

        // Emit volume_changed so QML slider updates to correct position
        model.volume_changed();

        // Apply saved volume to audio engine
        if let Ok(mut ff) = model.ffmpeg.lock() {
            ff.set_volume(model.volume as f32);
        }

        // Store config for saving later (wrap in Arc<Mutex> for sharing with ThemeManager)
        model.saved_config = Some(std::sync::Arc::new(std::sync::Mutex::new(
            saved_config.clone(),
        )));

        // Initialize DSP config manager
        model.dsp_config = DspConfigManager::new(model.saved_config.clone());

        // Initialize playback controller
        model.playback = PlaybackController::new(model.ffmpeg.clone(), model.audio.clone());
        model.playback.volume = model.volume;

        // Initialize library manager
        model.library = LibraryManager::new();
        model.library.load_folders(custom_folders, favorites);

        // Scan default Music folder on startup
        model.scan_music();

        model
    }

    pub fn scan_music(&mut self) {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => {
                eprintln!("Error: Could not determine home directory. Music scan aborted.");
                return;
            }
        };
        let music_dir = home.join("Music");

        self.current_folder = String::new();
        self.current_folder_path = String::new();
        self.current_folder_qml = QString::default();
        self.folders.clear();
        self.all_items.clear();
        self.expanded_folders.clear();

        self.library.scan_music_folder(&music_dir);
        self.all_items = self.library.all_items.clone();
        self.display_list = self.library.display_list.clone();

        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn scan_folder(&mut self, path: String) {
        let folder_path = Path::new(&path);
        if !folder_path.exists() || !folder_path.is_dir() {
            return;
        }

        self.current_folder = String::new();
        self.current_folder_path = String::new();
        self.current_folder_qml = QString::default();
        self.folders.clear();
        self.all_items.clear();
        self.expanded_folders.clear();

        self.library.scan_custom_directory(folder_path);
        self.all_items = self.library.all_items.clone();

        self.all_items
            .sort_by(|a, b| match (a.is_folder, b.is_folder) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });
        self.display_list = self.all_items.clone();

        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    fn scan_directory(&mut self, dir: &Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut dirs: Vec<_> = Vec::new();
            let mut files: Vec<_> = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if path.is_dir() {
                    dirs.push((name, path));
                } else if is_audio_file(&path) {
                    files.push((name, path));
                }
            }
            dirs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            for (name, path) in dirs {
                self.all_items.push(MusicItem {
                    name,
                    path: path.to_string_lossy().to_string(),
                    is_folder: true,
                    parent_folder: None,
                });
            }
            for (name, path) in files {
                self.all_items.push(MusicItem {
                    name,
                    path: path.to_string_lossy().to_string(),
                    is_folder: false,
                    parent_folder: None,
                });
            }
        }
        self.display_list = self.all_items.clone();
    }

    pub fn get_folder_contents(&self, folder_path: &Path) -> Vec<MusicItem> {
        let mut items = vec![];
        if let Ok(entries) = std::fs::read_dir(folder_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let folder_name = folder_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                if path.is_dir() {
                    items.push(MusicItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        is_folder: true,
                        parent_folder: Some(folder_name),
                    });
                } else if is_audio_file(&path) {
                    items.push(MusicItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        is_folder: false,
                        parent_folder: Some(folder_name),
                    });
                }
            }
        }
        items
    }

    pub fn add_folder_tab(&mut self, path: String) {
        let clean = clean_qml_path(&path);
        let folder_path = Path::new(&clean);
        if let Some(name) = folder_path.file_name() {
            let mut name_str = name.to_string_lossy().to_string();
            name_str.truncate(15);
            name_str = name_str.trim().to_string();

            if !self
                .library
                .custom_folders
                .iter()
                .any(|(n, p)| n == &name_str && p == &clean)
            {
                self.library.add_folder(clean.clone());
                self.custom_folders = self.library.custom_folders.clone();
                self.custom_folder_count = self.library.custom_folder_count;
                self.custom_folders_changed();
                self.save_custom_folders();

                let new_index = self.custom_folder_count - 1;
                self.folder_lock_version += 1;
                self.folder_lock_changed();

                if let Some(ref config) = &self.saved_config {
                    if let Ok(mut cfg) = config.lock() {
                        if !cfg.locked_folders.contains(&new_index) {
                            cfg.locked_folders.push(new_index);
                        }
                        let _ = cfg.save();
                    }
                }

                self.switch_to_folder(clean);
            }
        }
    }

    pub fn add_song(&mut self, path: String) {
        let clean = clean_qml_path(&path);
        let song_path = Path::new(&clean);
        if let Some(name) = song_path.file_name() {
            let name_str = name.to_string_lossy().to_string();

            // Add to all_items
            self.all_items.push(MusicItem {
                name: name_str,
                path: clean.clone(),
                is_folder: false,
                parent_folder: None,
            });

            // Sort items
            self.all_items
                .sort_by(|a, b| match (a.is_folder, b.is_folder) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                });

            // Update display list
            self.display_list = self.all_items.clone();

            // Notify model
            self.begin_reset_model();
            self.end_reset_model();
        }
    }

    pub fn remove_song(&mut self, index: i32) {
        let idx = index as usize;
        if idx >= self.display_list.len() {
            return;
        }

        let item = self.display_list[idx].clone();

        if item.is_folder {
            // Remove folder and all its children
            let folder_name = item.name.clone();
            self.display_list.retain(|i| {
                i.parent_folder.as_ref() != Some(&folder_name) && i.name != folder_name
            });
            self.all_items.retain(|i| {
                i.parent_folder.as_ref() != Some(&folder_name) && i.name != folder_name
            });
            self.expanded_folders.remove(&folder_name);
        } else {
            // Remove single song by path
            let path = item.path.clone();
            self.display_list.retain(|i| i.path != path);
            self.all_items.retain(|i| i.path != path);
        }

        // Adjust current index if needed
        if self.current_index >= self.display_list.len() as i32 {
            self.current_index = self.display_list.len() as i32 - 1;
            self.current_index_changed();
        }

        self.begin_reset_model();
        self.end_reset_model();
    }

    pub fn delete_item(&mut self, path: String, _is_folder: bool) {
        // 1. Hapus dari UI List dulu (universal)
        if let Some(index) = self.display_list.iter().position(|item| item.path == path) {
            self.remove_song(index as i32);
        }

        // 2. Logika penghapusan fisik (Conditional)
        #[cfg(target_os = "linux")]
        {
            if let Err(e) = trash::delete(&path) {
                eprintln!("[TRASH] Error: {}", e);
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Err(e) = trash::delete(&path) {
                eprintln!("[RECYCLE BIN] Error: {}", e);
            }
        }

        #[cfg(target_os = "android")]
        {
            if is_folder {
                let _ = std::fs::remove_dir_all(&path);
            } else {
                let _ = std::fs::remove_file(&path);
            }
            println!("[ANDROID] File deleted permanently: {}", path);
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "android")))]
        {
            // Fallback untuk platform lain: direct delete
            let p = std::path::Path::new(&path);
            if is_folder {
                let _ = std::fs::remove_dir_all(p);
            } else {
                let _ = std::fs::remove_file(p);
            }
        }
    }

    pub fn add_to_queue(&mut self, path: String, name: String) {
        self.library.add_to_queue(path, name);
        self.user_queue = self.library.user_queue.clone();
        self.queue_count = self.library.queue_count;
        self.queue_changed();
    }

    pub fn remove_from_queue(&mut self, index: i32) {
        self.library.remove_from_queue(index);
        self.user_queue = self.library.user_queue.clone();
        self.queue_count = self.library.queue_count;
        self.queue_changed();
    }

    pub fn clear_queue(&mut self) {
        self.user_queue.clear();
        self.queue_count = 0;
        self.queue_changed();
    }

    pub fn get_queue_item(&self, index: i32) -> QVariantMap {
        let idx = index as usize;
        if idx >= self.user_queue.len() {
            return QVariantMap::default();
        }
        let item = &self.user_queue[idx];
        let mut map = QVariantMap::default();
        map.insert("name".into(), QString::from(item.name.as_str()).into());
        map.insert("path".into(), QString::from(item.path.as_str()).into());
        map
    }

    pub fn switch_to_queue(&mut self) {
        self.current_folder = "Queue".to_string();
        self.current_folder_path = String::new();
        self.current_folder_qml = QString::from("QUEUE");
        // Set display_list to queue items
        self.all_items = self.user_queue.clone();
        self.display_list = self.all_items.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn add_temporary_folder(&mut self, path: String) {
        let folder_path = Path::new(&path);
        if let Some(name) = folder_path.file_name() {
            let name_str = name.to_string_lossy().to_string();
            // Create folder item
            let folder_item = MusicItem {
                name: name_str,
                path: path.clone(),
                is_folder: true,
                parent_folder: None,
            };
            // Add to all_items
            self.all_items.push(folder_item);
            // Sort items (folders first)
            self.all_items
                .sort_by(|a, b| match (a.is_folder, b.is_folder) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                });
            // Update display list
            self.display_list = self.all_items.clone();
            // Notify model
            self.begin_reset_model();
            self.end_reset_model();
        }
    }

    pub fn add_external_file(&mut self, path: String) {
        let song_path = Path::new(&path);
        if let Some(name) = song_path.file_name() {
            let name_str = name.to_string_lossy().to_string();

            // Add to external_files
            self.external_files.push(MusicItem {
                name: name_str,
                path: path.clone(),
                is_folder: false,
                parent_folder: None,
            });

            self.external_files_count = self.external_files.len() as i32;
            self.external_files_changed();

            // Switch to external files view
            self.switch_to_external_files();
        }
    }

    pub fn switch_to_external_files(&mut self) {
        self.current_folder = "External Files".to_string();
        self.current_folder_path = String::new();
        self.current_folder_qml = QString::from("EXTERNAL_FILES");

        self.all_items = self.external_files.clone();
        self.display_list = self.all_items.clone();

        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn clear_external_files(&mut self) {
        self.external_files.clear();
        self.external_files_count = 0;
        self.external_files_changed();

        // Switch back to Music folder
        self.scan_music();
    }

    pub fn process_command_line_files(&mut self) {
        let files = get_command_line_files();
        if !files.is_empty() {
            for file_path in files {
                self.add_external_file(file_path.clone());
            }
        }
    }

    pub fn save_custom_folders(&mut self) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.custom_folders = self.custom_folders.clone();
                cfg.volume = self.volume as f64;
                cfg.balance = self.balance as f64;
                let _ = cfg.save();
            }
        }
    }

    // --- FAVORITES ---
    pub fn add_favorite(&mut self, path: String, name: String) {
        if !self.favorites.iter().any(|(_, p)| p == &path) {
            self.favorites.push((name.clone(), path.clone()));
            self.favorites_count = self.favorites.len() as i32;
            self.favorites_changed();
            self.save_favorites();

            // Refresh display if currently in Favorites tab
            if self.current_folder == "FAVORITES" {
                self.all_items.clear();
                self.display_list.clear();
                for (fav_name, fav_path) in &self.favorites {
                    self.all_items.push(MusicItem {
                        name: fav_name.clone(),
                        path: fav_path.clone(),
                        is_folder: false,
                        parent_folder: None,
                    });
                }
                self.display_list = self.all_items.clone();
                self.begin_reset_model();
                self.end_reset_model();
            }
        }
    }

    pub fn remove_favorite(&mut self, path: String) {
        self.favorites.retain(|(_, p)| p != &path);
        self.favorites_count = self.favorites.len() as i32;
        self.favorites_changed();
        self.save_favorites();

        // Refresh display if currently in Favorites tab
        if self.current_folder == "FAVORITES" {
            self.all_items.clear();
            self.display_list.clear();
            for (fav_name, fav_path) in &self.favorites {
                self.all_items.push(MusicItem {
                    name: fav_name.clone(),
                    path: fav_path.clone(),
                    is_folder: false,
                    parent_folder: None,
                });
            }
            self.display_list = self.all_items.clone();
            self.begin_reset_model();
            self.end_reset_model();
        }
    }

    pub fn is_favorite(&self, path: String) -> bool {
        self.favorites.iter().any(|(_, p)| p == &path)
    }

    pub fn toggle_favorite(&mut self, path: String, name: String) {
        if self.is_favorite(path.clone()) {
            self.remove_favorite(path);
        } else {
            self.add_favorite(path, name);
        }
    }

    pub fn switch_to_favorites(&mut self) {
        self.current_folder = String::from("FAVORITES");
        self.current_folder_path = String::new();
        self.current_folder_qml = QString::from("FAVORITES");
        self.all_items.clear();
        self.display_list.clear();
        self.expanded_folders.clear();

        for (name, path) in &self.favorites {
            self.all_items.push(MusicItem {
                name: name.clone(),
                path: path.clone(),
                is_folder: false,
                parent_folder: None,
            });
        }

        self.display_list = self.all_items.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn switch_to_music(&mut self) {
        self.scan_music();
    }

    fn save_favorites(&mut self) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.favorites = self.favorites.clone();
                let _ = cfg.save();
            }
        }
    }

    pub fn change_folder(&mut self, index: i32, new_path: String) {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            let folder_path = Path::new(&new_path);
            if let Some(name) = folder_path.file_name() {
                let mut name_str = name.to_string_lossy().to_string();
                name_str.truncate(15);
                name_str = name_str.trim().to_string();
                self.custom_folders[index as usize] = (name_str, new_path.clone());
                self.custom_folders_changed();
                self.save_dsp_config();
                if let Some(ref config) = &self.saved_config {
                    if let Ok(mut cfg) = config.lock() {
                        cfg.custom_folders = self.custom_folders.clone();
                        let _ = cfg.save();
                    }
                }
                self.switch_to_folder(new_path);
            }
        }
    }

    pub fn switch_to_folder(&mut self, folder_path: String) {
        let path = Path::new(&folder_path);

        self.current_folder_path = folder_path.clone();

        // Try to find display name from custom_folders first
        let display_name = self
            .custom_folders
            .iter()
            .find(|(_, p)| p == &folder_path)
            .map(|(name, _)| name.clone());

        if let Some(name) = display_name {
            // Use display name (which can be renamed)
            self.current_folder = name.clone();
            self.current_folder_qml = QString::from(name);
        } else if let Some(name) = path.file_name() {
            // Fallback to actual folder name
            self.current_folder = name.to_string_lossy().to_string();
            self.current_folder_qml = QString::from(self.current_folder.clone());
        }

        self.all_items.clear();
        self.display_list.clear();
        self.expanded_folders.clear();

        if path.is_dir() {
            self.scan_custom_directory(path);
        }

        self.begin_reset_model();
        self.end_reset_model();
        self.custom_folders_changed();
        self.current_folder_changed();
    }

    fn scan_custom_directory(&mut self, dir: &Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut dirs: Vec<_> = Vec::new();
            let mut files: Vec<_> = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if path.is_dir() {
                    dirs.push((name, path));
                } else if is_audio_file(&path) {
                    files.push((name, path));
                }
            }
            dirs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            for (name, path) in dirs {
                self.all_items.push(MusicItem {
                    name,
                    path: path.to_string_lossy().to_string(),
                    is_folder: true,
                    parent_folder: None,
                });
            }
            for (name, path) in files {
                self.all_items.push(MusicItem {
                    name,
                    path: path.to_string_lossy().to_string(),
                    is_folder: false,
                    parent_folder: None,
                });
            }
        }
        self.display_list = self.all_items.clone();
    }

    pub fn get_custom_folder_name(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            let name = self.custom_folders[index as usize].0.clone();
            // Convert to uppercase
            QString::from(name.to_uppercase())
        } else {
            QString::default()
        }
    }

    pub fn get_custom_folder_path(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            QString::from(self.custom_folders[index as usize].1.clone())
        } else {
            QString::default()
        }
    }

    pub fn get_current_rename_name(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            QString::from(self.custom_folders[index as usize].0.clone())
        } else {
            QString::default()
        }
    }

    pub fn rename_folder(&mut self, index: i32, new_name: String) {
        if index < 0 || (index as usize) >= self.custom_folders.len() {
            return;
        }

        let mut trimmed = new_name.trim().to_string();
        trimmed.truncate(15);

        if trimmed.is_empty() {
            return;
        }

        self.custom_folders[index as usize].0 = trimmed;
        self.custom_folders_changed();
        self.save_dsp_config();

        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.custom_folders = self.custom_folders.clone();
                let _ = cfg.save();
            }
        }
    }

    pub fn get_custom_folder_count(&self) -> i32 {
        self.custom_folders.len() as i32
    }

    pub fn remove_custom_folder(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            if self.is_folder_locked(index) {
                return;
            }
            let removed_path = self.custom_folders[index as usize].1.clone();
            self.custom_folders.remove(index as usize);
            self.custom_folder_count = self.custom_folders.len() as i32;
            self.custom_folders_changed();
            self.save_custom_folders();

            if self.current_folder_path == removed_path {
                self.scan_music();
            }
        }
    }

    pub fn toggle_folder_lock(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            if let Some(ref config) = &self.saved_config {
                if let Ok(mut cfg) = config.lock() {
                    if cfg.locked_folders.contains(&index) {
                        cfg.locked_folders.retain(|&i| i != index);
                    } else {
                        cfg.locked_folders.push(index);
                    }
                    cfg.custom_folders = self.custom_folders.clone();
                    cfg.volume = self.volume as f64;
                    cfg.balance = self.balance as f64;
                    let _ = cfg.save();
                }
            }
            self.folder_lock_version += 1;
            self.folder_lock_changed();
            self.custom_folders_changed();
        }
    }

    pub fn is_folder_locked(&self, index: i32) -> bool {
        if let Some(ref config) = &self.saved_config {
            if let Ok(cfg) = config.lock() {
                return cfg.locked_folders.contains(&index);
            }
        }
        false
    }

    pub fn show_tab_context_menu(&mut self, _index: i32) {
        // This method is called from QML to trigger the tab context menu
        // The actual menu is shown in Ui.qml using the popup system
    }

    pub fn is_folder_expanded(&self, folder_name: QString) -> bool {
        self.expanded_folders.contains(&folder_name.to_string())
    }

    pub fn play_at(&mut self, index: i32) {
        if index < 0 || index as usize >= self.display_list.len() {
            return;
        }

        let item = &self.display_list[index as usize];

        if item.is_folder {
            return;
        }

        self.current_index = index;
        self.playback.play_at(item);
        self.is_playing = self.playback.is_playing;
        self.position = self.playback.position;
        self.duration = self.playback.duration;
        self.current_title = self.playback.current_title.clone();

        self.refreshDeviceStatus();

        self.current_index_changed();
        self.title_changed();
        self.playing_changed();
        self.position_changed();
        self.duration_changed();
    }

    pub fn stop_playback(&mut self) {
        self.playback.stop();
        self.is_playing = self.playback.is_playing;
        self.playing_changed();
    }

    pub fn stop(&mut self) {
        self.stop_playback();
    }

    fn play_next_from_queue(&mut self) -> bool {
        if self.user_queue.is_empty() {
            return false;
        }
        // Take first item
        let item = self.user_queue.remove(0);
        self.queue_count = self.user_queue.len() as i32;
        self.queue_changed();
        // Find index in display_list
        if let Some(index) = self.display_list.iter().position(|i| i.path == item.path) {
            self.play_at(index as i32);
            return true;
        }
        // Not found, maybe item is not in current folder; skip and try next queue item
        self.play_next_from_queue()
    }

    pub fn play_next(&mut self) {
        if self.display_list.is_empty() {
            return;
        }
        // Check user queue first
        if self.play_next_from_queue() {
            return;
        }

        let current_item = &self.display_list[self.current_index as usize];
        let current_parent = current_item.parent_folder.clone();

        // Get indices of tracks in same folder (for shuffle)
        let folder_indices: Vec<i32> = self
            .display_list
            .iter()
            .enumerate()
            .filter(|(_, item)| !item.is_folder && item.parent_folder == current_parent)
            .map(|(i, _)| i as i32)
            .collect();

        if folder_indices.is_empty() {
            return;
        }

        // Shuffle mode: pick random from folder
        if self.shuffle_active {
            use rand::seq::SliceRandom;
            let mut rng = rand::rng();

            // If shuffle queue is empty or not in folder, rebuild it
            if self.shuffle_queue.is_empty() || !folder_indices.contains(&self.shuffle_queue[0]) {
                self.shuffle_queue = folder_indices.clone();
                self.shuffle_queue.shuffle(&mut rng);
            }

            // Find current position in queue
            if let Some(pos) = self
                .shuffle_queue
                .iter()
                .position(|&x| x == self.current_index)
            {
                let next_pos = pos + 1;
                if (next_pos as usize) < self.shuffle_queue.len() {
                    self.play_at(self.shuffle_queue[next_pos as usize]);
                    return;
                } else if self.loop_active {
                    // Loop: reshuffle and start from beginning
                    self.shuffle_queue.shuffle(&mut rng);
                    self.play_at(self.shuffle_queue[0]);
                    return;
                }
            } else {
                // Current not in queue, start fresh
                self.play_at(folder_indices[0]);
            }
            // No next track in shuffle and no loop - stop playback
            self.stop_playback();
            return;
        }

        // No shuffle: sequential within folder
        let mut next = self.current_index + 1;

        while (next as usize) < self.display_list.len() {
            let next_item = &self.display_list[next as usize];
            if !next_item.is_folder && next_item.parent_folder == current_parent {
                self.play_at(next);
                return;
            }
            next += 1;
        }

        // Loop back to first track in folder
        if self.loop_active {
            let mut first_in_folder = 0;
            while first_in_folder < self.current_index {
                let item = &self.display_list[first_in_folder as usize];
                if !item.is_folder && item.parent_folder == current_parent {
                    self.play_at(first_in_folder);
                    return;
                }
                first_in_folder += 1;
            }
        }

        // No next track and no loop - stop playback
        self.stop_playback();
    }

    pub fn play_previous(&mut self) {
        self.play_prev_impl();
    }

    pub fn play_prev(&mut self) {
        self.play_prev_impl();
    }

    fn play_prev_impl(&mut self) {
        if self.display_list.is_empty() {
            return;
        }

        let current_item = &self.display_list[self.current_index as usize];
        let current_parent = &current_item.parent_folder;

        // Find previous track in same folder
        let mut prev = self.current_index - 1;
        while prev >= 0 {
            let prev_item = &self.display_list[prev as usize];
            if !prev_item.is_folder && &prev_item.parent_folder == current_parent {
                self.play_at(prev);
                return;
            }
            prev -= 1;
        }

        // No more tracks in same folder, go to last track in folder
        if self.loop_active {
            let mut last_in_folder = (self.display_list.len() - 1) as i32;
            while last_in_folder > self.current_index {
                let item = &self.display_list[last_in_folder as usize];
                if !item.is_folder && &item.parent_folder == current_parent {
                    self.play_at(last_in_folder);
                    return;
                }
                last_in_folder -= 1;
            }
        }
    }

    pub fn toggle_repeat(&mut self) {
        self.playback.toggle_repeat();
        self.loop_active = self.playback.loop_active;
        self.loop_playlist = self.loop_active;
        self.loop_changed();
    }

    pub fn toggle_shuffle(&mut self) {
        self.playback.toggle_shuffle(&self.display_list);
        self.shuffle_active = self.playback.shuffle_active;
        self.shuffle = self.shuffle_active;
        self.shuffle_changed();
    }

    pub fn toggle_folder(&mut self, folder_path_str: String) {
        let folder_path = Path::new(&folder_path_str);

        let folder_name = folder_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or(folder_path_str.clone());

        if self.expanded_folders.contains(&folder_name) {
            self.expanded_folders.remove(&folder_name);
            self.display_list.retain(|item| match &item.parent_folder {
                Some(parent) => parent != &folder_name,
                None => true,
            });
        } else {
            self.expanded_folders.insert(folder_name.clone());
            let folder_contents = self.get_folder_contents(folder_path);
            let insert_pos = self
                .display_list
                .iter()
                .position(|item| item.name == folder_name)
                .map(|pos| pos + 1)
                .unwrap_or(self.display_list.len());

            for item in folder_contents {
                self.display_list.insert(insert_pos, item);
            }
        }

        self.begin_reset_model();
        self.end_reset_model();
    }

    pub fn seek_to(&mut self, pos: i32) {
        self.position = pos;

        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.seek(pos as f64 / 1000.0);
            let dur = (ff.get_duration() * 1000.0) as i32;
            if dur != self.duration {
                self.duration = dur;
                self.duration_changed();
            }
        }

        self.position_changed();
    }

    pub fn format_time(&self, ms: i32) -> QString {
        let ms = if ms < 0 { 0 } else { ms };
        let total_seconds = ms / 1000;
        let hours = total_seconds / 3600;
        let mins = (total_seconds % 3600) / 60;
        let secs = total_seconds % 60;

        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, mins, secs)
        } else {
            format!("{:02}:{:02}", mins, secs)
        }
        .into()
    }

    pub fn set_volume(&mut self, vol: f64) {
        self.volume = vol;
        self.playback.set_volume(vol);
        self.volume_changed();
        self.save_state();
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.balance = balance;

        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_balance(balance as f32);
        }

        self.balance_changed();
        self.save_state();
    }

    // START EFFECT DSP

    pub fn set_reverb(&mut self, reverb: QString) {
        let p_str = reverb.to_string().to_lowercase();
        self.current_reverb = reverb;
        self.reverb_changed();

        let preset_id = match p_str.as_str() {
            "stage" => 1,
            "hall" => 2,
            "stadium" => 3,
            _ => 0,
        };

        crate::audio::dsp::reverb::get_reverb_preset_arc()
            .store(preset_id, std::sync::atomic::Ordering::Relaxed);

        self.reverb_preset = preset_id;
        self.reverb_active = preset_id > 0;
        self.reverb_active_changed();

        self.save_dsp_config();
    }

    pub fn toggleStdReverb(&mut self) {
        self.reverb_active = !self.reverb_active;
        self.reverb_active_changed();

        let preset_id = if self.reverb_active {
            if self.reverb_preset > 0 {
                self.reverb_preset
            } else {
                1
            }
        } else {
            0
        };

        crate::audio::dsp::reverb::get_reverb_preset_arc()
            .store(preset_id, std::sync::atomic::Ordering::Relaxed);

        self.reverb_preset = preset_id;
        self.current_reverb = QString::from(match preset_id {
            1 => "stage",
            2 => "hall",
            3 => "stadium",
            _ => "off",
        });

        self.reverb_changed();
        self.save_dsp_config();
    }

    pub fn setStdReverbRoomSize(&mut self, val: f64) {
        let val = val.clamp(0.0, 1.0);
        self.reverb_room_size = val;
        self.reverb_params_changed();
        crate::audio::dsp::reverb::get_reverb_room_size_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);
        self.save_dsp_config();
    }

    pub fn setStdReverbDamp(&mut self, val: f64) {
        let val = val.clamp(0.0, 1.0);
        self.reverb_damp = val;
        self.reverb_params_changed();
        crate::audio::dsp::reverb::get_reverb_damp_arc()
            .store((val as f32).to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.save_dsp_config();
    }

    pub fn toggleStdBassBooster(&mut self) {
        self.bass_magic_active = !self.bass_magic_active;
        self.bass_magic_changed();

        if self.bass_magic_active {
            self.bass_gain = 5.5;
            self.applyBassMode(self.bass_mode);
        } else {
            self.bass_gain = 0.0;
            crate::audio::dsp::bassbooster::get_bass_gain_arc()
                .store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        crate::audio::dsp::bassbooster::get_bass_enabled_arc()
            .store(self.bass_magic_active, std::sync::atomic::Ordering::Relaxed);

        self.bass_params_changed();
        self.save_dsp_config();
    }

    pub fn set_bass_mode(&mut self, mode: i32) {
        let mode = mode.clamp(0, 3);
        self.bass_mode = mode;
        self.bass_mode_changed();

        if self.bass_magic_active {
            self.applyBassMode(mode);
        }

        self.bass_params_changed();
        self.save_dsp_config();
    }

    fn applyBassMode(&mut self, mode: i32) {
        let freqs: [f32; 4] = [50.0, 60.0, 90.0, 150.0];
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
    }

    pub fn setStdBassGain(&mut self, val: f64) {
        self.bass_gain = val.clamp(0.0, 12.0);
        self.bass_params_changed();

        if self.bass_magic_active {
            crate::audio::dsp::bassbooster::get_bass_gain_arc().store(
                (self.bass_gain as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        }

        self.save_dsp_config();
    }

    pub fn setStdBassCutoff(&mut self, val: f64) {
        self.bass_cutoff = val.clamp(20.0, 500.0);
        self.bass_params_changed();

        if self.bass_magic_active {
            crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
                (self.bass_cutoff as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        }

        self.save_dsp_config();
    }

    pub fn toggleStdSurround(&mut self) {
        self.surround_magic_active = !self.surround_magic_active;
        self.surround_magic_changed();

        if self.surround_magic_active {
            self.surround_width = 1.3;
            crate::audio::dsp::surround::get_surround_width_arc()
                .store(1.3_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);
            crate::audio::dsp::surround::get_surround_bass_safe_arc()
                .store(1u32, std::sync::atomic::Ordering::Relaxed);
        } else {
            self.surround_width = 1.0;
            crate::audio::dsp::surround::get_surround_width_arc()
                .store(1.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        crate::audio::dsp::surround::get_surround_enabled_arc().store(
            self.surround_magic_active,
            std::sync::atomic::Ordering::Relaxed,
        );

        self.save_dsp_config();
    }

    pub fn setStdSurroundWidth(&mut self, val: f64) {
        let val = val.max(0.0).min(2.0);
        self.surround_width = val;

        if !self.surround_magic_active {
            self.surround_magic_active = true;
            self.surround_magic_changed();
            crate::audio::dsp::surround::get_surround_enabled_arc()
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }

        crate::audio::dsp::surround::get_surround_width_arc().store(
            (self.surround_width as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        self.save_dsp_config();
    }

    pub fn toggleStdCrystalizer(&mut self) {
        self.crystal_magic_active = !self.crystal_magic_active;
        self.crystal_magic_changed();

        if self.crystal_magic_active {
            if self.crystal_amount <= 0.0 {
                self.crystal_amount = 0.2;
            }
            crate::audio::dsp::crystalizer::get_crystal_amount_arc().store(
                (self.crystal_amount as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );
        } else {
            crate::audio::dsp::crystalizer::get_crystal_amount_arc()
                .store(0.0_f32.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }

        crate::audio::dsp::crystalizer::get_crystal_enabled_arc().store(
            self.crystal_magic_active,
            std::sync::atomic::Ordering::Relaxed,
        );

        self.save_dsp_config();
    }

    pub fn toggleStdCompressor(&mut self) {
        self.compressor_active = !self.compressor_active;
        self.compressor_changed();

        crate::audio::dsp::compressor::get_compressor_enabled_arc()
            .store(self.compressor_active, std::sync::atomic::Ordering::Relaxed);

        self.save_dsp_config();
    }

    pub fn reset_std_compressor(&mut self) {
        self.setStdCompressorThreshold(1.0);
    }

    pub fn reset_std_surround(&mut self) {
        self.setStdSurroundWidth(1.0);
    }

    pub fn reset_std_stereo_width(&mut self) {
        self.setStdStereoWidthAmount(1.0);
    }

    pub fn reset_std_middle_clarity(&mut self) {
        self.setStdMiddleClarityAmount(0.0);
    }

    pub fn reset_std_stereo_enhance(&mut self) {
        self.setStdStereoEnhanceAmount(0.0);
    }

    pub fn reset_std_crossfeed(&mut self) {
        self.setStdCrossfeedAmount(0.0);
    }

    pub fn reset_std_crystalizer(&mut self) {
        self.set_crystalizer_amount(0.0);
    }

    pub fn reset_std_bass(&mut self) {
        self.setStdBassGain(0.0);
        self.setStdBassCutoff(180.0);
    }

    pub fn setStdCompressorThreshold(&mut self, val: f64) {
        let threshold_db = -60.0 + (val * 60.0);
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            (threshold_db as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        self.save_dsp_config();
    }

    pub fn getStdCompressorThreshold(&self) -> f64 {
        let bits = crate::audio::dsp::compressor::get_compressor_threshold_arc()
            .load(std::sync::atomic::Ordering::Relaxed);
        let threshold_db = f32::from_bits(bits);
        ((threshold_db + 60.0) / 60.0) as f64
    }

    pub fn toggleStdPitch(&mut self) {
        self.pitch_active = !self.pitch_active;
        self.pitch_changed();

        crate::audio::dsp::pitchshifter::get_pitch_enabled_arc()
            .store(self.pitch_active, std::sync::atomic::Ordering::Relaxed);

        self.save_dsp_config();
    }

    pub fn setStdPitchSemitones(&mut self, val: f64) {
        let raw = val.max(-12.0).min(12.0);
        let semitones = if raw.abs() < 0.5 { 0.0 } else { raw };
        self.pitch_semitones = semitones;
        self.pitch_changed();

        let ratio = 2.0_f32.powf((semitones as f32) / 12.0);
        get_pitch_ratio_arc().store(ratio.to_bits(), std::sync::atomic::Ordering::Relaxed);

        self.save_dsp_config();
    }

    pub fn toggleStdMiddleClarity(&mut self) {
        self.middle_active = !self.middle_active;
        self.middle_changed();

        crate::audio::dsp::middleclarity::get_middle_enabled_arc()
            .store(self.middle_active, std::sync::atomic::Ordering::Relaxed);

        self.save_dsp_config();
    }

    pub fn setStdMiddleClarityAmount(&mut self, val: f64) {
        self.middle_amount = val.max(0.0).min(1.0);
        self.middle_amount_changed();
        crate::audio::dsp::middleclarity::get_middle_amount_arc().store(
            (self.middle_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_dsp_config();
    }

    pub fn toggleStdStereoWidth(&mut self) {
        self.mono_active = !self.mono_active;
        self.mono_changed();

        crate::audio::dsp::stereowidth::get_mono_enabled_arc()
            .store(self.mono_active, std::sync::atomic::Ordering::Relaxed);

        self.save_dsp_config();
    }

    pub fn setStdStereoWidthAmount(&mut self, val: f64) {
        self.mono_width = val.max(0.0).min(1.0);
        self.mono_width_changed();
        crate::audio::dsp::stereowidth::get_mono_width_arc().store(
            (self.mono_width as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_dsp_config();
    }

    pub fn toggleStdStereoEnhance(&mut self) {
        self.stereo_active = !self.stereo_active;
        self.stereo_changed();

        // Update enabled atomic
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
            .store(self.stereo_active, std::sync::atomic::Ordering::Relaxed);

        // Update amount atomic
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc().store(
            (self.stereo_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        self.save_dsp_config();
    }

    pub fn setStdStereoEnhanceAmount(&mut self, val: f64) {
        self.stereo_amount = val.max(0.0).min(1.0);
        self.stereo_amount_changed();
        let arc = crate::audio::dsp::stereoenhance::get_stereo_amount_arc();
        arc.store(
            (self.stereo_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        self.save_dsp_config();
    }

    pub fn toggleStdCrossfeed(&mut self) {
        self.crossfeed_active = !self.crossfeed_active;
        self.crossfeed_changed();

        crate::audio::dsp::crossfeed::get_crossfeed_enabled_arc()
            .store(self.crossfeed_active, std::sync::atomic::Ordering::Relaxed);

        self.save_dsp_config();
    }

    pub fn setStdCrossfeedAmount(&mut self, val: f64) {
        self.crossfeed_amount = val.max(0.0).min(1.0);
        self.crossfeed_amount_changed();
        crate::audio::dsp::crossfeed::get_crossfeed_amount_arc().store(
            (self.crossfeed_amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_dsp_config();
    }

    // END EFFECT DSP

    pub fn check_for_updates(&mut self) {
        self.update_status = QString::from("Checking for updates...");
        self.update_available = false;
        self.update_status_changed();

        let (tx, rx) = std::sync::mpsc::channel();
        self.update_rx = Some(rx);

        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::builder()
                .user_agent("loonix-tunes")
                .timeout(std::time::Duration::from_secs(10))
                .build();

            if let Ok(c) = client {
                let url = "https://api.github.com/repos/citzeye/loonix-tunes/releases/latest";
                if let Ok(res) = c.get(url).send() {
                    if let Ok(json) = res.json::<serde_json::Value>() {
                        let latest = json["tag_name"].as_str().unwrap_or("").replace('v', "");
                        let _ = tx.send(latest);
                        return;
                    }
                }
            }
            let _ = tx.send("error".to_string());
        });
    }

    fn compare_versions(a: &str, b: &str) -> i32 {
        let parse_part = |s: &str| -> (u32, u32, u32) {
            let mut parts = s.split('.').map(|p| p.parse::<u32>().unwrap_or(0));
            (
                parts.next().unwrap_or(0),
                parts.next().unwrap_or(0),
                parts.next().unwrap_or(0),
            )
        };
        let (a_major, a_minor, a_patch) = parse_part(a);
        let (b_major, b_minor, b_patch) = parse_part(b);

        if a_major != b_major {
            return (a_major as i32) - (b_major as i32);
        }
        if a_minor != b_minor {
            return (a_minor as i32) - (b_minor as i32);
        }
        (a_patch as i32) - (b_patch as i32)
    }

    pub fn poll_update_result(&mut self) {
        if let Some(ref rx) = self.update_rx {
            if let Ok(latest) = rx.try_recv() {
                self.update_rx = None;
                let current = env!("CARGO_PKG_VERSION");
                if latest == "error" {
                    self.update_status = QString::from("Failed to reach GitHub");
                    self.update_available = false;
                } else if Self::compare_versions(&latest, current) > 0 {
                    self.update_status =
                        QString::from(format!("New version available: v{}", latest));
                    self.update_available = true;
                } else {
                    self.update_status = QString::from("You are up to date!");
                    self.update_available = false;
                }
                self.update_status_changed();
            }
        }
    }

    pub fn toggleStdDsp(&mut self) {
        self.dsp_enabled = !self.dsp_enabled;
        self.dsp_changed();
        self.output.set_dsp_enabled(self.dsp_enabled);

        // Propagate to engine
        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_dsp_enabled(self.dsp_enabled);
        }

        // Flush filters when turning DSP off
        if !self.dsp_enabled {
            self.output.reset_dsp();
            if let Ok(mut ff) = self.ffmpeg.lock() {
                ff.reset_dsp();
            }
        }

        self.save_dsp_config();
    }

    pub fn toggleStdPreamp(&mut self) {
        self.preamp_active = !self.preamp_active;
        self.preamp_changed();
        crate::audio::dsp::preamp::get_preamp_enabled_arc()
            .store(self.preamp_active, std::sync::atomic::Ordering::Relaxed);
        self.save_dsp_config();
    }

    pub fn toggleStdLimiter(&mut self) {
        self.limiter_active = !self.limiter_active;
        self.limiter_changed();
        crate::audio::dsp::get_limiter_enabled_arc()
            .store(self.limiter_active, std::sync::atomic::Ordering::Relaxed);
        self.save_dsp_config();
    }

    pub fn toggle_normalizer(&mut self) {
        self.normalizer_enabled = !self.normalizer_enabled;
        self.normalizer_changed();
        self.output.set_normalizer_enabled(self.normalizer_enabled);
        self.save_dsp_config();
    }

    pub fn get_preamp_enabled(&self) -> bool {
        crate::audio::dsp::preamp::get_preamp_enabled_arc()
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_limiter_enabled(&self) -> bool {
        crate::audio::dsp::get_limiter_enabled_arc().load(std::sync::atomic::Ordering::Relaxed)
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
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.normalizer_target_lufs = clamped as f32;
                let _ = cfg.save();
            }
        }
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
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.normalizer_true_peak_dbtp = clamped as f32;
                let _ = cfg.save();
            }
        }
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
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.normalizer_max_gain_db = clamped as f32;
                let _ = cfg.save();
            }
        }
    }

    pub fn set_normalizer_smoothing(&mut self, val: f64) {
        let clamped = val.clamp(0.0005, 0.01);
        self.normalizer_smoothing = clamped;
        self.normalizer_params_changed();

        // Update the shared atomic (lock-free, read by audio callback)
        crate::audio::dsp::normalizer::get_normalizer_smoothing_arc().store(
            (clamped as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );

        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.normalizer_smoothing = clamped as f32;
                let _ = cfg.save();
            }
        }
    }

    pub fn get_normalizer_smoothing_label(&self) -> QString {
        let label = crate::audio::dsp::normalizer::SmoothingPreset::from_factor(
            self.normalizer_smoothing as f32,
        );
        QString::from(label)
    }

    pub fn get_output_devices(&self) -> QVariantList {
        let devices = self.output.get_output_devices();
        let mut list = QVariantList::default();
        for device in devices {
            list.push(QString::from(device).into());
        }
        list
    }

    pub fn set_output_device(&mut self, index: i32) {
        self.output.set_output_device(index as usize);
    }

    pub fn refreshDeviceList(&mut self) {
        let devices = self.output.get_output_devices();
        self.device_list = QStringList::from(
            devices
                .iter()
                .map(|s| QString::from(s.as_str()))
                .collect::<Vec<_>>(),
        );
        self.device_list_changed();
    }

    pub fn selectDevice(&mut self, deviceName: String) {
        self.output.selectDevice(deviceName);
        self.refreshDeviceStatus();
    }

    pub fn refreshDeviceStatus(&mut self) {
        self.device_status_changed();
    }

    pub fn set_eq_band(&mut self, index: i32, gain: f64) {
        let band = index as usize;
        if band < 10 {
            self.eq_bands[band] = gain as f32;

            // Lock-free: push directly to atomic (no chain rebuild)
            let arc = crate::audio::dsp::eq::get_eq_bands_arc();
            arc[band].store(
                (gain as f32).to_bits(),
                std::sync::atomic::Ordering::Relaxed,
            );

            // Emit band-specific NOTIFY signal for reactive QML binding
            match band {
                0 => self.eq_band_0_changed(),
                1 => self.eq_band_1_changed(),
                2 => self.eq_band_2_changed(),
                3 => self.eq_band_3_changed(),
                4 => self.eq_band_4_changed(),
                5 => self.eq_band_5_changed(),
                6 => self.eq_band_6_changed(),
                7 => self.eq_band_7_changed(),
                8 => self.eq_band_8_changed(),
                9 => self.eq_band_9_changed(),
                _ => {}
            }

            // Save state for persistence
            self.save_dsp_config();
        }
    }

    // Individual band setters for reactive property binding
    pub fn set_eq_band_0(&mut self, val: f64) {
        self.set_eq_band(0, val);
    }
    pub fn set_eq_band_1(&mut self, val: f64) {
        self.set_eq_band(1, val);
    }
    pub fn set_eq_band_2(&mut self, val: f64) {
        self.set_eq_band(2, val);
    }
    pub fn set_eq_band_3(&mut self, val: f64) {
        self.set_eq_band(3, val);
    }
    pub fn set_eq_band_4(&mut self, val: f64) {
        self.set_eq_band(4, val);
    }
    pub fn set_eq_band_5(&mut self, val: f64) {
        self.set_eq_band(5, val);
    }
    pub fn set_eq_band_6(&mut self, val: f64) {
        self.set_eq_band(6, val);
    }
    pub fn set_eq_band_7(&mut self, val: f64) {
        self.set_eq_band(7, val);
    }
    pub fn set_eq_band_8(&mut self, val: f64) {
        self.set_eq_band(8, val);
    }
    pub fn set_eq_band_9(&mut self, val: f64) {
        self.set_eq_band(9, val);
    }

    pub fn set_eq_enabled(&mut self, enabled: bool) {
        self.eq_enabled = enabled;

        let arc = crate::audio::dsp::eq::get_eq_enabled_arc();
        arc.store(
            if enabled { 1 } else { 0 },
            std::sync::atomic::Ordering::Relaxed,
        );

        self.eq_enabled_changed();
        self.save_dsp_config();
    }

    pub fn set_eq_instant_apply(&mut self) {
        // No-op: eq.rs already does instant updates in sync_from_atomics
    }

    pub fn set_active_preset_index(&mut self, index: i32) {
        self.active_preset_index = index;
        self.active_preset_index_changed();
        self.save_dsp_config();
    }

    pub fn get_active_preset_index(&self) -> i32 {
        self.active_preset_index
    }

    pub fn get_eq_band_value(&self, index: i32) -> f64 {
        let band = index as usize;
        if band < 10 {
            self.eq_bands[band] as f64
        } else {
            0.0
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
        self.save_dsp_config();
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

            self.user_presets_changed();
            self.save_dsp_config();
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

    // Load EQ preset with reactive property updates
    // This method updates internal state AND emits NOTIFY signals for all bands
    pub fn load_eq_preset(&mut self, index: i32) {
        if index < 0 || (index as usize) >= self.eq_presets.len() {
            return;
        }

        let preset = &self.eq_presets[index as usize];

        // Apply each band - set_eq_band updates self.eq_bands and emits NOTIFY
        for (i, &gain) in preset.gains.iter().enumerate() {
            self.eq_bands[i] = gain;

            // Update atomic for DSP engine
            let arc = crate::audio::dsp::eq::get_eq_bands_arc();
            arc[i].store(gain.to_bits(), std::sync::atomic::Ordering::Relaxed);

            // Emit band-specific NOTIFY signal for reactive QML binding
            match i {
                0 => self.eq_band_0_changed(),
                1 => self.eq_band_1_changed(),
                2 => self.eq_band_2_changed(),
                3 => self.eq_band_3_changed(),
                4 => self.eq_band_4_changed(),
                5 => self.eq_band_5_changed(),
                6 => self.eq_band_6_changed(),
                7 => self.eq_band_7_changed(),
                8 => self.eq_band_8_changed(),
                9 => self.eq_band_9_changed(),
                _ => {}
            }
        }

        self.active_preset_index = index;
        self.active_preset_index_changed();
        self.save_dsp_config();
    }

    // Load FX preset with reactive property updates
    // This method applies all FX parameters and emits their NOTIFY signals
    pub fn load_fx_preset(&mut self, index: i32) {
        if index < 0 || (index as usize) >= self.fx_presets.len() {
            return;
        }

        let preset = &self.fx_presets[index as usize];

        // Bass Booster
        self.bass_magic_active = preset.bass_enabled;
        self.bass_gain = preset.bass_gain as f64;
        self.bass_cutoff = preset.bass_cutoff as f64;
        crate::audio::dsp::bassbooster::get_bass_enabled_arc()
            .store(preset.bass_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::bassbooster::get_bass_gain_arc().store(
            preset.bass_gain.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::bassbooster::get_bass_freq_arc().store(
            preset.bass_cutoff.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.bass_magic_changed();
        self.bass_params_changed();

        // Crystalizer
        self.crystal_magic_active = preset.crystal_enabled;
        self.crystal_amount = preset.crystal_amount as f64;
        self.crystal_freq = preset.crystal_freq as f64;
        crate::audio::dsp::crystalizer::get_crystal_enabled_arc()
            .store(preset.crystal_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::crystalizer::get_crystal_amount_arc().store(
            preset.crystal_amount.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.crystal_magic_changed();

        // Surround
        self.surround_magic_active = preset.surround_enabled;
        self.surround_width = preset.surround_width as f64;
        crate::audio::dsp::surround::get_surround_enabled_arc().store(
            preset.surround_enabled,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::surround::get_surround_width_arc().store(
            preset.surround_width.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.surround_magic_changed();

        // Mono
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

        // Pitch Shifter
        self.pitch_active = preset.pitch_enabled;
        self.pitch_semitones = preset.pitch_semitones as f64;
        crate::audio::dsp::pitchshifter::get_pitch_enabled_arc()
            .store(preset.pitch_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::pitchshifter::get_pitch_ratio_arc().store(
            2.0_f32.powf(preset.pitch_semitones / 12.0).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.pitch_changed();

        // Middle Clarity
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

        // Stereo Enhancer
        self.stereo_active = preset.stereo_enabled;
        self.stereo_amount = preset.stereo_amount as f64;
        crate::audio::dsp::stereoenhance::get_stereo_enabled_arc()
            .store(preset.stereo_enabled, std::sync::atomic::Ordering::Relaxed);
        crate::audio::dsp::stereoenhance::get_stereo_amount_arc().store(
            preset.stereo_amount.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.stereo_changed();
        self.stereo_amount_changed();

        // Crossfeed
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

        // Compressor
        self.compressor_active = preset.compressor_enabled;
        self.compressor_threshold = preset.compressor_threshold as f64;
        crate::audio::dsp::compressor::get_compressor_enabled_arc().store(
            preset.compressor_enabled,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::audio::dsp::compressor::get_compressor_threshold_arc().store(
            preset.compressor_threshold.to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.compressor_changed();

        self.save_dsp_config();
    }

    fn apply_dsp_settings(&mut self, settings: &DspSettings) {
        // Update engine's DSP settings
        match self.ffmpeg.try_lock() {
            Ok(mut ff) => {
                ff.set_dsp_settings(settings.clone());
            }
            Err(_) => {
                // Retry after a small delay
                std::thread::sleep(std::time::Duration::from_millis(1));
                if let Ok(mut ff) = self.ffmpeg.lock() {
                    ff.set_dsp_settings(settings.clone());
                }
            }
        }
    }

    fn get_current_dsp_settings(&self) -> DspSettings {
        DspSettings {
            preamp_db: 0.0,
            bass_enabled: self.bass_magic_active,
            bass_gain: self.bass_gain as f32,
            bass_cutoff: self.bass_cutoff as f32,
            bass_q: 0.7,
            crystal_enabled: self.crystal_magic_active,
            crystal_amount: self.crystal_amount as f32,
            crystal_freq: 4000.0,
            surround_enabled: self.surround_magic_active,
            surround_width: self.surround_width as f32,
            surround_room_size: 15.0,
            surround_bass_safe: true,
            mono_enabled: self.mono_active,
            mono_width: self.mono_width as f32,
            pitch_enabled: self.pitch_active,
            pitch_semitones: self.pitch_semitones as f32,
            middle_enabled: self.middle_active,
            middle_amount: self.middle_amount as f32,
            compressor_enabled: self.compressor_active,
            stereo_enabled: self.stereo_active,
            stereo_amount: self.stereo_amount as f32,
            crossfeed_enabled: self.crossfeed_active,
            crossfeed_amount: self.crossfeed_amount as f32,
            eq_bands: self.eq_bands,
        }
    }

    fn save_dsp_config(&mut self) {
        let state = self.get_dsp_state_view();
        self.dsp_config.save_dsp_config(&state);
    }

    fn get_dsp_state_view(&self) -> DspStateView {
        DspStateView {
            dsp_enabled: self.dsp_enabled,
            dsp_bands: self.eq_bands,
            eq_enabled: self.eq_enabled,
            active_preset_index: self.active_preset_index,
            bass_magic_active: self.bass_magic_active,
            bass_gain: self.bass_gain,
            bass_cutoff: self.bass_cutoff,
            crystal_magic_active: self.crystal_magic_active,
            crystal_amount: self.crystal_amount,
            crystal_frdsp: self.crystal_freq,
            surround_magic_active: self.surround_magic_active,
            surround_width: self.surround_width,
            mono_active: self.mono_active,
            mono_width: self.mono_width,
            pitch_active: self.pitch_active,
            pitch_semitones: self.pitch_semitones,
            middle_active: self.middle_active,
            middle_amount: self.middle_amount,
            reverb_preset: self.reverb_preset,
            compressor_active: self.compressor_active,
            stereo_active: self.stereo_active,
            stereo_amount: self.stereo_amount,
            crossfeed_active: self.crossfeed_active,
            crossfeed_amount: self.crossfeed_amount,
            user_eq_names: self.user_eq_names.clone(),
            user_eq_gains: self.user_eq_gains,
            user_eq_macro: self.user_eq_macro,
        }
    }

    pub fn set_crystalizer_amount(&mut self, amount: f64) {
        let amount = amount.max(0.0).min(1.0);
        self.crystal_amount = amount;
        self.crystal_magic_active = amount > 0.0;
        self.crystal_magic_changed();

        // Update atomic directly
        let arc = get_crystal_amount_arc();
        arc.store(
            (amount as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_dsp_config();
    }

    pub fn set_crystalizer_freq(&mut self, freq: f64) {
        let freq = freq.max(2000.0).min(12000.0);
        self.crystal_freq = freq;
        self.crystal_magic_changed();
        crate::audio::dsp::crystalizer::get_crystal_freq_arc().store(
            (freq as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
        self.save_dsp_config();
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        self.playback.toggle_mute();
        self.mute_changed();
    }

    pub fn toggle_abrepeat(&mut self) {
        let current_position = self.position as f64 / 1000.0;
        self.abrepeat.toggle(current_position);

        // Emit signals for QML UI
        let state_val = match self.abrepeat.state() {
            crate::audio::dsp::abrepeat::ABRepeatState::Off => 0,
            crate::audio::dsp::abrepeat::ABRepeatState::ASet => 1,
            crate::audio::dsp::abrepeat::ABRepeatState::Active => 2,
        };
        self.ab_state = state_val;
        self.ab_state_changed();

        self.ab_point_a = (self.abrepeat.point_a() * 1000.0) as i32;
        self.ab_point_a_changed();

        self.ab_point_b = (self.abrepeat.point_b() * 1000.0) as i32;
        self.ab_point_b_changed();

        println!(
            "[ABREPEAT] State: {:?}, A: {:.2}s, B: {:.2}s",
            self.abrepeat.state(),
            self.abrepeat.point_a(),
            self.abrepeat.point_b()
        );
    }

    pub fn toggle_play(&mut self) {
        if self.is_playing {
            if let Ok(mut ff) = self.ffmpeg.lock() {
                ff.pause();
            }
            self.is_playing = false;
            self.playing_changed();
        } else {
            if let Ok(mut ff) = self.ffmpeg.lock() {
                ff.resume();
            }
            self.is_playing = true;
            self.playing_changed();
        }
    }

    pub fn load_track_info(&mut self, path: String) {
        self.track_info_visible = true;
        self.track_info_visible_changed();

        let meta = crate::audio::metadata::read_track_metadata(&path);
        self.track_info_title = QString::from(meta.title);
        self.track_info_artist = QString::from(meta.artist);
        self.track_info_album = QString::from(meta.album);
        self.track_info_year = QString::from(meta.year);
        self.track_info_genre = QString::from(meta.genre);
        self.track_info_codec = QString::from(meta.codec);
        self.track_info_channels = QString::from(match meta.channels {
            0 => String::new(),
            1 => "Mono".to_string(),
            2 => "Stereo".to_string(),
            n => format!("{} ch", n),
        });
        self.track_info_sample_rate = QString::from(if meta.sample_rate > 0 {
            format!("{} Hz", meta.sample_rate)
        } else {
            String::new()
        });
        self.track_info_bitrate = QString::from(if meta.bitrate_kbps > 0 {
            format!("{} kbps", meta.bitrate_kbps)
        } else {
            String::new()
        });

        let dur = meta.duration_sec;
        if dur > 0.0 {
            let mins = (dur as i64) / 60;
            let secs = (dur as i64) % 60;
            self.track_info_duration = QString::from(format!("{}:{:02}", mins, secs));
        } else {
            self.track_info_duration = QString::from("");
        }

        let bytes = meta.file_size_bytes;
        self.track_info_file_size = QString::from(if bytes > 0 {
            if bytes >= 1_073_741_824 {
                format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
            } else if bytes >= 1_048_576 {
                format!("{:.2} MB", bytes as f64 / 1_048_576.0)
            } else {
                format!("{} KB", bytes / 1024)
            }
        } else {
            String::new()
        });
        self.track_info_file_path = QString::from(meta.file_path.clone());

        self.track_info_changed();
    }

    pub fn close_track_info(&mut self) {
        self.track_info_visible = false;
        self.track_info_visible_changed();
    }

    pub fn update_tick(&mut self) {
        if !self.is_playing {
            return;
        }

        self.tick_counter += 1;

        // Periodic patroli ogni 50 tick (sekitar 2.5 detik)
        if self.tick_counter % 50 == 0 {
            self.refreshDeviceStatus();
        }

        let should_play_next = {
            if let Ok(mut ff) = self.ffmpeg.lock() {
                ff.update_tick();

                let pos_sec = ff.get_position();
                let pos = (pos_sec * 1000.0) as i32;

                if let Some(seek_to) = self.abrepeat.check_loop(pos_sec) {
                    ff.seek(seek_to);
                    self.position = (seek_to * 1000.0) as i32;
                    self.position_changed();
                }

                if pos != self.position {
                    self.position = pos;
                    self.position_changed();
                }

                let dur = (ff.get_duration() * 1000.0) as i32;

                if dur != self.duration {
                    self.duration = dur;
                    self.duration_changed();
                }

                ff.take_finished()
            } else {
                false
            }
        };

        if should_play_next {
            self.play_next();
        }

        if self.tick_counter % 50 == 0 {
            self.save_state();
        }
    }

    pub fn start_update_loop(&mut self) {
        // Restore last track if available (but start from beginning)
        let last_track = {
            if let Some(ref config) = &self.saved_config {
                if let Ok(cfg) = config.lock() {
                    if !cfg.last_track_path.is_empty() {
                        Some(cfg.last_track_path.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        let last_pos = 0; // Always start from beginning

        if let Some(track_path) = last_track {
            for (i, item) in self.display_list.iter().enumerate() {
                if item.path == track_path {
                    self.current_index = i as i32;
                    self.current_title = QString::from(item.name.clone());
                    // NOTE: Jangan auto-play saat startup, hanya ingat posisi terakhir
                    self.position = last_pos as i32;
                    self.is_playing = false; // Pastikan state playing = false saat startup
                    self.current_index_changed();
                    self.title_changed();
                    // Jangan panggil playing_changed() - biarkan user play manual
                    self.position_changed();

                    if let Ok(mut ff) = self.ffmpeg.lock() {
                        // Load track tapi langsung pause - agar siap play saat user klik
                        ff.play(&item.path);
                        if last_pos > 0 {
                            ff.seek(last_pos as f64 / 1000.0);
                        }
                        // Langsung pause agar tidak auto-play
                        ff.pause();
                    }

                    self.duration_changed();
                    break;
                }
            }
        }
    }

    pub fn save_state(&mut self) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.volume = self.volume as f64;
                cfg.balance = self.balance as f64;
                cfg.shuffle = self.shuffle_active;
                cfg.loop_playlist = self.loop_active;
                cfg.dsp_enabled = self.dsp_enabled;
                cfg.eq_enabled = self.eq_enabled;
                cfg.bass_enabled = self.bass_magic_active;
                cfg.bass_gain = self.bass_gain as f32;
                cfg.bass_cutoff = self.bass_cutoff as f32;
                cfg.crystal_enabled = self.crystal_magic_active;
                cfg.crystal_amount = self.crystal_amount as f32;
                cfg.crystal_freq = self.crystal_freq as f32;
                cfg.surround_enabled = self.surround_magic_active;
                cfg.surround_width = self.surround_width as f32;
                cfg.mono_enabled = self.mono_active;
                cfg.mono_width = self.mono_width as f32;
                cfg.pitch_enabled = self.pitch_active;
                cfg.pitch_semitones = self.pitch_semitones as f32;
                cfg.middle_enabled = self.middle_active;
                cfg.middle_amount = self.middle_amount as f32;
                cfg.reverb_preset = self.reverb_preset;
                cfg.compressor_enabled = self.compressor_active;
                let threshold_bits = crate::audio::dsp::compressor::get_compressor_threshold_arc()
                    .load(std::sync::atomic::Ordering::Relaxed);
                cfg.compressor_threshold = f32::from_bits(threshold_bits);

                // Save current track only (not position)
                if self.current_index >= 0
                    && (self.current_index as usize) < self.display_list.len()
                {
                    cfg.last_track_path =
                        self.display_list[self.current_index as usize].path.clone();
                }

                let _ = cfg.save();
            }
        }
    }

    pub fn save_window_position(&mut self, x: i32, y: i32, width: i32, height: i32) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.window_x = x;
                cfg.window_y = y;
                cfg.window_width = width;
                cfg.window_height = height;
                let _ = cfg.save();
            }
        }
    }

    pub fn get_window_config(&self) -> QVariantMap {
        let mut map = QVariantMap::default();
        if let Some(ref config) = &self.saved_config {
            if let Ok(cfg) = config.lock() {
                map.insert(QString::from("window_x"), QVariant::from(cfg.window_x));
                map.insert(QString::from("window_y"), QVariant::from(cfg.window_y));
                map.insert(
                    QString::from("window_width"),
                    QVariant::from(cfg.window_width),
                );
                map.insert(
                    QString::from("window_height"),
                    QVariant::from(cfg.window_height),
                );
            }
        }
        map
    }

    pub fn get_shared_config(
        &self,
    ) -> Option<std::sync::Arc<std::sync::Mutex<crate::audio::config::AppConfig>>> {
        self.saved_config.clone()
    }

    pub fn sync_theme_to_config(&mut self, theme_name: QString, custom_themes_json: QString) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.theme = theme_name.to_string();

                // Parse custom themes JSON
                let json_str = custom_themes_json.to_string();
                if let Ok(custom_themes) =
                    serde_json::from_str::<Vec<crate::audio::config::CustomTheme>>(&json_str)
                {
                    cfg.custom_themes = custom_themes;
                }

                // Save the config
                let _ = cfg.save();
            }
        }
    }
}
