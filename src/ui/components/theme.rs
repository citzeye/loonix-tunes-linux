/* --- loonixtunesv2/src/ui/components/theme.rs | theme --- */


use qmetaobject::prelude::*;
use qmetaobject::{QAbstractListModel, QByteArray, QModelIndex, QVariant, QVariantList, QVariantMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::audio::config::{AppConfig, ConfigError};

const ROLE_NAME: i32 = 257;
const ROLE_COLORS: i32 = 258;

#[derive(QObject, Default)]
pub struct CustomThemeListModel {
    base: qt_base_class!(trait QAbstractListModel),
    items: Vec<ThemeEntry>,
}

impl QAbstractListModel for CustomThemeListModel {
    fn row_count(&self) -> i32 {
        self.items.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let row = index.row() as usize;
        if row >= self.items.len() {
            return QVariant::default();
        }
        let item = &self.items[row];
        match role {
            ROLE_NAME => QString::from(item.name.clone()).into(),
            ROLE_COLORS => {
                let mut map = QVariantMap::default();
                if let Some(ref colors) = item.colors {
                    for (k, v) in colors {
                        map.insert(QString::from(k.as_str()), QVariant::from(QString::from(v.as_str())));
                    }
                }
                map.into()
            }
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();
        map.insert(ROLE_NAME, QByteArray::from("name"));
        map.insert(ROLE_COLORS, QByteArray::from("colors"));
        map
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeEntry {
    pub name: String,
    pub is_active: bool,
    #[serde(default)]
    pub colors: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeConfig {
    pub active_theme: String,
    pub themes: Vec<ThemeEntry>,
}

impl ThemeConfig {
    pub fn load() -> Self {
        match Self::load_theme_config() {
            Ok(cfg) => cfg,
            Err(_) => Self::default(),
        }
    }

    fn load_theme_config() -> Result<Self, ConfigError> {
        let path = Self::theme_path().ok_or(ConfigError::NotFound)?;
        let content = fs::read_to_string(&path)?;
        let config: ThemeConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::theme_path().ok_or(ConfigError::IoError("Invalid path".into()))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp_path = path.with_extension("tmp");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&temp_path, json)?;
        fs::rename(&temp_path, &path)?;
        Ok(())
    }

    fn theme_path() -> Option<PathBuf> {
        crate::audio::config::config_dir().map(|p| p.join("theme.json"))
    }

    pub fn user_template_colors() -> HashMap<String, String> {
    let mut map = HashMap::new();

        // --- Global & Backgrounds ---
        map.insert("bgmain".to_string(), "#121212".to_string());
        map.insert("bgoverlay".to_string(), "#1e1e1e".to_string());
        map.insert("graysolid".to_string(), "#333333".to_string());
        map.insert("contextmenubg".to_string(), "#181818".to_string());
        map.insert("overlay".to_string(), "#80000000".to_string());

        // --- Header Section ---
        map.insert("headerbg".to_string(), "#1e1e1e".to_string());
        map.insert("headericon".to_string(), "#6d6d6d".to_string());
        map.insert("headertext".to_string(), "#6d6d6d".to_string());
        map.insert("headerhover".to_string(), "#00ddff".to_string());

        // --- Player Section ---
        map.insert("playertitle".to_string(), "#00ffdd".to_string());
        map.insert("playersubtext".to_string(), "#6d6d6d".to_string());
        map.insert("playeraccent".to_string(), "#00ffdd".to_string());
        map.insert("playerhover".to_string(), "#843ff3".to_string());
        map.insert("playerslider".to_string(), "#00ffdd".to_string());
        map.insert("playerhandle".to_string(), "#843ff3".to_string());

        // --- Navigation Tabs ---
        map.insert("tabtext".to_string(), "#d1d8e6".to_string());
        map.insert("tabborder".to_string(), "#8a8a8a".to_string());
        map.insert("tabhover".to_string(), "#00ffdd".to_string());

        // --- Playlist Section ---
        map.insert("playlisttext".to_string(), "#d1d8e6".to_string());
        map.insert("playlistfolder".to_string(), "#f5a623".to_string());
        map.insert("playlistactive".to_string(), "#843ff3".to_string());
        map.insert("playlisticon".to_string(), "#00ffdd".to_string());

        // --- DSP Global ---
        map.insert("dspbg".to_string(), "#15151B".to_string());
        map.insert("dsptext".to_string(), "#6d6d6d".to_string());
        map.insert("dsptexthover".to_string(), "#fa7900".to_string());
        map.insert("dsptextactive".to_string(), "#fa7900".to_string());
        map.insert("dspborder".to_string(), "#6d6d6d".to_string());
        map.insert("dspgridbg".to_string(), "#111111".to_string());

        // --- DSP EQ Section ---
        map.insert("dspeqbg".to_string(), "#151515".to_string());
        map.insert("dspeqtext".to_string(), "#00ffdd".to_string());
        map.insert("dspeqsubtext".to_string(), "#6d6d6d".to_string());
        map.insert("dspeqicon".to_string(), "#9442ff".to_string());
        map.insert("dspeqhover".to_string(), "#843ff3".to_string());
        map.insert("dspeqpresettext".to_string(), "#6d6d6d".to_string());
        map.insert("dspeqpresetactive".to_string(), "#00ffdd".to_string());
        map.insert("dspeqslider".to_string(), "#ff1ae0".to_string());
        map.insert("dspeqsliderbg".to_string(), "#15151B".to_string());
        map.insert("dspeqhandle".to_string(), "#ff1ae0".to_string());

        // --- DSP Amp & Mix ---
        map.insert("dspeampbg".to_string(), "#111111".to_string());
        map.insert("dspampslider".to_string(), "#ff1ae0".to_string());
        map.insert("dspampsliderbg".to_string(), "#000000".to_string());
        map.insert("dspeamphandle".to_string(), "#9442ff".to_string());
        map.insert("dspeqfaderbg".to_string(), "#111111".to_string());
        map.insert("dspeqfaderslider".to_string(), "#ff1ae0".to_string());
        map.insert("dspeqfaderhandle".to_string(), "#9442ff".to_string());
        map.insert("dspeqmixslider".to_string(), "#00ffdd".to_string());
        map.insert("dspeqmixhandle".to_string(), "#843ff3".to_string());
        map.insert("dspeqmixbg".to_string(), "#15151B".to_string());

        // --- DSP FX Section ---
        map.insert("dspfxbg".to_string(), "#151515".to_string());
        map.insert("dspfxborder".to_string(), "#6d6d6d".to_string());
        map.insert("dspfxtext".to_string(), "#00ffdd".to_string());
        map.insert("dspfxsubtext".to_string(), "#6d6d6d".to_string());
        map.insert("dspfxicon".to_string(), "#9442ff".to_string());
        map.insert("dspfxhover".to_string(), "#843ff3".to_string());
        map.insert("dspfxactive".to_string(), "#00ffdd".to_string());
        map.insert("dspfxslider".to_string(), "#ff1ae0".to_string());
        map.insert("dspfxsliderbg".to_string(), "#111111".to_string());
        map.insert("dspfxhandle".to_string(), "#9442ff".to_string());

        // --- Generic DSP Controls ---
        map.insert("dspslider".to_string(), "#00ffdd".to_string());
        map.insert("dspsliderbg".to_string(), "#15151B".to_string());
        map.insert("dsphandle".to_string(), "#843ff3".to_string());
        map.insert("dsp10slider".to_string(), "#ff1ae0".to_string());
        map.insert("dsp10handle".to_string(), "#9442ff".to_string());
        map.insert("dsp10bg".to_string(), "#111111".to_string());
        map.insert("dspfaderslider".to_string(), "#ff1ae0".to_string());
        map.insert("dspfaderhandle".to_string(), "#9442ff".to_string());
        map.insert("dspfaderbg".to_string(), "#111111".to_string());
        map.insert("dspmixslider".to_string(), "#00ffdd".to_string());
        map.insert("dspmixhandle".to_string(), "#843ff3".to_string());
        map.insert("dspmixbg".to_string(), "#15151B".to_string());

        // --- DSP State UI ---
        map.insert("dspicon".to_string(), "#9442ff".to_string());
        map.insert("dsphover".to_string(), "#843ff3".to_string());
        map.insert("dspactive".to_string(), "#00ffdd".to_string());

        map
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let blue_colors = Self::user_template_colors();
        Self {
            active_theme: "Loonix".to_string(),
            themes: vec![
                ThemeEntry { name: "Loonix".to_string(), is_active: true, colors: None },
                ThemeEntry { name: "Blue".to_string(), is_active: false, colors: None },
                ThemeEntry { name: "Green".to_string(), is_active: false, colors: None },
                ThemeEntry { name: "Monochrome".to_string(), is_active: false, colors: None },
                ThemeEntry { name: "Orange".to_string(), is_active: false, colors: None },
                ThemeEntry { name: "Pink".to_string(), is_active: false, colors: None },
                ThemeEntry { name: "Red".to_string(), is_active: false, colors: None },
                ThemeEntry { name: "Yellow".to_string(), is_active: false, colors: None },
                ThemeEntry { name: "Custom 1".to_string(), is_active: false, colors: Some(blue_colors.clone()) },
                ThemeEntry { name: "Custom 2".to_string(), is_active: false, colors: Some(blue_colors.clone()) },
                ThemeEntry { name: "Custom 3".to_string(), is_active: false, colors: Some(blue_colors) },
            ],
        }
    }
}

macro_rules! c {
    ($map:expr, { $($key:expr, $val:expr),* $(,)? }) => {
        $( $map.insert($key.to_string(), $val.to_string()); )*
    };
}

#[derive(QObject, Default)]
pub struct ThemeManager {
    base: qt_base_class!(trait QObject),
    pub colormap: qt_property!(QVariantMap; NOTIFY colormap_changed),
    pub colormap_changed: qt_signal!(),
    pub current_theme: qt_property!(QString; NOTIFY current_theme_changed),
    pub current_theme_changed: qt_signal!(),
    pub get_custom_theme_count: qt_method!(fn(&self) -> i32),
    pub get_custom_theme_name: qt_method!(fn(&self, index: i32) -> QString),
    pub set_custom_theme_name: qt_method!(fn(&mut self, index: i32, name: String)),
    pub get_custom_theme_colors: qt_method!(fn(&self, index: i32) -> QVariantMap),
    pub set_custom_theme_colors: qt_method!(fn(&mut self, index: i32, colors: QVariantMap)),
    pub get_default_colors: qt_method!(fn(&self) -> QVariantMap),
    pub get_theme_list: qt_method!(fn(&self) -> QVariantList),
    pub get_custom_theme_list: qt_method!(fn(&self) -> QVariantList),
    pub set_theme: qt_method!(fn(&mut self, name: String)),
    pub cycle_theme: qt_method!(fn(&mut self)),
    pub get_editor_starter_colors:
        qt_method!(fn(&self, is_edit_mode: bool, index: i32) -> QVariantMap),
    pub get_color_template: qt_method!(fn(&self) -> QVariantMap),
    pub save_theme_editor: qt_method!(fn(&mut self, index: i32, name: String, colors: QVariantMap)),
    pub get_custom_themes: qt_method!(fn(&self) -> QVariantList),
    pub initialize_default_theme: qt_method!(fn(&mut self)),

    themes: Vec<ThemeEntry>,
    current_raw_colors: HashMap<String, String>,
    config: Option<Arc<Mutex<AppConfig>>>,

}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config(&mut self, config: Arc<Mutex<AppConfig>>) {
        let theme_config = ThemeConfig::load();

        self.themes = theme_config.themes;

        self.config = Some(config);

        // Find active theme
        let active_name = self.themes
            .iter()
            .find(|t| t.is_active)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "Loonix".to_string());

        self.set_theme(active_name);
    }

    pub fn set_loonix_manual(&mut self) {
        self.set_theme("Loonix".to_string());
    }

    pub fn set_wallpaper_path(&mut self, _path: String) {
        // No longer needed - wallpaper path handled in memory
    }

    pub fn get_custom_theme_count(&self) -> i32 {
        3
    }

    pub fn get_custom_theme_name(&self, index: i32) -> QString {
        if index >= 0 && index < self.themes.len() as i32 {
            QString::from(self.themes[index as usize].name.as_str())
        } else {
            QString::from("")
        }
    }

    pub fn set_custom_theme_name(&mut self, index: i32, name: String) {
        if index >= 0 && index < self.themes.len() as i32 {
            let old_name = self.themes[index as usize].name.clone();
            let is_current_theme = old_name == self.current_theme.to_string();

            self.themes[index as usize].name = name.clone();
            self.save_config();

            // Smart Apply: Refresh UI if renaming the active theme
            if is_current_theme {
                self.set_theme(name);
            }
        }
    }

    pub fn get_custom_theme_colors(&self, index: i32) -> QVariantMap {
        if index >= 0 && index < self.themes.len() as i32 {
            let colors = &self.themes[index as usize].colors;
            if let Some(ref c) = colors {
                if c.is_empty() {
                    return ThemeConfig::user_template_colors()
                        .iter()
                        .map(|(k, v)| {
                            (
                                QString::from(k.as_str()),
                                QVariant::from(QString::from(v.as_str())),
                            )
                        })
                        .collect();
                }
                c.iter().map(|(k, v)| {
                    (
                        QString::from(k.as_str()),
                        QVariant::from(QString::from(v.as_str())),
                    )
                }).collect()
            } else {
                QVariantMap::default()
            }
        } else {
            QVariantMap::default()
        }
    }

    pub fn set_custom_theme_colors(&mut self, index: i32, colors: QVariantMap) {
        let mut color_map = ThemeConfig::user_template_colors();
        for (k, v) in &colors {
            color_map.insert(k.to_string(), v.to_qstring().to_string());
        }

        let idx = index as usize;
        if idx < self.themes.len() {
            self.themes[idx].colors = Some(color_map);
            self.save_config();

            let theme_name = self.themes[idx].name.clone();
            self.set_theme(theme_name);
        }
    }

    pub fn get_color_template(&self) -> QVariantMap {
        ThemeConfig::user_template_colors()
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.as_str()),
                    QVariant::from(QString::from(v.as_str())),
                )
            })
            .collect()
    }

    pub fn save_theme_editor(&mut self, index: i32, name: String, colors: QVariantMap) {
        let mut color_map = ThemeConfig::user_template_colors();
        for (k, v) in &colors {
            color_map.insert(k.to_string(), v.to_qstring().to_string());
        }

        let idx = index as usize;
        if idx < self.themes.len() {
            self.themes[idx].name = name.clone();
            self.themes[idx].colors = Some(color_map);
            self.save_config();

            self.set_theme(name);
        }
    }

    pub fn get_default_colors(&self) -> QVariantMap {
        ThemeConfig::user_template_colors()
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.as_str()),
                    QVariant::from(QString::from(v.as_str())),
                )
            })
            .collect()
    }

    pub fn get_theme_list(&self) -> QVariantList {
        self.themes
            .iter()
            .map(|t| {
                let mut map = QVariantMap::default();
                map.insert(QString::from("name"), QVariant::from(QString::from(t.name.clone())));
                map.insert(QString::from("is_active"), QVariant::from(t.is_active));
                QVariant::from(map)
            })
            .collect()
    }

    pub fn get_custom_theme_entries(&self) -> Vec<ThemeEntry> {
        self.themes
            .iter()
            .filter(|t| t.colors.is_some())
            .cloned()
            .collect()
    }

    pub fn get_custom_theme_list(&self) -> QVariantList {
        self.themes
            .iter()
            .filter(|t| t.colors.is_some())
            .map(|t| {
                let mut map = QVariantMap::default();
                map.insert(QString::from("name"), QVariant::from(QString::from(t.name.clone())));
                map.insert(QString::from("is_active"), QVariant::from(t.is_active));
                QVariant::from(map)
            })
            .collect()
    }

    pub fn get_custom_themes(&self) -> QVariantList {
        self.themes
            .iter()
            .enumerate()
            .filter(|(_, t)| t.colors.is_some())
            .map(|(i, t)| {
                let mut map = QVariantMap::default();
                map.insert(QString::from("name"), QVariant::from(QString::from(t.name.clone())));
                map.insert(QString::from("is_active"), QVariant::from(t.is_active));
                map.insert(QString::from("original_index"), QVariant::from(i as i32));
                QVariant::from(map)
            })
            .collect()
    }

    pub fn initialize_default_theme(&mut self) {
        self.save_config();
    }

    pub fn get_editor_starter_colors(&self, is_edit_mode: bool, index: i32) -> QVariantMap {
        if is_edit_mode {
            if index >= 0 && index < self.themes.len() as i32 {
                let colors = &self.themes[index as usize].colors;
                if let Some(ref c) = colors {
                    if c.is_empty() {
                        return self.get_default_colors();
                    }
                    return c.iter().map(|(k, v)| {
                        (
                            QString::from(k.as_str()),
                            QVariant::from(QString::from(v.as_str())),
                        )
                    }).collect();
                }
            }
        }
        self.current_raw_colors
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.as_str()),
                    QVariant::from(QString::from(v.as_str())),
                )
            })
            .collect()
    }

    fn save_config(&self) {
        let theme_config = ThemeConfig {
            active_theme: self.current_theme.to_string(),
            themes: self.themes.clone(),
        };
        let _ = theme_config.save();
    }

    pub fn available_themes() -> Vec<String> {
        let mut themes = vec![
            "Blue".into(),
            "Green".into(),
            "Monochrome".into(),
            "Orange".into(),
            "Pink".into(),
            "Red".into(),
            "Yellow".into(),
        ];
        themes.sort();
        themes.insert(0, "Loonix".into());
        themes
    }

    pub fn cycle_theme(&mut self) {
        let themes = Self::available_themes();
        let current = self.current_theme.to_string();
        if let Some(idx) = themes.iter().position(|t| t == &current) {
            let next_idx = (idx + 1) % themes.len();
            self.set_theme(themes[next_idx].clone());
        } else {
            self.set_theme("Loonix".to_string());
        }
    }

    pub fn set_theme(&mut self, name: String) {
        // Get colors based on theme
        let base_colors = ThemeConfig::user_template_colors();
        let colors = if let Some(entry) = self.themes.iter().find(|t| t.name == name) {
            // Check if this is a custom theme (has colors) or built-in (no colors)
            if let Some(ref c) = entry.colors {
                if !c.is_empty() {
                    c.clone()
                } else {
                    // Empty custom theme - use blue
                    base_colors.clone()
                }
            } else {
                // Built-in theme - get from built-in list and merge with defaults
                let mut merged = base_colors.clone();
                merged.extend(Self::get_builtin_colors(&name));
                merged
            }
        } else {
            // Not in themes array - get from built-in list
            let mut merged = base_colors.clone();
            merged.extend(Self::get_builtin_colors(&name));
            merged
        };

        // Update is_active status
        for theme in &mut self.themes {
            theme.is_active = theme.name == name;
        }

        let qmap: QVariantMap = colors
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.as_str()),
                    QVariant::from(QString::from(v.as_str())),
                )
            })
            .collect();

        self.colormap = qmap;
        self.current_theme = QString::from(name);
        self.current_raw_colors = colors;
        self.colormap_changed();
        self.current_theme_changed();
        self.save_config();
    }

