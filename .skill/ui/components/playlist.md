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