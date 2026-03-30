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
                map.insert("bgmain", "#1E222A");
                map.insert("bgoverlay", "#252931");
                map.insert("graysolid", "#4B5263");
                map.insert("contextmenubg", "#E61E222A");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#252931");
                map.insert("headericon", "#828997");
                map.insert("headertext", "#828997");
                map.insert("headerhover", "#56B6C2");

                // --- PLAYER ---
                map.insert("playertitle", "#61AFEF");
                map.insert("playersubtext", "#ABB2BF");
                map.insert("playeraccent", "#56B6C2");
                map.insert("playerhover", "#61AFEF");

                // --- TABS ---
                map.insert("tabtext", "#ABB2BF");
                map.insert("tabborder", "#4B5263");
                map.insert("tabhover", "#61AFEF");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#ABB2BF");
                map.insert("playlistfolder", "#56B6C2");
                map.insert("playlistactive", "#61AFEF");
                map.insert("playlisticon", "#56B6C2");

                // --- FOOTER ---
                map.insert("footertext", "#56B6C2");
                map.insert("footeralttext", "#828997");
                map.insert("footerhover", "#61AFEF");
                map.insert("footerwarning", "#E5C07B");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#252931");
                map.insert("eqslideractive", "#56B6C2");
                map.insert("eqhandle", "#61AFEF");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#FF79C6");
                map.insert("fxnodeactive", "#61AFEF");
                map.insert("fxnodedim", "#4B5263");
            }
            "Green" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#1D2021");
                map.insert("bgoverlay", "#282828");
                map.insert("graysolid", "#504945");
                map.insert("contextmenubg", "#E61D2021");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#282828");
                map.insert("headericon", "#A89984");
                map.insert("headertext", "#A89984");
                map.insert("headerhover", "#B8BB26");

                // --- PLAYER ---
                map.insert("playertitle", "#B8BB26");
                map.insert("playersubtext", "#EBDBB2");
                map.insert("playeraccent", "#8EC07C");
                map.insert("playerhover", "#B8BB26");

                // --- TABS ---
                map.insert("tabtext", "#A89984");
                map.insert("tabborder", "#504945");
                map.insert("tabhover", "#8EC07C");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#EBDBB2");
                map.insert("playlistfolder", "#8EC07C");
                map.insert("playlistactive", "#B8BB26");
                map.insert("playlisticon", "#8EC07C");

                // --- FOOTER ---
                map.insert("footertext", "#8EC07C");
                map.insert("footeralttext", "#A89984");
                map.insert("footerhover", "#B8BB26");
                map.insert("footerwarning", "#FABD2F");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#282828");
                map.insert("eqslideractive", "#8EC07C");
                map.insert("eqhandle", "#B8BB26");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#FF79C6");
                map.insert("fxnodeactive", "#B8BB26");
                map.insert("fxnodedim", "#504945");
            }
            "Monochrome" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#121212");
                map.insert("bgoverlay", "#1E1E1E");
                map.insert("graysolid", "#333333");
                map.insert("contextmenubg", "#E6121212");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#1E1E1E");
                map.insert("headericon", "#888888");
                map.insert("headertext", "#888888");
                map.insert("headerhover", "#FFFFFF");

                // --- PLAYER ---
                map.insert("playertitle", "#FFFFFF");
                map.insert("playersubtext", "#B3B3B3");
                map.insert("playeraccent", "#E0E0E0");
                map.insert("playerhover", "#FFFFFF");

                // --- TABS ---
                map.insert("tabtext", "#B3B3B3");
                map.insert("tabborder", "#333333");
                map.insert("tabhover", "#FFFFFF");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#B3B3B3");
                map.insert("playlistfolder", "#E0E0E0");
                map.insert("playlistactive", "#FFFFFF");
                map.insert("playlisticon", "#B3B3B3");

                // --- FOOTER ---
                map.insert("footertext", "#E0E0E0");
                map.insert("footeralttext", "#888888");
                map.insert("footerhover", "#FFFFFF");
                map.insert("footerwarning", "#FFB86C");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#1E1E1E");
                map.insert("eqslideractive", "#E0E0E0");
                map.insert("eqhandle", "#FFFFFF");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#FF79C6");
                map.insert("fxnodeactive", "#FFFFFF");
                map.insert("fxnodedim", "#333333");
            }
            "Orange" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#1A1817");
                map.insert("bgoverlay", "#24211F");
                map.insert("graysolid", "#4A433F");
                map.insert("contextmenubg", "#E61A1817");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#24211F");
                map.insert("headericon", "#9E938E");
                map.insert("headertext", "#9E938E");
                map.insert("headerhover", "#FF8A65");

                // --- PLAYER ---
                map.insert("playertitle", "#FF7043");
                map.insert("playersubtext", "#D7CCC8");
                map.insert("playeraccent", "#FF8A65");
                map.insert("playerhover", "#FFAB91");

                // --- TABS ---
                map.insert("tabtext", "#9E938E");
                map.insert("tabborder", "#4A433F");
                map.insert("tabhover", "#FF7043");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#D7CCC8");
                map.insert("playlistfolder", "#FF8A65");
                map.insert("playlistactive", "#FF7043");
                map.insert("playlisticon", "#FF8A65");

                // --- FOOTER ---
                map.insert("footertext", "#FF8A65");
                map.insert("footeralttext", "#9E938E");
                map.insert("footerhover", "#FF7043");
                map.insert("footerwarning", "#FFCA28");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#24211F");
                map.insert("eqslideractive", "#FF8A65");
                map.insert("eqhandle", "#FF7043");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#FF79C6");
                map.insert("fxnodeactive", "#FF7043");
                map.insert("fxnodedim", "#4A433F");
            }
            "Pink" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#1F1D2E");
                map.insert("bgoverlay", "#26233A");
                map.insert("graysolid", "#524F67");
                map.insert("contextmenubg", "#E61F1D2E");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#26233A");
                map.insert("headericon", "#908CAA");
                map.insert("headertext", "#908CAA");
                map.insert("headerhover", "#EB6F92");

                // --- PLAYER ---
                map.insert("playertitle", "#EBBCBA");
                map.insert("playersubtext", "#E0DEF4");
                map.insert("playeraccent", "#C4A7E7");
                map.insert("playerhover", "#EB6F92");

                // --- TABS ---
                map.insert("tabtext", "#908CAA");
                map.insert("tabborder", "#524F67");
                map.insert("tabhover", "#FF79C6");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#E0DEF4");
                map.insert("playlistfolder", "#C4A7E7");
                map.insert("playlistactive", "#EB6F92");
                map.insert("playlisticon", "#C4A7E7");

                // --- FOOTER ---
                map.insert("footertext", "#C4A7E7");
                map.insert("footeralttext", "#908CAA");
                map.insert("footerhover", "#EB6F92");
                map.insert("footerwarning", "#F6C177");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#26233A");
                map.insert("eqslideractive", "#C4A7E7");
                map.insert("eqhandle", "#EB6F92");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#FF79C6");
                map.insert("fxnodeactive", "#EB6F92");
                map.insert("fxnodedim", "#524F67");
            }
            "Red" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#181818");
                map.insert("bgoverlay", "#222222");
                map.insert("graysolid", "#444444");
                map.insert("contextmenubg", "#E6181818");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#222222");
                map.insert("headericon", "#8C8C8C");
                map.insert("headertext", "#8C8C8C");
                map.insert("headerhover", "#FF4D4D");

                // --- PLAYER ---
                map.insert("playertitle", "#FF4D4D");
                map.insert("playersubtext", "#CCCCCC");
                map.insert("playeraccent", "#FF7373");
                map.insert("playerhover", "#FF9999");

                // --- TABS ---
                map.insert("tabtext", "#8C8C8C");
                map.insert("tabborder", "#444444");
                map.insert("tabhover", "#FF4D4D");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#CCCCCC");
                map.insert("playlistfolder", "#FF7373");
                map.insert("playlistactive", "#FF4D4D");
                map.insert("playlisticon", "#FF7373");

                // --- FOOTER ---
                map.insert("footertext", "#FF7373");
                map.insert("footeralttext", "#8C8C8C");
                map.insert("footerhover", "#FF4D4D");
                map.insert("footerwarning", "#FFB84D");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#222222");
                map.insert("eqslideractive", "#FF7373");
                map.insert("eqhandle", "#FF4D4D");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#FF79C6");
                map.insert("fxnodeactive", "#FF4D4D");
                map.insert("fxnodedim", "#444444");
            }
            "Yellow" => {
                // --- BACKGROUNDS ---
                map.insert("bgmain", "#1C1E26");
                map.insert("bgoverlay", "#232530");
                map.insert("graysolid", "#4B4D5B");
                map.insert("contextmenubg", "#E61C1E26");
                map.insert("overlay", "#80000000");

                // --- HEADER ---
                map.insert("headerbg", "#232530");
                map.insert("headericon", "#818392");
                map.insert("headertext", "#818392");
                map.insert("headerhover", "#F0C674");

                // --- PLAYER ---
                map.insert("playertitle", "#E5C07B");
                map.insert("playersubtext", "#D5D8E6");
                map.insert("playeraccent", "#D19A66");
                map.insert("playerhover", "#F0C674");

                // --- TABS ---
                map.insert("tabtext", "#818392");
                map.insert("tabborder", "#4B4D5B");
                map.insert("tabhover", "#E5C07B");

                // --- PLAYLIST ---
                map.insert("playlisttext", "#D5D8E6");
                map.insert("playlistfolder", "#D19A66");
                map.insert("playlistactive", "#E5C07B");
                map.insert("playlisticon", "#D19A66");

                // --- FOOTER ---
                map.insert("footertext", "#D19A66");
                map.insert("footeralttext", "#818392");
                map.insert("footerhover", "#E5C07B");
                map.insert("footerwarning", "#E06C75");

                // --- EQ & FX ---
                map.insert("eqsliderbg", "#232530");
                map.insert("eqslideractive", "#D19A66");
                map.insert("eqhandle", "#E5C07B");
                map.insert("eqgain", "#ff881a");
                map.insert("eqmix", "#FF79C6");
                map.insert("fxnodeactive", "#E5C07B");
                map.insert("fxnodedim", "#4B4D5B");
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
                map.insert("headerhover", "#f763ff");
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
                map.insert("eqslideractive", "#00ffa2");
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
