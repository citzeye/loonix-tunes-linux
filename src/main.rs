/* --- LOONIX-TUNES src/main.rs --- */
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
    // Memastikan PipeWire atau backend audio terdeteksi jika diperlukan
    std::env::set_var("RUST_LOG", "info");
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            println!("[System] Wayland terdeteksi! Memaksa Qt menggunakan XCB untuk kompatibilitas VST3.");
            std::env::set_var("QT_QPA_PLATFORM", "xcb");
        }
    }

    // Install panic hook to print backtrace
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic: {:?}", panic_info);
        // Print backtrace if available
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
    let mut engine = QmlEngine::new();

    // Registrasi Type untuk QML
    qml_register_type::<MusicModel>(cstr!("Loonix"), 1, 0, cstr!("MusicModel"));
    qml_register_type::<PopupMenu>(cstr!("Loonix"), 1, 0, cstr!("PopupMenu"));
    qml_register_type::<ThemeManager>(cstr!("Loonix"), 1, 0, cstr!("ThemeManager"));

    // Inisialisasi Model dengan Engine Audio baru
    let boxed_model = QObjectBox::new(MusicModel::new());
    engine.set_object_property("musicModel".into(), boxed_model.pinned());

    let boxed_theme = QObjectBox::new(ThemeManager::new());
    engine.set_object_property("theme".into(), boxed_theme.pinned());

    let boxed_popup = QObjectBox::new(PopupMenu::default());
    engine.set_object_property("popupMenu".into(), boxed_popup.pinned());

    // Player Bridge untuk UI communications
    let boxed_bridge = QObjectBox::new(PlayerBridge::new());
    engine.set_object_property("playerBridge".into(), boxed_bridge.pinned());

    // Check for command line arguments (file paths)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // Store command line files for later processing
        let files: Vec<String> = args[1..].to_vec();
        crate::ui::core::set_command_line_files(files);
        println!(
            "Stored {} file(s) from command line for processing",
            args.len() - 1
        );
    }

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
        "qml/ui/Eq.qml",
        "qml/ui/Fx.qml",
        "qml/ui/TrackInfo.qml",
        "qml/ui/LoonixDrawer.qml",
        "qml/ui/HeaderMenu.qml",
        "qml/ui/TabContextMenu.qml",
        "qml/ui/PlaylistContextMenu.qml",
        "qml/ui/Playlist.qml",
        "qml/ui/Preferences.qml",
        "qml/ui/preferences/About.qml",
        "qml/ui/preferences/Donate.qml",
        "qml/ui/preferences/Hardware.qml",
        "qml/ui/preferences/Audio.qml",
        "qml/ui/preferences/Library.qml",
        "qml/ui/preferences/Appearance.qml",
        "qml/ui/preferences/SettingTab.qml",
        "qml/ui/preferences/SettingSwitch.qml",
        "qml/ui/preferences/SettingHeader.qml",
        "qml/ui/preferences/SettingDropdown.qml",
        "qml/ui/preferences/SettingSlider.qml",
        "qml/ui/preferences/CollapsibleSection.qml",
        "qml/ui/preferences/SettingFooter.qml",
        "qml/ui/preferences/SettingButton.qml",
        "qml/ui/VstControl.qml",
        "qml/ui/vst/VstHead.qml",
        "qml/ui/vst/VstLeft.qml",
        "qml/ui/vst/VstRight.qml",
        "qml/ui/vst/VstFoot.qml",
        "qml/ui/qmldir",
        "assets/LoonixTunes.png",
        "assets/fonts/KodeMono-VariableFont_wght.ttf",
        "assets/fonts/SymbolsNerdFont-Regular.ttf",
        "assets/fonts/twemoji.ttf",
        "assets/fonts/Oswald-Regular.ttf",
    }
);
/* --- END --- */
