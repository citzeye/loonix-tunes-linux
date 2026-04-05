/* --- LOONIX-TUNES src/main.rs --- */
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
use cstr::cstr;
use qmetaobject::*;

pub mod audio;
pub mod ui;

#[cfg(target_os = "linux")]
pub mod dbus_service;

use crate::audio::popup::PopupMenu;
use crate::ui::core::MusicModel;
use crate::ui::playerbridge::PlayerBridge;
use crate::ui::theme::ThemeManager;

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
            println!("[System] Wayland terdeteksi! Memaksa Qt menggunakan XCB.");
            std::env::set_var("QT_QPA_PLATFORM", "xcb");
        }
    }

    // Install panic hook to print backtrace
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic: {:?}", panic_info);
        eprintln!("Backtrace:\n{:?}", std::backtrace::Backtrace::capture());
    }));
    
    setup_env();

    #[cfg(target_os = "linux")]
    {
        if let Err(e) = dbus_service::init_dbus() {
            eprintln!("[DBUS] Failed to setup: {}", e);
        }
    }

    init_resources_v4();

    // ==========================================
    // 2. MEMORY MANAGEMENT: BUAT DATA DULUAN
    // (Wajib di atas Engine biar gak Access Violation pas close app)
    // ==========================================
    let boxed_model = QObjectBox::new(MusicModel::new());
    let boxed_theme = QObjectBox::new(ThemeManager::new());
    let boxed_popup = QObjectBox::new(PopupMenu::default());
    let boxed_bridge = QObjectBox::new(PlayerBridge::new());

    // ==========================================
    // 3. BUAT ENGINE & REGISTRASI
    // (Engine dibuat belakangan, jadi dihancurkan duluan)
    // ==========================================
    let mut engine = QmlEngine::new();

    // Registrasi Type untuk QML
    qml_register_type::<MusicModel>(cstr!("Loonix"), 1, 0, cstr!("MusicModel"));
    qml_register_type::<PopupMenu>(cstr!("Loonix"), 1, 0, cstr!("PopupMenu"));
    qml_register_type::<ThemeManager>(cstr!("Loonix"), 1, 0, cstr!("ThemeManager"));

    // Inisialisasi Model dengan Engine Audio baru
    engine.set_object_property("musicModel".into(), boxed_model.pinned());
    engine.set_object_property("theme".into(), boxed_theme.pinned());
    engine.set_object_property("popupMenu".into(), boxed_popup.pinned());
    engine.set_object_property("playerBridge".into(), boxed_bridge.pinned());

    // Check for command line arguments (file paths)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // Store command line files for later processing
        let files: Vec<String> = args[1..].to_vec();
        crate::ui::core::set_command_line_files(files);
        if cfg!(debug_assertions) {
            println!("Stored {} file(s) from command line for processing", args.len() - 1);
        }
    }

    // ==========================================
    // 4. LOAD UI & EXECUTE
    // ==========================================
    engine.load_file("qrc:/qml/Ui.qml".into());
    engine.exec();
}

qmetaobject::qrc!(init_resources_v4,
    "/" {
        "qml/Ui.qml",
        "qml/ui/Tab.qml",
        "qml/ui/TabMusic.qml",
        "qml/ui/TabFavorites.qml",
        "qml/ui/TabQueue.qml",
        "qml/ui/TabCustom.qml",
        "qml/ui/EqPopup.qml",
        "qml/ui/FxPopup.qml",
        "qml/ui/TrackInfo.qml",
        "qml/ui/TabContextMenu.qml",
        "qml/ui/PlaylistContextMenu.qml",
        "qml/ui/Playlist.qml",
        "qml/ui/Pref.qml",
        "qml/ui/pref/PrefAbout.qml",
        "qml/ui/pref/PrefDonate.qml",
        "qml/ui/pref/PrefHardware.qml",
        "qml/ui/pref/PrefAudio.qml",
        "qml/ui/pref/PrefLibrary.qml",
        "qml/ui/pref/PrefAppearance.qml",
        "qml/ui/pref/PrefTab.qml",
        "qml/ui/pref/PrefSwitch.qml",
        "qml/ui/pref/PrefDropdown.qml",
        "qml/ui/pref/PrefSlider.qml",
        "qml/ui/pref/PrefCollapsibleSection.qml",
        "qml/ui/pref/PrefButton.qml",
        "qml/ui/ThemeSlider.qml",
        "qml/ui/qmldir",
        "assets/qtquickcontrols2.conf",
        "assets/LoonixTunes.png",
        "assets/eqpreset.json",
        "assets/fxpreset.json",
        "assets/fonts/KodeMono-VariableFont_wght.ttf",
        "assets/fonts/SymbolsNerdFont-Regular.ttf",
        "assets/fonts/twemoji.ttf",
        "assets/fonts/Oswald-Regular.ttf",
    }
);
/* --- END --- */