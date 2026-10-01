---
name: ui
description: Komponen UI: core, tab, theme, scroll, playlist.
---
## ui/core.md

```
lokasi: src/ui/core.rs

ROLE
core.rs adalah Qt bridge layer.
Dia adalah pintu masuk QML ke Rust.

core.rs bukan logic utama.
core.rs bukan DSP.
core.rs bukan engine.
core.rs bukan tempat konversi rumit.

core.rs hanya wrapper + signal dispatcher.

============================================================
TANGGUNG JAWAB

core.rs boleh:

- expose Q_PROPERTY
- expose qt_method
- emit qt_signal
- panggil fungsi di src/core/
- panggil fungsi di src/ui/dsp.rs (wrapper dsp)
- forward nilai dari QML ke layer logic
- forward hasil logic ke QML

core.rs tidak boleh:

- punya logika DSP
- punya atomic
- punya Arc audio state
- clamp parameter DSP
- konversi dB berat
- hitung preset logic
- load/save config langsung
- akses file system langsung
- proses audio buffer

============================================================
STRUKTUR YANG BENAR

core.rs harus terlihat seperti ini:

pub struct MusicModel {
    // Q_PROPERTY fields
}

impl MusicModel {

    // QML wrapper method
    pub fn toggle_compressor(&mut self) {
        self.app.toggle_compressor();
        self.compressor_active_changed();
    }

}

Artinya:
core.rs hanya memanggil app/core logic layer.

Tidak boleh ada:

crate::audio::dsp::compressor::get_compressor_threshold_arc().store(...)

Kalau ada ini di core.rs → arsitektur salah.

============================================================
RULE: SINGLE RESPONSIBILITY

core.rs = UI adapter.

Dia tidak boleh tahu:
- bagaimana compressor bekerja
- bagaimana rack menyimpan atomic
- bagaimana engine memproses audio

Dia hanya tahu:
- ada method toggle_compressor()
- ada property compressor_threshold

============================================================
Q_PROPERTY RULE

Setiap property harus:

1. Punya storage di struct
2. Punya NOTIFY signal
3. Emit signal ketika berubah

Contoh benar:

self.compressor_threshold = val;
self.compressor_threshold_changed();

Contoh salah:

update atomic tapi tidak update property.

============================================================
WRAPPER STYLE RULE

Semua method di core.rs harus:

Tipis.
Maksimal 3–6 baris.

Kalau satu method lebih dari 15 baris,
itu bukan wrapper lagi.

============================================================
RESET RULE

Reset di core.rs:

- Panggil logic reset di layer lain
- Update property
- Emit signal

Tidak boleh:
- tulis ulang nilai DSP langsung
- clamp manual tanpa logic layer

============================================================
ANTI-PATTERN YANG HARUS DIHINDARI

1. core.rs punya preset logic besar
2. core.rs baca file config
3. core.rs simpan Arc<AtomicF32>
4. core.rs konversi UI 0–1 ke dB besar
5. core.rs tahu default snapshot detail

Kalau ada ini → refactor ke src/core/.

============================================================
DEPENDENCY DIRECTION

QML
  ↓
core.rs (ui bridge)
  ↓
src/core/*
  ↓
audio engine
  ↓
dsp

Tidak boleh ada panah balik dari dsp ke core.rs.

============================================================
NAMING RULE

File: core.rs
Semua method lowercase.
Tidak pakai underscore untuk nama file baru.
Tidak duplikasi nama mirip seperti:
corebridge.rs
coreui.rs
modelcore.rs

Semua wrapper tetap di core.rs.

============================================================
KESIMPULAN

core.rs harus:

- Tipis
- Bersih
- Tidak pintar
- Hanya jembatan

Kalau core.rs mulai “pintar”,
itu tanda arsitektur mulai bocor.

END
```

---

# Tab UI Components

## 1. TabMusic.qml - Default Music Tab

- **Purpose**: Fixed tab for ~/Music folder (cannot be deleted/renamed)
- **Width**: Fixed at 60px
- **Text**: Always "MUSIC"
- **Left-click**: Calls `musicModel.scan_music()` (loads default Music folder)
- **Right-click**: Does nothing (prevents deletion)
- **Active state**: When `current_folder_qml` is empty or "MUSIC"

## 2. TabCustom.qml - Custom Folder Template

