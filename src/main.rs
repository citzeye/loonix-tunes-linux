/* --- loonixtunesv2/src/main.rs | main --- */
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
use cstr::cstr;
use qmetaobject::*;

pub mod audio;
pub mod core;
pub mod ui;

use crate::ui::bridge::MusicModel;
use crate::ui::bridge::DspController;
use crate::ui::bridge::PlayerBridge;
use crate::ui::components::ThemeManager;
use crate::ui::CustomThemeListModel;
use crate::ui::PopupMenu;
use crate::ui::reportbug::BugReportManager;
#[cfg(target_os = "linux")]
use crate::core::services::SysMediaManager;

struct App {
    music_model: QObjectBox<MusicModel>,
    dsp_model: QObjectBox<DspController>,
    theme: QObjectBox<ThemeManager>,
    custom_theme_list: QObjectBox<CustomThemeListModel>,
    popup: QObjectBox<PopupMenu>,
    bridge: QObjectBox<PlayerBridge>,
    bug_report: QObjectBox<BugReportManager>,
    #[cfg(target_os = "linux")]
    sysmedia: QObjectBox<SysMediaManager>,
}

impl App {
    fn new() -> Self {
        let music_raw = MusicModel::new();
        let ffmpeg = music_raw.get_ffmpeg();
        let config = music_raw.get_shared_config();

        let music_model = QObjectBox::new(music_raw);
        let dsp_model = QObjectBox::new(DspController::new(ffmpeg));

        // Initialize DSP config (load dsp.json or create fresh)
        if let Some(ref shared_config) = config {
            if let Ok(cfg) = shared_config.lock() {
                dsp_model.pinned().borrow_mut().init_from_config(&cfg);
            }
        }

        let theme = QObjectBox::new(ThemeManager::new());

        if let Some(shared_config) = config {
            theme.pinned().borrow_mut().set_config(shared_config);
        }

        let custom_list = CustomThemeListModel::default();
        let custom_theme_list = QObjectBox::new(custom_list);

        crate::audio::config::AppConfig::set_initializing(false);

        Self {
            music_model,
            dsp_model,
            theme,
            custom_theme_list,
            popup: QObjectBox::new(PopupMenu::default()),
            bridge: QObjectBox::new(PlayerBridge::new()),
            bug_report: QObjectBox::new(BugReportManager::default()),
            #[cfg(target_os = "linux")]
            sysmedia: QObjectBox::new(SysMediaManager::new()),
        }
    }
}

fn setup_env() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Fusion");
}

fn main() {
    if cfg!(debug_assertions) {
        println!("🛠️ LOONIX-TUNES [DEVELOPER MODE]");
    } else {
        println!("🚀 LOONIX-TUNES [RELEASE MODE]");
    }

    #[cfg(target_os = "linux")]
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            std::env::set_var("QT_QPA_PLATFORM", "xcb");
        }
    }

    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic: {:?}", panic_info);
        eprintln!("Backtrace:\n{:?}", std::backtrace::Backtrace::capture());
    }));

    setup_env();
    init_resources_v4();

    #[cfg(target_os = "linux")]
    {
        crate::core::services::wireless::startSystemCheck();
    }

    let app = App::new();
    let mut engine = QmlEngine::new();

    qml_register_type::<MusicModel>(cstr!("Loonix"), 1, 0, cstr!("MusicModel"));
    qml_register_type::<DspController>(cstr!("Loonix"), 1, 0, cstr!("DspController"));
    qml_register_type::<PopupMenu>(cstr!("Loonix"), 1, 0, cstr!("PopupMenu"));
    qml_register_type::<ThemeManager>(cstr!("Loonix"), 1, 0, cstr!("ThemeManager"));
    #[cfg(target_os = "linux")]
    qml_register_type::<SysMediaManager>(cstr!("Loonix"), 1, 0, cstr!("SysMediaManager"));
    qml_register_type::<CustomThemeListModel>(cstr!("Loonix"), 1, 0, cstr!("CustomThemeListModel"));
    qml_register_type::<BugReportManager>(cstr!("Loonix"), 1, 0, cstr!("BugReportManager"));
    qml_register_type::<PlayerBridge>(cstr!("Loonix"), 1, 0, cstr!("PlayerBridge"));

    engine.set_object_property("musicModel".into(), app.music_model.pinned());
    engine.set_object_property("dspModel".into(), app.dsp_model.pinned());
    engine.set_object_property("theme".into(), app.theme.pinned());
    engine.set_object_property("customThemeList".into(), app.custom_theme_list.pinned());
    engine.set_object_property("popupMenu".into(), app.popup.pinned());
    engine.set_object_property("playerBridge".into(), app.bridge.pinned());
    #[cfg(target_os = "linux")]
    engine.set_object_property("sysMedia".into(), app.sysmedia.pinned());
    engine.set_object_property("bugReport".into(), app.bug_report.pinned());

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let files: Vec<String> = args[1..].to_vec();
        crate::ui::bridge::core::set_command_line_files(files);
    }

    engine.load_file("qrc:/qml/Ui.qml".into());
    engine.exec();

    #[cfg(target_os = "linux")]
    {
        crate::core::services::wireless::stop_system_check();
    }

    println!("[MAIN] Clean shutdown complete");
}

qmetaobject::qrc!(init_resources_v4,
    "/" {
        "qml/Ui.qml",
        "qml/ui/tabs/Tab.qml",
        "qml/ui/tabs/TabMusic.qml",
        "qml/ui/tabs/TabFavorites.qml",
        "qml/ui/tabs/TabQueue.qml",
        "qml/ui/tabs/TabCustom.qml",
        "qml/ui/Dsp.qml",
        "qml/ui/components/TrackInfo.qml",
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
        "qml/ui/components/ThemeSlider.qml",
        "qml/ui/components/RenameDialog.qml",
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