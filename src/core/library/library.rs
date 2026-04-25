/* --- loonixtunesv2/src/core/library.rs | Library --- */

use crate::audio::engine::{is_audio_file, MusicItem};
use qmetaobject::QString;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Manages library data used by the UI.
#[derive(Default)]
pub struct LibraryManager {
    // Mapping from folder name to its items
    pub folders: HashMap<String, Vec<MusicItem>>, // cached folder contents
    pub all_items: Vec<MusicItem>,               // items for the current view
    pub display_list: Vec<MusicItem>,            // items presented to QML
    pub expanded_folders: HashSet<String>,
    pub custom_folders: Vec<(String, String)>, // (name, path)
    pub favorites: Vec<(String, String)>,
    pub external_files: Vec<MusicItem>,

    pub current_folder: String,
    pub current_folder_path: String,
    pub custom_folder_count: i32,
    pub favorites_count: i32,
    pub external_files_count: i32,

    pub library_metadata: HashMap<String, crate::core::library::metadata::TrackMetadata>,
}

/// Public alias used throughout the codebase.
pub type Library = LibraryManager;

impl LibraryManager {
    /// Create a new manager (default implementation).
    pub fn new() -> Self {
        Self::default()
    }

    /// Load saved custom folders and favorites from the config.
    pub fn load_folders(&mut self, custom_folders: Vec<(String, String)>, favorites: Vec<(String, String)>) {
        self.custom_folders = custom_folders;
        self.custom_folder_count = self.custom_folders.len() as i32;
        self.favorites = favorites;
        self.favorites_count = self.favorites.len() as i32;
    }

    /// Scan the user's Music directory and populate the library.
    pub fn scan_music_folder(&mut self, music_dir: &Path) {
        // Re‑use the engine library scanner for simplicity.
        let mut engine_lib = crate::audio::engine::Library::new();
        engine_lib.scan_music();
        self.all_items = engine_lib.items.clone();
        self.display_list = self.all_items.clone();
        self.current_folder.clear();
        self.current_folder_path.clear();
    }

    /// Scan an arbitrary directory selected by the user.
    pub fn scan_custom_directory(&mut self, dir: &Path) {
        let mut engine_lib = crate::audio::engine::Library::new();
        engine_lib.scan_custom_directory(dir);
        self.all_items = engine_lib.items.clone();
        self.display_list = self.all_items.clone();
        self.current_folder = dir.to_string_lossy().to_string();
        self.current_folder_path = self.current_folder.clone();
    }

    /// Switch view to a custom folder.
    pub fn switch_to_folder(&mut self, folder_path: &str) {
        // Find the folder name corresponding to the path.
        let name_opt = self
            .custom_folders
            .iter()
            .find(|(_, p)| p == folder_path)
            .map(|(n, _)| n.clone());
        if let Some(name) = name_opt {
            // Load folder contents lazily if not cached.
            if !self.folders.contains_key(&name) {
                let path = Path::new(folder_path);
                let mut items = Vec::new();
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if is_audio_file(&p) {
                            if let Some(fname) = p.file_name() {
                                items.push(MusicItem {
                                    name: fname.to_string_lossy().to_string(),
                                    path: p.to_string_lossy().to_string(),
                                    is_folder: false,
                                    parent_folder: Some(name.clone()),
                                });
                            }
                        }
                    }
                }
                items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                self.folders.insert(name.clone(), items);
            }
            self.all_items = self.folders.get(&name).cloned().unwrap_or_default();
            self.display_list = self.all_items.clone();
            self.current_folder = name;
            self.current_folder_path = folder_path.to_string();
        }
    }

    /// Add a new custom folder tab.
    pub fn add_folder(&mut self, path: String) {
        let name = Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        self.custom_folders.push((name.clone(), path.clone()));
        self.custom_folder_count = self.custom_folders.len() as i32;
        self.folders.remove(&name);
    }

    pub fn get_folder_name(&self, index: i32) -> QString {
        self.custom_folders
            .get(index as usize)
            .map(|(n, _)| QString::from(n.clone()))
            .unwrap_or_default()
    }

    pub fn get_folder_path(&self, index: i32) -> QString {
        self.custom_folders
            .get(index as usize)
            .map(|(_, p)| QString::from(p.clone()))
            .unwrap_or_default()
    }

    pub fn remove_folder(&mut self, index: i32) {
        if let Some((name, _)) = self.custom_folders.get(index as usize).cloned() {
            self.folders.remove(&name);
        }
        self.custom_folders.remove(index as usize);
        self.custom_folder_count = self.custom_folders.len() as i32;
    }

    /// Return items contained in the given folder (used for expand/collapse).
    pub fn get_folder_contents(&self, target_path: &Path) -> Vec<MusicItem> {
        let folder_name = target_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if let Some(cached) = self.folders.get(&folder_name) {
            return cached.clone();
        }
        // Fallback: read directory on the fly.
        let mut items = Vec::new();
        if let Ok(entries) = fs::read_dir(target_path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if is_audio_file(&p) {
                    if let Some(fname) = p.file_name() {
                        items.push(MusicItem {
                            name: fname.to_string_lossy().to_string(),
                            path: p.to_string_lossy().to_string(),
                            is_folder: false,
                            parent_folder: Some(folder_name.clone()),
                        });
                    }
                }
            }
        }
        items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        items
    }

    // ---------- Favorites ---------------------------------------------------
    pub fn add_favorite(&mut self, path: String, name: String) {
        self.favorites.push((name, path));
        self.favorites_count = self.favorites.len() as i32;
    }

    pub fn remove_favorite(&mut self, path: &str) {
        self.favorites.retain(|(_, p)| p != path);
        self.favorites_count = self.favorites.len() as i32;
    }

    pub fn is_favorite(&self, path: &str) -> bool {
        self.favorites.iter().any(|(_, p)| p == path)
    }

    pub fn toggle_favorite(&mut self, path: String, name: String) {
        if self.is_favorite(&path) {
            self.remove_favorite(&path);
        } else {
            self.add_favorite(path, name);
        }
    }

    pub fn switch_to_favorites(&mut self) {
        let items: Vec<MusicItem> = self
            .favorites
            .iter()
            .map(|(name, path)| MusicItem {
                name: name.clone(),
                path: path.clone(),
                is_folder: false,
                parent_folder: None,
            })
            .collect();
        self.all_items = items.clone();
        self.display_list = items;
        self.current_folder = "FAVORITES".into();
        self.current_folder_path.clear();
    }

    // ---------- Config persistence -----------------------------------------
    pub fn save_config(&self, cfg: &mut crate::audio::config::AppConfig) {
        cfg.custom_folders = self.custom_folders.clone();
        cfg.favorites = self.favorites.clone();
    }

    // ---------- External files --------------------------------------------
    pub fn add_external_file(&mut self, path: String) {
        if let Some(fname) = Path::new(&path).file_name() {
            self.external_files.push(MusicItem {
                name: fname.to_string_lossy().to_string(),
                path,
                is_folder: false,
                parent_folder: None,
            });
            self.external_files_count = self.external_files.len() as i32;
        }
    }

    pub fn switch_to_external(&mut self) {
        self.all_items = self.external_files.clone();
        self.display_list = self.all_items.clone();
        self.current_folder = "EXTERNAL_FILES".into();
        self.current_folder_path.clear();
    }

    pub fn clear_external_files(&mut self) {
        self.external_files.clear();
        self.external_files_count = 0;
    }
}
