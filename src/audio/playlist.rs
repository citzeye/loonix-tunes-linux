/* --- LOONIX-TUNES src/audio/playlist.rs --- */
use crate::ui::core::MusicModel;
use qmetaobject::*;
use std::path::Path;

impl MusicModel {
    pub fn get_music_folders(&self) -> Vec<(String, String)> {
        let mut folders = vec![];
        if let Some(home) = dirs::home_dir() {
            let music_dir = home.join("Music");
            if let Ok(rd) = std::fs::read_dir(music_dir) {
                for e in rd.flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        if let Some(name) = p.file_name() {
                            let name_str = name.to_string_lossy().to_string();
                            folders.push((name_str, p.to_string_lossy().to_string()));
                        }
                    }
                }
            }
        }
        folders
    }

    pub fn get_folder_count(&self) -> i32 {
        self.get_music_folders().len() as i32
    }

    pub fn get_folder_name(&self, index: i32) -> QString {
        let folders = self.get_music_folders();
        if index >= 0 && (index as usize) < folders.len() {
            QString::from(folders[index as usize].0.clone())
        } else {
            QString::default()
        }
    }

    pub fn get_folder_path(&self, index: i32) -> QString {
        let folders = self.get_music_folders();
        if index >= 0 && (index as usize) < folders.len() {
            QString::from(folders[index as usize].1.clone())
        } else {
            QString::default()
        }
    }

    pub fn add_custom_folder(&mut self, path: String) {
        let p = Path::new(&path);
        if let Some(name) = p.file_name() {
            let _folder_name = name.to_string_lossy().to_string();
            // Logika scanning file di dalam folder baru bisa ditaruh di sini
            self.rebuild_display_list();
        }
    }

    pub fn rebuild_display_list(&mut self) {
        // Simple implementation: reset display list to all items
        self.display_list = self.all_items.clone();

        // Reset model to notify QML
        self.begin_reset_model();
        self.end_reset_model();
    }
}
/* --- END OF SECTION --- */
