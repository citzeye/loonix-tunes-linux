/* --- LOONIX-TUNES src/audio/engine/library.rs --- */
use crate::audio::engine::{is_audio_file, MusicItem};
use std::path::Path;
use std::thread;

#[derive(Clone)]
pub struct Library {
    pub items: Vec<MusicItem>,
    pub folders: Vec<String>,
    pub current_folder: String,
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
        }
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
                        name,
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

        self.items
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
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

    items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
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
