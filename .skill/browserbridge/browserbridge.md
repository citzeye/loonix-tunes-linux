# Specs: LoonixTunes Live DSP (Browser Bridge & System Audio)
**Version:** 2.0.0 (Advanced Bridge Mode)
**Target:** Linux System-Wide Audio Processing via libpulse

---

## 1. Core Architecture (The Pipeline)
Menggunakan pendekatan "PulseAudio Null Sink" agar browser/OBS bisa langsung mendeteksi input/output tanpa konfigurasi manual dari user.

- **Virtual Input (Pintu Masuk):** Rust me-load modul `module-null-sink` di PulseAudio dengan nama "LoonixTunes_DSP".
- **Capture (Record Stream):** `libpulse` menangkap audio dari "LoonixTunes_DSP.monitor".
- **Processing:** Audio mentah dilempar ke Ring Buffer -> Dieksekusi oleh Modul DSP.
- **Output (Playback Stream):** `libpulse` memutar hasil audio yang sudah di-DSP ke Hardware Speaker asli.
- **OBS Integration:** OBS tinggal merekam "Desktop Audio" dari Hardware Speaker (hasil akhirnya).

---

## 2. UI Structure & UX Flow (`BrowserBridgePopup.qml`)
Menggunakan konsep Overlay Popup agar transisi dari internal player ke system-wide DSP terasa natural.

- **Entry Point:** Logo Bola Dunia di barisan Volume.
- **Overlay Transition:** Saat diklik, muncul popup dengan background gelap (opacity 50%) menutupi UI utama.
- **Safety Gate (Dialog):** Menampilkan konfirmasi: "Stop current track?" [Yes / No].
    - Jika Yes: Eksekusi `musicModel.stop()`, lalu Main Toggle Bridge otomatis ON.
- **Header & Main Toggle:** Switch "Enable Browser Bridge" di bagian paling atas.
- **Source Routing (Auto-Scan):**
    - List/Dropdown dinamis berisi browser yang terdeteksi sedang aktif (Chrome, Firefox, dll).
    - Terdapat toggle di sebelah kanan tiap list untuk mengizinkan aplikasi mana saja yang dikoneksikan ke DSP.
- **Metadata Status:**
    - Teks dinamis yang menampilkan judul lagu/video (Contoh: "YouTube - Sweet Child O' Mine").
    - Default/Fallback text: "No Source... Play something in your browser".
- **DSP Controls:**
    - Grid list untuk "Instant Presets" (Cinematic, Podcast, dll).
    - Slider/Amount manual untuk 5 DSP Esensial (EQ, Compressor, Limiter, MiddleClarity, StereoWidth).
- **Visualizer:** Animasi bar volume di bagian bawah yang bergerak responsif (maju-mundur) sesuai dinamika suara yang ditangkap.

---

## 3. Backend Logic (Rust + libpulse)

### A. The Setup / Teardown
Saat Main Toggle dinyalakan, Rust mengeksekusi perintah:
- **ON:** `pactl load-module module-null-sink sink_name=LoonixTunes_DSP sink_properties=device.description="LoonixTunes_Live"`
- **OFF:** Membunuh modul tersebut agar audio OS kembali normal.

### B. Browser Scanner & Metadata
- **Scanner:** Menggunakan `pa_context_get_sink_input_info_list` dari libpulse untuk mendeteksi `application.name`.
- **Metadata:** Menggunakan standar Linux (MPRIS via DBus) untuk menangkap `Metadata.title` dari browser.

### C. The Stream Loop (Low Latency)
- `pa_buffer_attr`: Dikonfigurasi untuk latensi sangat rendah.
- Fungsi `read_callback` mengisi `ringbuf::Producer`.
- Fungsi `write_callback` menarik dari `ringbuf::Consumer`, memasukkannya ke rantai DSP, lalu melempar ke hardware.

---

## 4. The "Instant" Presets Logic

| Preset Name | Configuration Logic |
| :--- | :--- |
| **Cinematic** | EQ V-Shape, Width +20%, Limiter ON (-0.5dB) mencegah ledakan clip. |
| **Podcast** | MiddleClarity ON (vocal boost), Compressor agresif (meratakan suara bisik/teriak). |
| **Music Enhancer** | Bass +15%, Crystalizer tipis untuk mengembalikan detail yang terkompresi. |
| **Late Night** | Extreme Compressor. Suara pelan kedengaran jelas, suara keras tertahan. |

