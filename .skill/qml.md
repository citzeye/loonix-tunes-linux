.
├── qml.qrc
└── ui/
    ├── components/          # 🧩 Komponen Reusable (Bisa dipake di mana aja)
    │   ├── RenameDialog.qml
    │   ├── ThemeSlider.qml
    │   └── TrackInfo.qml
    │
    ├── contextmenu/         # 🖱️ Menu Klik Kanan (Hapus duplikat yang di luar!)
    │   ├── AppearanceContextMenu.qml
    │   ├── PlaylistContextMenu.qml
    │   └── TabContextMenu.qml
    │
    ├── pref/                # ⚙️ Jeroan Menu Preferences (Udah perfect ini)
    │   ├── PrefAbout.qml
    │   ├── PrefAppearance.qml
    │   ├── PrefButton.qml
    │   ├── PrefCollapsibleSection.qml
    │   ├── PrefDonate.qml
    │   ├── PrefDropdown.qml
    │   ├── PrefLibrary.qml
    │   ├── PrefReportBug.qml
    │   ├── PrefSlider.qml
    │   ├── PrefSwitch.qml
    │   ├── PrefTab.qml
    │   └── PrefThemeEditor.qml
    │
    ├── tabs/                # 📑 Konten tiap Tab di Main UI
    │   ├── Tab.qml          # (Base component buat tab)
    │   ├── TabCustom.qml
    │   ├── TabFavorites.qml
    │   ├── TabMusic.qml
    │   └── TabQueue.qml
    │
    ├── qmldir               # Definisi module QML (biar gampang di-import)
    │
    ├── Dsp.qml              # 🪟 MAIN VIEW: Jendela DSP
    ├── Playlist.qml         # 🪟 MAIN VIEW: Jendela Playlist
    ├── Pref.qml             # 🪟 MAIN VIEW: Jendela Preferences
    └── Ui.qml               # 🪟 MAIN WINDOW: Jendela Utama App lu