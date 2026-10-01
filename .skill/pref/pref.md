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