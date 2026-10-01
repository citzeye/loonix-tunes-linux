# Specs: LoonixTunes MP3 Editor Module (The Lab)
**Version:** 1.0.0
**Target:** Integrated QML Window with Rust `lofty` Backend

---

## 1. UI Structure & Layout (`Mp3EditorWindow.qml`)
Jendela ini berjalan independen dari `MainWindow.qml` namun berbagi instance engine yang sama untuk memfasilitasi *Single Source of Truth* (SSoT).

* **Window Element**: `Window { id: editorWindow; visible: false; modality: Qt.NonModal }`
* **Global Layout**: `RowLayout` (membagi Sidebar dan Main Area).
* **Sidebar (Kiri - Lebar: 250px)**:
    * **Header**: Label "EXPLORE" (Permanent & Bold).
    * **Content**: `TreeView` atau `ListView` yang membaca *directory model* dari Rust.
    * **Akses**: Menampilkan `~/Music` dan custom library paths.
* **Main Area (Kanan - Sisanya)**:
    * **Header**: `TabBar` dengan 4 TabButton: `[ Edit | Lyric | Batch | Playlist Maker ]`.
    * **Content**: `StackLayout` terikat pada `TabBar.currentIndex`.

---

## 2. Backend (Rust - `src/ui/bridge/mp3editor.rs`)
Modul ini bertindak sebagai jembatan *QObject* (`EditorModel`) antara QML dan *crate* Rust.

### A. Dependencies
* `lofty`: Untuk ekstraksi dan manipulasi metadata audio (ID3v2, FLAC, MP4, dll).
* `walkdir`: Untuk *recursive scanning* yang efisien di Playlist Maker.
* `regex`: Untuk *pattern matching* di fitur Batch Rename.

### B. Qt Meta Object (`qt_base_class!`)
qt_base_class!(
    pub struct EditorModel {
        // Properties
        pub target_file: qt_property!(QString; READ target_file WRITE set_target_file NOTIFY target_file_changed),
        pub current_metadata: qt_property!(QVariantMap; READ current_metadata NOTIFY metadata_loaded),
        pub is_processing: qt_property!(bool; READ is_processing NOTIFY processing_state_changed),
        
        // Signals
        pub target_file_changed: qt_signal!(),
        pub metadata_loaded: qt_signal!(),
        pub processing_state_changed: qt_signal!(),
        pub library_updated: qt_signal!(), // Memaksa MusicList utama untuk refresh
        
        // Slots / Invocables
        pub load_file: qt_method!(fn(&mut self, path: QString)),
        pub save_metadata: qt_method!(fn(&mut self, new_data: QVariantMap) -> bool),
        pub generate_batch_preview: qt_method!(fn(&self, template: QString, files: QVariantList) -> QVariantList),
        pub execute_batch_rename: qt_method!(fn(&mut self, template: QString, files: QVariantList) -> bool),
        pub start_playlist_scan: qt_method!(fn(&mut self, root_path: QString, criteria: QString)),
    }
);

---

## 3. Architecture & Data Flow

1.  **Trigger (Entry Point)**: 
    * User klik kanan di `MusicList.qml`.
    * **Constraint Check**: `if (selected_path === musicModel.current_path) { return; }` (Tampilkan notifikasi: "Cannot edit playing track").
    * Buka jendela: `editorWindow.show(); editorModel.load_file(selected_path);`
2.  **Data Extraction**: Rust membaca file via `lofty::Probe`. Data di-*mapping* ke `QVariantMap` dan dikirim ke QML. Cover Art di-ekstrak sebagai *temporary file* atau *Base64 string* agar bisa di-*render* QML `Image`.
3.  **SSoT Update**: Saat fungsi `save_metadata` berhasil dijalankan, Rust melakukan *write* ke disk, lalu memanggil `self.library_updated()`. QML utama yang nge- *listen* sinyal ini akan me-*reload* tampilan *list* lagu secara otomatis tanpa *restart*.