fn get_builtin_colors(name: &str) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();

        match name {
            "Loonix" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#15151B",
                    "bgoverlay", "#201f2b",
                    "graysolid", "#6d6d6d",
                    "contextmenubg", "#2d2d2d",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#201f2b",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff1ae0",

                    // --- Player Section ---
                    "playertitle", "#00ffa2",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#9442ff",
                    "playerhover", "#ff1ae0",
                    "playerslider", "#ff1ae0",
                    "playerhandle", "#9442ff",

                    // --- Navigation Tabs ---
                    "tabtext", "#c6c6c6",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff1ae0",

                    // --- Playlist Section ---
                    "playlisttext", "#c6c6c6",
                    "playlistfolder", "#fa7900",
                    "playlistactive", "#00ffa2",
                    "playlisticon", "#fa7900",

                    // --- DSP Master Section ---
                    "dspbg", "#15151B",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#ff1ae0",
                    "dsptextactive", "#00ffa2",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",

                    // --- DSP EQ Section ---
                    "dspeqbg", "#151515",
                    "dspeqicon", "#9442ff",
                    "dspeqslider", "#ff1ae0",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#9442ff",

                    // --- DSP Amp & Fader ---
                    "dspeampbg", "#111111",
                    "dspampslider", "#ff1ae0",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#9442ff",
                    "dspeqfaderbg", "#111111",
                    "dspeqfaderslider", "#ff1ae0",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#9442ff",

                    // --- DSP FX Section ---
                    "dspfxbg", "#151515",
                    "dspfxicon", "#9442ff",
                    "dspfxslider", "#ff1ae0",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#9442ff",
                });
            }

            "Blue" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ddff",

                    // --- Player Section ---
                    "playertitle", "#00ffdd",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ffdd",
                    "playerhover", "#843ff3",
                    "playerslider", "#9442ff",
                    "playerhandle", "#00ffa2",

                    // --- Navigation Tabs ---
                    "tabtext", "#d1d8e6",
                    "tabborder", "#8a8a8a",
                    "tabhover", "#00ffdd",

                    // --- Playlist Section ---
                    "playlisttext", "#d1d8e6",
                    "playlistfolder", "#f5a623",
                    "playlistactive", "#843ff3",
                    "playlisticon", "#843ff3",

                    // --- DSP Section ---
                    "dspbg", "#15151B",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#fa7900",
                    "dsptextactive", "#fa7900",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#151515",
                    "dspeqicon", "#9442ff",
                    "dspeqslider", "#00ffdd",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#9442ff",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ff1ae0",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#9442ff",
                    "dspeqfaderbg", "#111111",
                    "dspeqfaderslider", "#ff1ae0",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#9442ff",
                    "dspfxbg", "#151515",
                    "dspfxicon", "#9442ff",
                    "dspfxslider", "#00ffdd",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#9442ff",
                });
            }

            "Green" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ff26",

                    // --- Player Section ---
                    "playertitle", "#00ff26",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ff26",
                    "playerhover", "#ffcc00",
                    "playerslider", "#00ffa2",
                    "playerhandle", "#9442ff",

                    // --- Navigation Tabs ---
                    "tabtext", "#d1e6d8",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#00ff26",

                    // --- Playlist Section ---
                    "playlisttext", "#d1e6d8",
                    "playlistfolder", "#00ff26",
                    "playlistactive", "#ffcc00",
                    "playlisticon", "#00ff26",

                    // --- DSP Section ---
                    "dspbg", "#1e1e1e",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#00ff26",
                    "dsptextactive", "#00ff26",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#121c15",
                    "dspeqicon", "#00ff26",
                    "dspeqslider", "#00ff66",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#ffcc00",
                    "dspeampbg", "#111111",
                    "dspampslider", "#00ff66",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#00ff26",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#8b0000",
                    "dspfxbg", "#121c15",
                    "dspfxicon", "#00ff26",
                    "dspfxslider", "#00ff66",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#00ff26",
                });
            }

            "Monochrome" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffffff",

                    // --- Player Section ---
                    "playertitle", "#ffffff",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#555555",
                    "playerhover", "#ffffff",
                    "playerslider", "#ffffff",
                    "playerhandle", "#555555",

                    // --- Navigation Tabs ---
                    "tabtext", "#e0e0e0",
                    "tabborder", "#ffffff",
                    "tabhover", "#ffffff",

                    // --- Playlist Section ---
                    "playlisttext", "#e0e0e0",
                    "playlistfolder", "#d4d4d4",
                    "playlistactive", "#ffffff",
                    "playlisticon", "#d4d4d4",

                    // --- DSP Section ---
                    "dspbg", "#1e1e1e",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#ffffff",
                    "dsptextactive", "#ffffff",
                    "dspborder", "#ffffff",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#121212",
                    "dspeqicon", "#d4d4d4",
                    "dspeqslider", "#ffffff",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#ffffff",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ffffff",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#555555",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#8b0000",
                    "dspfxbg", "#121212",
                    "dspfxicon", "#d4d4d4",
                    "dspfxslider", "#ffffff",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#d4d4d4",
                });
            }

            "Orange" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffea00",

                    // --- Player Section ---
                    "playertitle", "#ff5500",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ff5500",
                    "playerhover", "#ffea00",
                    "playerslider", "#ff5500",
                    "playerhandle", "#ffea00",

                    // --- Navigation Tabs ---
                    "tabtext", "#ecdcd9",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff5500",

                    // --- Playlist Section ---
                    "playlisttext", "#ecdcd9",
                    "playlistfolder", "#ffea00",
                    "playlistactive", "#ff5500",
                    "playlisticon", "#ff5500",

                    // --- DSP Section ---
                    "dspbg", "#1e1e1e",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#ffea00",
                    "dsptextactive", "#ff5500",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#1c1210",
                    "dspeqicon", "#ff5500",
                    "dspeqslider", "#ff5500",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#ffea00",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ff5500",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#ffea00",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#8b0000",
                    "dspfxbg", "#1c1210",
                    "dspfxicon", "#ff5500",
                    "dspfxslider", "#ff5500",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#ff5500",
                });
            }

            "Pink" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ffcc",

                    // --- Player Section ---
                    "playertitle", "#f965d9",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#f965d9",
                    "playerhover", "#00ffcc",
                    "playerslider", "#f965d9",
                    "playerhandle", "#00ffcc",

                    // --- Navigation Tabs ---
                    "tabtext", "#eedef2",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#f965d9",

                    // --- Playlist Section ---
                    "playlisttext", "#eedef2",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#65f996",
                    "playlisticon", "#f965d9",

                    // --- DSP Section ---
                    "dspbg", "#1e1e1e",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#00ffcc",
                    "dsptextactive", "#f965d9",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#1b101f",
                    "dspeqicon", "#f965d9",
                    "dspeqslider", "#f965d9",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#00ffcc",
                    "dspeampbg", "#111111",
                    "dspampslider", "#f965d9",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#00ffcc",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#8b0000",
                    "dspfxbg", "#1b101f",
                    "dspfxicon", "#f965d9",
                    "dspfxslider", "#f965d9",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#00ffcc",
                });
            }

            "Red" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff003c",

                    // --- Player Section ---
                    "playertitle", "#ff003c",
                    "playersubtext", "#bdbdbd",
                    "playeraccent", "#ff003c",
                    "playerhover", "#2b00ff",
                    "playerslider", "#ff003c",
                    "playerhandle", "#2b00ff",

                    // --- Navigation Tabs ---
                    "tabtext", "#bdbdbd",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff003c",

                    // --- Playlist Section ---
                    "playlisttext", "#bdbdbd",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ff003c",
                    "playlisticon", "#2b00ff",

                    // --- DSP Section ---
                    "dspbg", "#1e1e1e",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#ff003c",
                    "dsptextactive", "#ff003c",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#1c0d0d",
                    "dspeqicon", "#ff003c",
                    "dspeqslider", "#ff003c",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#2b00ff",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ff003c",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#2b00ff",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#8b0000",
                    "dspfxbg", "#1c0d0d",
                    "dspfxicon", "#ff003c",
                    "dspfxslider", "#ff003c",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#2b00ff",
                });
            }

            "Yellow" => {
                c!(map, {
                    // --- Global & Backgrounds ---
                    "bgmain", "#0d1012",
                    "bgoverlay", "#15191c",
                    "graysolid", "#2d353b",
                    "contextmenubg", "#0a0c0e",
                    "overlay", "#80000000",

                    // --- Header Section ---
                    "headerbg", "#15191c",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#f965d9",

                    // --- Player Section ---
                    "playertitle", "#ffea00",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ffea00",
                    "playerhover", "#f965d9",
                    "playerslider", "#ffea00",
                    "playerhandle", "#f965d9",

                    // --- Navigation Tabs ---
                    "tabtext", "#dde0d1",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ffea00",

                    // --- Playlist Section ---
                    "playlisttext", "#dde0d1",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ffea00",
                    "playlisticon", "#f965d9",

                    // --- DSP Section ---
                    "dspbg", "#15191c",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#ffea00",
                    "dsptextactive", "#ffea00",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#0d1012",
                    "dspeqicon", "#ffea00",
                    "dspeqslider", "#ffea00",
                    "dspeqsliderbg", "#000000",
                    "dspeqhandle", "#f965d9",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ffea00",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#f965d9",
                    "dspeqfaderbg", "#15191c",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfadersliderbg", "#000000",
                    "dspeqfaderhandle", "#8b0000",
                    "dspfxbg", "#0d1012",
                    "dspfxicon", "#ffea00",
                    "dspfxslider", "#ffea00",
                    "dspfxsliderbg", "#000000",
                    "dspfxhandle", "#f965d9",
                });
            }

            _ => { 
                map = Self::get_builtin_colors("Loonix");
            }
        }

        map
    }
}


