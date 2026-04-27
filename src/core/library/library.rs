/* --- loonixtunesv2/src/audio/engine/library.rs | library --- */
use crate::audio::config::AppConfig;
use crate::audio::engine::{is_audio_file, MusicItem};
use std::path::Path;
use std::thread;

#[derive(Clone)]
pub struct Library {
    pub items: Vec<MusicItem>,
    pub folders: Vec<String>,
    pub current_folder: String,
    pub all_items: Vec<MusicItem>,
    pub display_list: Vec<MusicItem>,
    pub custom_folders: Vec<(String, String)>,
    pub custom_folder_count: usize,
    pub favorites: Vec<MusicItem>,
    pub favorites_count: usize,
    pub external_files: Vec<MusicItem>,
    pub external_files_count: usize,
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

impl Library {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            folders: Vec::new(),
            current_folder: String::new(),
            all_items: Vec::new(),
            display_list: Vec::new(),
            custom_folders: Vec::new(),
            custom_folder_count: 0,
            favorites: Vec::new(),
            favorites_count: 0,
            external_files: Vec::new(),
            external_files_count: 0,
        }
    }

    pub fn load_folders(&mut self, folders: Vec<(String, String)>) {
        self.custom_folders = folders;
        self.custom_folder_count = self.custom_folders.len();
    }

    pub fn add_folder(&mut self, path: String) {
        let name = Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        self.custom_folders.push((name, path));
        self.custom_folder_count = self.custom_folders.len();
    }

    pub fn remove_folder(&mut self, index: usize) {
        if index < self.custom_folders.len() {
            self.custom_folders.remove(index);
            self.custom_folder_count = self.custom_folders.len();
        }
    }

    pub fn get_folder_name(&self, index: usize) -> String {
        self.custom_folders.get(index).map(|f| f.0.clone()).unwrap_or_default()
    }

    pub fn get_folder_path(&self, index: usize) -> String {
        self.custom_folders.get(index).map(|f| f.1.clone()).unwrap_or_default()
    }

    pub fn scan_music_folder(&mut self, dir: &Path) {
        self.all_items.clear();
        self.display_list.clear();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                if path.is_dir() {
                    self.all_items.push(MusicItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        is_folder: true,
                        parent_folder: None,
                    });
                } else if is_audio_file(&path) {
                    self.all_items.push(MusicItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        is_folder: false,
                        parent_folder: None,
                    });
                }
            }
        }
        
        self.all_items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        
        self.display_list = self.all_items.clone();
    }

    pub fn switch_to_folder(&mut self, folder_path: &str) {
        self.current_folder = folder_path.to_string();
        self.scan_music_folder(Path::new(folder_path));
    }

    pub fn get_folder_contents(&mut self, target_path: &str) {
        self.switch_to_folder(target_path);
    }

    pub fn add_favorite(&mut self, path: String, name: String) {
        if !self.is_favorite(&path) {
            self.favorites.push(MusicItem {
                name,
                path,
                is_folder: false,
                parent_folder: None,
            });
            self.favorites_count = self.favorites.len();
        }
    }

    pub fn remove_favorite(&mut self, path: &str) {
        self.favorites.retain(|item| item.path != path);
        self.favorites_count = self.favorites.len();
    }

    pub fn is_favorite(&self, path: &str) -> bool {
        self.favorites.iter().any(|item| item.path == path)
    }

    pub fn toggle_favorite(&mut self, path: String, name: String) {
        if self.is_favorite(&path) {
            self.remove_favorite(&path);
        } else {
            self.add_favorite(path, name);
        }
    }

    pub fn switch_to_favorites(&mut self) {
        self.display_list = self.favorites.clone();
        self.current_folder = "Favorites".to_string();
    }

    pub fn add_external_file(&mut self, path: String) {
        let name = Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());

        if !self.external_files.iter().any(|i| i.path == path) {
            self.external_files.push(MusicItem {
                name,
                path,
                is_folder: false,
                parent_folder: None,
            });
            self.external_files_count = self.external_files.len();
        }
    }

    pub fn switch_to_external(&mut self) {
        self.display_list = self.external_files.clone();
        self.current_folder = "External Files".to_string();
    }

    pub fn clear_external_files(&mut self) {
        self.external_files.clear();
        self.external_files_count = 0;
    }

    pub fn save_config(&self, cfg: &mut AppConfig) {
        cfg.custom_folders = self.custom_folders.clone();
    }

    pub fn scan_music_async<F>(callback: F)
    where
        F: FnOnce(Vec<MusicItem>) + Send + 'static,
    {
        thread::spawn(move || {
            let music_dir = get_music_directory();

            let mut items = Vec::new();
            scan_directory_sync(&music_dir, &mut items);

            items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });

            callback(items);
        });
    }

    pub fn scan_music(&mut self) {
        let music_dir = get_music_directory();

        self.current_folder = String::new();
        self.items.clear();
        self.folders.clear();

        self.scan_directory(&music_dir);

        self.items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
    }

    pub fn scan_directory(&mut self, dir: &Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                if path.is_dir() {
                    self.items.push(MusicItem {
                        name: name.clone(),
                        path: path.to_string_lossy().to_string(),
                        is_folder: true,
                        parent_folder: None,
                    });
                } else if is_audio_file(&path) {
                    self.items.push(MusicItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        is_folder: false,
                        parent_folder: None,
                    });
                }
            }
        }
    }

    pub fn scan_custom_directory(&mut self, dir: &Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                if is_audio_file(&path) {
                    self.items.push(MusicItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        is_folder: false,
                        parent_folder: None,
                    });
                }
            }
        }

        self.items
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

    pub fn get_sorted_items(&self) -> Vec<MusicItem> {
        let mut items = self.items.clone();
        items.sort_by(|a, b| match (a.is_folder, b.is_folder) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        items
    }
}

fn scan_directory_sync(dir: &Path, items: &mut Vec<MusicItem>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if path.is_dir() {
                items.push(MusicItem {
                    name,
                    path: path.to_string_lossy().to_string(),
                    is_folder: true,
                    parent_folder: None,
                });
            } else if is_audio_file(&path) {
                items.push(MusicItem {
                    name,
                    path: path.to_string_lossy().to_string(),
                    is_folder: false,
                    parent_folder: None,
                });
            }
        }
    }
}

fn get_music_directory() -> std::path::PathBuf {
    if let Some(audio_dir) = dirs::audio_dir() {
        return audio_dir;
    }
    if let Some(home) = dirs::home_dir() {
        return home.join("Music");
    }
    std::path::PathBuf::from(".")
}