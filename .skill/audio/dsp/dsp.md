# DSP Architecture & Flow Protocol – Loonix Tunes
**Version:** 2.1
**Status:** Current Architecture (After Refactor)
**Principle:** Blind Engine Isolation + Engine Core Separation

---

## I. DSP Architecture (Current)

### Leveling System:
| Level | Nama | Location | Tanggung Jawab | Status |
|:---|:---|:---|:---|:---|
| **Level 1** | **Effect Units** | `audio/dsp/*.rs` | Pemrosesan sinyal individu (EQ, Compressor, dll). | Toggleable |
| **Level 2** | **Rack** | `audio/dsp/rack.rs` | Container & Routing urutan efek. | Toggleable |
| **Level 3** | **Chain** | `audio/dsp/chain.rs` | Thread-safe wrapper untuk rack. | Toggleable |
| **Level 4** | **Engine Core** | `audio/io/audiooutput.rs` | Normalizer + Limiter (ALWAYS ON). | **Always Active** |

### DSP Rack Contents (Toggleable):
```
EqPreamp (with headroom check)
    ↓
EQ Processor (10-band)
    ↓
Compressor
    ↓
BassBooster
    ↓
Reverb
    ↓
StereoEnhance
    ↓
Crystalizer
    ↓
SurroundProcessor
    ↓
StereoWidth
    ↓
PitchShifter
    ↓
MiddleClarity
    ↓
Crossfeed
```
**Note:** Normalizer and Limiter are NOT in DSP rack - they're in Engine Core.

---

## II. State Management: Atomics vs Mutex

### Audio Thread (Lock-free):
- **Atomics**: `atomic.load()` untuk performa tinggi
- **Usage**: Reading enable flags, gain values in audio callback

### UI Thread:
- **Atomics**: `atomic.store()` untuk mengubah parameter
- **Mutex**: Only for normalizer/limiter instances in engine core

### Larangan:
- Dilarang menggunakan Mutex berat di `audio/dsp/` processors
- Dilarang menggunakan tipe data Qt (`QString`, `QVariant`) di DSP
- Dilarang akses UI langsung dari DSP processors

---

## III. DSP Rack Implementation

### File: `src/audio/dsp/rack.rs`

### Current `build_processors()` Order:
```rust
pub fn build_processors(_settings: &DspSettings) -> Vec<Box<dyn DspProcessor + Send + Sync>> {
    let mut processors = Vec::new();

    // Normalizer & Limiter REMOVED - now in engine core

    processors.push(Box::new(EqPreamp::new()));        // With headroom check
    processors.push(Box::new(EqProcessor::new()));
    processors.push(Box::new(Compressor::new()));
    processors.push(Box::new(BassBooster::new()));
    processors.push(Box::new(Reverb::new()));
    processors.push(Box::new(StereoEnhance::new()));
    processors.push(Box::new(Crystalizer::new(48000.0)));
    processors.push(Box::new(SurroundProcessor::new()));
    processors.push(Box::new(StereoWidth::new()));
    processors.push(Box::new(PitchShifter::new()));
    processors.push(Box::new(MiddleClarity::new()));
    processors.push(Box::new(Crossfeed::new()));
    // Limiter REMOVED - now in engine core

    processors
}
```

### DSP Bypass Logic:
```rust
pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
    // DSP Rack bypassed if DSP OFF, but Engine Core (Normalizer/Limiter) still run
    if crate::audio::dsp::is_dsp_bypass() {
        output[..input.len()].copy_from_slice(input);
        return;
    }
    // Process all effects in rack...
}
```

---

## IV. EqPreamp with Headroom Management

### File: `src/audio/dsp/eqpreamp.rs`

### Purpose:
Volume adjustment before EQ, with headroom protection to prevent clipping.

### Headroom Check Implementation:
```rust
fn process(&mut self, input: &[f32], output: &mut [f32]) {
    let is_on = get_preamp_enabled_arc().load(Ordering::Relaxed);
    let gain = bits_to_f32(get_preamp_gain_arc().load(Ordering::Relaxed));

    if !is_on || (gain - 1.0).abs() < f32::EPSILON {
        output.copy_from_slice(input);
        return;
    }

    // HEADROOM CHECK: Prevent clipping before EQ
    let headroom_threshold = 0.5; // -6 dBFS headroom
    let mut peak = 0.0f32;
    for &sample in input.iter() {
        peak = peak.max(sample.abs());
    }

    // If applying gain would exceed headroom, reduce gain
    let safe_gain = if peak * gain > headroom_threshold {
        headroom_threshold / peak
    } else {
        gain
    };

    for (i, &sample) in input.iter().enumerate() {
        output[i] = sample * safe_gain;
    }
}
```

### Why Headroom Check?
- Prevents clipping when normalizer boosts quiet tracks
- Ensures signal doesn't exceed -6 dBFS before EQ processing
- Protects dynamic range for subsequent effects

---

## V. Engine Core Separation

### What's in Engine Core (Always ON):
1. **Normalizer** (`normalizer.rs`)
   - Applies RMS gain from scanner
   - Runs FIRST in signal chain
   - Smooth gain transitions

2. **Limiter** (`limiter.rs`)
   - Prevents clipping
   - Runs LAST in signal chain (before volume)
   - Stereo-linked peak limiting

