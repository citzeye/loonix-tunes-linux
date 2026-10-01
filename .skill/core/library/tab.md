# Tab & Playlist

Mengelola navigasi folder musik melalui integrasi antara sistem Tab (QML UI) dan `library.rs` (Backend Engine Rust).

### 1. Arsitektur Tab (UI Hierarchy)
Sistem tab dibagi menjadi dua kategori utama dalam `Tab.qml`:
* **Static Tabs (Fixed):**
    * `EXTERNAL_FILES`: Menangani file yang dibuka secara manual (Drag & Drop).
    * `QUEUE`: Daftar antrian putar aktif saat ini.
    * `FAVORITES`: Filter otomatis untuk lagu yang ditandai bintang.
    * `MUSIC`: Folder musik default sistem (Hasil dari `get_music_directory()`).
* **Dynamic Tabs (Custom Folders):**
    * Dirender menggunakan `Repeater` yang terhubung ke `musicModel.custom_folder_count`.
    * Menggunakan komponen `TabCustom.qml` sebagai delegate.
    * **Aturan Absolut:** Tab custom dan tab lainnya HANYA boleh berisi file musik. DILARANG menambahkan folder/subfolder ke dalam tab ini selain di `TabMusic`.

### 2. Logic Scanning (Backend Library.rs)
Pemuatan data musik dilakukan secara non-blocking untuk menjaga UI tetap responsif:
* **Async Scanning:** Fungsi `scan_music_async` menjalankan `thread::spawn` untuk memindai file tanpa mengunci (freeze) aplikasi.
* **Sorting Policy:** 
    1. **Folder First:** Folder selalu ditampilkan di urutan atas sebelum file musik.
    2. **Case-Insensitive Alpha:** Pengurutan nama menggunakan `.to_lowercase()` agar alfabetis murni (A-Z).
* **Path Resolution:** Menggunakan `dirs::audio_dir()` dengan fallback ke `$HOME/Music` jika folder audio standar OS tidak ditemukan.

### 3. Sinkronisasi UI & Model (Switching Logic)
Proses perpindahan tab mengikuti alur berikut:
1. **Trigger:** User mengklik tab (contoh: `TabCustom.qml`).
2. **Request:** Memanggil `musicModel.switch_to_folder(path)`.
3. **Validation:** Backend melakukan `scan_custom_directory(path)`.
4. **UI Update:** 
    * `current_folder_qml` diperbarui.
    * `isActive` property di QML otomatis berubah warna (`theme.colormap.tabhover`).
    * `refreshTicker` di `TabCustom.qml` dipicu untuk memaksa re-draw label jika ada perubahan nama.

### 4. Manajemen Folder (Add/Remove & Proteksi)
* **FolderDialog:** Terintegrasi di `Tab.qml` dan `TabMusic.qml`.
* **Path Sanitization:** String `file://` dihapus secara otomatis di sisi QML sebelum dikirim ke model Rust untuk menghindari error pathing di OS Linux.
* **Tab Naming:** Setelah path folder didapatkan dari user, nama folder diekstrak dan secara otomatis diubah menjadi **HURUF BESAR SEMUA (UPPERCASE)** untuk dijadikan nama Tab.
* **Auto-Lock Policy (Initial Import):** 
    * Setiap folder/tab baru yang ditambahkan melalui `add_folder_tab(path)` akan secara otomatis diset ke status **LOCKED** di backend.
    * User tidak dapat menghapus tab tersebut selama statusnya masih terkunci (proteksi integritas tab).
* **Dynamic Context Menu:** 
    * Menu klik kanan mendeteksi status kunci folder secara real-time.
    * Jika folder terkunci: Menu menampilkan opsi **"Unlock"**. User wajib melakukan klik *Unlock* sebelum opsi "Remove Tab" menjadi aktif.
    * Jika folder terbuka: Menu menampilkan opsi **"Lock"** dan **"Remove Tab"**.
* **Persistence:** Status *Lock* disimpan ke dalam konfigurasi lokal aplikasi (`config.json`) agar tetap konsisten meskipun aplikasi ditutup.

