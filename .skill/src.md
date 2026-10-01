# Project Architecture & Folder Structure – Loonix Tunes
**Version:** 2.0  
**Design Pattern:** Domain-Driven Design (DDD) & Blind Engine Isolation  

Dokumen ini adalah panduan mutlak untuk tata letak file (Project Structure). Setiap modul memiliki batas domain yang ketat. **Dilarang keras menyilangkan dependensi yang melanggar aturan Leveling (terutama pada folder DSP).**

---

## 📂 Struktur Direktori Utama

```text
src/
├── audio/                 # 🎧 THE SOUND FACTORY: Murni Pemrosesan & Buffer Suara
│   │                      # Constraint: Dilarang import UI, Config, atau Preset ke folder ini.
│   │
│   ├── dsp/               # Level 1-3: The Blind Machine (Hanya menerima angka final f32/bool)
│   │   ├── bassbooster.rs
│   │   ├── biquad.rs
│   │   ├── chain.rs       # Level 3: Entry point dsp, urus routing limiter & preamp
│   │   ├── compressor.rs
│   │   ├── crossfeed.rs
│   │   ├── crystalizer.rs
│   │   ├── eq.rs
│   │   ├── eqpreamp.rs
│   │   ├── limiter.rs
│   │   ├── middleclarity.rs
│   │   ├── mod.rs
│   │   ├── normalizer.rs
│   │   ├── pitchshifter.rs
│   │   ├── preamp.rs
│   │   ├── rack.rs        # Level 2: Penampung semua unit efek & routing
│   │   ├── reverb.rs
│   │   ├── rubberbandffi.rs
│   │   ├── stereoenhance.rs
│   │   ├── stereowidth.rs
│   │   └── surround.rs
│   │
│   ├── engine/            # Level 4: The Driver (Waktu & Playback Logic)
│   │   ├── abrepeat.rs    # Logika A-B repeat (pembanding timestamp)
│   │   ├── clock.rs
│   │   ├── engine.rs      # Main loop audio
│   │   ├── library.rs     # Engine state library
│   │   ├── mod.rs
│   │   ├── scheduler.rs
│   │   └── seek.rs
│   │
│   └── io/                # Hardware, Stream & Konversi Sinyal
│       ├── audiobus.rs
│       ├── audiooutput.rs
│       ├── buffer/
│       │   ├── mod.rs
│       │   └── ringbuffer.rs
│       ├── decoder.rs
│       ├── mod.rs
│       └── resample.rs
│
├── core/                  # 🧠 THE BRAIN: Logika Bisnis, State, & Interaksi OS
│   │                      # Constraint: Tempat SSoT (Single Source of Truth) berada.
│   │
│   ├── config/            # Manajemen State & Penyimpanan
│   │   ├── appconfig.rs   # Global App settings
│   │   ├── dspconfig.rs   # Parser & handler untuk dsp.json (Volatile & File Sync)
│   │   └── presets.rs     # SSoT Hardcoded Konstanta untuk Built-in Presets (0-5)
│   │
│   ├── library/           # Manajemen Database Lagu & File
│   │   ├── favorites.rs
│   │   ├── library.rs     # Logika CRUD library
│   │   ├── metadata.rs    # Ekstraksi ID3 / Vorbis Comments
│   │   └── scanner.rs     # Crawler folder musik
│   │
│   └── services/          # Layanan Background & Integrasi OS
│       ├── fileservice.rs
│       ├── playback.rs
│       ├── sysmedia.rs    # MPRIS / Media Keys hardware bindings
│       └── wireless.rs    # Deteksi ganti output audio (Bluetooth/Jack)
│
├── ui/                    # 🖥️ THE FACE: Presentasi & Jembatan QML/Qt
│   │                      # Constraint: UI tidak memproses data, hanya memanggil core/audio.
│   │
│   ├── components/        # Elemen Visual Murni
│   │   ├── popup.rs
│   │   └── theme.rs
│   │
│   ├── bridge/            # QObject Bindings (Rust <-> C++/QML)
│   │   ├── core.rs        # Main Context Bridge
│   │   ├── dspcontroller.rs # Manajer Sinyal DSP & Protokol Snapshot
│   │   ├── playerbridge.rs
│   │   └── queue.rs
│   │
│   ├── reportbug.rs
│   └── updater.rs
│
└── main.rs                # 🚀 Entry Point: Inisialisasi Logger, Config, dan QGuiApplication