### What's in DSP Rack (Toggleable):
- All cosmetic effects (EQ, Compressor, Reverb, etc.)
- EqPreamp (with headroom check)
- Bypassed when DSP OFF

---

## VI. Alur Inisialisasi (Bootstrapping)

### Fresh Install (Flow A):
*Picu: `dsp.json` tidak ditemukan.*
1. **Generator:** Membuat struktur file `dsp.json` default.
2. **Built-in Mapping (Index 0-5):** Rust membaca array `EQ_PRESETS` dan `FX_PRESETS` dari `presets.rs`.
3. **User Slot Allocation (Index 6-11):** Mengisi 6 slot `user_presets` dengan parameter netral.
4. **Handover:** File disimpan ke disk, lalu eksekusi otomatis dialihkan ke Flow B.

### Routine Reload (Flow B):
*Picu: Aplikasi dibuka.*
1. **UI List Population:** Rust membaca preset names dari `dsp.json`.
2. **Identify State:** Membaca `dsp_enabled` dan `active_preset_index`.
3. **The SSoT Fork:**
   - **Built-in (0-5):** Rust memanggil `get_eq_preset()` dan `get_fx_preset()` dari `presets.rs`.
   - **User (6-11):** Rust membaca parameter dari `user_presets` di `dsp.json`.
4. **Snapshot Deployment:** Data parameter disalin ke `default_fx_snapshot` di RAM.
5. **Atomic Dispatcher:** Nilai dikirim ke Atomics di mesin DSP.
6. **UI State Sync:** Rust menembakkan sinyal `_changed()` ke QML.

---

## VII. Alur Interaksi & Hybrid Persistence

### Auto-Save Navigasi (State Index):
- **Picu:** User memilih preset lain dari UI.
- **Aksi:** `active_preset_index` langsung disimpan ke `dsp.json`.

### Live & Volatile Parameters (Eksperimen):
- **Picu:** User menggeser slider atau menekan toggle efek.
- **Aksi:** Nilai ke RAM & Atomics. **TIDAK DISIMPAN** ke disk otomatis.
- **App Exit:** Eksperimen hilang, preset asli dimuat ulang.

### Manual Persistence (Save As):
- **Picu:** User menekan tombol **Save** atau **Save As**.
- **Aksi:** Konfigurasi volatile di RAM ditulis permanen ke `user_presets` (index 6-11).

---

## VIII. Protokol Reset

### Reset Indie (Per-Effect):
Mengembalikan efek tertentu ke titik awal loading preset.
- **Logic:** `Value = Snapshot.value` → `Atomic.store(Value)` → `UI Emit`.

### Reset All (The Nuclear Option):
Membersihkan state ke kondisi netral.

| Komponen | Status | Keterangan |
|:---|:---|:---|
| **DSP Power** | **ON** | Sistem tetap aktif. |
| **EQ Bands** | **0.0 dB** | Flat. |
| **FX Toggles** | **OFF** | Semua efek dibypass. |
| **Compressor** | **-14.0 dB** | Safe threshold. |
| **Snapshot** | **Updated** | Diperbarui ke kondisi "Clean". |

---

## IX. Prinsip Utama (The Blind Machine)

DSP adalah mesin buta. Dia tidak tahu UI, Preset, atau Slider 0-100%.
**DSP hanya menerima nilai final, memproses buffer, dan mengeluarkan suara.**

---

## X. File Structure (Current)

```
src/audio/dsp/
├── mod.rs              # Module definitions, DspProcessor trait, DspSettings
├── rack.rs            # DSP Rack (NO Normalizer/Limiter)
├── chain.rs           # Thread-safe DspChain wrapper
├── eqpreamp.rs        # EqPreamp WITH headroom check
├── eq.rs              # EQ Processor (10-band)
├── compressor.rs      # Compressor
├── bassbooster.rs     # BassBooster
├── reverb.rs          # Reverb
├── stereoenhance.rs   # StereoEnhance
├── crystalizer.rs     # Crystalizer
├── surround.rs        # SurroundProcessor
├── stereowidth.rs     # StereoWidth
├── pitchshifter.rs    # PitchShifter
├── middleclarity.rs   # MiddleClarity
├── crossfeed.rs       # Crossfeed
├── normalizer.rs      # Normalizer (used by ENGINE CORE)
├── limiter.rs         # Limiter (used by ENGINE CORE)
├── biquad.rs          # Biquad filter (utility)
└── preamp.rs          # Old Preamp (deprecated, use EqPreamp)
```

---

## XI. Recent Changes (v2.1)

### Refactor Completed:
- [x] Removed Normalizer from DSP rack (`rack.rs`)
- [x] Removed Limiter from DSP rack (`rack.rs`)
- [x] Added Normalizer to engine core (`audiooutput.rs`)
- [x] Added Limiter to engine core (`audiooutput.rs`)
- [x] Normalizer runs BEFORE DSP rack in signal chain
- [x] Limiter runs AFTER DSP rack in signal chain
- [x] Both always active regardless of DSP on/off
- [x] EqPreamp headroom management implemented
- [x] Updated DSP bypass comments in `rack.rs`

### Signal Chain Verification:
```
Source → [Normalizer] → [DSP Rack: EqPreamp → ... → FX] → [Limiter] → Volume → Output
         ↑                                  ↑
    Engine Core (Always ON)         Toggleable (DSP ON/OFF)
```
