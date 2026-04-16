/* --- LOONIX-TUNES src/main.rs --- */

#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
use cstr::cstr;
use qmetaobject::*;

pub mod audio;
pub mod core;
pub mod ui;

use crate::audio::popup::PopupMenu;
use crate::audio::sysmedia::SysMediaManager;
use crate::ui::core::MusicModel;
use crate::ui::playerbridge::PlayerBridge;
use crate::ui::theme::ThemeManager;

struct App {
    music_model: QObjectBox<MusicModel>,
    theme: QObjectBox<ThemeManager>,
    popup: QObjectBox<PopupMenu>,
    bridge: QObjectBox<PlayerBridge>,
    #[cfg(target_os = "linux")]
    sysmedia: QObjectBox<SysMediaManager>,
}

impl App {
    fn new() -> Self {
        let music_model = QObjectBox::new(MusicModel::new());
        let theme = QObjectBox::new(ThemeManager::new());

        if let Some(shared_config) = music_model.pinned().borrow().get_shared_config() {
            theme.pinned().borrow_mut().set_config(shared_config);
        }

        Self {
            music_model,
            theme,
            popup: QObjectBox::new(PopupMenu::default()),
            bridge: QObjectBox::new(PlayerBridge::new()),
            #[cfg(target_os = "linux")]
            sysmedia: QObjectBox::new(SysMediaManager::new()),
        }
    }
}

fn setup_env() {
    // Basic environment untuk Qt
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Fusion");
}

fn main() {
    // ==========================================
    // 1. SMART ENVIRONMENT DETECTOR
    // ==========================================
    if cfg!(debug_assertions) {
        println!("========================================");
        println!("🛠️ LOONIX-TUNES [DEVELOPER MODE]");
        println!("Menjalankan dari Cargo. Log diaktifkan.");
        println!("========================================");
    } else {
        // Mode ini jalan kalau lo pake 'cargo build --release' (Buat User)
        // Di Windows, terminal hitam bakal hilang berkat windows_subsystem di atas.
        println!("🚀 LOONIX-TUNES [RELEASE MODE]");
    }

    #[cfg(target_os = "linux")]
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            std::env::set_var("QT_QPA_PLATFORM", "xcb");
        }
    }

    // Install panic hook to print backtrace
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic: {:?}", panic_info);
        eprintln!("Backtrace:\n{:?}", std::backtrace::Backtrace::capture());
    }));

    setup_env();

    init_resources_v4();

    // ==========================================
    // 2. BUAT APP - Semua object hidup dalam struct App
    // App struct memastikan lifetime yang benar dan drop order deterministic
    // ==========================================
    #[cfg(target_os = "linux")]
    crate::audio::wireless::startSystemCheck();

    let app = App::new();

    // ==========================================
    // 3. BUAT ENGINE & REGISTRASI
    // ==========================================
    let mut engine = QmlEngine::new();

    // Registrasi Type untuk QML
    qml_register_type::<MusicModel>(cstr!("Loonix"), 1, 0, cstr!("MusicModel"));
    qml_register_type::<PopupMenu>(cstr!("Loonix"), 1, 0, cstr!("PopupMenu"));
    qml_register_type::<ThemeManager>(cstr!("Loonix"), 1, 0, cstr!("ThemeManager"));
    qml_register_type::<SysMediaManager>(cstr!("Loonix"), 1, 0, cstr!("SysMediaManager"));

    // set_object_property() internally calls QQmlEngine::rootContext()->setContextProperty()
    // Yang penting: App struct hidup selama main() scope
    engine.set_object_property("musicModel".into(), app.music_model.pinned());
    engine.set_object_property("theme".into(), app.theme.pinned());
    engine.set_object_property("popupMenu".into(), app.popup.pinned());
    engine.set_object_property("playerBridge".into(), app.bridge.pinned());
    #[cfg(target_os = "linux")]
    engine.set_object_property("sysMedia".into(), app.sysmedia.pinned());

    // Check for command line arguments (file paths)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // Store command line files for later processing
        let files: Vec<String> = args[1..].to_vec();
        crate::ui::core::set_command_line_files(files);
        if cfg!(debug_assertions) {
            println!(
                "Stored {} file(s) from command line for processing",
                args.len() - 1
            );
        }
    }

    // ==========================================
    // 4. LOAD UI & EXECUTE
    // ==========================================
    engine.load_file("qrc:/qml/Ui.qml".into());
    engine.exec();

    // ==========================================
    // 5. EXPLICIT SHUTDOWN - CRITICAL FOR CLEAN EXIT
    // ==========================================

    // Stop system check threads first
    #[cfg(target_os = "linux")]
    {
        crate::audio::wireless::stop_system_check();
    }

    // App struct will drop automatically here, in reverse field order
    // Drop order: sysmedia -> bridge -> popup -> theme -> music_model
    // This triggers all Drop implementations for clean shutdown

    println!("[MAIN] Clean shutdown complete");
}

qmetaobject::qrc!(init_resources_v4,
    "/" {
        "qml/Ui.qml",
        "qml/ui/Tab.qml",
        "qml/ui/TabMusic.qml",
        "qml/ui/TabFavorites.qml",
        "qml/ui/TabQueue.qml",
        "qml/ui/TabCustom.qml",
        "qml/ui/Dsp.qml",
        "qml/ui/TrackInfo.qml",
        "qml/ui/contextmenu/TabContextMenu.qml",
        "qml/ui/contextmenu/PlaylistContextMenu.qml",
        "qml/ui/contextmenu/AppearanceContextMenu.qml",
        "qml/ui/Playlist.qml",
        "qml/ui/Pref.qml",
        "qml/ui/pref/PrefAbout.qml",
        "qml/ui/pref/PrefDonate.qml",
        "qml/ui/pref/PrefAppearance.qml",
        "qml/ui/pref/PrefTab.qml",
        "qml/ui/pref/PrefSwitch.qml",
        "qml/ui/pref/PrefDropdown.qml",
        "qml/ui/pref/PrefSlider.qml",
        "qml/ui/pref/PrefCollapsibleSection.qml",
        "qml/ui/pref/PrefButton.qml",
        "qml/ui/pref/PrefReportBug.qml",
        "qml/ui/pref/PrefThemeEditor.qml",
        "qml/ui/ThemeSlider.qml",
        "qml/ui/RenameDialog.qml",
        "qml/ui/qmldir",
        "assets/qtquickcontrols2.conf",
        "assets/LoonixTunes.png",
        "assets/fonts/KodeMono-VariableFont_wght.ttf",
        "assets/fonts/SymbolsNerdFont-Regular.ttf",
        "assets/fonts/twemoji.ttf",
        "assets/fonts/Oswald-Regular.ttf",
        "assets/images/saweriaqrcode.png",
        "assets/images/kofiqrcode.png",
    }
);
/* --- END --- */