---

## 5. "Power User" Parameters (Anti-Latency Setup)
Panel khusus di bagian bawah UI untuk tuning performa sistem Linux guna mencapai "Zero Latency".

| Parameter | Logic in Rust/Linux | Impact |
| :--- | :--- | :--- |
| **RT Priority** | Meminta akses `SCHED_FIFO` ke kernel Linux. | Mencegah audio thread disela oleh proses OS lain (Anti-Kresek). |
| **Buffer/Quantum** | Mengatur ukuran `fragsize` pada `pa_buffer_attr`. | Semakin kecil slider, video dan audio semakin sinkron. |
| **MemLock** | Mengeksekusi `mlockall()` pada thread audio. | Mencegah RAM dialokasikan ke Swap, menjamin performa konstan. |
| **Exclusive Access**| Meminta akses sink eksklusif. | Bypass system mixer untuk bit-perfect audio processing. |

---

## 6. Execution Flow Constraints
1. **Concurrency Lock:** Live DSP dan Internal Music Player saling eksklusif. Konfirmasi "Stop current track?" wajib disetujui.
2. **CPU Efficiency:** Jika Main Toggle OFF, thread PulseAudio harus di-drop dan Ring Buffer dikosongkan.

---

## 7. Implementation Roadmap (File Changes)

### NEW FILES

// File 1: src/audio/io/browserbridge.rs
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct BrowserBridgeEngine {
    pub is_active: Arc<AtomicBool>,
    null_sink_id: Option<String>,
}

impl BrowserBridgeEngine {
    pub fn new() -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            null_sink_id: None,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.is_active.load(Ordering::SeqCst) {
            return Ok(());
        }

        let output = Command::new("pactl")
            .arg("load-module")
            .arg("module-null-sink")
            .arg("sink_name=LoonixTunes_DSP")
            .arg("sink_properties=device.description=\"LoonixTunes_Live_DSP\"")
            .output()
            .map_err(|e| format!("Failed to execute pactl: {}", e))?;

        if output.status.success() {
            let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            self.null_sink_id = Some(id);
            self.is_active.store(true, Ordering::SeqCst);
            Ok(())
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!("pactl error: {}", err))
        }
    }

    pub fn stop(&mut self) {
        if !self.is_active.load(Ordering::SeqCst) { return; }
        if let Some(id) = &self.null_sink_id {
            let _ = Command::new("pactl").arg("unload-module").arg(id).output();
        }
        self.null_sink_id = None;
        self.is_active.store(false, Ordering::SeqCst);
    }
}

impl Drop for BrowserBridgeEngine {
    fn drop(&mut self) { self.stop(); }
}


// File 2: src/ui/bridge/browserbridgecontroller.rs
use qmetaobject::prelude::*;
use std::sync::{Arc, Mutex};
use crate::audio::io::browserbridge::BrowserBridgeEngine;

#[derive(QObject)]
pub struct BrowserBridgeController {
    base: qt_base_class!(trait QObject),

    pub bridge_active: qt_property!(bool; READ get_bridge_active NOTIFY bridge_active_changed),
    pub current_source_name: qt_property!(QString; READ get_current_source_name NOTIFY current_source_name_changed),

    pub bridge_active_changed: qt_signal!(),
    pub current_source_name_changed: qt_signal!(),

    pub toggle_bridge: qt_method!(fn(&mut self, state: bool)),
    pub apply_preset: qt_method!(fn(&mut self, preset_name: QString)),

    // Internal Rust state
    engine: Arc<Mutex<BrowserBridgeEngine>>,
    _bridge_active: bool,
    _current_source_name: QString,
}

impl Default for BrowserBridgeController {
    fn default() -> Self {
        Self {
            base: Default::default(),
            bridge_active_changed: Default::default(),
            current_source_name_changed: Default::default(),
            toggle_bridge: Default::default(),
            apply_preset: Default::default(),
            engine: Arc::new(Mutex::new(BrowserBridgeEngine::new())),
            _bridge_active: false,
            _current_source_name: QString::from("No Source..."),
        }
    }
}

impl BrowserBridgeController {
    fn get_bridge_active(&self) -> bool {
        self._bridge_active
    }

    fn get_current_source_name(&self) -> QString {
        self._current_source_name.clone()
    }

