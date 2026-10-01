---
name: pref
description: Sistem preferensi, integrasi matugen, laporan bug, theme editor.
---
# Pref System Architecture

## Folder Structure

```
qml/ui/
├── Pref.qml              # Main dialog container
├── pref/
│   ├── PrefTab.qml       # Sidebar navigation button
│   ├── PrefSwitch.qml    # Toggle switch (label + description)
│   ├── PrefSlider.qml    # Slider with label + value display
│   ├── PrefDropdown.qml  # ComboBox with label + description
│   ├── PrefHardware.qml  # Audio output device settings
│   ├── PrefAudio.qml     # DSP effects settings (PANJANG - scroll)
│   ├── PrefLibrary.qml   # Library scan settings
│   ├── PrefAppearance.qml # Theme selection (PANJANG - scroll)
│   ├── PrefAbout.qml     # About page
│   ├── PrefDonate.qml    # Donate page
│   └── ThemeEditor.qml   # Custom theme editor
```

---

## Hierarchy

```
Pref.qml (Item)
├── MouseArea (block all mouse events to underlying UI - SIBLING of popupContainer)
└── popupContainer (Rectangle)
    └── ColumnLayout
        ├── Header (20px height - "PREFERENCES" title + close button)
        └── RowLayout
            ├── Rectangle (5px, bgmain - left border)
            ├── Rectangle (4px, playeraccent - vertical bar)
            ├── Rectangle (100px, sidebar)
            │   └── ColumnLayout
            │       └── PrefTab { text, icon, isActive, onClicked }
            ├── Rectangle (1px, spacer)
            └── Rectangle (fillWidth, content area)
                └── StackLayout
                    ├── PrefHardware (index 0)
                    ├── PrefAudio (index 1) - USE Flickable + ScrollBar
                    ├── PrefLibrary (index 2) - NO Flickable
                    ├── PrefAppearance (index 3) - USE Flickable + ScrollBar
                    ├── PrefAbout (index 4) - NO Flickable
                    └── PrefDonate (index 5) - NO Flickable
```

---

## Rules

### 1. Block Mouse - Sibling Approach

MouseArea pemblokir harus **SIBLING** dari popupContainer, bukan child:

```qml
// BENAR - MouseArea sebagai sibling
Item {
    MouseArea {
        anchors.fill: parent
        acceptedButtons: Qt.AllButtons
        onClicked: {} // Block all mouse events
    }
    Rectangle {
        id: popupContainer
        // content
    }
}
```

### 2. Flickable vs ScrollView

**Gunakan Flickable** dengan custom ScrollBar untuk konten PANJANG. Konten PENDEK tidak perlu Flickable.

| Page | Panjang? | Pakai Flickable? |
|------|----------|-------------------|
| PrefHardware | Tidak | ❌ |
| PrefAudio | YA | ✅ |
| PrefLibrary | Tidak | ❌ |
| PrefAppearance | YA | ✅ |
| PrefAbout | Tidak | ❌ |
| PrefDonate | Tidak | ❌ |

### 3. Width Layout - NO Magic Numbers

**LARANG:** `width: parent.width - 15`

**WAJIB:** Gunakan anchor margins + hitung width:

```qml
// BENAR
Flickable {
    anchors.fill: parent
    contentWidth: width  // PENTING untuk horizontal fill

    ColumnLayout {
        anchors.leftMargin: 10
        anchors.rightMargin: 10
        width: parent.width - 20
        // atau:
        Layout.fillWidth: true
    }
}

ScrollBar {
    // sebagai sibling, bukan child
}
```

### 4. Flickable-Slider Conflict

Saat Slider di dalam Flickable, scroll bisa intercept slider drag. **WAJIB** handle di PrefSlider.qml:

```qml
Slider {
    onPressedChanged: {
        let p = rootSlider.parent
        while (p) {
            if (p.hasOwnProperty('interactive') && typeof p.interactive === 'boolean') {
                p.interactive = !pressed
                break
            }
            p = p.parent
        }
    }
}
```

### 5. Pattern Flickable + ScrollBar (PANJANG)