---

## 4. Breakdown Logika per Tab

### Tab 1: Edit (Metadata Manual)
* **Fungsi**: Manipulasi 1-on-1 (*Title, Artist, Album, Year, Genre, Track Number, BPM*).
* **Logika Placeholder**: Jika *Title* kosong, GUI menampilkan nama file (tanpa ekstensi) berwarna abu-abu redup sebagai saran pengisian.
* **Cover Art**: Drag-and-drop *image support* untuk mengganti gambar album (mengubah `.jpg/.png` ke *byte array* dan menempelkannya ke dalam *tag* ID3v2/FLAC).

### Tab 2: Lyric
* **Fungsi**: Membaca dan menulis *Unsynchronized Lyrics* (USLT) ke dalam metadata.
* **UI**: `TextArea` besar. Bisa di-*copy-paste* langsung.

### Tab 3: Batch (Template Renamer)
* **Fungsi**: Mengubah nama file fisik secara massal berdasarkan metadata.
* **Alur Kerja**: Select Multiple Files -> Pilih Template -> Lihat Preview -> Eksekusi.
* **Template Tags**: `{artist}`, `{title}`, `{album}`, `{year}`, `{track}`.
* **Transform Options** (Via Regex & String methods di Rust):
    * `Uppercase` (HELLO - WORLD.mp3)
    * `Lowercase` (hello - world.mp3)
    * `Title Case` (Hello - World.mp3)
    * `Replace Spaces with Underscores` (Hello_-_World.mp3)
* **Safety Guard**: Rust **WAJIB** mereturn *Array of Strings* (Preview) sebelum melakukan fungsi `std::fs::rename()`.

### Tab 4: Playlist Maker (The Intelligent Scanner)
* **Fungsi**: Membaca metadata ribuan lagu dan membuat *smart groupings*.
* **Threading**: Harus dieksekusi di `std::thread::spawn` (atau dikirim ke *worker pool*) lalu mengirim sinyal kembali ke *Main Thread* (Qt) agar UI tidak *freeze*.
* **Kriteria Algoritma (Hash Mapping)**:
    * **Genre**: `group_by(|track| track.genre())`
    * **Quality**: Hitung via `file_size / duration`. Filter `High` (>= 256kbps), `Low` (< 128kbps).
    * **Year**: `group_by(|track| (track.year() / 10) * 10)` (Menghasilkan era 80s, 90s, 00s).
    * **Format**: `group_by(|track| track.extension())` (Membagi zona *Audiophile* FLAC dan *Daily* MP3).
    * **Vibe (BPM & Duration Heuristics)**:
        * *Chill*: `bpm < 90` ATAU `duration > 6:00` ATAU `genre.contains("Lo-Fi|Ambient|Jazz")`.
        * *Workout*: `bpm > 120` ATAU `genre.contains("Rock|Metal|EDM|Dance")`.

---

## 5. Implementation Roadmap (Langkah Selanjutnya)

1.  **Tahap 1: Pondasi Rust (Backend)**
    * Buat `src/ui/bridge/mp3editor.rs`.
    * Implementasikan fungsi `load_file` dan `save_metadata` menggunakan *crate* `lofty`.
    * Daftarkan `EditorModel` di `src/main.rs` sebagai `qml_register_type`.
2.  **Tahap 2: Kerangka UI (Frontend)**
    * Buat `Mp3EditorWindow.qml`.
    * Bangun struktur `RowLayout`, `Sidebar`, `TabBar`, dan `StackLayout`.
3.  **Tahap 3: Koneksi Dasar**
    * Tambahkan `MenuItem` di *context menu* `MusicList.qml`.
    * Pastikan *Lock Check* (pencegahan edit lagu yang sedang diputar) berfungsi.
4.  **Tahap 4: Fitur Lanjutan (Iteratif)**
    * Selesaikan logika Batch Renamer di Rust.
    * Implementasikan *Multithreaded Directory Scanner* untuk Playlist Maker.