/* --- loonixtunesv2/src/ui/core.rs | The Bridge (QML) --- */

#![allow(non_snake_case)]

use crate::audio::audiooutput::AudioOutput;
use crate::audio::engine::{is_audio_file, AudioState, FfmpegEngine, MusicItem};

use crate::audio::dsp::abrepeat::ABRepeat;
use crate::core::library::LibraryManager;
use crate::core::playback::PlaybackController;
use crate::ui::{DspController, QueueController};
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
    // Playback context - SEPARATED from UI display_list
    pub(crate) playback_playlist: Vec<MusicItem>,
    pub(crate) playback_index: i32,
    pub(crate) expanded_folders: HashSet<String>,
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
    pub(crate) loop_active: bool,
    pub(crate) abrepeat: ABRepeat,
    pub(crate) tick_counter: u32,

    pub(crate) output: AudioOutput,
    pub(crate) playback: PlaybackController,
    pub(crate) library: LibraryManager,
    pub(crate) dsp: DspController,
    pub(crate) queue: QueueController,

    pub is_playing: qt_property!(bool; READ get_is_playing NOTIFY playing_changed),
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

    pub balance: qt_property!(f64; NOTIFY balance_changed),
    pub balance_changed: qt_signal!(),

    pub ab_state: qt_property!(i32; NOTIFY ab_state_changed),
    pub ab_state_changed: qt_signal!(),
    pub ab_point_a: qt_property!(i32; NOTIFY ab_point_a_changed),
    pub ab_point_a_changed: qt_signal!(),
    pub ab_point_b: qt_property!(i32; NOTIFY ab_point_b_changed),
    pub ab_point_b_changed: qt_signal!(),

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

    pub(crate) saved_config:
        Option<std::sync::Arc<std::sync::Mutex<crate::audio::config::AppConfig>>>,

    // DSP wrapper properties for QML
    pub dsp_enabled: qt_property!(bool; NOTIFY dsp_changed),
    pub dsp_changed: qt_signal!(),
    pub reverb_active: qt_property!(bool; NOTIFY reverb_active_changed),
    pub reverb_active_changed: qt_signal!(),
    pub reverb_mode: qt_property!(i32; NOTIFY reverb_mode_changed),
    pub reverb_mode_changed: qt_signal!(),
    pub reverb_amount: qt_property!(i32; NOTIFY reverb_amount_changed),
    pub reverb_amount_changed: qt_signal!(),
    pub bass_active: qt_property!(bool; NOTIFY bass_active_changed),
    pub bass_active_changed: qt_signal!(),
    pub bass_gain: qt_property!(f64; NOTIFY bass_gain_changed),
    pub bass_cutoff: qt_property!(f64; NOTIFY bass_cutoff_changed),
    pub bass_mode: qt_property!(i32; NOTIFY bass_mode_changed),
    pub bass_gain_changed: qt_signal!(),
    pub bass_cutoff_changed: qt_signal!(),
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
    pub eqBands: qt_property!(QVariantList; NOTIFY eqBandsChanged),
    pub eqBandsChanged: qt_signal!(),
    pub fader_offset: qt_property!(f64; NOTIFY faderOffsetChanged),
    pub faderOffsetChanged: qt_signal!(),
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
    pub reverb_room_size: qt_property!(f64; NOTIFY reverb_room_size_changed),
    pub reverb_damp: qt_property!(f64; NOTIFY reverb_damp_changed),
    pub reverb_room_size_changed: qt_signal!(),
    pub reverb_damp_changed: qt_signal!(),
    pub user_preset_names: qt_property!(QVariantList; NOTIFY user_presets_changed),
    pub user_presets_changed: qt_signal!(),

    // DSP wrapper methods for QML
    pub set_reverb_mode: qt_method!(fn(&mut self, mode: i32)),
    pub set_reverb_amount: qt_method!(fn(&mut self, amount: i32)),
    pub setStdReverbRoomSize: qt_method!(fn(&mut self, val: f64)),
    pub setStdReverbDamp: qt_method!(fn(&mut self, val: f64)),
    pub toggleReverb: qt_method!(fn(&mut self)),
    pub set_reverb: qt_method!(fn(&mut self, reverb: QString)),
    pub toggleBassBooster: qt_method!(fn(&mut self)),
    pub set_bass_mode: qt_method!(fn(&mut self, mode: i32)),
    pub setStdBassGain: qt_method!(fn(&mut self, val: f64)),
    pub setStdBassCutoff: qt_method!(fn(&mut self, val: f64)),
    pub toggleSurround: qt_method!(fn(&mut self)),
    pub setStdSurroundWidth: qt_method!(fn(&mut self, val: f64)),
    pub toggleCrystalizer: qt_method!(fn(&mut self)),
    pub set_crystalizer_amount: qt_method!(fn(&mut self, amount: f64)),
    pub toggleCompressor: qt_method!(fn(&mut self)),
    pub setStdCompressorThreshold: qt_method!(fn(&mut self, val: f64)),
    pub getStdCompressorThreshold: qt_method!(fn(&self) -> f64),
    pub togglePitch: qt_method!(fn(&mut self)),
    pub setStdPitchSemitones: qt_method!(fn(&mut self, val: f64)),
    pub toggleMiddleClarity: qt_method!(fn(&mut self)),
    pub setStdMiddleClarityAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleStereoWidth: qt_method!(fn(&mut self)),
    pub setStdStereoWidthAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleStereoEnhance: qt_method!(fn(&mut self)),
    pub setStdStereoEnhanceAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleCrossfeed: qt_method!(fn(&mut self)),
    pub setStdCrossfeedAmount: qt_method!(fn(&mut self, val: f64)),
    pub toggleDsp: qt_method!(fn(&mut self)),
    pub resetAllDsp: qt_method!(fn(&mut self)),
    pub togglePreamp: qt_method!(fn(&mut self)),
    pub toggleLimiter: qt_method!(fn(&mut self)),
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
    pub toggle_mute: qt_method!(fn(&mut self)),
    pub toggle_abrepeat: qt_method!(fn(&mut self)),
    pub toggle_play: qt_method!(fn(&mut self)),

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

    pub sync_theme_to_config:
        qt_method!(fn(&mut self, theme_name: QString, custom_themes_json: QString)),

    pub external_files_count: qt_property!(i32; NOTIFY external_files_changed),
    pub external_files_changed: qt_signal!(),
    pub add_external_file: qt_method!(fn(&mut self, path: String)),
    pub switch_to_external_files: qt_method!(fn(&mut self)),
    pub clear_external_files: qt_method!(fn(&mut self)),
    pub process_command_line_files: qt_method!(fn(&mut self)),

    pub update_status: qt_property!(QString; NOTIFY update_status_changed),
    pub update_available: qt_property!(bool; NOTIFY update_status_changed),
    pub update_status_changed: qt_signal!(),
    pub check_for_updates: qt_method!(fn(&mut self)),
    pub poll_update_result: qt_method!(fn(&mut self)),
    update_rx: Option<std::sync::mpsc::Receiver<String>>,

    pub device_list: qt_property!(QStringList; NOTIFY device_list_changed),
    pub selected_device: qt_property!(QString; NOTIFY device_list_changed),
    pub device_list_changed: qt_signal!(),
    pub bluetooth_detected: qt_property!(bool; NOTIFY device_status_changed),
    pub systemMuted: qt_property!(bool; NOTIFY systemMutedChanged),
    pub systemMutedChanged: qt_signal!(),
    pub device_status_changed: qt_signal!(),
    pub refreshDeviceList: qt_method!(fn(&mut self)),
    pub selectDevice: qt_method!(fn(&mut self, deviceName: String)),

    pub favorites_count: qt_property!(i32; NOTIFY favorites_changed),
    pub favorites_changed: qt_signal!(),
    pub add_favorite: qt_method!(fn(&mut self, path: String, name: String)),
    pub remove_favorite: qt_method!(fn(&mut self, path: String)),
    pub is_favorite: qt_method!(fn(&self, path: String) -> bool),
    pub toggle_favorite: qt_method!(fn(&mut self, path: String, name: String)),
    pub switch_to_favorites: qt_method!(fn(&mut self)),
    pub switch_to_music: qt_method!(fn(&mut self)),

    pub get_output_devices: qt_method!(fn(&self) -> QVariantList),
    pub set_output_device: qt_method!(fn(&mut self, index: i32)),
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
    pub fn get_is_playing(&self) -> bool {
        self.playback.is_playing()
    }

    pub fn new() -> Self {
        let saved_config = crate::audio::config::AppConfig::load();

        let custom_folders = saved_config.custom_folders.clone();
        let favorites = saved_config.favorites.clone();

        let ffmpeg = Arc::new(Mutex::new(FfmpegEngine::new()));

        let saved_config_arc = Some(std::sync::Arc::new(std::sync::Mutex::new(
            saved_config.clone(),
        )));

        let mut dsp = DspController::new(ffmpeg.clone(), saved_config_arc.clone());
        dsp.init_from_config(&saved_config);

        let mut model = Self {
            ffmpeg,
            audio: Arc::new(Mutex::new(AudioState::default())),
            output: AudioOutput::default(),
            volume: saved_config.volume as f64,
            current_index: -1,
            balance: saved_config.balance as f64,
            custom_folder_count: saved_config.custom_folders.len() as i32,
            favorites: saved_config.favorites.clone(),
            favorites_count: saved_config.favorites.len() as i32,
            external_files: Vec::new(),
            external_files_count: 0,
            queue: QueueController::new(),
            queue_count: 0,
            dsp,
            ..Default::default()
        };

        model.saved_config = saved_config_arc;

        let dsp_settings = crate::audio::dsp::DspSettings::default();
        model.output.set_dsp_enabled(dsp_settings.dsp_enabled);
        model
            .output
            .set_normalizer_enabled(saved_config.normalizer_enabled);
        model.output.mode = saved_config.mode;

        model.dsp.emit_all_signals();

        // Initialize user preset names
        model.user_preset_names = model.dsp.get_user_preset_names_list();

        // Initialize DSP wrapper properties
        model.sync_dsp_from_controller();

        model.volume_changed();

        // Apply saved volume to audio engine
        if let Ok(mut ff) = model.ffmpeg.lock() {
            ff.set_volume(model.volume as f32);
        }

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
                self.library.custom_folders = self.library.custom_folders.clone();
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
        self.queue.add(path, name);
        self.queue_count = self.queue.len() as i32;
        self.queue_changed();
    }

    pub fn remove_from_queue(&mut self, index: i32) {
        self.queue.remove(index as usize);
        self.queue_count = self.queue.len() as i32;
        self.queue_changed();
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();
        self.queue_count = 0;
        self.queue_changed();
    }

    pub fn get_queue_item(&self, index: i32) -> QVariantMap {
        self.queue.get_item_map(index)
    }

    pub fn switch_to_queue(&mut self) {
        self.current_folder = "Queue".to_string();
        self.current_folder_path = String::new();
        self.current_folder_qml = QString::from("QUEUE");
        self.all_items = self.queue.get_all();
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
        // Save from library (single source of truth), NOT from self.library.custom_folders
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.custom_folders = self.library.custom_folders.clone();
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
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            let folder_path = Path::new(&new_path);
            if let Some(name) = folder_path.file_name() {
                let mut name_str = name.to_string_lossy().to_string();
                name_str.truncate(15);
                name_str = name_str.trim().to_string();
                self.library.custom_folders[index as usize] = (name_str, new_path.clone());
                self.custom_folders_changed();
                self.save_custom_folders();
                self.switch_to_folder(new_path);
            }
        }
    }

    pub fn switch_to_folder(&mut self, folder_path: String) {
        let path = Path::new(&folder_path);

        self.current_folder_path = folder_path.clone();

        // Try to find display name from library.custom_folders first
        let display_name = self
            .library
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
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            let name = self.library.custom_folders[index as usize].0.clone();
            // Convert to uppercase
            QString::from(name.to_uppercase())
        } else {
            QString::default()
        }
    }

    pub fn get_custom_folder_path(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            QString::from(self.library.custom_folders[index as usize].1.clone())
        } else {
            QString::default()
        }
    }

    pub fn get_current_rename_name(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            QString::from(self.library.custom_folders[index as usize].0.clone())
        } else {
            QString::default()
        }
    }

    pub fn rename_folder(&mut self, index: i32, new_name: String) {
        if index < 0 || (index as usize) >= self.library.custom_folders.len() {
            return;
        }

        let mut trimmed = new_name.trim().to_string();
        trimmed.truncate(15);

        if trimmed.is_empty() {
            return;
        }

        self.library.custom_folders[index as usize].0 = trimmed;
        self.custom_folders_changed();
        self.save_custom_folders();
    }

    pub fn get_custom_folder_count(&self) -> i32 {
        self.library.custom_folders.len() as i32
    }

    pub fn remove_custom_folder(&mut self, index: i32) {
        if index < 0 || (index as usize) >= self.library.custom_folders.len() {
            return;
        }
        if self.is_folder_locked(index) {
            return;
        }
        let removed_path = self.library.custom_folders[index as usize].1.clone();

        self.library.custom_folders.remove(index as usize);

        self.custom_folder_count = self.library.custom_folders.len() as i32;
        self.custom_folders_changed();
        self.save_custom_folders();

        if self.current_folder_path == removed_path {
            self.scan_music();
        }
    }

    pub fn toggle_folder_lock(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            if let Some(ref config) = &self.saved_config {
                if let Ok(mut cfg) = config.lock() {
                    if cfg.locked_folders.contains(&index) {
                        cfg.locked_folders.retain(|&i| i != index);
                    } else {
                        cfg.locked_folders.push(index);
                    }
                    cfg.custom_folders = self.library.custom_folders.clone();
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

        // Set playback context - THIS IS THE SOURCE OF TRUTH for auto-next
        self.playback_playlist = self.display_list.clone();
        self.playback_index = index;

        self.current_index = index;
        self.playback.play_at(item);
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
        self.playing_changed();
    }

    pub fn stop(&mut self) {
        self.stop_playback();
    }

    fn play_next_from_queue(&mut self) -> bool {
        if let Some(item) = self.queue.pop_front() {
            self.queue_count = self.queue.len() as i32;
            self.queue_changed();
            if let Some(index) = self.display_list.iter().position(|i| i.path == item.path) {
                self.play_at(index as i32);
                return true;
            }
            self.play_next_from_queue()
        } else {
            false
        }
    }

    pub fn play_next(&mut self) {
        // Check user queue first
        if self.play_next_from_queue() {
            return;
        }

        // Use PlaybackController's shuffle logic
        if let Some((next_idx, next_item)) = self.playback.play_next(&self.playback_playlist, self.playback_index) {
            // Update state to sync with controller
            self.playback_index = next_idx as i32;
            self.current_index = self.playback_index;
            self.current_title = QString::from(next_item.name.clone());

            // Tell audio engine to play
            self.playback.play_at(&next_item);
            self.position = self.playback.position;
            self.duration = self.playback.duration;

            // Notify QML that data changed
            self.current_index_changed();
            self.title_changed();
            self.is_playing = true;
            self.playing_changed();
            self.position_changed();
            self.duration_changed();
        } else {
            // No more tracks and loop is off
            self.playback.stop();
            self.is_playing = false;
            self.playing_changed();
        }
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
        self.playback.toggle_shuffle(&self.display_list, self.current_index);
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

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        self.playback.toggle_mute();
        self.mute_changed();
    }

    // DSP wrapper methods - delegate to DspController
    pub fn set_reverb_mode(&mut self, mode: i32) {
        self.dsp.set_reverb_mode(mode);
        self.reverb_mode = self.dsp.reverb_mode;
        self.reverb_mode_changed();
        self.reverb_active = self.dsp.reverb_active;
        self.reverb_active_changed();
    }

    pub fn set_reverb_amount(&mut self, amount: i32) {
        self.dsp.set_reverb_amount(amount);
        self.reverb_amount = self.dsp.reverb_amount;
        self.reverb_amount_changed();
    }

    pub fn setStdReverbRoomSize(&mut self, val: f64) {
        self.dsp.set_reverb_room_size(val);
        self.reverb_room_size = self.dsp.reverb_room_size;
        self.reverb_room_size_changed();
    }

    pub fn setStdReverbDamp(&mut self, val: f64) {
        self.dsp.set_reverb_damp(val);
        self.reverb_damp = self.dsp.reverb_damp;
        self.reverb_damp_changed();
    }

    pub fn toggleReverb(&mut self) {
        self.dsp.toggle_reverb();
        self.reverb_mode = self.dsp.reverb_mode;
        self.reverb_active = self.dsp.reverb_active;
        self.reverb_active_changed();
        self.reverb_mode_changed();
    }

    pub fn set_reverb(&mut self, reverb: QString) {
        self.dsp.set_reverb(reverb.to_string());
        self.reverb_mode = self.dsp.reverb_mode;
        self.reverb_active = self.dsp.reverb_active;
        self.reverb_active_changed();
        self.reverb_mode_changed();
    }

    pub fn toggleBassBooster(&mut self) {
        self.dsp.toggle_bass();
        self.sync_dsp_from_controller();
    }

    pub fn set_bass_mode(&mut self, mode: i32) {
        self.dsp.set_bass_mode(mode);
        self.sync_dsp_from_controller();
    }

    pub fn setStdBassGain(&mut self, val: f64) {
        self.dsp.set_bass_gain(val);
        self.sync_dsp_from_controller();
    }

    pub fn setStdBassCutoff(&mut self, val: f64) {
        self.dsp.set_bass_cutoff(val);
        self.sync_dsp_from_controller();
    }

    pub fn toggleSurround(&mut self) {
        self.dsp.toggle_surround();
        self.sync_dsp_from_controller();
    }

    pub fn setStdSurroundWidth(&mut self, val: f64) {
        self.dsp.set_surround_width(val);
        self.sync_dsp_from_controller();
    }

    pub fn toggleCrystalizer(&mut self) {
        self.dsp.toggle_crystalizer();
        self.sync_dsp_from_controller();
    }

    pub fn set_crystalizer_amount(&mut self, amount: f64) {
        self.dsp.set_crystalizer_amount(amount);
        self.crystal_amount = self.dsp.crystal_amount;
        self.crystal_active_changed();
    }

    pub fn toggleCompressor(&mut self) {
        self.dsp.toggle_compressor();
        self.sync_dsp_from_controller();
    }

    pub fn setStdCompressorThreshold(&mut self, val: f64) {
        self.dsp.set_compressor_threshold(val);
        self.compressor_threshold = self.dsp.compressor_threshold;
        self.compressor_threshold_changed();
    }

    pub fn getStdCompressorThreshold(&self) -> f64 {
        self.dsp.get_compressor_threshold()
    }

    pub fn togglePitch(&mut self) {
        self.dsp.toggle_pitch();
        self.sync_dsp_from_controller();
    }

    pub fn setStdPitchSemitones(&mut self, val: f64) {
        self.dsp.set_pitch_semitones(val);
        self.sync_dsp_from_controller();
    }

    pub fn toggleMiddleClarity(&mut self) {
        self.dsp.toggle_middle_clarity();
        self.sync_dsp_from_controller();
    }

    pub fn setStdMiddleClarityAmount(&mut self, val: f64) {
        self.dsp.set_middle_clarity_amount(val);
        self.sync_dsp_from_controller();
    }

    pub fn toggleStereoWidth(&mut self) {
        self.dsp.toggle_stereo_width();
        self.sync_dsp_from_controller();
    }

    pub fn setStdStereoWidthAmount(&mut self, val: f64) {
        self.dsp.set_stereo_width_amount(val);
        self.sync_dsp_from_controller();
    }

    pub fn toggleStereoEnhance(&mut self) {
        self.dsp.toggle_stereo_enhance();
        self.sync_dsp_from_controller();
    }

    pub fn setStdStereoEnhanceAmount(&mut self, val: f64) {
        self.dsp.set_stereo_enhance_amount(val);
        self.sync_dsp_from_controller();
    }

    pub fn toggleCrossfeed(&mut self) {
        self.dsp.toggle_crossfeed();
        self.sync_dsp_from_controller();
    }

    pub fn setStdCrossfeedAmount(&mut self, val: f64) {
        self.dsp.set_crossfeed_amount(val);
        self.sync_dsp_from_controller();
    }

    pub fn toggleDsp(&mut self) {
        self.dsp.toggle_dsp();
        self.sync_dsp_from_controller();
    }

    pub fn resetAllDsp(&mut self) {
        // FIRST: Handle Compressor - stay ON with 50% threshold
        if !self.compressor_active {
            self.toggleCompressor();
        }
        self.setStdCompressorThreshold(0.5);

        // SECOND: Force OFF all other FX
        if self.surround_active {
            self.toggleSurround();
        }
        if self.mono_active {
            self.toggleStereoWidth();
        }
        if self.middle_active {
            self.toggleMiddleClarity();
        }
        if self.stereo_active {
            self.toggleStereoEnhance();
        }
        if self.crossfeed_active {
            self.toggleCrossfeed();
        }
        if self.crystal_active {
            self.toggleCrystalizer();
        }
        if self.bass_active {
            self.toggleBassBooster();
        }
        if self.reverb_active {
            self.toggleReverb();
        }
        if self.pitch_active {
            self.togglePitch();
        }

        // THIRD: Set all values to ZERO (except compressor which is handled above)
        // Set EQ flat
        for i in 0..10 {
            self.set_eq_band(i as i32, 0.0);
        }
        // Note: preamp and fader NOT reset - they are always ON

        // Set all FX amounts to ZERO
        self.setStdSurroundWidth(0.0);
        self.setStdStereoWidthAmount(0.0);
        self.setStdMiddleClarityAmount(0.0);
        self.setStdStereoEnhanceAmount(0.0);
        self.setStdCrossfeedAmount(0.0);
        self.set_crystalizer_amount(0.0);
        self.setStdBassGain(0.0);
        self.set_reverb_amount(0);
        self.setStdPitchSemitones(0.0);

        // FINAL: Sync everything to ensure UI matches DSP state
        self.sync_dsp_from_controller();
    }

    pub fn togglePreamp(&mut self) {
        self.dsp.toggle_preamp();
        self.sync_dsp_from_controller();
    }

    pub fn toggleLimiter(&mut self) {
        self.dsp.toggle_limiter();
        self.sync_dsp_from_controller();
    }

    pub fn toggle_normalizer(&mut self) {
        self.dsp.toggle_normalizer();
        self.sync_dsp_from_controller();
    }

    pub fn set_eq_band(&mut self, index: i32, gain: f64) {
        self.dsp.set_eq_band(index, gain);
        self.eqBands = self.dsp.sync_eq_bands();
        self.eqBandsChanged();
    }

    pub fn set_fader(&mut self, offset: f64) {
        self.dsp.set_fader(offset);
        self.fader_offset = self.dsp.fader_offset;
        self.eqBands = self.dsp.sync_eq_bands();
        self.faderOffsetChanged();
        self.eqBandsChanged();
    }

    pub fn set_eq_enabled(&mut self, enabled: bool) {
        self.dsp.set_eq_enabled(enabled);
        self.eq_enabled = self.dsp.eq_enabled;
        self.eq_enabled_changed();
    }

    pub fn set_eq_instant_apply(&mut self) {
        self.dsp.set_eq_instant_apply();
    }

    pub fn get_preamp_gain(&self) -> f64 {
        self.dsp.get_preamp_gain()
    }

    pub fn set_preamp_gain(&mut self, gain: f64) {
        self.dsp.set_preamp_gain(gain);
    }

    pub fn save_user_eq(&mut self, preset: i32, name: String, macro_val: f64) {
        self.dsp.save_user_eq(preset, name, macro_val);
        self.user_presets_changed();
    }

    pub fn save_user_preset(&mut self, slot: usize, name: String) -> i32 {
        let result = self.dsp.save_user_preset(slot, name);
        self.user_preset_names = self.dsp.get_user_preset_names_list();
        self.user_presets_changed();
        result
    }

    pub fn get_eq_preset_count(&self) -> i32 {
        self.dsp.get_eq_preset_count()
    }

    pub fn get_eq_preset_name(&self, index: i32) -> QString {
        self.dsp.get_eq_preset_name(index)
    }

    pub fn get_eq_preset_gains(&self, index: i32) -> QVariantList {
        self.dsp.get_eq_preset_gains(index)
    }

    pub fn get_fx_preset_count(&self) -> i32 {
        self.dsp.get_fx_preset_count()
    }

    pub fn get_fx_preset_name(&self, index: i32) -> QString {
        self.dsp.get_fx_preset_name(index)
    }

    pub fn load_preset(&mut self, index: i32) {
        self.dsp.load_preset(index);
        self.sync_dsp_from_controller();
        self.active_preset_index = self.dsp.active_preset_index;
        self.active_preset_index_changed();
    }

    pub fn load_eq_preset(&mut self, index: i32) {
        self.dsp.load_eq_preset(index);
        self.sync_dsp_from_controller();
    }

    pub fn load_fx_preset(&mut self, index: i32) {
        self.dsp.load_fx_preset(index);
        self.sync_dsp_from_controller();
    }

    pub fn set_active_preset_index(&mut self, index: i32) {
        self.dsp.set_active_preset_index(index);
        self.active_preset_index = self.dsp.active_preset_index;
        self.active_preset_index_changed();
    }

    pub fn get_active_preset_index(&self) -> i32 {
        self.dsp.active_preset_index
    }

    pub fn get_user_eq_gains(&self, preset: i32) -> QVariantList {
        self.dsp.get_user_eq_gains(preset)
    }

    pub fn get_user_eq_macro(&self, preset: i32) -> f64 {
        self.dsp.get_user_eq_macro(preset)
    }

    pub fn get_user_preset_name(&self, preset: i32) -> QString {
        self.dsp.get_user_preset_name(preset)
    }

    pub fn reset_compressor(&mut self) {
        self.dsp.compressor_indie_reset();
        self.compressor_threshold = self.dsp.get_compressor_threshold();
        self.compressor_threshold_changed();
        self.sync_dsp_from_controller();
    }
    pub fn reset_surround(&mut self) {
        self.dsp.surround_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_stereo_width(&mut self) {
        self.dsp.stereo_width_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_middle_clarity(&mut self) {
        self.dsp.middle_clarity_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_stereo_enhance(&mut self) {
        self.dsp.stereo_enhance_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_crossfeed(&mut self) {
        self.dsp.crossfeed_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_crystalizer(&mut self) {
        self.dsp.crystalizer_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_bass(&mut self) {
        self.dsp.bass_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_reverb(&mut self) {
        self.dsp.reverb_indie_reset();
        self.sync_dsp_from_controller();
    }
    pub fn reset_pitch(&mut self) {
        self.dsp.pitch_indie_reset();
        self.sync_dsp_from_controller();
    }

    fn sync_dsp_from_controller(&mut self) {
        self.dsp_enabled = self.dsp.dsp_enabled;
        self.dsp_changed();
        self.reverb_active = self.dsp.reverb_active;
        self.reverb_active_changed();
        self.reverb_mode = self.dsp.reverb_mode;
        self.reverb_mode_changed();
        self.reverb_amount = self.dsp.reverb_amount;
        self.reverb_amount_changed();
        self.reverb_room_size = self.dsp.reverb_room_size;
        self.reverb_room_size_changed();
        self.reverb_damp = self.dsp.reverb_damp;
        self.reverb_damp_changed();
        self.bass_active = self.dsp.bass_active;
        self.bass_active_changed();
        self.bass_gain = self.dsp.bass_gain;
        self.bass_gain_changed();
        self.bass_cutoff = self.dsp.bass_cutoff;
        self.bass_cutoff_changed();
        self.bass_mode = self.dsp.bass_mode;
        self.bass_mode_changed();
        self.surround_active = self.dsp.surround_active;
        self.surround_active_changed();
        self.surround_width = self.dsp.surround_width;
        self.surround_width_changed();
        self.crystal_active = self.dsp.crystal_active;
        self.crystal_active_changed();
        self.crystal_amount = self.dsp.crystal_amount;
        self.crystal_amount_changed();
        self.compressor_active = self.dsp.compressor_active;
        self.compressor_active_changed();
        self.compressor_threshold = self.dsp.compressor_threshold;
        self.compressor_threshold_changed();
        self.mono_active = self.dsp.mono_active;
        self.mono_changed();
        self.mono_width = self.dsp.mono_width;
        self.mono_width_changed();
        self.middle_active = self.dsp.middle_active;
        self.middle_changed();
        self.middle_amount = self.dsp.middle_amount;
        self.middle_amount_changed();
        self.stereo_active = self.dsp.stereo_active;
        self.stereo_changed();
        self.stereo_amount = self.dsp.stereo_amount;
        self.stereo_amount_changed();
        self.crossfeed_active = self.dsp.crossfeed_active;
        self.crossfeed_changed();
        self.crossfeed_amount = self.dsp.crossfeed_amount;
        self.crossfeed_amount_changed();
        self.eq_enabled = self.dsp.eq_enabled;
        self.eq_enabled_changed();
        self.eqBands = self.dsp.sync_eq_bands();
        self.eqBandsChanged();
        self.fader_offset = self.dsp.fader_offset;
        self.faderOffsetChanged();
        self.pitch_active = self.dsp.pitch_active;
        self.pitch_changed();
        self.pitch_semitones = self.dsp.pitch_semitones;
        self.preamp_active = self.dsp.preamp_active;
        self.preamp_changed();
        self.limiter_active = self.dsp.limiter_active;
        self.limiter_changed();
        self.normalizer_enabled = self.dsp.normalizer_enabled;
        self.normalizer_changed();
        self.active_preset_index = self.dsp.active_preset_index;
        self.active_preset_index_changed();
    }

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
                let url = "https://api.github.com/repos/citzeye/loonix-tunes-linux/releases/latest";
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

    pub fn is_playing(&self) -> bool {
        self.playback.is_playing()
    }

    pub fn toggle_play(&mut self) {
        self.playback.toggle();
        self.playing_changed();
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
                    self.position = last_pos as i32;
                    self.current_index_changed();
                    self.title_changed();
                    self.position_changed();

                    if let Ok(mut ff) = self.ffmpeg.lock() {
                        ff.load(&item.path);
                        if last_pos > 0 {
                            ff.seek(last_pos as f64 / 1000.0);
                        }
                    }

                    self.duration_changed();
                    break;
                }
            }
        }
    }

    pub fn save_state(&mut self) {
        self.dsp.save_config();

        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.volume = self.volume as f64;
                cfg.balance = self.balance as f64;
                cfg.shuffle = self.shuffle_active;
                cfg.loop_playlist = self.loop_active;

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
        use crate::ui::theme::{CustomTheme, ThemeConfig, ThemeEntry};

        let json_str = custom_themes_json.to_string();
        let custom_themes = serde_json::from_str::<Vec<CustomTheme>>(&json_str)
            .unwrap_or_default();

        let theme_config = ThemeConfig {
            active_theme: theme_name.to_string(),
            themes: custom_themes.into_iter().map(|c| ThemeEntry {
                name: c.name,
                is_active: false,
                colors: Some(c.colors),
            }).collect(),
        };
        let _ = theme_config.save();
    }
}