```qml
Flickable {
    id: pageFlick
    anchors.fill: parent
    contentWidth: width
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds

    ScrollBar.vertical: ScrollBar {
        policy: ScrollBar.AsNeeded
        width: 4
        z: 1
        background: Rectangle { implicitWidth: 4; implicitHeight: 20; color: theme.colormap.bgmain; opacity: 0.0 }
        contentItem: Rectangle {
            implicitWidth: 4
            implicitHeight: 30
            radius: 2
            color: theme.colormap.playeraccent
        }
    }

    ColumnLayout {
        id: contentColumn
        anchors.leftMargin: 10
        anchors.rightMargin: 10
        anchors.topMargin: 10
        anchors.bottomMargin: 10
        width: pageFlick.width - 20
        spacing: 24

        // ... content
    }
}
```

### 6. Pattern ColumnLayout Saja (PENDEK)

```qml
ColumnLayout {
    id: pageColumn
    anchors.leftMargin: 10
    anchors.rightMargin: 10
    anchors.topMargin: 10
    anchors.bottomMargin: 10
    width: parent.width - 20
    spacing: 12

    // ... content
}
```

---

## Komponen Referensi

### PrefTab.qml

Sidebar button:
- `property string text` - Label button
- `property string icon` - Icon (Nerd Font)
- `property bool isActive` - State aktif
- `signal clicked()` - Click handler

### PrefSwitch.qml

Toggle dengan label:
- `property string label` - Judul setting
- `property string description` - Deskripsi opsional
- `property bool checked` - State switch
- `signal toggled()` - Toggle handler

### PrefSlider.qml

Slider dengan value:
- `property string label` - Judul
- `property string valueText` - Nilai display
- `property real fromValue`, `toValue`, `stepValue`, `currentValue`, `defaultValue`
- `signal moved(real value)` - Value change
- `signal resetToDefault()` - Reset handler

### PrefDropdown.qml

ComboBox:
- `property string label`, `description`
- `property var model` - Array string
- `property int currentIndex`
- `signal optionSelected(int index, string value)`

---

## StackLayout Integration

Pref page menggunakan StackLayout dengan `currentIndex` dikontrol oleh `prefPage.currentTabIndex`:

```qml
StackLayout {
    currentIndex: prefPage.currentTabIndex
    PrefHardware {}
    PrefAudio {}
    PrefLibrary {}
    PrefAppearance {}
    PrefAbout {}
    PrefDonate {}
}
```

Tambah tab baru = 2 langkah:
1. Tambah `PrefTab` di sidebar
2. Tambah item di StackLayout



Role: Expert Qt/QML & Rust Developer.
Project Context: Loonix-tunes (Fast audio processing with Rust back-end, Native compact UI with QML/Qt).

Objective: Audit dan cari semua bug (visual, logic, maupun performance) di file Pref.qml dan semua komponen terkaitnya (PrefLibrary.qml, PrefAudio.qml, PrefSwitch.qml, PrefSlider.qml, PrefDropdown.qml).

Fokus Audit:

Layout & Alignment: Cari elemen yang "balapan" keluar kotak, teks yang tidak wrapping, dan RowLayout yang tidak sejajar. Pastikan penggunaan Layout.preferredWidth benar untuk mengunci alignment label.

Binding Errors: Identifikasi potensi error Unable to assign [undefined] to bool/string pada properti yang terhubung ke musicModel (Rust). Periksa inkonsistensi snake_case vs camelCase.

Z-Order & Clipping: Cek apakah Popup pada ComboBox atau Flickable terpotong karena clip: true atau Z-index yang rendah.

Event Blocking: Pastikan MouseArea di root Pref.qml tidak mematikan interaktivitas Slider atau Switch di dalamnya.

Interaction Conflict: Cek konflik antara Flickable (scroll vertikal) dan Slider (drag horizontal).

Path Logic: Audit pembersihan path di FolderDialog agar kompatibel dengan sistem file (Linux/Windows).

Constraints Desain:

Harus Compact (Pot Player inspired).

Tidak boleh ada hardcoded colors (Wajib pake theme.colormap).

Harus Modular (Jangan tumpuk semua kode di satu file).

