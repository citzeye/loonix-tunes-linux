/* --- LOONIX-TUNES src/ui/theme.rs --- */
use qmetaobject::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "Default".to_string(),
        }
    }
}

#[derive(QObject, Default)]
pub struct ThemeManager {
    base: qt_base_class!(trait QObject),
    pub colormap: qt_property!(QVariantMap; NOTIFY colormap_changed),
    pub colormap_changed: qt_signal!(),
    pub current_theme: qt_property!(QString; NOTIFY current_theme_changed),
    pub current_theme_changed: qt_signal!(),
    pub set_theme: qt_method!(fn(&mut self, name: String)),
    pub cycle_theme: qt_method!(fn(&mut self)),
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut s = Self::default();
        let cfg: AppConfig = confy::load("loonix-tunes", "config").unwrap_or_default();

        let theme_name = if cfg.theme.is_empty() {
            "Default".to_string()
        } else {
            cfg.theme
        };
        s.set_theme(theme_name);
        s
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
        themes.insert(0, "Default".into());
        themes
    }

    pub fn cycle_theme(&mut self) {
        let themes = Self::available_themes();
        let current = self.current_theme.to_string();
        if let Some(idx) = themes.iter().position(|t| t == &current) {
            let next_idx = (idx + 1) % themes.len();
            self.set_theme(themes[next_idx].clone());
        } else {
            self.set_theme("Default".to_string());
        }
    }

    pub fn set_theme(&mut self, name: String) {
        let cfg = AppConfig {
            theme: name.clone(),
        };
        let _ = confy::store("loonix-tunes", "config", cfg);

        let mut map = HashMap::new();

        match name.as_str() {
            "Blue" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#121212");
                map.insert("bgoverlay", "#1e1e1e");
                map.insert("graysolid", "#333333");
                map.insert("contextmenubg", "#181818");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#1e1e1e");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#00ddff");

                // --- PLAYER ---
                map.insert("playertitle", "#00ffdd");
                map.insert("playersubtext", "#6d6d6d");
                map.insert("playeraccent", "#00ffdd");
                map.insert("playerhover", "#843ff3");

                // --- TABS ---
                map.insert("tabtext", "#d1d8e6");
                map.insert("tabborder", "#8a8a8a");
                map.insert("tabhover", "#00ffdd");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#d1d8e6");
                map.insert("playlistfolder", "#f5a623");
                map.insert("playlistactive", "#843ff3");
                map.insert("playlisticon", "#00ffdd");

                // --- FOOTER ---
                map.insert("footertext", "#00e5ff");
                map.insert("footeralttext", "#5b6c8f");
                map.insert("footerhover", "#843ff3");
                map.insert("footerwarning", "#ffea00");
                map.insert("footeralert", "#ff1744");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#121a2f");
                map.insert("eqborder", "#8a8a8a");
                map.insert("eqslideractive", "#00e5ff");
                map.insert("eqslider", "#00ffdd");
                map.insert("eqhandle", "#843ff3");
                map.insert("eqgain", "#f5a623");
                map.insert("eqmix", "#843ff3");
                map.insert("fxnodeactive", "#00e5ff");
                map.insert("fxnodedim", "#5b6c8f");
            }
            "Green" => {
                // --- BACKGROUNDS (60%) ---
                map.insert("bgmain", "#121212");
                map.insert("bgoverlay", "#1e1e1e");
                map.insert("graysolid", "#333333");
                map.insert("contextmenubg", "#181818");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#1e1e1e");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#00ff26");

                // --- PLAYER ---
                map.insert("playertitle", "#00ff26");
                map.insert("playersubtext", "#6d6d6d");
                map.insert("playeraccent", "#00ff26");
                map.insert("playerhover", "#ffcc00");

                // --- TABS ---
                map.insert("tabtext", "#d1e6d8");
                map.insert("tabborder", "#6d6d6d");
                map.insert("tabhover", "#00ff26");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#d1e6d8");
                map.insert("playlistfolder", "#00ff26");
                map.insert("playlistactive", "#ffcc00");
                map.insert("playlisticon", "#00ff26");

                // --- FOOTER ---
                map.insert("footertext", "#00ff66");
                map.insert("footeralttext", "#5a8065");
                map.insert("footerhover", "#ff00ff");
                map.insert("footerwarning", "#ffcc00");
                map.insert("footeralert", "#ff3300");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#121c15");
                map.insert("eqborder", "#6d6d6d");
                map.insert("eqslideractive", "#00ff66");
                map.insert("eqslider", "#00ff26");
                map.insert("eqhandle", "#ff00ff");
                map.insert("eqgain", "#ff3300");
                map.insert("eqmix", "#ff00ff");
                map.insert("fxnodeactive", "#00ff66");
                map.insert("fxnodedim", "#213326");
            }
            "Monochrome" => {
                // --- BACKGROUNDS (60%) ---
                map.insert("bgmain", "#121212");
                map.insert("bgoverlay", "#1e1e1e");
                map.insert("graysolid", "#333333");
                map.insert("contextmenubg", "#181818");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#1e1e1e");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#ffffff");

                // --- PLAYER ---
                map.insert("playertitle", "#ffffff");
                map.insert("playersubtext", "#6d6d6d");
                map.insert("playeraccent", "#555555");
                map.insert("playerhover", "#ffffff");

                // --- TABS ---
                map.insert("tabtext", "#e0e0e0");
                map.insert("tabborder", "#ffffff");
                map.insert("tabhover", "#ffffff");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#e0e0e0");
                map.insert("playlistfolder", "#d4d4d4");
                map.insert("playlistactive", "#ffffff");
                map.insert("playlisticon", "#d4d4d4");

                // --- FOOTER ---
                map.insert("footertext", "#ffffff");
                map.insert("footeralttext", "#8b8b8b");
                map.insert("footerhover", "#ffffff");
                map.insert("footerwarning", "#d4d4d4");
                map.insert("footeralert", "#ffffff");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#1e1e1e");
                map.insert("eqborder", "#ffffff");
                map.insert("eqslideractive", "#ffffff");
                map.insert("eqslider", "#555555");
                map.insert("eqhandle", "#ffffff");
                map.insert("eqgain", "#d4d4d4");
                map.insert("eqmix", "#ffffff");
                map.insert("fxnodeactive", "#ffffff");
                map.insert("fxnodedim", "#333333");
            }
            "Orange" => {
                // --- BACKGROUNDS (60%) ---
                map.insert("bgmain", "#121212");
                map.insert("bgoverlay", "#1e1e1e");
                map.insert("graysolid", "#333333");
                map.insert("contextmenubg", "#181818");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#1e1e1e");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#ffea00");

                // --- PLAYER ---
                map.insert("playertitle", "#ff5500");
                map.insert("playersubtext", "#6d6d6d");
                map.insert("playeraccent", "#ff5500");
                map.insert("playerhover", "#ffea00");

                // --- TABS ---
                map.insert("tabtext", "#ecdcd9");
                map.insert("tabborder", "#6d6d6d");
                map.insert("tabhover", "#ff5500");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#ecdcd9");
                map.insert("playlistfolder", "#ffea00");
                map.insert("playlistactive", "#ff5500");
                map.insert("playlisticon", "#ff5500");

                // --- FOOTER ---
                map.insert("footertext", "#ff5500");
                map.insert("footeralttext", "#a36b5e");
                map.insert("footerhover", "#ffea00");
                map.insert("footerwarning", "#ffea00");
                map.insert("footeralert", "#ff1744");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#1c1210");
                map.insert("eqborder", "#6d6d6d");
                map.insert("eqslideractive", "#ff5500");
                map.insert("eqslider", "#ff5500");
                map.insert("eqhandle", "#ffea00");
                map.insert("eqgain", "#00e5ff");
                map.insert("eqmix", "#ffea00");
                map.insert("fxnodeactive", "#ff5500");
                map.insert("fxnodedim", "#36211e");
            }
            "Pink" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#121212");
                map.insert("bgoverlay", "#1e1e1e");
                map.insert("graysolid", "#333333");
                map.insert("contextmenubg", "#181818");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#1e1e1e");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#00ffcc");

                // --- PLAYER ---
                map.insert("playertitle", "#f965d9");
                map.insert("playersubtext", "#6d6d6d");
                map.insert("playeraccent", "#f965d9");
                map.insert("playerhover", "#00ffcc");

                // --- TABS ---
                map.insert("tabtext", "#eedef2");
                map.insert("tabborder", "#6d6d6d");
                map.insert("tabhover", "#f965d9");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#eedef2");
                map.insert("playlistfolder", "#d59407");
                map.insert("playlistactive", "#65f996");
                map.insert("playlisticon", "#f965d9");

                // --- FOOTER ---
                map.insert("footertext", "#f965d9");
                map.insert("footeralttext", "#9b74ab");
                map.insert("footerhover", "#00ffcc");
                map.insert("footerwarning", "#ccff00");
                map.insert("footeralert", "#ff1744");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#1b101f");
                map.insert("eqborder", "#6d6d6d");
                map.insert("eqslideractive", "#f965d9");
                map.insert("eqslider", "#f965d9");
                map.insert("eqhandle", "#00ffcc");
                map.insert("eqgain", "#ccff00");
                map.insert("eqmix", "#00ffcc");
                map.insert("fxnodeactive", "#f965d9");
                map.insert("fxnodedim", "#382042");
            }
            "Red" => {
                // --- BACKGROUNDS (60%) ---
                map.insert("bgmain", "#121212");
                map.insert("bgoverlay", "#1e1e1e");
                map.insert("graysolid", "#333333");
                map.insert("contextmenubg", "#181818");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#1e1e1e");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#ff003c");

                // --- PLAYER ---
                map.insert("playertitle", "#ff003c");
                map.insert("playersubtext", "#bdbdbd");
                map.insert("playeraccent", "#ff003c");
                map.insert("playerhover", "#2b00ff");

                // --- TABS ---
                map.insert("tabtext", "#bdbdbd");
                map.insert("tabborder", "#6d6d6d");
                map.insert("tabhover", "#ff003c");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#bdbdbd");
                map.insert("playlistfolder", "#d59407");
                map.insert("playlistactive", "#ff003c");
                map.insert("playlisticon", "#2b00ff");

                // --- FOOTER ---
                map.insert("footertext", "#ff003c");
                map.insert("footeralttext", "#a86363");
                map.insert("footerhover", "#00ff88");
                map.insert("footerwarning", "#ffea00");
                map.insert("footeralert", "#ff003c");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#1c0d0d");
                map.insert("eqborder", "#6d6d6d");
                map.insert("eqslideractive", "#2b00ff");
                map.insert("eqslider", "#ff003c");
                map.insert("eqhandle", "#00ff88");
                map.insert("eqgain", "#bdbdbd");
                map.insert("eqmix", "#2b00ff");
                map.insert("fxnodeactive", "#ff003c");
                map.insert("fxnodedim", "#3d1b1b");
            }
            "Yellow" => {
                // --- BACKGROUNDS (60%) ---
                map.insert("bgmain", "#0d1012");
                map.insert("bgoverlay", "#15191c");
                map.insert("graysolid", "#2d353b");
                map.insert("contextmenubg", "#0a0c0e");
                map.insert("overlay", "#000000");

                // --- HEADER ---
                map.insert("headerbg", "#15191c");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#f965d9");

                // --- PLAYER ---
                map.insert("playertitle", "#ffea00");
                map.insert("playersubtext", "#6d6d6d");
                map.insert("playeraccent", "#ffea00");
                map.insert("playerhover", "#f965d9");

                // --- TABS ---
                map.insert("tabtext", "#dde0d1");
                map.insert("tabborder", "#6d6d6d");
                map.insert("tabhover", "#ffea00");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#dde0d1");
                map.insert("playlistfolder", "#d59407");
                map.insert("playlistactive", "#ffea00");
                map.insert("playlisticon", "#f965d9");

                // --- FOOTER ---
                map.insert("footertext", "#ffea00");
                map.insert("footeralttext", "#9fa67c");
                map.insert("footerhover", "#f965d9");
                map.insert("footerwarning", "#f965d9");
                map.insert("footeralert", "#ff1744");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#15191c");
                map.insert("eqborder", "#6d6d6d");
                map.insert("eqslideractive", "#ffea00");
                map.insert("eqslider", "#ffea00");
                map.insert("eqhandle", "#f965d9");
                map.insert("eqgain", "#b000ff");
                map.insert("eqmix", "#f965d9");
                map.insert("fxnodeactive", "#ffea00");
                map.insert("fxnodedim", "#2d353b");
            }
            _ => {
                // --- DEFAULT FALLBACK ---
                map.insert("bgmain", "#15141b");
                map.insert("bgoverlay", "#201f2b");
                map.insert("graysolid", "#6d6d6d");
                map.insert("contextmenubg", "#2d2d2d");
                map.insert("overlay", "#000000");
                // --- HEADER ---
                map.insert("headerbg", "#201f2b");
                map.insert("headericon", "#6d6d6d");
                map.insert("headertext", "#6d6d6d");
                map.insert("headerhover", "#ff1ae0");
                // --- PLAYER ---
                map.insert("playertitle", "#00ffa2");
                map.insert("playersubtext", "#57caab");
                map.insert("playeraccent", "#9442ff");
                map.insert("playerhover", "#ff1ae0");
                // --- TABS ---
                map.insert("tabtext", "#c6c6c6");
                map.insert("tabborder", "#00ffa2");
                map.insert("tabhover", "#ff1ae0");
                // --- PLAYLIST ---
                map.insert("playlisttext", "#c6c6c6");
                map.insert("playlistfolder", "#ff881a");
                map.insert("playlistactive", "#00ffa2");
                map.insert("playlisticon", "#ff881a");
                // --- FOOTER ---
                map.insert("footertext", "#00ffa2");
                map.insert("footeralttext", "#6d6d6d");
                map.insert("footerhover", "#ff1ae0");
                map.insert("footerwarning", "#ffaa33");
                map.insert("footeralert", "#cc3333");
                // --- EQ & FX ---
                map.insert("eqsliderbg", "#201f2b");
                map.insert("eqborder", "#00ffa2");
                map.insert("eqslideractive", "#00ffa2");
                map.insert("eqslider", "#9442ff");
                map.insert("eqhandle", "#ff1ae0");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#ff1ae0");
                map.insert("fxnodeactive", "#00ffa2");
                map.insert("fxnodedim", "#6d6d6d");
            }
        }

        let qmap: QVariantMap = map
            .into_iter()
            .map(|(k, v)| (QString::from(k), QVariant::from(QString::from(v))))
            .collect();

        self.colormap = qmap;
        self.current_theme = QString::from(name);
        self.colormap_changed();
        self.current_theme_changed();
    }
}
