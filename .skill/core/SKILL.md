---
name: core
description: Lapisan core: konfigurasi, library/tab, dan deteksi wireless.
---
## core/config/config.md

```
🎯 FULL PRODUCTION REFACTOR
Centralized Config System → config.rs
🎯 OBJECTIVE

Refactor total sistem konfigurasi agar:

src/audio/config.rs menjadi single source of truth
Default config → hardcoded di Default::default()
User config → ~/.config/loonix-tunes/config.json
Startup logic:
Jika user config ada → load & override default
Jika tidak ada → pakai default
Remove obsolete Pro flags:
highres_enabled
dac_exclusive_mode
Zero warning
Clippy clean
Tidak ada unwrap di runtime path
🔥 HARD RULES (WAJIB)
❌ Tidak boleh unwrap() di runtime
❌ Tidak boleh panic kecuali fatal startup
❌ Tidak boleh global allow unused/dead_code
❌ Tidak boleh baca file assets lama
❌ Tidak boleh sentuh Git
❌ Tidak boleh delete file permanent
✅ Semua error harus propagate atau fallback
✅ Config loading hanya terjadi saat startup atau reload
✅ Audio thread tidak boleh menyentuh file I/O
🧠 ARCHITECTURE DESIGN
1️⃣ Config Location

User config path:

Linux:

~/.config/loonix-tunes/config.json

Implement via:

dirs::config_dir()

Folder harus auto-create jika belum ada.

2️⃣ Config Loading Strategy
Flow:
AppConfig::load()
    ↓
if user_config_exists:
    read file
    deserialize
    merge with default
else:
    return Default::default()

Tidak boleh panic jika file corrupt.
Fallback ke default + log warning.

🧱 STRUCT DESIGN
AppConfig (FINAL)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub volume: f32,
    pub balance: f32,
    pub theme: String,
    pub eq_enabled: bool,
    pub eq_bands: [f32; 10],
    pub fx_enabled: bool,
    pub reverb_amount: f32,
    pub compressor_threshold: f32,
}

❌ REMOVE:

highres_enabled
dac_exclusive_mode
🏗 Default Implementation

Default harus pure hardcoded:

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            volume: 1.0,
            balance: 0.0,
            theme: "dark".to_string(),
            eq_enabled: false,
            eq_bands: [0.0; 10],
            fx_enabled: false,
            reverb_amount: 0.0,
            compressor_threshold: -12.0,
        }
    }
}

❌ Tidak boleh load file
❌ Tidak boleh panggil fungsi eksternal

💾 Load Implementation
impl AppConfig {
    pub fn load() -> Self {
        let default = Self::default();

        match Self::load_user_config() {
            Ok(cfg) => cfg,
            Err(_) => default,
        }
    }
}
Safe User Load
fn load_user_config() -> Result<Self, ConfigError>

Rules:

Jika file tidak ada → return Err(NotFound)
Jika JSON invalid → return Err(ParseError)
Tidak unwrap
Tidak panic
💾 Save Implementation
pub fn save(&self) -> Result<(), ConfigError>
Serialize JSON pretty
Atomic write (write temp → rename)
Create directory if missing
Tidak panic
🧹 REMOVE LEGACY

Hapus total:

load_fx_config()
load_eq_presets()

Jika masih dipakai di tempat lain → refactor caller.

Jangan delete file permanent.
Jika ada file obsolete .rs → pindah ke .backup/

🧠 DSPCONFIG INTEGRATION DECISION
Jangan hapus DspConfigManager dan DspStateView

Mereka bukan duplicate config.
Mereka adalah:

runtime state sync layer
bukan persistent storage

Tapi:

dspconfig.rs tidak boleh lagi baca file.

Refactor agar:

DspConfigManager::new(app_config: &AppConfig)

Jadi pure runtime layer.

🔄 Reload Logic

Saat user tekan reload config:

engine.pause()
load config baru
apply ke dsp manager
engine.resume()

Tidak boleh recreate ringbuffer.
Tidak boleh restart audio device.
Tidak boleh spawn thread baru.

Patuh lifecycle rule.

🧪 TESTING CHECKLIST

Sebelum dianggap selesai:

App start tanpa config file
App start dengan config valid
App start dengan config corrupt
Delete config saat app running → reload
Rapid reload spam
Close app while saving

Tidak boleh panic.
Tidak boleh leak thread.

📁 FILES TO MODIFY
src/audio/config.rs
src/core/dspconfig.rs (refactor dependency)
src/main.rs (call load())
🎯 ACCEPTANCE CRITERIA
cargo check → 0 warning
cargo clippy → clean
Tidak ada unwrap runtime
Tidak ada file asset dependency
Config pusat hanya di config.rs
User override bekerja
Audio thread untouched
Engine authority tetap utuh
```