Tugas: Berikan list bug yang ditemukan, alasan teknisnya, dan berikan perbaikan kode yang langsung "Plug-and-Play" tanpa merusak fitur aslinya.

---

# Panduan Integrasi Matugen (Wallpaper Sync) ke Loonix

Berikut adalah arsitektur dan langkah-langkah untuk mengintegrasikan Matugen ke dalam Loonix-Tunes menggunakan prinsip *Single Source of Truth* (SSOT). Alur utamanya: **Ambil Warna ➔ Lempar ke UI ➔ Gembok di Config.**

---

## 1. Siapkan "Wadah" di `AppConfig` (`src/audio/config.rs`)

Kita butuh *flag* untuk menentukan apakah user sedang menggunakan mode tema manual atau mode *Wallpaper Sync*, beserta wadah untuk menyimpan warna terakhir dari wallpaper.

Tambahkan *field* berikut di dalam `struct AppConfig`:

```rust
// Tambahin di struct AppConfig
pub use_wallpaper_theme: bool,
pub matugen_colors: HashMap<String, String>, // Buat nyimpen warna terakhir dari wallpaper
```

Jangan lupa *update* implementasi `Default`-nya (di sekitar baris `impl Default for AppConfig`):

```rust
// Di dalam Self { ... }
use_wallpaper_theme: false,
matugen_colors: HashMap::new(),
```

---

## 2. Mesin Penarik Warna di `ThemeManager` (`src/ui/theme.rs`)

Kita akan menambahkan dua fungsi yang bisa dipanggil dari QML (`sync_with_wallpaper` dan `set_loonix_manual`), plus satu fungsi internal untuk membaca file JSON *output* dari Matugen (`~/.cache/matugen/colors.json`).

Tambahkan fungsi-fungsi ini di dalam `impl ThemeManager`:

```rust
use std::process::Command;
use serde_json::Value;

// --- 1. FUNGSI INTERNAL BUAT BACA MATUGEN ---
fn fetch_matugen_colors(&self) -> Option<HashMap<String, String>> {
    // Path default Matugen (Sesuaikan kalau path di sistem lo beda)
    let cache_path = dirs::home_dir()?.join(".cache/matugen/colors.json");
    
    let json_str = std::fs::read_to_string(cache_path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&json_str).ok()?;
    
    // Ambil skema warna (misal kita ambil yang dark mode)
    let colors = v["colors"]["dark"].as_object()?; 
    
    let mut map = HashMap::new();
    
    // MAPPING: Cocokin format nama Matugen ke key Loonix
    if let Some(surface) = colors["surface"].as_str() { c!(map, { "bgmain", surface }); }
    if let Some(surface_variant) = colors["surface_variant"].as_str() { c!(map, { "bgoverlay", surface_variant, "headerbg", surface_variant }); }
    if let Some(primary) = colors["primary"].as_str() { 
        c!(map, { 
            "playeraccent", primary, 
            "playlistactive", primary, 
            "eqactive", primary, 
            "fxactive", primary, 
            "tabhover", primary 
        }); 
    }
    if let Some(on_surface) = colors["on_surface"].as_str() { 
        c!(map, { 
            "playertitle", on_surface, 
            "tabtext", on_surface, 
            "playlisttext", on_surface, 
            "eqtext", on_surface 
        }); 
    }
    if let Some(outline) = colors["outline"].as_str() { c!(map, { "graysolid", outline, "tabborder", outline }); }
    if let Some(tertiary) = colors["tertiary"].as_str() { c!(map, { "playerhover", tertiary, "eqmix", tertiary }); }
    
    Some(map)
}

// --- 2. TRIGGER DARI QML: NYALAIN SYNC WALLPAPER ---
#[qt_method]
pub fn sync_with_wallpaper(&mut self) {
    if let Some(new_colors) = self.fetch_matugen_colors() {
        // 1. Terapin ke QML (UI Langsung Berubah)
        let qmap: QVariantMap = new_colors.iter()
            .map(|(k, v)| (QString::from(k.clone()), QVariant::from(QString::from(v.clone()))))
            .collect();
        
        self.colormap = qmap;
        self.current_raw_colors = new_colors.clone();
        self.colormap_changed();
        
        // 2. Simpan ke Config (Biar gak ilang pas restart)
        if let Some(ref config) = self.config {
            if let Ok(mut cfg) = config.lock() {
                cfg.use_wallpaper_theme = true;
                cfg.matugen_colors = new_colors; // Simpan warnanya di config
                cfg.save();
            }
        }
    } else {
        println!("Gagal baca Matugen JSON! Pastikan ~/.cache/matugen/colors.json ada.");
    }
}

// --- 3. TRIGGER DARI QML: BALIK KE LOONIX MANUAL ---
#[qt_method]
pub fn set_loonix_manual(&mut self) {
    // 1. Update status di config
    if let Some(ref config) = self.config {
        if let Ok(mut cfg) = config.lock() {
            cfg.use_wallpaper_theme = false;
            cfg.save();
        }
    }
    
    // 2. Panggil ulang tema manual yang terakhir aktif
    let current = self.current_theme.to_string();
    self.set_theme(current);
}
```