- **Purpose**: Reusable template for user-added folders
- **Width**: Dynamic (based on folder name width, max 100px)
- **Text**: Shows custom folder name
- **Left-click**: Switches to that folder via `musicModel.switch_to_folder()`
- **Right-click**: Shows context menu (change folder/lock/remove/close)
- **Active state**: When current folder matches this tab's name

## 3. Tab.qml - Main Tab Bar Container

- **Structure**:
  1. `TabMusic` component (leftmost, static)
  2. `Row` with `Repeater` that creates `TabCustom` for each custom folder
  3. Add button (`+`) on the right

## How It Works Together

```
[ QUEUE ] [ FAV ] [ MUSIC ] [ rock ] [ Jazz ] [ classical ] [ + ]
    ↑        ↑        ↑        ↑        ↑          ↑
  Queue  Favorite  TabMusic TabCustom TabCustom TabCustom  Add Button
(static) (static)  (static)  (dynamic) (dynamic) (dynamic)
```

- **TabMusic**: Always visible, cannot be removed
- **TabCustom**: Created dynamically based on `musicModel.custom_folder_count`
- **Add button**: Opens folder picker to add new `TabCustom`

## Architecture

The architecture separates concerns:

- `TabMusic` = default folder behavior
- `TabCustom` = reusable custom folder template
- `Tab.qml` = container that combines both

This makes it easier to maintain and modify tab behavior without touching the main Tab.qml file.

## Resource Registration

Both `TabMusic.qml` and `TabCustom.qml` must be registered in two places:

### 1. `src/main.rs` - qrc! macro (for Rust binary)

```rust
qmetaobject::qrc!(init_resources_v4,
    "/" {
        "qml/Ui.qml",
        "qml/ui/Tab.qml",
        "qml/ui/TabMusic.qml",
        "qml/ui/TabCustom.qml",
        // ... other files
    }
);
```

### 2. `qml/ui/qmldir` - for QML module

```
module ui
Tab 1.0 Tab.qml
TabMusic 1.0 TabMusic.qml
TabCustom 1.0 TabCustom.qml
// ... other components
```

## File Structure

```
qml/
├── Ui.qml
└── ui/
    ├── Tab.qml          # Main tab bar container
    ├── TabMusic.qml     # Default MUSIC tab component
    ├── TabCustom.qml    # Custom folder tab template
    ├── qmldir           # QML module definition
    └── ...              # Other UI components
```

---

## ui/theme.md

```
ini logic nya :
app di install : Loonix langsung jadi tema utama.
saat app di instal pasti dibuatkan theme.json, langsung isi dengan 8 tema built-in tapi jangan masukkan array warna, cukup masukkan nama dan status (status sedang aktif di gunakan atau tidak)
user buka PrefThemeEditor, user ubah nama, klik slot mana yang mau di ganti 1,2 atay 3. lalu klik save, lalu kirim semua ke theme.json dan aktifkan tema yang user baru edit.
saat aplikasi di tutup dan dibuka lagi, tinggal cari saja dari 11 tema mana yang memiliki status aktif.
kalau user klik toggle theme di Ui.qml atau melalui PrefAppearance.qml maka hanya kirim status aja menjadi akrif sekaligua meatikan theme lain yang memiliki status aktif sebelumnya.
```

---

## ui/scroll.md

