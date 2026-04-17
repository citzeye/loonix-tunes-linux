/* --- LOONIX-TUNES src/core/library.rs --- */

#![allow(non_snake_case)]

use crate::audio::engine::{is_audio_file, MusicItem};
use qmetaobject::prelude::*;
use qmetaobject::QString;
use qmetaobject::QVariant;
use qmetaobject::QVariantList;
use qmetaobject::QVariantMap;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub struct LibraryManager {
    pub folders: HashMap<String, Vec<MusicItem>>,
    pub all_items: Vec<MusicItem>,
    pub display_list: Vec<MusicItem>,
    pub expanded_folders: HashSet<String>,
    pub custom_folders: Vec<(String, String)>,
    pub favorites: Vec<(String, String)>,
    pub external_files: Vec<MusicItem>,

    pub current_folder: String,
    pub current_folder_path: String,
    pub custom_folder_count: i32,
    pub favorites_count: i32,
    pub external_files_count: i32,
}

impl Default for LibraryManager {
    fn default() -> Self {
        Self {
            folders: HashMap::new(),
            all_items: Vec::new(),
            display_list: Vec::new(),
            expanded_folders: HashSet::new(),
            custom_folders: Vec::new(),
            favorites: Vec::new(),
            external_files: Vec::new(),
            current_folder: String::new(),
            current_folder_path: String::new(),
            custom_folder_count: 0,
            favorites_count: 0,
            external_files_count: 0,
        }
    }
}

impl LibraryManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_folders(
        &mut self,
        custom_folders: Vec<(String, String)>,
        favorites: Vec<(String, String)>,
    ) {
        self.custom_folders = custom_folders;
        self.custom_folder_count = self.custom_folders.len() as i32;
        self.favorites = favorites;
        self.favorites_count = self.favorites.len() as i32;
    }
}

impl LibraryManager {
    pub fn scan_music_folder(&mut self, base_path: &std::path::Path) {
        self.all_items.clear();
        self.display_list.clear();

        if !base_path.is_dir() {
            return;
        }

        if let Ok(entries) = std::fs::read_dir(base_path) {
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

    pub fn scan_custom_directory(&mut self, dir: &Path) {
        self.all_items.clear();
        self.display_list.clear();

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

    pub fn add_folder(&mut self, path: String) {
        let path_ref = Path::new(&path);
        if let Some(name) = path_ref.file_name() {
            let name_str = name.to_string_lossy().to_string();
            let truncated = if name_str.len() > 15 {
                &name_str[..15]
            } else {
                &name_str
            };
            let trimmed = truncated.trim();
            self.custom_folders.push((trimmed.to_string(), path));
            self.custom_folder_count = self.custom_folders.len() as i32;
        }
    }

    pub fn remove_folder(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            self.custom_folders.remove(index as usize);
            self.custom_folder_count = self.custom_folders.len() as i32;
        }
    }

    pub fn get_folder_name(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            QString::from(self.custom_folders[index as usize].0.to_uppercase())
        } else {
            QString::default()
        }
    }

    pub fn get_folder_path(&self, index: i32) -> QString {
        if index >= 0 && (index as usize) < self.custom_folders.len() {
            QString::from(self.custom_folders[index as usize].1.clone())
        } else {
            QString::default()
        }
    }

    pub fn add_external_file(&mut self, path: String) {
        let name = Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        self.external_files.push(MusicItem {
            name,
            path,
            is_folder: false,
            parent_folder: None,
        });
        self.external_files_count = self.external_files.len() as i32;
    }

    pub fn clear_external_files(&mut self) {
        self.external_files.clear();
        self.external_files_count = 0;
    }

    pub fn is_favorite(&self, path: &str) -> bool {
        self.favorites.iter().any(|(_, p)| p == path)
    }

    pub fn add_favorite(&mut self, path: String, name: String) {
        if !self.is_favorite(&path) {
            self.favorites.push((name, path));
            self.favorites_count = self.favorites.len() as i32;
        }
    }

    pub fn remove_favorite(&mut self, path: &str) {
        self.favorites.retain(|(_, p)| p != path);
        self.favorites_count = self.favorites.len() as i32;
    }

    pub fn toggle_favorite(&mut self, path: String, name: String) {
        if self.is_favorite(&path) {
            self.remove_favorite(&path);
        } else {
            self.add_favorite(path, name);
        }
    }

    pub fn get_favorites_list(&self) -> Vec<MusicItem> {
        self.favorites
            .iter()
            .map(|(n, p)| MusicItem {
                name: n.clone(),
                path: p.clone(),
                is_folder: false,
                parent_folder: None,
            })
            .collect()
    }

    pub fn switch_to_folder(&mut self, folder_path: &str) {
        let path = Path::new(folder_path);
        self.current_folder_path = folder_path.to_string();

        if let Some(name) = path.file_name() {
            self.current_folder = name.to_string_lossy().to_string();
        }

        self.all_items.clear();
        self.display_list.clear();
        self.expanded_folders.clear();

        if path.is_dir() {
            self.scan_custom_directory(path);
        }
    }

    pub fn switch_to_favorites(&mut self) {
        self.current_folder = "FAVORITES".to_string();
        self.current_folder_path = String::new();

        self.all_items.clear();
        self.display_list = self.get_favorites_list();
    }

    pub fn switch_to_external(&mut self) {
        self.current_folder = "EXTERNAL".to_string();
        self.current_folder_path = String::new();

        self.all_items.clear();
        self.display_list = self.external_files.clone();
    }

    pub fn save_config(&self, config: &mut crate::audio::config::AppConfig) {
        config.custom_folders = self.custom_folders.clone();
        config.favorites = self.favorites.clone();
    }
}
