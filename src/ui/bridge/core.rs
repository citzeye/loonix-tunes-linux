/* --- loonixtunesv2/src/ui/bridge/core.rs | core --- */
use crate::audio::io::audiooutput::AudioOutput;
use crate::audio::engine::{AudioState, FfmpegEngine, MusicItem};
use crate::core::services::get_file_service;
use crate::core::library::library::Library;
use crate::core::services::PlaybackController;
use crate::core::library::metadata::read_track_metadata;
use crate::ui::QueueController;
use dirs;
use qmetaobject::prelude::*;
use qmetaobject::QAbstractListModel;
use qmetaobject::QStringList;
use qmetaobject::QVariantList;
use qmetaobject::QVariantMap;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

static COMMAND_LINE_FILES: OnceLock<Vec<String>> = OnceLock::new();

pub fn set_command_line_files(files: Vec<String>) {
    COMMAND_LINE_FILES.set(files).ok();
}

pub fn get_command_line_files() -> &'static Vec<String> {
    COMMAND_LINE_FILES.get_or_init(Vec::new)
}

fn clean_qml_path(path: &str) -> String {
    if path.starts_with("file://") {
        if let Ok(url) = url::Url::parse(path) {
            if let Ok(p) = url.to_file_path() {
                return p.to_string_lossy().to_string();
            }
        }
    }
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

    // --- Data Bridge ---
    pub(crate) all_items: Vec<MusicItem>,
    pub(crate) display_list: Vec<MusicItem>,
    pub(crate) playback_playlist: Vec<MusicItem>,
    pub(crate) playback_index: i32,
    pub(crate) expanded_folders: HashSet<String>,
    pub(crate) session_folders: Vec<String>, // Session-only folders (RAM only, not saved)

    pub current_folder_qml: qt_property!(QString; NOTIFY current_folder_changed),
    pub current_folder_changed: qt_signal!(),
    
    // Current tab root for clean slate logic
    pub current_tab_root: qt_property!(QString; NOTIFY current_tab_root_changed),
    pub current_tab_root_changed: qt_signal!(),

    // --- Controllers ---
    pub(crate) ffmpeg: Arc<Mutex<FfmpegEngine>>,
    pub(crate) audio: Arc<Mutex<AudioState>>,
    pub(crate) output: AudioOutput,
    pub(crate) playback: PlaybackController,
    pub(crate) library: Library,
    pub(crate) queue: QueueController,

    // --- State Properties ---
    pub(crate) tick_counter: u32,
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

    // --- Track Info Metadata ---
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

    pub(crate) saved_config: Option<std::sync::Arc<std::sync::Mutex<crate::audio::config::AppConfig>>>,

    // --- QML Methods ---
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
    pub toggle_play: qt_method!(fn(&mut self)),
    pub toggle_folder: qt_method!(fn(&mut self, index: i32)),
    pub load_track_info: qt_method!(fn(&mut self, path: String)),
    pub close_track_info: qt_method!(fn(&mut self)),
    pub update_tick: qt_method!(fn(&mut self)),
    pub start_update_loop: qt_method!(fn(&mut self)),
    pub save_state: qt_method!(fn(&mut self)),
    pub save_window_position: qt_method!(fn(&mut self, x: i32, y: i32, width: i32, height: i32)),
    pub get_window_config: qt_method!(fn(&self) -> QVariantMap),
    
    // Bridge to Library
    pub add_folder_tab: qt_method!(fn(&mut self, path: String)),
    pub add_song: qt_method!(fn(&mut self, path: String)),
    pub add_temporary_folder: qt_method!(fn(&mut self, path: String)),
    pub add_folder_to_list: qt_method!(fn(&mut self, path: String)),
    pub remove_song: qt_method!(fn(&mut self, index: i32)),
    pub delete_item: qt_method!(fn(&mut self, path: String, is_folder: bool)),
    pub switch_to_folder: qt_method!(fn(&mut self, folder_path: String)),
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
    pub sync_theme_to_config: qt_method!(fn(&mut self, theme_name: QString, custom_themes_json: QString)),

    // Bridge to Queue
    pub add_to_queue: qt_method!(fn(&mut self, path: String, name: String)),
    pub remove_from_queue: qt_method!(fn(&mut self, index: i32)),
    pub clear_queue: qt_method!(fn(&mut self)),
    pub get_queue_item: qt_method!(fn(&self, index: i32) -> QVariantMap),
    pub switch_to_queue: qt_method!(fn(&mut self)),
    pub queue_count: qt_property!(i32; NOTIFY queue_changed),
    pub queue_changed: qt_signal!(),

    // External Files
    pub external_files_count: qt_property!(i32; NOTIFY external_files_changed),
    pub external_files_changed: qt_signal!(),
    pub add_external_file: qt_method!(fn(&mut self, path: String)),
    pub switch_to_external_files: qt_method!(fn(&mut self)),
    pub clear_external_files: qt_method!(fn(&mut self)),
    pub process_command_line_files: qt_method!(fn(&mut self)),

    // Updates & Devices
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
    pub system_muted: qt_property!(bool; NOTIFY system_muted_changed),
    pub system_muted_changed: qt_signal!(),
    pub device_status_changed: qt_signal!(),
    pub select_device: qt_method!(fn(&mut self, device_name: String)),
    pub get_output_devices: qt_method!(fn(&self) -> QVariantList),
    pub set_output_device: qt_method!(fn(&mut self, index: i32)),

    // Bridge to Favorites
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
    pub fn get_is_playing(&self) -> bool {
        self.playback.is_playing()
    }

    pub fn get_ffmpeg(&self) -> std::sync::Arc<std::sync::Mutex<crate::audio::engine::FfmpegEngine>> {
        self.ffmpeg.clone()
    }

    pub fn get_shared_config(&self) -> Option<std::sync::Arc<std::sync::Mutex<crate::audio::config::AppConfig>>> {
        self.saved_config.clone()
    }

    pub fn new() -> Self {
        let saved_config = crate::audio::config::AppConfig::load();
        let ffmpeg = Arc::new(Mutex::new(FfmpegEngine::new()));
        let saved_config_arc = Some(std::sync::Arc::new(std::sync::Mutex::new(
            saved_config.clone(),
        )));

        let mut model = Self {
            ffmpeg: ffmpeg.clone(),
            audio: Arc::new(Mutex::new(AudioState::default())),
            output: AudioOutput::default(),
            volume: saved_config.volume as f64,
            current_index: -1,
            balance: saved_config.balance as f64,
            favorites_count: saved_config.favorites.len() as i32,
            queue: QueueController::new(),
            ..Default::default()
        };

        model.saved_config = saved_config_arc;
        model.output.set_normalizer_enabled(saved_config.normalizer_enabled);
        model.output.mode = saved_config.mode;

        if let Ok(mut ff) = model.ffmpeg.lock() {
            ff.set_volume(model.volume as f32);
        }

        model.playback = PlaybackController::new(model.ffmpeg.clone(), model.audio.clone());
        model.playback.volume = model.volume;
        
        model.library = crate::core::library::Library::new();
        model.library.load_folders(saved_config.custom_folders.clone());
        model.custom_folder_count = model.library.custom_folder_count as i32;
        model.custom_folders_changed();
        model.scan_music();

        model
    }

    // ==========================================
    // LIBRARY BRIDGES
    // ==========================================
    pub fn scan_music(&mut self) {
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return,
        };
        let music_dir = home.join("Music");
        self.current_folder_qml = QString::from("MUSIC");
        self.library.scan_music_folder(&music_dir);
        self.all_items = self.library.all_items.clone();
        
        // Add session folders to the display
        for session_path in &self.session_folders {
            let session_folder_path = Path::new(session_path);
            if let Some(name) = session_folder_path.file_name() {
                let session_item = MusicItem {
                    name: name.to_string_lossy().to_string(),
                    path: session_path.clone(),
                    is_folder: true,
                    parent_folder: None,
                };
                self.all_items.push(session_item);
            }
        }
        
        // Sort combined list
        self.all_items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        
        self.display_list = self.all_items.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn scan_folder(&mut self, path: String) {
        let folder_path = Path::new(&path);
        if !folder_path.exists() || !folder_path.is_dir() { return; }
        self.current_folder_qml = QString::default();
        self.library.scan_custom_directory(folder_path);
        self.all_items = self.library.all_items.clone();
        self.display_list = self.all_items.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn switch_to_folder(&mut self, folder_path: String) {
        // Clean slate: clear display list and set root
        self.display_list.clear();
        self.current_tab_root = QString::from(folder_path.clone());
        self.current_tab_root_changed();
        
        self.library.switch_to_folder(&folder_path);
        self.current_folder_qml = QString::from(self.library.current_folder.clone());
        self.all_items = self.library.all_items.clone();
        self.display_list = self.library.display_list.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn add_folder_tab(&mut self, path: String) {
        let clean = clean_qml_path(&path);
        self.library.add_folder(clean.clone());
        let new_index = self.library.custom_folder_count - 1;
        self.custom_folder_count = self.library.custom_folder_count as i32;
        self.custom_folders_changed();
        
        // Auto-lock new folder and save to config
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                if !cfg.locked_folders.contains(&(new_index as i32)) {
                    cfg.locked_folders.push(new_index as i32);
                    let _ = cfg.save();
                }
            }
        }
        self.folder_lock_version += 1;
        self.folder_lock_changed();
        
        self.save_custom_folders();
        self.switch_to_folder(clean);
    }

    pub fn get_custom_folder_name(&self, index: i32) -> QString {
        self.library.get_folder_name(index as usize).into()
    }

    pub fn get_custom_folder_path(&self, index: i32) -> QString {
        self.library.get_folder_path(index as usize).into()
    }

    pub fn get_current_rename_name(&self, index: i32) -> QString {
        self.library.get_folder_name(index as usize).into()
    }

    pub fn rename_folder(&mut self, index: i32, new_name: String) {
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            let mut trimmed = new_name.trim().to_string();
            trimmed.truncate(15);
            if !trimmed.is_empty() {
                self.library.custom_folders[index as usize].0 = trimmed;
                self.custom_folders_changed();
                self.save_custom_folders();
            }
        }
    }

    pub fn get_custom_folder_count(&self) -> i32 { 
        self.library.custom_folder_count as i32
    }

    pub fn remove_custom_folder(&mut self, index: i32) {
        self.library.remove_folder(index as usize);
        self.custom_folder_count = self.library.custom_folder_count as i32;
        self.custom_folders_changed();
        self.save_custom_folders();
    }

    pub fn change_folder(&mut self, index: i32, new_path: String) {
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            let folder_path = Path::new(&new_path);
            if let Some(name) = folder_path.file_name() {
                let mut name_str = name.to_string_lossy().to_string();
                name_str.truncate(15);
                self.library.custom_folders[index as usize] = (name_str, new_path.clone());
                self.custom_folders_changed();
                self.save_custom_folders();
                self.switch_to_folder(new_path);
            }
        }
    }

    pub fn toggle_folder(&mut self, index: i32) {
        let idx = index as usize;
        if idx >= self.display_list.len() { return; }
        
        // 1. Ambil item secara utuh
        let item = self.display_list[idx].clone();
        if !item.is_folder { return; }
        
        // 2. Gunakan PATH asli sebagai ID unik, bukan nama.
        let folder_path = item.path.clone();

        if self.expanded_folders.contains(&folder_path) {
            // COLLAPSE LOGIC
            self.expanded_folders.remove(&folder_path);
            self.display_list.retain(|i| i.parent_folder.as_ref() != Some(&folder_path));
        } else {
            // EXPAND LOGIC
            self.expanded_folders.insert(folder_path.clone());
            self.library.get_folder_contents(&folder_path);
            let contents = self.library.display_list.clone();
            
            if let Some(pos) = self.display_list.iter().position(|i| i.path == folder_path) {
                for (offset, mut sub_item) in contents.into_iter().enumerate() {
                    sub_item.parent_folder = Some(folder_path.clone());
                    self.display_list.insert(pos + 1 + offset, sub_item);
                }
            }
        }
        // 5. Paksa UI Gambar Ulang
        self.begin_reset_model();
        self.end_reset_model();
    }

    // ==========================================
    // FAVORITES BRIDGES
    // ==========================================
    pub fn add_favorite(&mut self, path: String, name: String) {
        self.library.add_favorite(path, name);
        self.favorites_count = self.library.favorites_count as i32;
        self.favorites_changed();
        self.save_favorites();
    }

    pub fn remove_favorite(&mut self, path: String) {
        self.library.remove_favorite(&path);
        self.favorites_count = self.library.favorites_count as i32;
        self.favorites_changed();
        self.save_favorites();
    }

    pub fn is_favorite(&self, path: String) -> bool {
        self.library.is_favorite(&path)
    }

    pub fn toggle_favorite(&mut self, path: String, name: String) {
        self.library.toggle_favorite(path, name);
        self.favorites_count = self.library.favorites_count as i32;
        self.favorites_changed();
        self.save_favorites();
    }

    pub fn switch_to_favorites(&mut self) {
        // Clean slate: clear display list and set root
        self.display_list.clear();
        self.current_tab_root = QString::from("FAVORITES");
        self.current_tab_root_changed();
        
        self.library.switch_to_favorites();
        self.current_folder_qml = QString::from("FAVORITES");
        self.all_items = self.library.all_items.clone();
        self.display_list = self.library.display_list.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn switch_to_music(&mut self) {
        // Clean slate: clear display list and set root
        self.display_list.clear();
        self.current_tab_root = QString::from("MUSIC");
        self.current_tab_root_changed();
        
        let home = match dirs::home_dir() {
            Some(path) => path,
            None => return,
        };
        let music_dir = home.join("Music");
        self.current_folder_qml = QString::from("MUSIC");
        self.library.scan_music_folder(&music_dir);
        self.all_items = self.library.all_items.clone();
        self.display_list = self.library.display_list.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    fn save_favorites(&mut self) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                self.library.save_config(&mut cfg);
                let _ = cfg.save();
            }
        }
    }

    // ==========================================
    // QUEUE BRIDGES
    // ==========================================
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
        // Clean slate: clear display list and set root
        self.display_list.clear();
        self.current_tab_root = QString::from("QUEUE");
        self.current_tab_root_changed();
        
        self.current_folder_qml = QString::from("QUEUE");
        self.all_items = self.queue.get_all();
        self.display_list = self.all_items.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    // ==========================================
    // PLAYBACK CONTROLLER BRIDGES
    // ==========================================
    pub fn play_at(&mut self, index: i32) {
        if index < 0 || index as usize >= self.display_list.len() { return; }
        let item = &self.display_list[index as usize];
        if item.is_folder { return; }

        self.playback_playlist = self.display_list.clone();
        self.playback_index = index;
        self.current_index = index;
        
        self.playback.play_at(item);
        self.position = self.playback.position;
        self.duration = self.playback.duration;
        self.current_title = QString::from(item.name.clone());

        self.current_index_changed();
        self.title_changed();
        self.is_playing = true;
        self.playing_changed();
        self.position_changed();
        self.duration_changed();
    }

    pub fn stop_playback(&mut self) {
        self.playback.stop();
        self.is_playing = false;
        self.playing_changed();
    }

    pub fn toggle_play(&mut self) {
        self.playback.toggle();
        self.is_playing = self.playback.is_playing();
        self.playing_changed();
    }

    pub fn play_next(&mut self) {
        if let Some(item) = self.queue.pop_front() {
            self.queue_count = self.queue.len() as i32;
            self.queue_changed();
            if let Some(index) = self.display_list.iter().position(|i| i.path == item.path) {
                self.play_at(index as i32);
                return;
            }
        }

        if let Some((next_idx, next_item)) = self.playback.play_next(&self.playback_playlist, self.playback_index) {
            self.playback_index = next_idx as i32;
            self.current_index = self.playback_index;
            self.current_title = QString::from(next_item.name.clone());

            self.playback.play_at(&next_item);
            self.position = self.playback.position;
            self.duration = self.playback.duration;

            self.current_index_changed();
            self.title_changed();
            self.is_playing = true;
            self.playing_changed();
            self.position_changed();
            self.duration_changed();
        } else {
            self.stop_playback();
        }
    }

    pub fn play_prev(&mut self) {
        if let Some((prev_idx, prev_item)) = self.playback.play_prev(&self.playback_playlist, self.playback_index) {
            self.playback_index = prev_idx as i32;
            self.current_index = self.playback_index;
            self.current_title = QString::from(prev_item.name.clone());

            self.playback.play_at(&prev_item);
            self.position = self.playback.position;
            self.duration = self.playback.duration;

            self.current_index_changed();
            self.title_changed();
            self.is_playing = true;
            self.playing_changed();
            self.position_changed();
            self.duration_changed();
        }
    }

    pub fn play_previous(&mut self) { self.play_prev(); }

    pub fn toggle_shuffle(&mut self) {
        self.playback.toggle_shuffle(&self.display_list, self.current_index);
        // Langsung tembak ke property QML (self.shuffle) ngambil dari playback
        self.shuffle = self.playback.shuffle_active;
        self.shuffle_changed();
    }

    pub fn toggle_repeat(&mut self) {
        self.playback.toggle_repeat();
        // Langsung tembak ke property QML (self.loop_playlist) ngambil dari playback
        self.loop_playlist = self.playback.loop_active;
        self.loop_changed();
    }

    pub fn seek_to(&mut self, pos: i32) {
        self.position = pos;
        if let Ok(mut ff) = self.ffmpeg.lock() { ff.seek(pos as f64 / 1000.0); }
        self.position_changed();
    }

    pub fn format_time(&self, ms: i32) -> QString {
        let s = (ms / 1000) % 60;
        let m = (ms / 60000) % 60;
        let h = ms / 3600000;
        if h > 0 { format!("{}:{:02}:{:02}", h, m, s).into() }
        else { format!("{:02}:{:02}", m, s).into() }
    }

    pub fn set_volume(&mut self, vol: f64) {
        self.volume = vol;
        self.playback.set_volume(vol);
        self.volume_changed();
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
        if let Ok(mut ff) = self.ffmpeg.lock() { ff.set_balance(balance as f32); }
        self.balance_changed();
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        self.playback.toggle_mute();
        self.mute_changed();
    }

    // ==========================================
    // OTHER UTILS (External, Sync, System)
    // ==========================================
    pub fn update_tick(&mut self) {
        self.tick_counter += 1;
        let should_next = if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.update_tick();
            let pos = (ff.get_position() * 1000.0) as i32;
            if pos != self.position { self.position = pos; self.position_changed(); }
            let dur = (ff.get_duration() * 1000.0) as i32;
            if dur != self.duration { self.duration = dur; self.duration_changed(); }
            ff.take_finished()
        } else { false };
        if should_next { self.play_next(); }
        if self.tick_counter % 100 == 0 { self.save_state(); }
    }

    pub fn start_update_loop(&mut self) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(cfg) = config.lock() {
                let path = cfg.last_track_path.clone();
                if let Some(pos) = self.display_list.iter().position(|i| i.path == path) {
                    self.current_index = pos as i32;
                    self.current_title = QString::from(self.display_list[pos].name.clone());
                    if let Ok(mut ff) = self.ffmpeg.lock() { ff.load(&path); }
                    self.current_index_changed();
                    self.title_changed();
                }
            }
        }
    }

    pub fn save_state(&mut self) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.volume = self.volume;
                cfg.balance = self.balance;
                if self.current_index >= 0 && (self.current_index as usize) < self.display_list.len() {
                    cfg.last_track_path = self.display_list[self.current_index as usize].path.clone();
                }
                let _ = cfg.save();
            }
        }
    }

    pub fn save_window_position(&mut self, x: i32, y: i32, width: i32, height: i32) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                cfg.window_x = x; cfg.window_y = y;
                cfg.window_width = width; cfg.window_height = height;
                let _ = cfg.save();
            }
        }
    }

    pub fn get_window_config(&self) -> QVariantMap {
        let mut map = QVariantMap::default();
        if let Some(ref config) = &self.saved_config {
            if let Ok(cfg) = config.lock() {
                map.insert("window_x".into(), cfg.window_x.into());
                map.insert("window_y".into(), cfg.window_y.into());
                map.insert("window_width".into(), cfg.window_width.into());
                map.insert("window_height".into(), cfg.window_height.into());
            }
        }
        map
    }

    pub fn load_track_info(&mut self, path: String) {
        let meta = crate::core::library::metadata::read_track_metadata(&path);
        self.track_info_title = meta.title.into();
        self.track_info_artist = meta.artist.into();
        self.track_info_album = meta.album.into();
        self.track_info_file_path = path.into();
        self.track_info_visible = true;
        self.track_info_visible_changed();
        self.track_info_changed();
    }

    pub fn close_track_info(&mut self) {
        self.track_info_visible = false;
        self.track_info_visible_changed();
    }

    pub fn add_song(&mut self, path: String) {
        let clean = clean_qml_path(&path);
        let song_path = Path::new(&clean);
        if let Some(name) = song_path.file_name() {
            self.all_items.push(MusicItem {
                name: name.to_string_lossy().to_string(),
                path: clean,
                is_folder: false,
                parent_folder: None,
            });
            self.all_items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });
            self.display_list = self.all_items.clone();
            self.begin_reset_model();
            self.end_reset_model();
        }
    }

    pub fn remove_song(&mut self, index: i32) {
        let idx = index as usize;
        if idx >= self.display_list.len() { return; }
        let item = self.display_list[idx].clone();
        if item.is_folder {
            let name = item.name.clone();
            self.display_list.retain(|i| i.parent_folder.as_ref() != Some(&name) && i.name != name);
            self.all_items.retain(|i| i.parent_folder.as_ref() != Some(&name) && i.name != name);
            self.expanded_folders.remove(&name);
        } else {
            self.all_items.retain(|i| i.path != item.path);
            self.display_list.retain(|i| i.path != item.path);
        }
        self.begin_reset_model();
        self.end_reset_model();
    }

    pub fn delete_item(&mut self, path: String, _is_folder: bool) {
        if let Some(pos) = self.display_list.iter().position(|i| i.path == path) {
            self.remove_song(pos as i32);
        }
        let _ = get_file_service().delete_file(&path);
    }

    pub fn add_temporary_folder(&mut self, path: String) {
        let clean = clean_qml_path(&path);
        
        // Add to session folders if not already exists
        if !self.session_folders.contains(&clean) {
            self.session_folders.push(clean.clone());
        }
        
        let folder_path = Path::new(&clean);
        if let Some(name) = folder_path.file_name() {
            self.all_items.push(MusicItem {
                name: name.to_string_lossy().to_string(),
                path: clean,
                is_folder: true,
                parent_folder: None,
            });
            self.all_items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });
            self.display_list = self.all_items.clone();
            self.begin_reset_model();
            self.end_reset_model();
        }
    }

    pub fn add_folder_to_list(&mut self, path: String) {
        let clean = clean_qml_path(&path);
        
        // Add to session folders if not already exists
        if !self.session_folders.contains(&clean) {
            self.session_folders.push(clean.clone());
        }
        
        let folder_path = Path::new(&clean);
        if let Some(name) = folder_path.file_name() {
            // Check if already exists in all_items
            if self.all_items.iter().any(|i| i.path == clean) {
                return;
            }
            
            self.all_items.push(MusicItem {
                name: name.to_string_lossy().to_string(),
                path: clean,
                is_folder: true,
                parent_folder: None,
            });
            self.all_items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });
            self.display_list = self.all_items.clone();
            self.begin_reset_model();
            self.end_reset_model();
        }
    }

    pub fn add_external_file(&mut self, path: String) {
        self.library.add_external_file(path);
        self.external_files_count = self.library.external_files_count as i32;
        self.external_files_changed();
        self.switch_to_external_files();
    }

    pub fn switch_to_external_files(&mut self) {
        // Clean slate: clear display list and set root
        self.display_list.clear();
        self.current_tab_root = QString::from("EXTERNAL_FILES");
        self.current_tab_root_changed();
        
        self.library.switch_to_external();
        self.current_folder_qml = QString::from("EXTERNAL_FILES");
        self.all_items = self.library.all_items.clone();
        self.display_list = self.library.display_list.clone();
        self.begin_reset_model();
        self.end_reset_model();
        self.current_folder_changed();
    }

    pub fn clear_external_files(&mut self) {
        self.library.clear_external_files();
        self.external_files_count = self.library.external_files_count as i32;
        self.external_files_changed();
        self.scan_music();
    }

    pub fn process_command_line_files(&mut self) {
        let files = get_command_line_files();
        for file in files { self.add_external_file(file.clone()); }
    }

    pub fn save_custom_folders(&mut self) {
        if let Some(ref config) = &self.saved_config {
            if let Ok(mut cfg) = config.lock() {
                self.library.save_config(&mut cfg);
                let _ = cfg.save();
            }
        }
    }

    pub fn is_folder_locked(&self, index: i32) -> bool {
        if let Some(ref config) = &self.saved_config {
            if let Ok(cfg) = config.lock() { return cfg.locked_folders.contains(&index); }
        }
        false
    }

    pub fn toggle_folder_lock(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.library.custom_folders.len() {
            if let Some(ref config) = &self.saved_config {
                if let Ok(mut cfg) = config.lock() {
                    if cfg.locked_folders.contains(&index) { cfg.locked_folders.retain(|&i| i != index); }
                    else { cfg.locked_folders.push(index); }
                    let _ = cfg.save();
                }
            }
            self.folder_lock_version += 1;
            self.folder_lock_changed();
        }
    }

    pub fn is_folder_expanded(&self, folder_name: QString) -> bool {
        self.expanded_folders.contains(&folder_name.to_string())
    }

    pub fn show_tab_context_menu(&mut self, _index: i32) {}
    pub fn check_for_updates(&mut self) {}
    pub fn poll_update_result(&mut self) {}
    pub fn select_device(&mut self, _device_name: String) {}
    pub fn get_output_devices(&self) -> QVariantList { QVariantList::default() }
    pub fn set_output_device(&mut self, _index: i32) {}
    pub fn sync_theme_to_config(&mut self, _theme_name: QString, _custom_themes_json: QString) {}
}