### 5. Logika Context-Switching (Clean Slate Policy)
Untuk menghindari bug "Ghost Padding" (koordinat tab sebelumnya terbawa), sistem menerapkan pemisahan konteks total saat perpindahan tab:
* **Context Reset:** Setiap kali `switch_to_folder(path)` dipanggil, backend WAJIB menjalankan `display_list.clear()` dan `expanded_folders.clear()`.
* **Root Definition:** Backend menetapkan `current_tab_root` sebagai titik nol koordinat. 
    * Jika Tab Music aktif: `current_tab_root` = Music Directory.
    * Jika Tab Custom aktif: `current_tab_root` = Path folder custom tersebut.
* **Model Signaling:** Menggunakan `beginResetModel()` dan `endResetModel()` di Rust untuk memaksa QML menghapus seluruh delegat lama dan merender ulang dari awal.

### 6. Arsitektur "Knob" & Indentasi Relatif (UI Layer)
Indentasi baris (margin kiri) diatur MURNI oleh UI (QML) tanpa membebani Rust. Rust hanya menyediakan data, QML yang mengatur tampilan.
* **Aturan Indentasi:** 
    * **HANYA** `TabMusic.qml` yang memiliki logika indentasi (karena hanya tab ini yang mengizinkan subfolder).
    * Margin indentasi adalah **15px** untuk isi di dalam subfolder, dan **0px** untuk item yang berada di luar/root.
    * Tab selain Music (`TabCustom`, `TabFavorites`, `TabQueue`, dll) **selalu 0px** karena tidak memiliki subfolder.
* **Implementasi "Playlist as a Dumb Canvas":**
    * `Playlist.qml` bertindak sebagai kanvas kosong yang memiliki satu properti kenop: `property int dynamicMargin: 0`.
    * Delegate di `Playlist.qml` membaca kenop ini dipadukan dengan status hierarki: `anchors.leftMargin: (model.parent_path !== musicModel.current_tab_root) ? playlistView.dynamicMargin : 0`.
* **Controller:**
    * Saat `TabMusic.qml` diklik, ia akan memutar kenop: `playlistSection.dynamicMargin = 15`.
    * Saat tab lain diklik, ia akan mereset kenop: `playlistSection.dynamicMargin = 0`.

### 7. Expansion & Persistence Logic (Folder Toggling - Khusus Tab Music)
* **Toggle Mechanism:** Fungsi `toggle_folder(index)` bekerja secara rekursif pada `display_list`:
    * **Expand:** Memasukkan konten folder tepat di bawah `index` folder tersebut dan menandai path-nya ke dalam `expanded_folders`.
    * **Collapse:** Menghapus semua item yang memiliki `parent_path` yang mengandung path folder yang di-collapse.
* **Dynamic Rendering:** Perubahan list akan langsung diadaptasi oleh `Playlist.qml` yang mengaplikasikan `dynamicMargin` secara otomatis.

### 8. Session-Based Temporary Folders (RAM Only)
Fitur khusus untuk organisasi cepat tanpa mengubah konfigurasi permanen aplikasi:
* **Manual Injection:** User dapat menambahkan folder/grup sementara ke dalam list melalui `PlaylistContextMenu`.
* **Storage Policy:** Data ini disimpan murni di dalam RAM (variabel `session_folders` pada model Rust).
* **Persistence Level:**
    * **Pindah Tab:** Item tetap bertahan karena Rust menyertakan `session_folders` saat melakukan pemindaian ulang.
    * **Reload/Close App:** Data otomatis hangus dan terhapus total karena tidak didaftarkan ke dalam `AppConfig::save()`.

### 9. Non-Persistent State Policy
Menjamin integritas `config.json` dari perubahan yang tidak disengaja:
* **Atomic Lock:** Menggunakan `IS_INITIALIZING` (AtomicBool) untuk mengunci (prevent) penulisan file saat proses startup sedang berjalan.
* **Selective Save:** Hanya perubahan pada "Custom Tab" permanen atau "Lock State" yang memicu `save()`. Fitur temporary (seperti Session Folders) sepenuhnya diabaikan oleh sistem persistence.