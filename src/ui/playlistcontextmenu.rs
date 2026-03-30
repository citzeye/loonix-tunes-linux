/* --- LOONIX-TUNES src/ui/playlistcontextmenu.rs --- */
use qmetaobject::*;

#[derive(QObject, Default)]
pub struct PlaylistContextMenu {
    base: qt_base_class!(trait QObject),

    pub is_visible: qt_property!(bool; NOTIFY menu_changed),
    pub menu_x: qt_property!(i32; NOTIFY menu_changed),
    pub menu_y: qt_property!(i32; NOTIFY menu_changed),
    pub item_index: qt_property!(i32; NOTIFY menu_changed),
    pub item_name: qt_property!(QString; NOTIFY menu_changed),
    pub item_path: qt_property!(QString; NOTIFY menu_changed),
    pub is_folder: qt_property!(bool; NOTIFY menu_changed),
    pub is_empty: qt_property!(bool; NOTIFY menu_changed),
    pub menu_changed: qt_signal!(),

    pub trigger: qt_method!(
        fn(&mut self, idx: i32, name: QString, path: QString, folder: bool, x: i32, y: i32)
    ),

    pub trigger_empty: qt_method!(fn(&mut self, x: i32, y: i32)),

    pub hide_menu: qt_method!(fn(&mut self)),
}

impl PlaylistContextMenu {
    pub fn new() -> Self {
        Self::default()
    }

    fn clamp_pos(x: i32, y: i32) -> (i32, i32) {
        let mut cx = if x < 0 { 0 } else { x };
        let mut cy = if y < 0 { 0 } else { y };
        // Clamp to reasonable screen bounds
        if cx > 2000 {
            cx = 2000;
        }
        if cy > 1200 {
            cy = 1200;
        }
        (cx, cy)
    }

    pub fn trigger(
        &mut self,
        idx: i32,
        name: QString,
        path: QString,
        folder: bool,
        x: i32,
        y: i32,
    ) {
        let (cx, cy) = Self::clamp_pos(x, y);
        self.item_index = idx;
        self.item_name = name;
        self.item_path = path;
        self.is_folder = folder;
        self.is_empty = false;
        self.menu_x = cx;
        self.menu_y = cy;
        self.is_visible = true;
        self.menu_changed();
    }

    pub fn trigger_empty(&mut self, x: i32, y: i32) {
        let (cx, cy) = Self::clamp_pos(x, y);
        self.item_index = -1;
        self.item_name = QString::default();
        self.item_path = QString::default();
        self.is_folder = false;
        self.is_empty = true;
        self.menu_x = cx;
        self.menu_y = cy;
        self.is_visible = true;
        self.menu_changed();
    }

    pub fn hide_menu(&mut self) {
        self.is_visible = false;
        self.menu_changed();
    }
}