---

## 3. Modifikasi Fungsi Load di Saat Startup

Buka `set_config` di `theme.rs` dan ubah cara *load* tema agar Rust bisa mengecek status `use_wallpaper_theme` pada saat aplikasi dibuka.

```rust
pub fn set_config(&mut self, config: Arc<Mutex<AppConfig>>) {
    let cfg = config.lock().unwrap();
    
    let theme_name = if cfg.theme.is_empty() { "Default".to_string() } else { cfg.theme.clone() };
    let custom_themes = cfg.custom_themes.clone();
    
    // Ambil data wallpaper engine
    let use_wallpaper = cfg.use_wallpaper_theme;
    let matugen_saved = cfg.matugen_colors.clone();
    
    drop(cfg); // Lepas lock sebelum manggil fungsi lain
    
    self.custom_themes = custom_themes;
    self.config = Some(config);

    // LOGIKA MELEK: Load Matugen atau Load Manual?
    if use_wallpaper && !matugen_saved.is_empty() {
        // Apply warna wallpaper yang udah kesimpen di config
        let qmap: QVariantMap = matugen_saved.iter()
            .map(|(k, v)| (QString::from(k.clone()), QVariant::from(QString::from(v.clone()))))
            .collect();
        self.colormap = qmap;
        self.current_raw_colors = matugen_saved;
        self.colormap_changed();
    } else {
        // Apply tema manual
        self.set_theme(theme_name);
    }
}
```

---

## 4. UI/UX di Pref Theme (`ThemeEditor.qml` / `PrefAppearance.qml`)

Tambahkan interaksi pada *toggle* atau tombol di QML agar bisa memanggil *backend* Rust yang sudah dibuat.

```qml
// Di dalam MouseArea loonixToggle:
onClicked: {
    wallSyncToggle.active = false
    active = true
    theme.set_loonix_manual() // Mematikan sync dan balik ke manual
}

// Di dalam MouseArea wallSyncToggle:
onClicked: {
    if (!active) {
        active = true
        loonixToggle.active = false
        theme.sync_with_wallpaper() // Menarik warna Matugen
    }
}
```

---


## ⚠️ Catatan Tambahan
* **Check Availability:** Saat aplikasi berjalan, sebaiknya buat fungsi untuk mengecek apakah `matugen` tersedia di `$PATH`. Jika tidak ada, disable *toggle* Matugen di UI untuk mencegah *error*.
* **Auto-Update (Opsional):** Bisa dikembangkan lebih lanjut dengan menambahkan *file watcher* di Rust untuk memantau perubahan *file* wallpaper. Jadi saat user mengganti wallpaper, Loonix otomatis mengganti temanya tanpa perlu interaksi klik.

---

## pref/reportbug.md

