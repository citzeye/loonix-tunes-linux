/* --- LOONIX-TUNES src/ui/theme.rs --- */
use qmetaobject::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::audio::config::{AppConfig, ConfigError};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CustomTheme {
    pub name: String,
    pub colors: HashMap<String, String>,
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
        map.insert("bgmain".to_string(), "#121212".to_string());
        map.insert("bgoverlay".to_string(), "#1e1e1e".to_string());
        map.insert("graysolid".to_string(), "#333333".to_string());
        map.insert("contextmenubg".to_string(), "#181818".to_string());
        map.insert("overlay".to_string(), "#80000000".to_string());
        map.insert("headerbg".to_string(), "#1e1e1e".to_string());
        map.insert("headericon".to_string(), "#6d6d6d".to_string());
        map.insert("headertext".to_string(), "#6d6d6d".to_string());
        map.insert("headerhover".to_string(), "#00ddff".to_string());
        map.insert("playertitle".to_string(), "#00ffdd".to_string());
        map.insert("playersubtext".to_string(), "#6d6d6d".to_string());
        map.insert("playeraccent".to_string(), "#00ffdd".to_string());
        map.insert("playerhover".to_string(), "#843ff3".to_string());
        map.insert("tabtext".to_string(), "#d1d8e6".to_string());
        map.insert("tabborder".to_string(), "#8a8a8a".to_string());
        map.insert("tabhover".to_string(), "#00ffdd".to_string());
        map.insert("playlisttext".to_string(), "#d1d8e6".to_string());
        map.insert("playlistfolder".to_string(), "#f5a623".to_string());
        map.insert("playlistactive".to_string(), "#843ff3".to_string());
        map.insert("playlisticon".to_string(), "#00ffdd".to_string());
        map.insert("dspbg".to_string(), "#15151B".to_string());
        map.insert("dsptext".to_string(), "#6d6d6d".to_string());
        map.insert("dsptexthover".to_string(), "#fa7900".to_string());
        map.insert("dsptextactive".to_string(), "#fa7900".to_string());
        map.insert("dspborder".to_string(), "#6d6d6d".to_string());
        map.insert("dspgridbg".to_string(), "#111111".to_string());
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
        map.insert("dspeampbg".to_string(), "#111111".to_string());
        map.insert("dspampslider".to_string(), "#ff1ae0".to_string());
        map.insert("dspampsliderbg".to_string(), "#000000".to_string());
        map.insert("dspeamphandle".to_string(), "#9442ff".to_string());
        map.insert("dspeq10slider".to_string(), "#ff1ae0".to_string());
        map.insert("dspeq10sliderbg".to_string(), "#000000".to_string());
        map.insert("dspeq10handle".to_string(), "#9442ff".to_string());
        map.insert("dspeqfaderbg".to_string(), "#111111".to_string());
        map.insert("dspeqfaderslider".to_string(), "#ff1ae0".to_string());
        map.insert("dspeqfaderhandle".to_string(), "#9442ff".to_string());
        map.insert("dspeqmixslider".to_string(), "#00ffdd".to_string());
        map.insert("dspeqmixhandle".to_string(), "#843ff3".to_string());
        map.insert("dspeqmixbg".to_string(), "#15151B".to_string());
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
    pub themes_changed: qt_signal!(),
    pub get_custom_theme_count: qt_method!(fn(&self) -> i32),
    pub get_custom_theme_name: qt_method!(fn(&self, index: i32) -> QString),
    pub set_custom_theme_name: qt_method!(fn(&mut self, index: i32, name: String)),
    pub get_custom_theme_colors: qt_method!(fn(&self, index: i32) -> QVariantMap),
    pub set_custom_theme_colors: qt_method!(fn(&mut self, index: i32, colors: QVariantMap)),
    pub get_default_colors: qt_method!(fn(&self) -> QVariantMap),
    pub get_theme_list: qt_method!(fn(&self) -> QVariantList),
    pub get_custom_themes: qt_method!(fn(&self) -> QVariantList),
    pub set_theme: qt_method!(fn(&mut self, name: String)),
    pub cycle_theme: qt_method!(fn(&mut self)),
    pub get_editor_starter_colors:
        qt_method!(fn(&self, is_edit_mode: bool, index: i32) -> QVariantMap),
    pub get_color_template: qt_method!(fn(&self) -> QVariantMap),
    pub save_theme_editor: qt_method!(fn(&mut self, index: i32, name: String, colors: QVariantMap)),
    pub sync_with_wallpaper: qt_method!(fn(&mut self)),
    pub set_loonix_manual: qt_method!(fn(&mut self)),
    pub set_wallpaper_path: qt_method!(fn(&mut self, path: String)),
    pub is_matugen_available: qt_method!(fn(&self) -> bool),
    pub get_system_report: qt_method!(fn(&self) -> QVariantMap),
    pub report_bug_on_github: qt_method!(fn(&self, bug_title: QString, bug_desc: QString)),
    pub wallpaper_sync_status: qt_signal!(success: bool, message: QString),

    themes: Vec<ThemeEntry>,
    current_raw_colors: HashMap<String, String>,
    config: Option<Arc<Mutex<AppConfig>>>,
    matugen_available: bool,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

pub fn set_config(&mut self, config: Arc<Mutex<AppConfig>>) {
        let theme_config = ThemeConfig::load();

        self.themes = theme_config.themes;

        self.config = Some(config);
        self.check_matugen();

        // Find active theme
        let active_name = self.themes
            .iter()
            .find(|t| t.is_active)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "Loonix".to_string());

        self.set_theme(active_name);
    }

    fn check_matugen(&mut self) {
        if let Some(path) = Self::get_matugen_path() {
            let output = Command::new(&path).arg("--version").output();
            self.matugen_available = output.map(|o| o.status.success()).unwrap_or(false);
        } else {
            self.matugen_available = false;
        }
    }

    pub fn is_matugen_available(&self) -> bool {
        self.matugen_available
    }

    fn get_matugen_path() -> Option<String> {
        if let Ok(path) = which::which("matugen") {
            return Some(path.to_string_lossy().into_owned());
        }

        if let Ok(home) = std::env::var("HOME") {
            let cargo_path = format!("{}/.cargo/bin/matugen", home);
            if std::path::Path::new(&cargo_path).exists() {
                return Some(cargo_path);
            }
        }

        None
    }

    fn detect_desktop_environment() -> Option<&'static str> {
        if Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.background", "picture-uri"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some("GNOME");
        }
        if Command::new("hyprctl")
            .arg("version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some("Hyprland");
        }
        if Command::new("swww")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some("swww");
        }
        if std::path::Path::new(&format!(
            "{}/.fehbg",
            std::env::var("HOME").unwrap_or_default()
        ))
        .exists()
        {
            return Some("feh");
        }
        None
    }

    pub fn get_system_report(&self) -> QVariantMap {
        let mut report = std::collections::HashMap::new();

        let distro = std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("PRETTY_NAME="))
                    .map(|l| l.replace("PRETTY_NAME=", "").replace("\"", ""))
            })
            .unwrap_or_else(|| "Unknown Distro".to_string());
        report.insert("distro".to_string(), distro);

        let de = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());
        report.insert("de".to_string(), de.clone());

        let de_supported = Self::detect_desktop_environment().is_some();
        report.insert("de_supported".to_string(), de_supported.to_string());

        let matugen_path = Self::get_matugen_path();
        let has_matugen = matugen_path.is_some();
        report.insert("has_matugen".to_string(), has_matugen.to_string());

        let wall_result = Self::get_active_wallpaper();
        let has_wallpaper = wall_result.is_ok();
        report.insert("has_wallpaper".to_string(), has_wallpaper.to_string());

        let mut status = "OK".to_string();
        if !de_supported {
            status = format!(
                "DE {} tidak support. Support: GNOME, Hyprland, swww, feh.",
                de
            );
        } else if !has_matugen {
            status = "Matugen belum install. Run: cargo install matugen".to_string();
        } else if !has_wallpaper {
            status = "Wallpaper tidak ditemukan. Set wallpaper dulu di DE.".to_string();
        }
        report.insert("status".to_string(), status);

        report
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.as_str()),
                    QVariant::from(QString::from(v.as_str())),
                )
            })
            .collect()
    }

    pub fn report_bug_on_github(&self, bug_title: QString, bug_desc: QString) {
        let repo_url = "https://github.com/citzeye/loonix-tunes/issues/new";
        let os_info = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let version = env!("CARGO_PKG_VERSION");

        let title_str = bug_title.to_string();
        let desc_str = bug_desc.to_string();

        let body = format!(
            "### Describe the bug\n{}\n\n### System Info\n- OS: {}\n- Arch: {}\n- Version: v{}",
            desc_str, os_info, arch, version
        );

        let encoded_title = urlencoding::encode(&title_str);
        let encoded_body = urlencoding::encode(&body);
        let final_url = format!("{}?title={}&body={}", repo_url, encoded_title, encoded_body);

        let _ = std::process::Command::new("xdg-open")
            .arg(final_url)
            .spawn();
    }

    fn get_active_wallpaper() -> Result<String, String> {
        let de =
            Self::detect_desktop_environment().ok_or_else(|| "DE tidak support".to_string())?;

        let home = std::env::var("HOME").unwrap_or_default();

        let clean = |raw: &str| -> String {
            raw.trim()
                .trim_matches(|c| c == '\'' || c == '"' || c == '\n' || c == '\r')
                .replace("file://", "")
        };

        match de {
            "GNOME" => {
                if let Ok(out) = Command::new("gsettings")
                    .args(&["get", "org.gnome.desktop.background", "picture-uri"])
                    .output()
                {
                    let p = clean(&String::from_utf8_lossy(&out.stdout));
                    if !p.is_empty() && std::path::Path::new(&p).exists() {
                        return Ok(p);
                    }
                }
            }
            "Hyprland" => {
                if let Ok(out) = Command::new("hyprctl")
                    .args(&["hyprpaper", "listactive"])
                    .output()
                {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if let Some(raw) = stdout.split(" = ").last() {
                        let p = clean(raw);
                        if !p.is_empty() && std::path::Path::new(&p).exists() {
                            return Ok(p);
                        }
                    }
                }
            }
            "swww" => {
                if let Ok(out) = Command::new("swww").arg("query").output() {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if let Some(raw) = stdout.split(": ").last() {
                        let p = clean(raw);
                        if !p.is_empty() && std::path::Path::new(&p).exists() {
                            return Ok(p);
                        }
                    }
                }
            }
            "feh" => {
                let feh_path = std::path::Path::new(&home).join(".fehbg");
                if let Ok(content) = std::fs::read_to_string(&feh_path) {
                    if let Some(raw) = content.split('\'').nth(1) {
                        let p = clean(raw);
                        if !p.is_empty() && std::path::Path::new(&p).exists() {
                            return Ok(p);
                        }
                    }
                }
            }
            _ => {}
        }

        Err("Wallpaper tidak ditemukan".to_string())
    }

    fn fetch_matugen_colors(&self) -> Result<HashMap<String, String>, String> {
        let matugen_bin = Self::get_matugen_path()
            .ok_or_else(|| "Matugen tidak ditemukan. Install: cargo install matugen".to_string())?;

        let wall_path = Self::get_active_wallpaper().map_err(|e| e)?;

        let output = Command::new(&matugen_bin)
            .args(&["image", &wall_path, "--json", "hex"])
            .output()
            .map_err(|e| format!("Gagal eksekusi Matugen: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let v: Value = serde_json::from_str(&json_str)
            .map_err(|_| "Gagal parsing JSON Matugen.".to_string())?;

        let colors = v["colors"]["dark"]
            .as_object()
            .ok_or_else(|| "Struktur JSON Matugen tidak valid.".to_string())?;

        let primary = colors
            .get("primary")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Color 'primary' tidak ditemukan.".to_string())?;
        let secondary = colors
            .get("secondary")
            .or(colors.get("tertiary"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Color 'secondary/tertiary' tidak ditemukan.".to_string())?;
        let on_surface = colors
            .get("on_surface")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Color 'on_surface' tidak ditemukan.".to_string())?;
        let on_surface_variant = colors
            .get("on_surface_variant")
            .or(colors.get("on_surface"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Color 'on_surface_variant' tidak ditemukan.".to_string())?;
        let surface = colors
            .get("surface_container")
            .or(colors.get("surface"))
            .or(colors.get("background"))
            .or(colors.get("base"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Color 'surface' tidak ditemukan.".to_string())?;
        let outline = colors
            .get("outline")
            .or(colors.get("outline_variant"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Color 'outline' tidak ditemukan.".to_string())?;

        let mut map = HashMap::new();
        map.insert("bgmain".to_string(), surface.to_string());
        map.insert("bgoverlay".to_string(), surface.to_string());
        map.insert("graysolid".to_string(), outline.to_string());
        map.insert("contextmenubg".to_string(), surface.to_string());
        map.insert("overlay".to_string(), outline.to_string());
        map.insert("headerbg".to_string(), surface.to_string());
        map.insert("headericon".to_string(), on_surface_variant.to_string());
        map.insert("headertext".to_string(), on_surface_variant.to_string());
        map.insert("headerhover".to_string(), primary.to_string());
        map.insert("playertitle".to_string(), on_surface.to_string());
        map.insert("playersubtext".to_string(), on_surface_variant.to_string());
        map.insert("playeraccent".to_string(), primary.to_string());
        map.insert("playerhover".to_string(), secondary.to_string());
        map.insert("tabtext".to_string(), on_surface.to_string());
        map.insert("tabborder".to_string(), outline.to_string());
        map.insert("tabhover".to_string(), primary.to_string());
        map.insert("playlisttext".to_string(), on_surface.to_string());
        map.insert("playlistfolder".to_string(), secondary.to_string());
        map.insert("playlistactive".to_string(), primary.to_string());
        map.insert("playlisticon".to_string(), secondary.to_string());
        map.insert("dspbg".to_string(), surface.to_string());
        map.insert("dspborder".to_string(), outline.to_string());
        map.insert("dsptext".to_string(), on_surface.to_string());
        map.insert("dspsubtext".to_string(), on_surface_variant.to_string());
        map.insert("dspicon".to_string(), secondary.to_string());
        map.insert("dsphover".to_string(), secondary.to_string());
        map.insert("dsppresettext".to_string(), on_surface_variant.to_string());
        map.insert("dsppresetactive".to_string(), primary.to_string());
        map.insert("dsp10slider".to_string(), secondary.to_string());
        map.insert("dsp10handle".to_string(), primary.to_string());
        map.insert("dsp10bg".to_string(), "#111111".to_string());
        map.insert("dspfaderslider".to_string(), secondary.to_string());
        map.insert("dspfaderhandle".to_string(), primary.to_string());
        map.insert("dspfaderbg".to_string(), "#111111".to_string());
        map.insert("dspmixslider".to_string(), secondary.to_string());
        map.insert("dspmixhandle".to_string(), primary.to_string());
        map.insert("dspmixbg".to_string(), "#111111".to_string());
        map.insert("dspbg".to_string(), surface.to_string());
        map.insert("dspborder".to_string(), outline.to_string());
        map.insert("dsptext".to_string(), on_surface.to_string());
        map.insert("dspsubtext".to_string(), on_surface_variant.to_string());
        map.insert("dspicon".to_string(), secondary.to_string());
        map.insert("dsphover".to_string(), secondary.to_string());
        map.insert("dspactive".to_string(), primary.to_string());
        map.insert("dspslider".to_string(), primary.to_string());
        map.insert("dspsliderbg".to_string(), surface.to_string());
        map.insert("dsphandle".to_string(), secondary.to_string());

        Ok(map)
    }

    pub fn sync_with_wallpaper(&mut self) {
        match self.fetch_matugen_colors() {
            Ok(new_colors) => {
                let qmap: QVariantMap = new_colors
                    .iter()
                    .map(|(k, v)| {
                        (
                            QString::from(k.as_str()),
                            QVariant::from(QString::from(v.as_str())),
                        )
                    })
                    .collect();

                self.colormap = qmap.clone();
                self.current_raw_colors = new_colors;
                self.colormap_changed();

                self.wallpaper_sync_status(true, QString::from("Tema berhasil disinkronisasi!"));
            }
            Err(e) => {
                self.wallpaper_sync_status(false, QString::from(e));
            }
        }
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
            self.themes_changed();

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
            self.themes_changed();

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
            self.themes_changed();

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
                    "bgmain", "#15151B",
                    "bgoverlay", "#201f2b",
                    "graysolid", "#6d6d6d",
                    "contextmenubg", "#2d2d2d",
                    "overlay", "#000000",
                    "headerbg", "#201f2b",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff1ae0",
                    "playertitle", "#00ffa2",
                    "playersubtext", "#57caab",
                    "playeraccent", "#9442ff",
                    "playerhover", "#ff1ae0",
                    "tabtext", "#c6c6c6",
                    "tabborder", "#00ffa2",
                    "tabhover", "#ff1ae0",
                    "tabbg", "#1a1a2e",
                    "tabactive", "#3d3d5c",
                    "tabtextactive", "#ffffff",
                    "playlisttext", "#c6c6c6",
                    "playlistfolder", "#fa7900",
                    "playlistactive", "#00ffa2",
                    "playlisticon", "#fa7900",

                    "dspbg", "#15151B",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#fa7900",
                    "dsptextactive", "#fa7900",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#151515",
                    "dspeqicon", "#9442ff",
                    "dspeqslider", "#ff1ae0",
                    "dspeqsliderbg", "#15151B",
                    "dspeqhandle", "#ff1ae0",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ff1ae0",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#9442ff",
                    "dspeq10slider", "#ff1ae0",
                    "dspeq10sliderbg", "#000000",
                    "dspeq10handle", "#9442ff",
                    "dspeqfaderbg", "#111111",
                    "dspeqfaderslider", "#ff1ae0",
                    "dspeqfaderhandle", "#9442ff",
                    "dspfxbg", "#151515",
                    "dspfxicon", "#9442ff",
                    "dspfxslider", "#ff1ae0",
                    "dspfxsliderbg", "#111111",
                    "dspfxhandle", "#9442ff",
                });
            }
            "Blue" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ddff",
                    "playertitle", "#00ffdd",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ffdd",
                    "playerhover", "#843ff3",
                    "tabtext", "#d1d8e6",
                    "tabborder", "#8a8a8a",
                    "tabhover", "#00ffdd",
                    "playlisttext", "#d1d8e6",
                    "playlistfolder", "#f5a623",
                    "playlistactive", "#843ff3",
"playlisticon", "#00ffdd",

                    "dspbg", "#15151B",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#fa7900",
                    "dsptextactive", "#fa7900",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#151515",
                    "dspeqicon", "#9442ff",
                    "dspeqslider", "#ff1ae0",
                    "dspeqsliderbg", "#15151B",
                    "dspeqhandle", "#ff1ae0",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ff1ae0",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#9442ff",
                    "dspeq10slider", "#ff1ae0",
                    "dspeq10sliderbg", "#000000",
                    "dspeq10handle", "#9442ff",
                    "dspeqfaderbg", "#111111",
                    "dspeqfaderslider", "#ff1ae0",
                    "dspeqfaderhandle", "#9442ff",
                    "dspfxbg", "#151515",
                    "dspfxicon", "#9442ff",
                    "dspfxslider", "#ff1ae0",
                    "dspfxsliderbg", "#111111",
                    "dspfxhandle", "#9442ff",
                });
            }
            "Green" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ff26",
                    "playertitle", "#00ff26",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ff26",
                    "playerhover", "#ffcc00",
                    "tabtext", "#d1e6d8",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#00ff26",
                    "playlisttext", "#d1e6d8",
                    "playlistfolder", "#00ff26",
                    "playlistactive", "#ffcc00",
                    "playlisticon", "#00ff26",

                    "dspbg", "#1e1e1e",
                    "dspborder", "#6d6d6d",

                    "dspeqbg", "#121c15",
                    "dspeqtext", "#00ff66",
                    "dspeqsubtext", "#6d6d6d",
                    "dspeqicon", "#00ff26",
                    "dspeqhover", "#ffcc00",
                    "dspeqpresettext", "#6d6d6d",
                    "dspeqpresetactive", "#00ff66",
                    "dspeq10slider", "#ffcc00",
                    "dspeq10handle", "#00ff26",
                    "dspeq10bg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfaderhandle", "#8b0000",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqmixslider", "#00ff26",
                    "dspeqmixhandle", "#ffcc00",
                    "dspeqmixbg", "#1e1e1e",

                    "dspfxbg", "#121c15",
                    "dspfxborder", "#6d6d6d",
                    "dspfxtext", "#00ff66",
                    "dspfxsubtext", "#6d6d6d",
                    "dspfxicon", "#00ff26",
                    "dspfxhover", "#ffcc00",
                    "dspfxactive", "#00ff66",
                    "dspfxslider", "#00ff66",
                    "dspfxsliderbg", "#121c15",
                    "dspfxhandle", "#00ff26",

                    "dspslider", "#00ff66",
                    "dspsliderbg", "#121c15",
                    "dsphandle", "#00ff26",
                });
            }
            "Monochrome" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffffff",
                    "playertitle", "#ffffff",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#555555",
                    "playerhover", "#ffffff",
                    "tabtext", "#e0e0e0",
                    "tabborder", "#ffffff",
                    "tabhover", "#ffffff",
                    "playlisttext", "#e0e0e0",
                    "playlistfolder", "#d4d4d4",
                    "playlistactive", "#ffffff",
                    "playlisticon", "#d4d4d4",

                    "dspbg", "#1e1e1e",
                    "dspborder", "#ffffff",

                    "dspeqbg", "#121212",
                    "dspeqtext", "#ffffff",
                    "dspeqsubtext", "#8b8b8b",
                    "dspeqicon", "#d4d4d4",
                    "dspeqhover", "#ffffff",
                    "dspeqpresettext", "#8b8b8b",
                    "dspeqpresetactive", "#ffffff",
                    "dspeq10slider", "#ffffff",
                    "dspeq10handle", "#555555",
                    "dspeq10bg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfaderhandle", "#8b0000",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqmixslider", "#555555",
                    "dspeqmixhandle", "#ffffff",
                    "dspeqmixbg", "#1e1e1e",

                    "dspfxbg", "#121212",
                    "dspfxborder", "#ffffff",
                    "dspfxtext", "#ffffff",
                    "dspfxsubtext", "#8b8b8b",
                    "dspfxicon", "#d4d4d4",
                    "dspfxhover", "#ffffff",
                    "dspfxactive", "#ffffff",
                    "dspfxslider", "#ffffff",
                    "dspfxsliderbg", "#121212",
                    "dspfxhandle", "#d4d4d4",

                    "dspslider", "#ffffff",
                    "dspsliderbg", "#121212",
                    "dsphandle", "#d4d4d4",
                });
            }
            "Orange" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffea00",
                    "playertitle", "#ff5500",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ff5500",
                    "playerhover", "#ffea00",
                    "tabtext", "#ecdcd9",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff5500",
                    "playlisttext", "#ecdcd9",
                    "playlistfolder", "#ffea00",
                    "playlistactive", "#ff5500",
                    "playlisticon", "#ff5500",

                    "dspbg", "#1e1e1e",
                    "dspborder", "#6d6d6d",

                    "dspeqbg", "#1c1210",
                    "dspeqtext", "#ff5500",
                    "dspeqsubtext", "#6d6d6d",
                    "dspeqicon", "#ff5500",
                    "dspeqhover", "#ffea00",
                    "dspeqpresettext", "#6d6d6d",
                    "dspeqpresetactive", "#ff5500",
                    "dspeq10slider", "#ffea00",
                    "dspeq10handle", "#ff5500",
                    "dspeq10bg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfaderhandle", "#8b0000",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqmixslider", "#ff5500",
                    "dspeqmixhandle", "#ffea00",
                    "dspeqmixbg", "#1e1e1e",

                    "dspfxbg", "#1c1210",
                    "dspfxborder", "#6d6d6d",
                    "dspfxtext", "#ff5500",
                    "dspfxsubtext", "#6d6d6d",
                    "dspfxicon", "#ff5500",
                    "dspfxhover", "#ffea00",
                    "dspfxactive", "#ff5500",
                    "dspfxslider", "#ff5500",
                    "dspfxsliderbg", "#1c1210",
                    "dspfxhandle", "#ff5500",

                    "dspslider", "#ff5500",
                    "dspsliderbg", "#1c1210",
                    "dsphandle", "#ff5500",
                });
            }
            "Pink" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ffcc",
                    "playertitle", "#f965d9",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#f965d9",
                    "playerhover", "#00ffcc",
                    "tabtext", "#eedef2",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#f965d9",
                    "playlisttext", "#eedef2",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#65f996",
                    "playlisticon", "#f965d9",

                    "dspbg", "#1e1e1e",
                    "dspborder", "#6d6d6d",

                    "dspeqbg", "#1b101f",
                    "dspeqtext", "#f965d9",
                    "dspeqsubtext", "#6d6d6d",
                    "dspeqicon", "#f965d9",
                    "dspeqhover", "#00ffcc",
                    "dspeqpresettext", "#6d6d6d",
                    "dspeqpresetactive", "#f965d9",
                    "dspeq10slider", "#00ffcc",
                    "dspeq10handle", "#f965d9",
                    "dspeq10bg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfaderhandle", "#8b0000",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqmixslider", "#f965d9",
                    "dspeqmixhandle", "#00ffcc",
                    "dspeqmixbg", "#1e1e1e",

                    "dspfxbg", "#1b101f",
                    "dspfxborder", "#6d6d6d",
                    "dspfxtext", "#f965d9",
                    "dspfxsubtext", "#6d6d6d",
                    "dspfxicon", "#f965d9",
                    "dspfxhover", "#00ffcc",
                    "dspfxactive", "#f965d9",
                    "dspfxslider", "#f965d9",
                    "dspfxsliderbg", "#1b101f",
                    "dspfxhandle", "#00ffcc",
                });
            }
            "Red" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff003c",
                    "playertitle", "#ff003c",
                    "playersubtext", "#bdbdbd",
                    "playeraccent", "#ff003c",
                    "playerhover", "#2b00ff",
                    "tabtext", "#bdbdbd",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff003c",
                    "playlisttext", "#bdbdbd",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ff003c",
                    "playlisticon", "#2b00ff",

                    "dspbg", "#1e1e1e",
                    "dspborder", "#6d6d6d",

                    "dspeqbg", "#1c0d0d",
                    "dspeqtext", "#ff003c",
                    "dspeqsubtext", "#bdbdbd",
                    "dspeqicon", "#ff003c",
                    "dspeqhover", "#2b00ff",
                    "dspeqpresettext", "#bdbdbd",
                    "dspeqpresetactive", "#ff003c",
                    "dspeq10slider", "#2b00ff",
                    "dspeq10handle", "#ff003c",
                    "dspeq10bg", "#1e1e1e",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfaderhandle", "#8b0000",
                    "dspeqfaderbg", "#1e1e1e",
                    "dspeqmixslider", "#ff003c",
                    "dspeqmixhandle", "#2b00ff",
                    "dspeqmixbg", "#1e1e1e",

                    "dspfxbg", "#1c0d0d",
                    "dspfxborder", "#6d6d6d",
                    "dspfxtext", "#ff003c",
                    "dspfxsubtext", "#bdbdbd",
                    "dspfxicon", "#ff003c",
                    "dspfxhover", "#2b00ff",
                    "dspfxactive", "#ff003c",
                    "dspfxslider", "#ff003c",
                    "dspfxsliderbg", "#1c0d0d",
                    "dspfxhandle", "#2b00ff",

                    "dspslider", "#ff003c",
                    "dspsliderbg", "#1c0d0d",
                    "dsphandle", "#2b00ff",
                });
            }
            "Yellow" => {
                c!(map, {
                    "bgmain", "#0d1012",
                    "bgoverlay", "#15191c",
                    "graysolid", "#2d353b",
                    "contextmenubg", "#0a0c0e",
                    "overlay", "#000000",
                    "headerbg", "#15191c",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#f965d9",
                    "playertitle", "#ffea00",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ffea00",
                    "playerhover", "#f965d9",
                    "tabtext", "#dde0d1",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ffea00",
                    "playlisttext", "#dde0d1",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ffea00",
                    "playlisticon", "#f965d9",

                    "dspbg", "#15191c",
                    "dspborder", "#6d6d6d",

                    "dspeqbg", "#0d1012",
                    "dspeqtext", "#ffea00",
                    "dspeqsubtext", "#6d6d6d",
                    "dspeqicon", "#ffea00",
                    "dspeqhover", "#f965d9",
                    "dspeqpresettext", "#6d6d6d",
                    "dspeqpresetactive", "#ffea00",
                    "dspeq10slider", "#f965d9",
                    "dspeq10handle", "#ffea00",
                    "dspeq10bg", "#15191c",
                    "dspeqfaderslider", "#f5a623",
                    "dspeqfaderhandle", "#8b0000",
                    "dspeqfaderbg", "#15191c",
                    "dspeqmixslider", "#ffea00",
                    "dspeqmixhandle", "#f965d9",
                    "dspeqmixbg", "#15191c",

                    "dspfxbg", "#0d1012",
                    "dspfxborder", "#6d6d6d",
                    "dspfxtext", "#ffea00",
                    "dspfxsubtext", "#6d6d6d",
                    "dspfxicon", "#ffea00",
                    "dspfxhover", "#f965d9",
                    "dspfxactive", "#ffea00",
                    "dspfxslider", "#ffea00",
                    "dspfxsliderbg", "#0d1012",
                    "dspfxhandle", "#f965d9",

                    "dspslider", "#ffea00",
                    "dspsliderbg", "#0d1012",
                    "dsphandle", "#f965d9",
});
            }
            _ => {
                c!(map, {
                    "bgmain", "#15151B",
                    "bgoverlay", "#201f2b",
                    "graysolid", "#6d6d6d",
                    "contextmenubg", "#2d2d2d",
                    "overlay", "#000000",
                    "headerbg", "#201f2b",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff1ae0",
                    "playertitle", "#00ffa2",
                    "playersubtext", "#57caab",
                    "playeraccent", "#9442ff",
                    "playerhover", "#ff1ae0",
                    "tabtext", "#c6c6c6",
                    "tabborder", "#00ffa2",
                    "tabhover", "#ff1ae0",
                    "tabbg", "#1a1a2e",
                    "tabactive", "#3d3d5c",
                    "tabtextactive", "#ffffff",
                    "playlisttext", "#c6c6c6",
                    "playlistfolder", "#fa7900",
                    "playlistactive", "#00ffa2",
                    "playlisticon", "#fa7900",
                    "dspbg", "#15151B",
                    "dsptext", "#6d6d6d",
                    "dsptexthover", "#fa7900",
                    "dsptextactive", "#fa7900",
                    "dspborder", "#6d6d6d",
                    "dspgridbg", "#111111",
                    "dspeqbg", "#151515",
                    "dspeqicon", "#9442ff",
                    "dspeqslider", "#ff1ae0",
                    "dspeqsliderbg", "#15151B",
                    "dspeqhandle", "#ff1ae0",
                    "dspeampbg", "#111111",
                    "dspampslider", "#ff1ae0",
                    "dspampsliderbg", "#000000",
                    "dspeamphandle", "#9442ff",
                    "dspeq10slider", "#ff1ae0",
                    "dspeq10sliderbg", "#000000",
                    "dspeq10handle", "#9442ff",
                    "dspeqfaderbg", "#111111",
                    "dspeqfaderslider", "#ff1ae0",
                    "dspeqfaderhandle", "#9442ff",
                    "dspfxbg", "#151515",
                    "dspfxicon", "#9442ff",
                    "dspfxslider", "#ff1ae0",
                    "dspfxsliderbg", "#111111",
                    "dspfxhandle", "#9442ff",
                });
            }
        }

        map
    }
}
