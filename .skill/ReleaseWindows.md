# Loonix Tunes v2.0.0 — Native Windows Music Player

**Loonix Tunes** is a native, high-performance desktop music player for Windows 10/11 (64-bit). Built with Rust + Qt6 + FFmpeg, designed for audio enthusiasts who want speed, customization, and full DSP control.

---

## Features

### Audio Engine
- FFmpeg-based decoding: mp3, wav, flac, ogg, m4a, aac, wma, plus any format FFmpeg supports
- WASAPI audio output via CPAL — low-latency, exclusive/shared mode
- Lock-free ringbuffer, atomic audio thread — no blocking in audio callback
- Hardware sample rate conversion and resampling
- Gapless playback, pre-buffering, seek with flush + resync

### DSP Engine — 20 Modules
Unified DSP rack with per-effect bypass, real-time control:

| Module | Description |
|--------|-------------|
| 10-Band EQ | 31–16k Hz, biquad IIR (lowshelf/peak/highshelf) |
| Preamp | Gain stage with cubic soft-clipping |
| Compressor | Threshold, ratio, attack/release, makeup gain |
| Bass Booster | 4 modes: Deep, Soft, Punch, Warm |
| Reverb | Schroeder reverb — 3 modes: Studio, Stage, Stadium |
| Crystalizer | High-frequency enhancement |
| Surround | Virtual surround sound |
| Stereo Enhance | Stereo field widening |
| Mono Stereo | Mono-to-stereo width control |
| Middle Clarity | Mid-range clarity enhancement |
| Crossfeed | Headphone crossfeed for spatial audio |
| Pitch Shifter | Real-time pitch shifting via RubberBand (formant preservation) |
| Limiter | Brickwall limiter, stereo-linked |
| Normalizer | Audio normalizer (Slow/Balanced/Fast) |

### Presets
- 6 factory presets: LOONIX, BASS, ROCK, POP, METAL, JAZZ
- 6 user-preset slots: save/load full DSP state (EQ + all FX)
- Preamp calibrated at −8 dB for headroom

### UI & Customization
- Frameless window with custom titlebar (drag, snap, minimize, maximize, close, always-on-top pin)
- **Theme system**: 8 built-in themes (Loonix, Blue, Green, Monochrome, Orange, Pink, Red, Yellow) + 3 custom theme slots
- **Theme Editor**: 50+ color properties organized by section (Backgrounds, Header, Player, Tabs, Playlist, DSP)
- **Keyboard shortcuts**: Playback, Volume, Modes, Navigation, Queue, Library, DSP toggle, Presets, Window
- **AB Loop** with state machine
- Instant FX toggles in player bar: Bass (B), Crystalizer (C), Surround (S), DSP popup, Theme cycle (T)
- Seekbar with drag + scroll (5s steps)

### Library & Playback
- Folder-based music library navigation
- Favorites, Queue, and External tabs
- Shuffle, repeat modes
- Drag-and-drop file loading
- Update checker (GitHub releases API)

### Preferences
- About (version, logo, GitHub check + download)
- Appearance (theme selector, create/edit/rename themes)
- Donate (Saweria + Ko-fi)
- Report Bug (GitHub Issues with auto-generated system info)
- Shortcuts reference

---

## System Requirements

- **OS**: Windows 10 or Windows 11 (64-bit)
- **Architecture**: x86_64
- **Storage**: 130 MB (extracted - ALL INCLUDE)
- **Audio**: Any WASAPI-compatible device (built-in, USB, Bluetooth)

---

## Installation

1. Download `LoonixTunesWin64.zip` from the latest release
2. Extract the archive to any folder
3. Run `LoonixTunesWin64.exe` — no installation required, no registry changes

All dependencies (Qt6 DLLs, FFmpeg, RubberBand) are bundled inside the zip.

---

## Build from Source

```powershell
# Prerequisites: Rust 1.75+, Visual Studio 2022 C++ toolchain
git clone https://github.com/citzeye/loonix-tunes-windows
cd loonix-tunes-windows
cargo build --release
```

Qt6 SDK (6.6+) must be discoverable via `find-msvc-tools` or set `QT6_DIR`. The `.vendor/qt/` folder in the repo contains the required Qt DLLs for deployment.

---

## License

GNU General Public License v3.0 — see [LICENSE](LICENSE).

---

## Credits

- **Author**: citzeye
- **Built with**: Rust, Qt 6, FFmpeg, CPAL, RubberBand
- **Icon**: Nerd Fonts (symbols for UI)