```
1. Backend: Fungsi "Penyusun Laporan" (src/ui/theme.rs atau file lain)
Kita bikin fungsi di Rust buat nyusun URL GitHub dan ngebukanya di browser.

Rust
// Tambahin di impl ThemeManager atau sejenisnya
#[qt_method]
pub fn report_bug_on_github(&self, bug_title: String, bug_desc: String) {
    let repo_url = "https://github.com/citzeye/loonix-tunes-linux/issues/new";
    
    // Ambil info sistem biar user gak perlu ngetik manual
    let os_info = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    // Susun template body-nya
    let body = format!(
        "### Describe the bug\n{}\n\n### System Info\n- OS: {}\n- Arch: {}\n- Version: v1.0.2",
        bug_desc, os_info, arch
    );

    // Encode biar URL-nya gak berantakan
    let encoded_title = urlencoding::encode(&bug_title);
    let encoded_body = urlencoding::encode(&body);

    let final_url = format!("{}?title={}&body={}", repo_url, encoded_title, encoded_body);

    // Buka browser
    let _ = std::process::Command::new("xdg-open")
        .arg(final_url)
        .spawn();
}
(Note: Pastiin tambah urlencoding = "2.1" di Cargo.toml biar karakter aneh nggak bikin URL patah).

2. Frontend: qml/ui/pref/ReportBug.qml
Bikin tampilannya simpel, compact, dan tetep selaras sama warna colormap.

QML
import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15

Rectangle {
    id: root
    color: theme.colormap.bgmain
    radius: 6
    clip: true

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 15
        spacing: 12

        Text {
            text: "REPORT BUG / FEEDBACK"
            font.family: kodeMono.name
            font.pixelSize: 14
            font.bold: true
            color: theme.colormap.playeraccent
        }

        // Input Judul
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 35
            color: theme.colormap.bgoverlay
            border.color: titleInput.activeFocus ? theme.colormap.playeraccent : theme.colormap.graysolid
            
            TextInput {
                id: titleInput
                anchors.fill: parent
                anchors.margins: 8
                color: theme.colormap.tabtext
                font.pixelSize: 12
                verticalAlignment: Text.AlignVCenter
                clip: true
                // Placeholder
                Text {
                    text: "Judul masalah..."
                    color: theme.colormap.playersubtext
                    visible: !parent.text && !parent.activeFocus
                    anchors.fill: parent
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        // Input Deskripsi
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: theme.colormap.bgoverlay
            border.color: descInput.activeFocus ? theme.colormap.playeraccent : theme.colormap.graysolid

            Flickable {
                anchors.fill: parent
                contentWidth: width
                contentHeight: descInput.implicitHeight
                clip: true

                TextEdit {
                    id: descInput
                    width: parent.width
                    padding: 8
                    color: theme.colormap.tabtext
                    font.pixelSize: 12
                    wrapMode: TextEdit.Wrap
                    selectByMouse: true
                    
                    Text {
                        text: "Jelaskan kronologi bugnya bray..."
                        color: theme.colormap.playersubtext
                        visible: !parent.text && !parent.activeFocus
                        x: 8; y: 8
                    }
                }
            }
        }

        // Tombol Submit
        Rectangle {
            Layout.alignment: Qt.AlignRight
            width: 120
            height: 35
            radius: 4
            color: (titleInput.text && descInput.text) ? theme.colormap.playeraccent : theme.colormap.graysolid
            
            Text {
                anchors.centerIn: parent
                text: "OPEN GITHUB"
                font.bold: true
                font.pixelSize: 11
                color: theme.colormap.bgmain
            }

            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    if (titleInput.text && descInput.text) {
                        theme.report_bug_on_github(titleInput.text, descInput.text)
                        // Reset form setelah kirim
                        titleInput.text = ""
                        descInput.text = ""
                    }
                }
            }
        }
    }
}
```

---

## pref/themeeditor.md

