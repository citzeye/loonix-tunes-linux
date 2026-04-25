/* --- loonixtunesv2/src/audio/popup.rs | Popup --- */
use crate::ui::components::ThemeManager;
use qmetaobject::*;

#[derive(QObject, Default)]
pub struct PopupMenu {
    base: qt_base_class!(trait QObject),

    // --- PROPERTIES ---
    pub current_menu: qt_property!(QString; NOTIFY current_menu_changed),
    current_menu_changed: qt_signal!(),

    pub menu_items: qt_property!(QVariantList; NOTIFY menu_items_changed),
    menu_items_changed: qt_signal!(),

    pub selected_action: qt_property!(QString; NOTIFY action_triggered),
    action_triggered: qt_signal!(),

    pub active_theme: String,

    // --- METHODS ---
    pub show_menu: qt_method!(fn(&mut self, menu_type: String, x: f64, y: f64)),
    pub get_theme_items: qt_method!(fn(&self) -> QVariantList),
    pub get_settings_items: qt_method!(fn(&self) -> QVariantList),
    pub hide_menu: qt_method!(fn(&mut self)),
    pub clear_items: qt_method!(fn(&mut self)),
    pub trigger_action: qt_method!(fn(&mut self, action: String)),
}

impl PopupMenu {
    pub fn show_menu(&mut self, menu_type: String, _x: f64, _y: f64) {
        self.current_menu = QString::from(menu_type.clone());

        let items = match menu_type.as_str() {
            "hamburger" => self.get_hamburger_items_data(),
            "tab" => self.get_tab_items(),
            "playlist" => self.get_playlist_items(),
            _ => vec![],
        };

        let mut qlist = QVariantList::default();
        for (text, enabled, action) in items {
            let mut map = QVariantMap::default();
            map.insert("text".into(), QString::from(text).into());
            map.insert("enabled".into(), enabled.into());
            map.insert("action".into(), QString::from(action).into());
            qlist.push(map.into());
        }

        self.menu_items = qlist;
        self.current_menu_changed();
        self.menu_items_changed();
    }

    // --- DATA GENERATORS ---

    fn get_hamburger_items_data(&self) -> Vec<(String, bool, String)> {
        vec![
            ("Preferences".into(), true, "SUBMENU_SETTINGS".into()),
            ("Donate".into(), true, "donate".into()),
            ("About".into(), true, "about".into()),
        ]
    }

    fn get_tab_items(&self) -> Vec<(String, bool, String)> {
        vec![
            ("Open Folder".into(), true, "open_folder".into()),
            ("Remove Tab".into(), true, "remove_tab".into()),
            ("---".into(), false, "".into()),
            ("Refresh".into(), true, "refresh".into()),
        ]
    }

    fn get_playlist_items(&self) -> Vec<(String, bool, String)> {
        vec![
            ("Play".into(), true, "play".into()),
            ("Add to Queue".into(), true, "add_queue".into()),
            ("---".into(), false, "".into()),
            ("Show in Files".into(), true, "show_files".into()),
            ("---".into(), false, "".into()),
            ("Delete".into(), false, "delete".into()),
        ]
    }

    // --- THEME LOGIC ---

    pub fn get_theme_items(&self) -> QVariantList {
        let mut qlist = QVariantList::default();
        let themes = ThemeManager::available_themes();

        for name in themes {
            let is_active = name == self.active_theme;
            let label = if is_active {
                format!("󰮺 {}", name)
            } else {
                format!("  {}", name)
            };

            let mut map = QVariantMap::default();
            map.insert("text".into(), QString::from(label).into());
            map.insert("enabled".into(), true.into());
            map.insert("action".into(), QString::from(name).into());
            qlist.push(map.into());
        }
        qlist
    }

    pub fn get_settings_items(&self) -> QVariantList {
        let mut qlist = QVariantList::default();
        let mut map = QVariantMap::default();
        map.insert("text".into(), QString::from("DSP ").into());
        map.insert("enabled".into(), true.into());
        map.insert("action".into(), QString::from("toggle_dsp").into());
        qlist.push(map.into());
        qlist
    }

    pub fn trigger_action(&mut self, action: String) {
        let themes = ThemeManager::available_themes();

        if themes.contains(&action) {
            self.active_theme = action.clone();
            self.menu_items_changed();
        }

        self.selected_action = QString::from(action);
        self.action_triggered();
    }

    // --- UTILITIES ---

    pub fn hide_menu(&mut self) {
        self.current_menu = QString::default();
        self.menu_items = QVariantList::default();
        self.current_menu_changed();
        self.menu_items_changed();
    }

    pub fn clear_items(&mut self) {
        self.menu_items = QVariantList::default();
        self.menu_items_changed();
    }
}