    fn toggle_bridge(&mut self, state: bool) {
        let mut engine = self.engine.lock().unwrap();
        if state {
            if let Ok(_) = engine.start() {
                self._bridge_active = true;
                self._current_source_name = QString::from("Listening to System Audio...");
                self.bridge_active_changed();
                self.current_source_name_changed();
            }
        } else {
            engine.stop();
            self._bridge_active = false;
            self._current_source_name = QString::from("No Source...");
            self.bridge_active_changed();
            self.current_source_name_changed();
        }
    }

    fn apply_preset(&mut self, preset_name: QString) {
        println!("Menerapkan preset Live DSP: {}", preset_name.to_string());
    }
}


// File 3: qml/ui/components/BrowserBridgePopup.qml
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

Popup {
    id: browserBridgePopup
    width: parent.width
    height: parent.height
    modal: true
    focus: true
    background: Rectangle { color: "#80000000" }

    MessageDialog {
        id: stopTrackDialog
        title: "Stop Current Track?"
        text: "Enabling Browser Bridge requires stopping the current internal track. Proceed?"
        buttons: MessageDialog.Yes | MessageDialog.No
        onAccepted: { musicModel.stop(); browserBridgeController.toggle_bridge(true) }
        onRejected: { bridgeSwitch.checked = false }
    }

    ColumnLayout {
        anchors.centerIn: parent
        width: 400
        spacing: 20

        Rectangle {
            Layout.fillWidth: true
            height: 300
            color: "#1e1e1e"
            radius: 10
            border.color: "#333"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 20

                RowLayout {
                    Text { text: "🌐 Browser Bridge"; color: "white"; font.pixelSize: 18; font.bold: true; Layout.fillWidth: true }
                    Switch {
                        id: bridgeSwitch
                        checked: browserBridgeController.bridge_active
                        onCheckedChanged: {
                            if (checked && musicModel.is_playing) { stopTrackDialog.open() }
                            else { browserBridgeController.toggle_bridge(checked) }
                        }
                    }
                }

                Text { text: browserBridgeController.current_source_name; color: "#aaa"; font.pixelSize: 12; Layout.alignment: Qt.AlignHCenter }

                GridLayout {
                    columns: 2
                    Layout.fillWidth: true
                    Button { text: "🎬 Cinematic"; Layout.fillWidth: true; onClicked: browserBridgeController.apply_preset("cinematic") }
                    Button { text: "🎙️ Podcast"; Layout.fillWidth: true; onClicked: browserBridgeController.apply_preset("podcast") }
                    Button { text: "🎵 Enhancer"; Layout.fillWidth: true; onClicked: browserBridgeController.apply_preset("enhancer") }
                    Button { text: "🌙 Late Night"; Layout.fillWidth: true; onClicked: browserBridgeController.apply_preset("latenight") }
                }

                Button { text: "Close"; Layout.alignment: Qt.AlignRight; onClicked: browserBridgePopup.close() }
            }
        }
    }
}


### FILES TO EDIT

// 1. src/audio/io/mod.rs
// Tambahkan export module baru
pub mod audiobus;
pub mod audiooutput;
pub mod buffer;
pub mod decoder;
pub mod resample;
pub mod browserbridge;

// 2. src/ui/bridge/mod.rs
// Tambahkan export controller baru
pub mod core;
pub mod dspcontroller;
pub mod playerbridge;
pub mod queue;
pub mod browserbridgecontroller;

// 3. src/main.rs
// Daftarkan controller QML menggunakan qmetaobject
use crate::ui::bridge::browserbridgecontroller::BrowserBridgeController;
// Di dalam main() sebelum load QML engine tambahkan:
qmetaobject::qml_register_type::<BrowserBridgeController>(
    cstr!("com.loonixtunes.bridge"),
    1,
    0,
    cstr!("BrowserBridgeController"),
);

// 4. qml/Ui.qml (atau file yang berisi volume bar)
// Import komponen dan panggil Popup
import "components"

BrowserBridgeController { id: browserBridgeController }
BrowserBridgePopup { id: browserBridgePopup }

// Tambahkan ToolButton di samping slider volume
ToolButton {
    text: "🌐"
    font.pixelSize: 18
    ToolTip.text: "System Browser Bridge"
    ToolTip.visible: hovered
    onClicked: browserBridgePopup.open()
}