```
📜 LOONIX-TUNES SCROLLBAR STANDARD OPERATING PROCEDURE

Dokumen ini berisi standar teknis dan visual untuk implementasi ScrollBar di seluruh halaman Loonix-Tunes.

## 1. THE GOLDEN RULES (PRINSIP UTAMA)
VERTICAL ONLY: DILARANG keras menggunakan ScrollBar.horizontal. 
Jika konten meluber ke samping, perbaiki Responsive Layout-nya (gunakan elide: Text.ElideRight atau Layout.fillWidth), jangan tambahkan scrollbar horizontal.

INSIDE SCOPE: ScrollBar.vertical WAJIB didefinisikan di DALAM kurung kurawal { } milik komponen parent (seperti ListView, Flickable, atau GridView).

DYNAMIC SIZING: Batang gulir (handle) harus memiliki tinggi yang dinamis sesuai dengan rasio konten yang terlihat.

## 2. HIGH-END SCROLLBAR TEMPLATE
Gunakan template kode di bawah ini untuk setiap halaman baru atau perbaikan:
QMLScrollBar.vertical:
```
ScrollBar {
    id: vBar
    width: 6
    policy: ScrollBar.AsNeeded
    
    // Background dibuat transparan agar tidak mengganggu visual list
    background: Rectangle { 
        color: "transparent" 
    }

    contentItem: Rectangle {
        implicitWidth: 6
        // Menghitung tinggi dinamis berdasarkan rasio konten vs tinggi view
        implicitHeight: Math.max(30, parent.height * parent.visibleArea.heightRatio)
        radius: 3
        
        // Interaktivitas Warna: Accent saat normal/klik, Hover saat disentuh
        color: vBar.pressed ? theme.colormap.playeraccent : 
               vBar.hovered ? theme.colormap.headerhover : 
               theme.colormap.playeraccent
        
        // Redup saat idle (0.5), Terang saat aktif/sedang scroll (1.0)
        opacity: vBar.active ? 1.0 : 0.5
        
        // Animasi transisi halus
        Behavior on color { ColorAnimation { duration: 150 } }
        Behavior on opacity { NumberAnimation { duration: 150 } }
    }
}
```

## 3. CHECKLIST IMPLEMENTASI
Saat membuat halaman baru, pastikan poin-poin ini terpenuhi:[ ] ID Consistency: Gunakan id: vBar jika hanya ada satu list di file tersebut. 
Jika ada lebih, gunakan prefix unik (misal: prefVBar).[ ] Parent Reference: Pastikan parent.height merujuk langsung ke ListView atau Flickable yang menaunginya.[ ] Z-Order: Jika scrollbar tertutup elemen lain, tambahkan z: 1 di dalam blok ScrollBar.[ ] No Overlap: Jika scrollbar menutupi teks, tambahkan rightMargin pada elemen anak list tersebut.

## 4. TROUBLESHOOTING (PEMECAHAN MASALAH)
MasalahPenyebab UmumSolusiScrollBar Gak MunculDitaruh di luar kurung kurawal ListView.Pindahkan kode ke dalam blok ListView/Flickable.
Batang (Handle) 0pxGagal membaca heightRatio.
Pastikan parent memiliki height yang jelas dan clip: true.ScrollBar Horizontal MunculcontentWidth lebih besar dari width.
Hapus ScrollBar.horizontal dan set width anak-anaknya ke parent.width.Update Nama/Data LagQML Binding tidak reaktif.
Gunakan trik refreshTicker dan Connections.

## 5. REAKTIVITAS (REFRESH TICKER)
Jika list data berubah di Rust tapi ScrollBar tidak memperbarui posisinya, gunakan mekanisme ini:
QML// Di root element
```
property int refreshTicker: 0

Connections {
    target: musicModel
    function onData_changed() { refreshTicker++ }
}
```
```
// Di binding scrollbar
implicitHeight: (refreshTicker, Math.max(30, parent.height * parent.visibleArea.heightRatio))
```
```

---

# Playlist Component

Playlist adalah komponen utama untuk menampilkan dan mengelola list lagu di aplikasi LoonixTunes.

## 1. Arsitektur UI

### 1.1 File Structure
```
qml/ui/playlist/Playlist.qml
```

### 1.2 Layout Dasar
- **Container**: `Rectangle` dengan `ListView` di dalamnya
- **Padding**: 
  - `anchors.leftMargin: 16` (ListView total margin)
  - `anchors.rightMargin: 8`
  - `anchors.topMargin: 8`
  - `anchors.bottomMargin: 16`
- **ScrollBar**: Vertikal, otomatis muncul kalo perlu

### 1.3 Delegate Component
Setiap item di-render menggunakan delegate dengan struktur:
```qml
delegate: Component {
    Rectangle {
        width: ListView.view.width
        height: 26
        color: 'transparent'
        
        // Properti
        property bool isPlayingNow     // Lagu yang sedang putar
        property bool isHovered        // Hover state
        
        // Indentasi: 15px kalau ada parent_folder, 0 kalau root
        property int folderIndent: (model.parent_folder !== undefined && model.parent_folder !== "") ? 15 : 0
        
        // Left Border: Garis vertikal 1px untuk menunjukkan hierarchy folder
        Rectangle {
            anchors.left: parent.left
            anchors.leftMargin: folderIndent
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: 1
            color: theme.colormap.graysolid
            visible: folderIndent > 0
        }
    }
}
```

## 2. Indentasi & Hierarchy

### 2.1 Logika Indentasi
- **Root items** (`parent_folder = ""` atau undefined): `folderIndent = 0`, tidak ada border
- **Subfolder items** (ada `parent_folder`): `folderIndent = 15`, ada left border 1px

