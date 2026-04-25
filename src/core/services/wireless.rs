/* --- loonixtunesv2/src/audio/wireless.rs | Wireless --- */


use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static SYSTEM_MUTED: AtomicBool = AtomicBool::new(false);
static BLUETOOTH_DETECTED: AtomicBool = AtomicBool::new(false);
static SHOULD_STOP: AtomicBool = AtomicBool::new(false);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DeviceType {
    Bluetooth,
    WiFi,
    Headset,
    InternalSpeaker,
    Unknown,
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::Unknown
    }
}

#[derive(Default)]
pub struct SystemAudioStatus {
    pub isMuted: bool,
    pub isBluetooth: bool,
    pub deviceType: String,
}

pub fn setSystemMuted(muted: bool) {
    SYSTEM_MUTED.store(muted, Ordering::SeqCst);
}

pub fn setBluetoothDetected(is_bt: bool) {
    BLUETOOTH_DETECTED.store(is_bt, Ordering::SeqCst);
}

pub fn isSystemMuted() -> bool {
    SYSTEM_MUTED.load(Ordering::Relaxed)
}

pub fn isBluetoothDetected() -> bool {
    BLUETOOTH_DETECTED.load(Ordering::Relaxed)
}

pub fn detectDeviceType(device_name: &str) -> DeviceType {
    let name = device_name.to_lowercase();

    if name.contains("bluez") || name.contains("a2dp") || name.contains("bluetooth") {
        return DeviceType::Bluetooth;
    }

    if name.contains("raop") || name.contains("network") || name.contains("upnp") {
        return DeviceType::WiFi;
    }

    if name.contains("usb") || name.contains("headset") || name.contains("headphone") {
        return DeviceType::Headset;
    }

    if name.contains("pci")
        || name.contains("analog")
        || name.contains("speaker")
        || name.contains("hdmi")
    {
        return DeviceType::InternalSpeaker;
    }

    DeviceType::Unknown
}

pub fn getSystemAudioStatus(device_name: &str) -> SystemAudioStatus {
    let dev_type = detectDeviceType(device_name);
    let is_bt = dev_type == DeviceType::Bluetooth;
    setBluetoothDetected(is_bt);

    let type_string = match dev_type {
        DeviceType::Bluetooth => "Bluetooth",
        DeviceType::WiFi => "WiFi Audio",
        DeviceType::Headset => "Headset/USB",
        DeviceType::InternalSpeaker => "Internal Speaker",
        DeviceType::Unknown => "Unknown Device",
    }
    .to_string();

    SystemAudioStatus {
        isMuted: SYSTEM_MUTED.load(Ordering::SeqCst),
        isBluetooth: is_bt,
        deviceType: type_string,
    }
}

pub fn getSystemAudioStatus_simple() -> SystemAudioStatus {
    SystemAudioStatus {
        isMuted: SYSTEM_MUTED.load(Ordering::SeqCst),
        isBluetooth: BLUETOOTH_DETECTED.load(Ordering::SeqCst),
        deviceType: if BLUETOOTH_DETECTED.load(Ordering::SeqCst) {
            "Bluetooth".to_string()
        } else {
            "Wired".to_string()
        },
    }
}

pub fn stop_system_check() {
    SHOULD_STOP.store(true, Ordering::SeqCst);
}

pub fn startSystemCheck() {
    SHOULD_STOP.store(false, Ordering::SeqCst);
    thread::spawn(|| loop {
        if SHOULD_STOP.load(Ordering::Relaxed) {
            break;
        }
        let output = Command::new("pactl")
            .args(["get-sink-mute", "@DEFAULT_SINK@"])
            .output();

        if let Ok(out) = output {
            let status_str = String::from_utf8_lossy(&out.stdout).to_lowercase();
            let muted = status_str.contains("yes");
            SYSTEM_MUTED.store(muted, Ordering::Relaxed);
        }

        let sink_info = Command::new("pactl").args(["get-default-sink"]).output();

        if let Ok(out) = sink_info {
            let sink_name = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            let is_bt = sink_name.contains("bluez")
                || sink_name.contains("a2dp")
                || sink_name.contains("bluetooth")
                || sink_name.contains("jbl")
                || sink_name.contains("airpods")
                || sink_name.contains("headphone")
                || sink_name.contains("headset");
            BLUETOOTH_DETECTED.store(is_bt, Ordering::Relaxed);
        }

        thread::sleep(Duration::from_millis(800));
    });
}

pub type WirelessManager = SystemAudioStatus;