---

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

---

# Wireless/Bluetooth Detection System

## Overview

Sistem deteksi koneksi audio di Loonix-Tunes untuk optimizing buffer size dan sample rate berdasarkan tipe device.

## File: `src/audio/wireless.rs`

### DeviceType Enum

```rust
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DeviceType {
    Bluetooth,      // bluez, a2dp, bluetooth
    WiFi,           // raop, network, upnp
    Headset,        // usb, headset, headphone
    InternalSpeaker, // pci, analog, speaker, hdmi
    Unknown,
}
```

### SystemAudioStatus Struct

```rust
#[derive(Default)]
pub struct SystemAudioStatus {
    pub isMuted: bool,
    pub isBluetooth: bool,
    pub deviceType: String,
}
```

### Public API

| Function | Return | Description |
|----------|--------|-------------|
| `detectDeviceType(name: &str)` | `DeviceType` | Parse device name → type |
| `getSystemAudioStatus(name: &str)` | `SystemAudioStatus` | Full status + auto-detect |
| `getSystemAudioStatus_simple()` | `SystemAudioStatus` | From cached state |
| `setSystemMuted(muted: bool)` | `()` | Set mute state |
| `setBluetoothDetected(is_bt: bool)` | `()` | Set BT flag |
| `isSystemMuted()` | `bool` | Get mute state |
| `isBluetoothDetected()` | `bool` | Get BT flag |
| `startSystemCheck()` | `()` | Start background monitor |

### Heuristic Detection

```rust
pub fn detectDeviceType(device_name: &str) -> DeviceType {
    let name = device_name.to_lowercase();

    // Bluetooth (Linux)
    if name.contains("bluez") || name.contains("a2dp") || name.contains("bluetooth") {
        return DeviceType::Bluetooth;
    }

    // WiFi / Network Audio
    if name.contains("raop") || name.contains("network") || name.contains("upnp") {
        return DeviceType::WiFi;
    }

    // Headset / USB DAC
    if name.contains("usb") || name.contains("headset") || name.contains("headphone") {
        return DeviceType::Headset;
    }

    // Internal Speaker
    if name.contains("pci") || name.contains("analog") || name.contains("speaker") || name.contains("hdmi") {
        return DeviceType::InternalSpeaker;
    }

    DeviceType::Unknown
}
```

## Adaptive Audio Configuration

### audiooutput.rs Integration

```rust
let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());
let status = crate::audio::wireless::getSystemAudioStatus(&device_name);

let (sample_rate, buffer_size) = if status.isBluetooth {
    // Bluetooth: slower codec, needs bigger buffer
    (44100, 4096)
} else if status.deviceType == "WiFi Audio" {
    // WiFi: even slower, max buffer
    (48000, 8192)
} else {
    // Wired: low latency
    (48000, 512)
};
```

### Buffer Configuration

| Device Type | Sample Rate | Buffer Size | Latency |
|-------------|-------------|-------------|---------|
| Bluetooth | 44100 Hz | 4096 | ~93ms |
| WiFi | 48000 Hz | 8192 | ~170ms |
| Headset/USB | 48000 Hz | 512 | ~11ms |
| Internal | 48000 Hz | 512 | ~11ms |

## Background Monitoring

System check loop polling setiap 800ms untuk:
1. Sink mute status (`pactl get-sink-mute @DEFAULT_SINK@`)
2. Default sink name (`pactl get-default-sink`)

```rust
pub fn startSystemCheck() {
    thread::spawn(|| loop {
        // Check mute status
        let output = Command::new("pactl")
            .args(["get-sink-mute", "@DEFAULT_SINK@"])
            .output();

        // Check device type
        let sink_info = Command::new("pactl")
            .args(["get-default-sink"])
            .output();

        thread::sleep(Duration::from_millis(800));
    });
}
```

## Thread Safety

- `SYSTEM_MUTED` - AtomicBool (SeqCst)
- `BLUETOOTH_DETECTED` - AtomicBool (SeqCst)
- No mutex needed - UI writes, audio reads directly

## Module Registration

```rust
// src/audio/mod.rs
pub mod wireless;
```

## Usage in QML

```rust
// Check current status
let status = wireless::getSystemAudioStatus_simple();
if status.isBluetooth {
    // Show Bluetooth indicator
}
```

## Why This Works

1. **Zero UI Clutter** - No buttons, automatic
2. **Scalable** - Easy to add new device types
3. **DeviceType string** - Can be sent to UI for status display
4. **Background polling** - Always up-to-date