### 2.2 Implementasi
```qml
property int folderIndent: (model.parent_folder !== undefined && model.parent_folder !== "") ? 15 : 0
```
- `folderIndent` dihitung dari `model.parent_folder`
- Indentasi diterapkan ke `playlistIcon.leftMargin`

### 2.3 Left Border (Garis Vertikal)
- Width: 1px
- Color: `theme.colormap.graysolid`
- Visible: hanya saat `folderIndent > 0`
- Anchors: dari atas ke bawah (`top` ke `bottom`)

## 3. Item Display

### 3.1 Ikon
- **Position**: `anchors.left: parent.left` + `folderIndent`
- **Font**: `symbols.name`
- **Size**: 
  - Folder: 20px
  - File: 14px
- **Symbols**:
  - Playing: `󰶻`
  - Folder: `󱍙`
  - File: `󰽷`
- **Padding**: `leftPadding: 6`

### 3.2 Text (Nama Lagu)
- **Position**: setelah ikon
- **Padding**:
  - Setelah ikon: `leftPadding: 6`
  - Kanan: `anchors.rightMargin: 6`
- **Font**: `kodeMono.name`
- **Size**:
  - Folder: 14px
  - File: 13px
- **Elide**: `Text.ElideRight` (nama dipotong kalo kepanjangan)

### 3.3 Warna
- **Active/Hover/RightClicked**: `theme.colormap.playlistactive`
- **Folder**: `theme.colormap.playlistfolder`
- **Normal**: `theme.colormap.playlisttext`

## 4. Interaksi (Click Logic)

### 4.1 Single Click (onClicked)
```qml
onClicked: function(mouse) {
    if (mouse.button === Qt.LeftButton) {
        if (model.is_folder) {
            musicModel.toggle_folder(index)  // Expand/Collapse folder
        } else {
            musicModel.play_at(index)     // Main lagu
        }
    } else if (mouse.button === Qt.RightButton) {
        // Show context menu
    }
}
```

### 4.2 Double Click (onDoubleClicked)
```qml
onDoubleClicked: function(mouse) {
    if (mouse.button === Qt.LeftButton && model.is_folder) {
        musicModel.switch_to_folder(model.path)
        root.playlistSource = "qrc:/qml/ui/playlist/Playlist.qml"
    }
}
```

### 4.3 Press & Hold (onPressAndHold)
- Show popup menu di posisi click

## 5. Right Click Menu (PlaylistContextMenu)

### 5.1 Trigger
Right click di item playlist memanggil context menu dengan posisi:
```qml
root.playlistContextMenuVisible = true
root.playlistContextMenuX = ...
root.playlistContextMenuY = ...
```

### 5.2 Data yang Passed ke Menu
| Property | Keterangan |
|----------|----------|
| playlistContextItemIndex | Index item yang diklik |
| playlistContextItemName | Nama item |
| playlistContextItemPath | Path item |
| playlistContextIsFolder | Apakah folder? |

### 5.3 Menu Layout
- **3x3 Grid Layout** (9 tiles)
- **Tile Size**: 50x50px each
- **Spacing**: 2px

### 5.4 Menu Tiles

| Tile | Icon | Label | Action |
|------|------|-------|--------|
| 1 | 󰷞 | File | `musicFilePicker.open()` - Add file |
| 2 | 󰉗 | Folder | `musicFolderPicker.open()` - Add folder |
| 3 |  | Remove | `musicModel.remove_song(index)` |
| 4 | 󰗨 | Del | `deleteConfirmDialog.open()` - Delete permanen |
| 5 | 󱕱 | Queue | `musicModel.add_to_queue(path, name)` |
| 6 | 󰓎 | Fav | `musicModel.toggle_favorite(path, name)` |
| 7 | 󰋼 | Info | `musicModel.load_track_info(path)` |
| 8 | 󰒓 | Pref | Open preferences |
| 9 |  | Close | Close menu |

### 5.5 FileDialogs dalam Menu

#### musicFilePicker
```qml
FileDialog {
    id: musicFilePicker
    title: "Select Music File"
    fileMode: FileDialog.OpenFiles
    nameFilters: ["Audio files (*.mp3 *.wav *.flac *.ogg *.m4a *.aac)"]
}
```

#### musicFolderPicker
```qml
FolderDialog {
    id: musicFolderPicker
    title: "Select Music Folder"
}
```

