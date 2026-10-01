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