```
🧠 Logic Flow: The "Melek" System
Entry Scan: User klik edit -> QML tanya: "Gue lagi megang apa?" -> Rust jawab: "Lo lagi megang tema [X], ini data lengkapnya (Warna + Nama)."

State Loading: QML terima data -> Langsung sebar ke semua picker dan placeholder.

The "Smart Save" Chain:

User klik Save -> Scan semua input UI.

Kirim ke Rust: "Ini data barunya, namanya [Nama], warnanya [Warna]."

Rust Logic: Scan config.json.

Ada nama yang sama? Langsung tindas (Overwrite).

Nggak ada? Kirim sinyal balik: "Bray, nama ini belum ada di data, mau bikin baru?"

Post-Save Check: "Apakah tema yang baru gue save ini adalah tema yang lagi nempel di layar?" -> Kalau iya, Trigger Refresh.


🛠️ Backend (Rust): src/ui/theme.rs
Kita bikin fungsi smart_save yang nggak peduli profile, dia peduli Nama dan State.

Rust
#[invokable]
pub fn smart_save_theme(&mut self, current_editor_name: String, colors: QVariantMap) -> String {
    let mut color_map: HashMap<String, String> = HashMap::new();
    for (k, v) in &colors {
        color_map.insert(k.to_string(), v.to_qstring().to_string());
    }

    // 1. SCAN CONFIG: Cari apakah nama di editor sudah ada di database custom_themes
    let theme_exists = self.custom_themes.iter().any(|t| t.name == current_editor_name);

    if theme_exists {
        // 2. REPLACE LOGIC: Update warnanya saja
        if let Some(pos) = self.custom_themes.iter().position(|t| t.name == current_editor_name) {
            self.custom_themes[pos].colors = color_map;
        }
        
        self.save_config();
        self.custom_themes_changed();

        // 3. ACTIVE CHECK: Kalau tema yang di-save lagi aktif, REFRESH UI
        if current_editor_name == self.current_theme.to_string() {
            self.set_theme(current_editor_name);
        }
        
        return "SUCCESS".into();
    } else {
        // 4. NEW THEME LOGIC: Kirim pesan ke QML buat tanya User
        // Di sini kita gak langsung save, tapi minta konfirmasi lewat return string
        return "CONFIRM_NEW".into();
    }
}

// Fungsi paksa save (panggil kalau user klik 'OKE' pas ditanya mau save nama baru)
#[invokable]
pub fn force_save_new_theme(&mut self, name: String, colors: QVariantMap, target_slot: i32) {
    let mut color_map: HashMap<String, String> = HashMap::new();
    for (k, v) in &colors {
        color_map.insert(k.to_string(), v.to_qstring().to_string());
    }

    let idx = target_slot as usize;
    if idx < self.custom_themes.len() {
        self.custom_themes[idx].name = name.clone();
        self.custom_themes[idx].colors = color_map;
        self.save_config();
        self.custom_themes_changed();
        self.set_theme(name); // Langsung aktifin
    }
}
🎨 UI (QML): ThemeEditor.qml
UI sekarang cuma lapor: "Ini data yang gue punya sekarang."

QML
// Tombol SAVE yang "mikir"
Rectangle {
    id: saveBtn
    // ... styling ...
    MouseArea {
        anchors.fill: parent
        onClicked: {
            var scannedColors = scanCurrentEditorColors();
            var result = theme.smart_save_theme(themeNameInput.text, scannedColors);

            if (result === "CONFIRM_NEW") {
                // TAMPILIN DIALOG KE USER
                confirmDialog.open(); 
            } else {
                root.themeEditorVisible = false;
            }
        }
    }
}

// Dialog konfirmasi (bukan judol bray, murni logic)
MessageDialog {
    id: confirmDialog
    title: "Save New Theme?"
    text: "Nama '" + themeNameInput.text + "' nggak ada di config. Mau ganti nama di profile ini?"
    buttons: MessageDialog.Ok | MessageDialog.Cancel
    onAccepted: {
        theme.force_save_new_theme(
            themeNameInput.text, 
            scanCurrentEditorColors(), 
            themeEditorRoot.selectedSlotIndex
        );
        root.themeEditorVisible = false;
    }
}



1.No More Ghost Updates: Karena ada current_theme check di akhir proses save, nggak akan ada lagi kejadian "Warna udah di-save tapi layar nggak berubah".
2.Double-Blind Check: Rust nggak cuma nerima data, dia nge-cek integritas data ke config dulu sebelum beraksi.
3.Flexible Entry: Mau lo klik kanan dari tema aktif, atau lo buka lewat menu "Create", Rust bakal Scan Starter Colors dulu (get_editor_starter_colors) buat mastiin placeholder nggak kosong atau item.
```