### 5.6 Delete Confirmation
```qml
MessageDialog {
    id: deleteConfirmDialog
    title: "Delete Permanent"
    text: "This will permanently delete..."
    buttons: MessageDialog.Yes | MessageDialog.No
}
```

### 5.7 Menu Visibility Logic
- Visible: `root.playlistContextMenuVisible`
- Position: `root.playlistContextMenuX`, `root.playlistContextMenuY`
- Auto-close kalo klik di luar menu

## 6. Toggle Folder (Expand/Collapse)

### 6.1 Fungsi
- **Method**: `toggle_folder(index)` di QML → `musicModel.toggle_folder(index)` di Rust
- **Logic**:
  - Kalo folder belum di-expand → expand (masukkan semua item dalam folder ke display_list)
  - Kalo folder sudah di-expand → collapse (hapus semua item dalam folder dari display_list)

### 6.2 Expanded Folders Tracking
- Disimpan di `expanded_folders: HashSet<String>` di Rust
- Setiap item dalam folder punya `parent_folder` field yang ngarah ke folder parent-nya

### 6.3 UI Update
- Pakai `begin_reset_model()` dan `end_reset_model()` untuk force QML redraw

## 7. Shuffle Logic

### 7.1 Aturan Shuffle per Tab

#### Tab MUSIC (`current_tab_root = ""`)
1. **Tidak ada folder di-expand**:
   - Shuffle hanya lagu di ROOT (`parent_folder = ""` atau `parent_folder = None`)
   - TIDAK masuk ke dalam folder
   
2. **Ada folder di-expand**:
   - Shuffle hanya di folder yang sedang di-expand
   - TIDAK lompat ke folder lain atau root

#### Tab CUSTOM (`current_tab_root = path`)
- Shuffle hanya dalam folder tsb
- TIDAK lompat keluar folder

#### Tab LAIN (Favorites, Queue, External Files)
- Shuffle di semua item dalam list tsb

### 7.2 Implementasi di Backend
Dilakukan di `toggle_shuffle()` function di `src/ui/bridge/core.rs`:
```rust
let folder_scope: Option<String> = {
    let tab_root = self.current_tab_root.to_string();
    if !tab_root.is_empty() {
        // Tab Custom: stay dalam folder tsb
        Some(tab_root)
    } else if self.expanded_folders.is_empty() {
        // Tab Music + TIDAK ada folder di-expand: shuffle cm lagu di root
        Some(String::new())
    } else {
        // Tab Music + ada folder yang di-expand
        self.expanded_folders.iter().next().cloned()
    }
};
```

### 7.3 Folder Scope Logic
- `"": String::new()` = Root only (parent_folder = "" or None)
- `"path/to/folder"` = Specific folder
- `None` = All (untuk Tab lain)

### 7.4 Shuffle Queue
- Diproses di `PlaybackController` (`src/core/services/playback.rs`)
- Fisher-Yates shuffle algorithm
- Current song selalu di depan queue

## 8. Model Data (MusicModel)

### 8.1 Fields yang Dipake
| Field | Type | Keterangan |
|-------|------|----------|
| name | String | Nama file/folder |
| path | String | Full path |
| is_folder | bool | Apakah folder? |
| parent_folder | String | Path folder parent (kosong kalau root) |

### 8.2 Display List
- `display_list: Vec<MusicItem>` - list yang sedang di-display
- Di-update kalo toggle folder atau switch tab

## 9. Padding & Margin Summary

| Element | Left | Right | Top | Bottom |
|---------|------|-------|-----|-------|
| ListView | 16 | 8 | 8 | 16 |
| Item Icon | folderIndent + 6 | - | - | - |
| Item Text | setelah icon + 6 | 6 | - | - |
| Item Delegate | folderIndent | 6 | 0 | 0 |
| Left Border | folderIndent | - | 0 | 0 |

## 10. Empty State

Kalo `playlistView.count === 0`, tampilkan:
- Text: "No music found"
- Font: kodeMono, 14px
- Color: theme.colormap.playersubtext

## 11. Auto-Scroll

Kalo lagu yang lagi main berubah, auto-scroll ke posisi lago tsb:
```qml
Component.onCompleted: {
    musicModel.current_index_changed.connect(function() {
        if (musicModel.current_index >= 0) {
            playlistView.positionViewAtIndex(musicModel.current_index, ListView.Center)
        }
    })
}
```
