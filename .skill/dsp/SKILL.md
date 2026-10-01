---
name: dsp
description: Arsitektur DSP: crystalizer, EQ, normalizer, reverb, surround.
---
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

---

## audio/dsp/crystalizer.md

```
Act as an expert Rust audio DSP and Qt/QML developer. I am building a music player with a native Rust audio backend and a QML frontend. I already have the base `.rs` and `.qml` files ready. 

I need you to write the implementation for a "Crystalizer" native DSP effect. 

Here are the strict architectural and DSP requirements:

1. **The DSP Algorithm (Crystalizer):**
   - The effect should act as a harmonic exciter + high-shelf boost.
   - Split the incoming audio into a "dry" and "wet" path.
   - On the wet path: Apply a High-Pass Filter (cutoff around 3kHz - 5kHz).
   - Then, apply a soft-clipping/saturation function (e.g., using `tanh`) to the high-passed signal to generate high-frequency harmonics (sparkle).
   - Finally, mix the wet signal back with the dry signal.

2. **Rust Backend Implementation (`crystalizer.rs` or similar):**
   - Create a `Crystalizer` struct.
   - It must have a `process(&mut [f32])` or `process_sample(sample: f32) -> f32` method to process audio buffers in real-time.
   - Expose an "Amount" or "Intensity" parameter (range 0.0 to 1.0).
   - **CRITICAL:** The audio thread must be lock-free. Use `std::sync::atomic::AtomicF32` (or `AtomicU32` with bit-casting) to receive parameter updates from the QML GUI thread. Do NOT use `Mutex` for audio parameters.

3. **QML Frontend Integration (`Ui.qml` SECTION SLIDER CONTROL ):**
   - Provide a clean QML implementation using a `Slider` to control the Crystalizer's "Amount".
   - Show the exact C++/Rust boilerplate or Qt bindings needed to connect this QML slider's `onValueChanged` signal to update the `Atomic` variable in the Rust backend without blocking the UI or Audio threads.

Please provide the minimal, modular, and highly performant code for both the Rust DSP logic and the QML UI binding.
```

---

# EQ System — LoonixTunesWin64

## Architecture

```
eqpreamp → eq (10 bands) → compressor → bass → reverb → stereo enhance → crystalizer → surround → mono/stereo → pitch → middle clarity → crossfeed
```

### Processing Order (rack.rs)

1. `EqPreamp` – broadband gain/attenuation (headroom)
2. `EqProcessor` – 10-band peaking/shelving EQ
3. FX chain: Compressor → BassBooster → Reverb → StereoEnhance → Crystalizer → SurroundProcessor → MonoStereo → PitchShifter → MiddleClarity → Crossfeed

All DSP is bypassed when DSP OFF (`dsp_enabled = false`).

## Preamp (`eqpreamp.rs`)

- **Stored as linear gain** (f32 bits in `AtomicU32`), range: -20 dB to +20 dB
- Soft-clipper at ±0.95 (cubic curve) prevents digital clipping
- Always ON (cannot be disabled by user toggle)
- Applied BEFORE the 10-band EQ

## 10-Band EQ (`eq.rs`)

### Frequencies
```
[31, 62, 125, 250, 500, 1000, 2000, 4000, 8000, 16000] Hz
```

### Filter Types (per band)
| Index | Freq | Type | Q |
|-------|------|------|----|
| 0 | 31 Hz | Low-shelf | 0.5 |
| 1 | 62 Hz | Low-shelf | 0.5 |
| 2 | 125 Hz | Peaking | 1.414 |
| 3 | 250 Hz | Peaking | 1.414 |
| 4 | 500 Hz | Peaking | 1.0 |
| 5 | 1000 Hz | Peaking | 1.0 |
| 6 | 2000 Hz | Peaking | 1.0 |
| 7 | 4000 Hz | Peaking | 1.0 |
| 8 | 8000 Hz | High-shelf | 0.5 |
| 9 | 16000 Hz | High-shelf | 0.5 |

### Biquad Implementation

- Direct Form I (DF1) with 64-bit internal processing
- Anti-denormal bias (1e-18)
- NaN guard: resets state on non-finite output
- Audio EQ Cookbook formulas (Robert Bristow-Johnson)

## Factory Presets (`presets.rs`)

| Name | Gains | Preamp | Character |
|------|-------|--------|-----------|
| LOONIX | [0, +8, −8, −8, −8, −8, −8, −4, +4, −4] | −8 dB | V-shape, extreme mid scoop, slight air boost |
| BASS | [+6, +8, 0, −5, −5, −5, −5, −5, +2, −5] | −8 dB | Heavy low-end, recessed mids |
| ROCK | [+6, +8, +1, −2, −4, −2, +1, +3, +6, +6] | −8 dB | Smile curve, boosted lows/highs |
| POP | [0, +1, +2, +4, +6, +2, +4, +2, +1, 0] | −8 dB | Mid-forward, vocal presence |
| METAL | [+6, +8, 0, −4, −6, −4, 0, +3, +5, +6] | −8 dB | Aggressive V-shape, scooped mids |
| JAZZ | [+4, +3, +2, −1, −1, −1, −1, +2, +3, +4] | −8 dB | Gentle smile, subtle warmth |

- All presets use **−8 dB preamp** (pure headroom, no loudness normalization)
- Max gain = +8 dB, so peak = 0 dBFS (no clipping)
- Gains can be tweaked per-preset for volume balance

## Fader

- Global offset (−20 to +20 dB) applied on top of all band gains
- Affects all bands equally (shifts the entire EQ curve)
- Default = 0 (neutral)
- Reset to 0 on preset change (factory presets)
- User presets store fader value as `macro_val`

## Autio Engine Integration

### Atomic State Flow

```
UI (QML) → DspController → AtomicU32 (per band + preamp) → EqProcessor::sync_from_atomics() → BiquadFilter coefficients
```

### State Management

- `EqProcessor.sync_from_atomics()` polls atomics at the start of every audio callback
- Only recalculates filter coefficients when gain changes (>0.001 dB delta)
- `is_flat` flag: when all bands ≈ 0 dB and preamp ≈ 0 dB, entire EQ bypasses (direct copy)

### Sample Rate Changes

- Dirty flag (`RATE_CHANGED`) triggers full filter coefficient recalculation
- Supported rates: 22050–192000 Hz (validated via `samplerate.rs`)

## DSP Processing (`chain.rs`)

### Buffer-Fed Processing

```
input → output (copy)
foreach processor:
    temp ← output
    processor.process(temp, output)
```

MAX_BUFFER = 8192 samples. If input exceeds this, bypass all DSP (safety guard).

### Thrash-Buffer Safety

- Linear state-carrying approach (no ping-pong)
- Each processor reads from temp_buffer, writes to output
- No shadowing bugs (fixed in chain.rs refactor)

## QML UI (`Dsp.qml`)

### EQ Section Layout

```
Row 1: Number display (current values)
Row 2: Vertical sliders (user interaction)
Row 3: Name labels (double-click resets to preset)
```

### User Presets (6 slots)

- Gains + macro (fader offset) saved per user slot
- User presets use 0 dB preamp (no auto-normalization)
- Names stored in `DspConfig.user_preset_names`
- Gains stored in `DspConfig.user_preset_gains`
- FX settings also stored per user slot

### Seek Safety

- On seek: `EqProcessor.reset()` clears all biquad state (zero delay lines)
- Prevents filter state corruption after format change / flush

---

# Normalizer Architecture (Loonix-Tunes)

## Overview

Normalizer di Loonix-Tunes menggunakan sistem **Fixed Gain per Track** dengan **Smoothing Transitions**. Gain dihitung sekali saat track di-load, lalu di-apply dengan transisi mulus saat berganti track.

---

## Arsitektur DSP Chain

```
Input → Preamp → Normalizer → EQ → ... → Limiter → Output
```

### Urutan Processing:
1. **Preamp** - Gain adjustment manual (headroom control)
2. **Normalizer** - Gain dari scanner (per-track, smooth transition)
3. **EQ** - 10-band biquad
4. **Spatial FX** - Crystalizer, Surround, StereoWidth, dll
5. **Limiter** - Final stage protection

---

## Alur Kerja

### 1. Saat Track Change (engine.rs)

```
Track change
    ↓
set_normalizer_gain(1.0)      ← langsung set gain=1.0 agar bisa langsung play
    ↓
start_audiooutput()
    ↓
spawn scanner thread           ← hitung gain di background (non-blocking)
    ↓
scanner selesai → store gain ← hasil di-store ke gain_arc atomic
```

### 2. Saat DSP Processing (setiap sample)

```
Audio callback (audiooutput.rs)
    ↓
dsp_chain.process(input, output)
    ↓
Normalizer.process():
    - baca gain_arc (dari scanner)
    - baca smoothing_arc (dari UI)
    - apply: current_gain += (target - current_gain) * smoothing
    - apply: soft_clip(input[i] * current_gain)
```

---

## Scanner (scanner.rs)

### Fungsi Utama

```rust
pub fn calculate_track_gain(path: &str, params: &ScanParams) -> f32
```

### Parameter:

| Parameter | Default | Range | Keterangan |
|-----------|---------|-------|------------|
| target_lufs | -14.0 | -24.0 s/d -10.0 | Standar streaming modern |
| true_peak_dbtp | -1.5 | -3.0 s/d 0.0 | Batas plafon biar nggak clipping |
| max_gain_db | 12.0 | 0 s/d +12.0 | Max gain boost yang diizinkan |

### Constraints:

1. **Target LUFS** - Rata-rata loudness track disesuaikan ke target
2. **Max Gain** - Gain nggak boleh melebihi ini
3. **True Peak Ceiling** - Kalau projected peak > ceiling, gain direduce

### Caching:

Hasil scan di-cache di `GAIN_CACHE` (static OnceLock<Mutex<HashMap>>). Track yang sama nggak perlu di-scan ulang.

---

## Normalizer (normalizer.rs)
Refactor file normalizer.rs agar menjadi track gain applicator yang stabil dan transparan, bukan dynamic limiter.

Tujuan:
Gain hanya dipakai di awal lagu untuk menyamakan volume antar track.
Bukan untuk compress / react ke transient.

🎯 Objective

Ubah AudioNormalizer menjadi:

Fixed gain applicator
Dengan short fade-in smoothing
Tanpa transient detection
Tanpa dynamic gain chasing
Tanpa per-sample reactivity

Ini bukan compressor.

1️⃣ Hapus Instant Attack Logic

Hapus seluruh bagian ini dari process():

let input_abs = input[i].abs();
if input_abs > self.current_gain * 2.0 {
    self.current_gain = (target).max(input_abs * 1.05);
} else {
    self.current_gain += (target - self.current_gain) * smoothing;
}

Tidak boleh ada deteksi transient.

2️⃣ Gain Behavior Baru

Implementasi baru:

fixed_gain adalah hasil dari scanner
current_gain mulai dari 1.0 saat lagu baru
Smooth menuju fixed_gain selama beberapa ratus milidetik
Setelah itu stabil dan tidak berubah

Gunakan ini:

self.current_gain += (self.fixed_gain - self.current_gain) * smoothing;

Itu saja.

Tidak ada kondisi lain.

3️⃣ Soft Clip Tetap Dipakai

Pertahankan soft_clip().

Tapi tambahkan komentar:

// Safety clipper to prevent overshoot after normalization.
// Should rarely engage if scanner peak constraint works correctly.
4️⃣ Default Smoothing Lebih Stabil

Ubah default smoothing:

0.002 → 0.0015

Transition sedikit lebih halus.

5️⃣ Tambahkan Reset Logic yang Benar

Pastikan reset() mengembalikan:

self.current_gain = 1.0;

Bukan ke fixed_gain.

Karena fade harus terjadi setiap lagu baru.

6️⃣ Jangan Ubah
Jangan ubah atomic sharing
Jangan ubah Arc structure
Jangan tambahkan dependency
Jangan ubah public API
7️⃣ Tambahkan Komentar di Atas Struct

Track-level gain applicator.
Applies precomputed RMS gain from scanner.
Not a compressor. Not a limiter.

## Catatan Performance

- Scanner jalan di dedicated thread, nggak block audio thread
- Normalizer menggunakan lock-free atomics
- Gain calculation cuma sekali per track (di-cache)
- Soft clip pakai `tanh()` yang optimized di Rust stdlib

STATUS VERIFIKASI NORMALIZER: MATCH (Sesuai Arsitektur)

1. Workflow Perpindahan Track (engine.rs)
   - Status: SESUAI
   - Logic: engine.set_normalizer_gain(1.0) dipanggil langsung saat load agar lagu bunyi duluan, baru kemudian scanner thread dispawn secara non-blocking. Hasil scan di-store ke atomic gain_arc.

2. DSP Processing & Smoothing (normalizer.rs)
   - Status: SESUAI
   - Rumus: current_gain += (target - current_gain) * smoothing.
   - Konstanta: Slow (0.001), Balanced (0.002), Fast (0.005) cocok dengan rencana.
   - Verifikasi Transisi: Untuk preset Balanced (0.002), waktu tempuh mencapai 95% target adalah ~31ms pada 48kHz. Akurat.

3. Perlindungan Output (Limiter/Clip)
   - Status: SESUAI
   - Logic: Menggunakan soft_clip dengan (0.99 * sample.tanh()).clamp(-0.99, 0.99). Ini "safety net" yang pas buat nahan lonjakan gain hasil normalisasi.

4. Scanner & Caching (scanner.rs)
   - Status: SESUAI
   - Parameter: Target -14.0 LUFS, True Peak -1.5 dBTP, dan Max Gain 12.0 dB sudah ter-hardcode sebagai default params.
   - Caching: Menggunakan GAIN_CACHE (OnceLock<Mutex<HashMap>>) sehingga track yang sama tidak perlu di-scan ulang.

5. Urutan DSP Chain
   - Status: SESUAI
   - Posisi: Input -> Preamp -> Normalizer -> EQ -> ... -> Limiter. Normalizer diletakkan sebelum EQ agar tone shaping tidak mengacaukan perhitungan loudness awal.

---

## audio/dsp/reverb.md

```
Kamu sedang mengerjakan Loonix Tunes (production app).
Backend Rust, audio engine terpisah dari UI, DSP chain sudah ada (EQ → Compressor → Bass → dll).

Sekarang tambahkan Reverb DSP module profesional dengan spesifikasi berikut.

1️⃣ TUJUAN

Implement:

Reverb DSP engine internal (bukan placeholder)
3 Mode: Studio, Stage, Stadium
1 intensity slider (0–100%)
Proper state sync backend ↔ QML
Masuk ke DSP chain sebelum stereo enhancer
State tersimpan di config

Tidak boleh fake processing. Harus benar-benar memproses audio buffer.

2️⃣ DSP ARCHITECTURE (WAJIB)

Gunakan algoritma:

Schroeder + Freeverb style hybrid

Komponen wajib:

4x parallel comb filters
2x serial allpass filters
stereo cross feedback
damping lowpass di feedback loop

Harus realtime-safe.
Tidak boleh alokasi memory di audio thread.

3️⃣ PARAMETER YANG HARUS DIBUAT

Tambahkan struct baru:

pub struct ReverbParams {
    pub room_size: f32,
    pub decay_time: f32,
    pub pre_delay: f32,
    pub damping: f32,
    pub width: f32,
    pub wet: f32,
    pub dry: f32,
    pub early_reflection_mix: f32,
    pub late_reflection_mix: f32,
}

Buat ReverbMode enum:

pub enum ReverbMode {
    Studio,
    Stage,
    Stadium,
}
4️⃣ FIXED PARAMETER BASE (JANGAN DIUBAH)

Mode menentukan base parameter berikut:

STUDIO
room_size = 0.25
decay_time = 0.8
pre_delay = 0.008
damping = 0.65
width = 1.1
wet = 0.18
dry = 1.0
early_reflection_mix = 0.7
late_reflection_mix = 0.3
STAGE
room_size = 0.55
decay_time = 1.8
pre_delay = 0.018
damping = 0.55
width = 1.3
wet = 0.28
dry = 1.0
early_reflection_mix = 0.5
late_reflection_mix = 0.5
STADIUM
room_size = 0.85
decay_time = 3.5
pre_delay = 0.035
damping = 0.4
width = 1.5
wet = 0.38
dry = 1.0
early_reflection_mix = 0.35
late_reflection_mix = 0.65
5️⃣ INTENSITY SLIDER (WAJIB BEGITU)

Slider 0–100% hanya memodifikasi:

wet = base_wet * (amount / 100.0)

Parameter lain tidak berubah.

6️⃣ DSP CHAIN POSITION (KRITIS)

Chain harus:

Engine Output
→ EQ
→ Compressor
→ Bass Enhancement
→ Reverb   ← DI SINI
→ Stereo Enhancer
→ Limiter (jika ada)
→ PulseAudio Output

Tidak boleh sebelum EQ.
Tidak boleh setelah stereo enhancer.

7️⃣ BACKEND API WAJIB

Expose ke QML:

#[qt_property(i32, notify = reverb_mode_changed)]
pub reverb_mode: i32;

#[qt_property(i32, notify = reverb_amount_changed)]
pub reverb_amount: i32;

Functions:

fn set_reverb_mode(&mut self, mode: i32);
fn set_reverb_amount(&mut self, amount: i32);

Harus update DSP engine langsung.
Harus thread-safe (gunakan Arc<Mutex<...>> atau atomic param block).

8️⃣ QML IMPLEMENTATION

Tambahkan:

ComboBox: Studio / Stage / Stadium
Slider: 0–100
On change → panggil backend

Harus reactive.
Harus sync saat load preset.
Harus tersimpan di config.

9️⃣ CONFIG INTEGRATION

Tambahkan ke AppConfig:

reverb_enabled: bool
reverb_mode: i32
reverb_amount: i32

Load saat startup.
Apply ke DSP engine setelah audio engine ready.

🔟 VALIDATION TEST

AI harus memastikan:

Tidak ada denormal CPU spike
Tidak clipping
Tidak memory allocation di callback
Tidak crash saat ganti mode realtime
Tidak pop saat ganti amount
OUTPUT YANG DIHARAPKAN
File Rust reverb engine lengkap
Integrasi ke DSP chain
Update core.rs binding
Update QML
Update config
Tidak placeholder
Tidak pseudo code


# Plan: Reverb DSP Module Implementation
1. Core Data Structures
ReverbMode enum (src/audio/dsp/reverb.rs):
pub enum ReverbMode {
    Studio = 0,
    Stage = 1,
    Stadium = 2,
}
ReverbParams struct (src/audio/dsp/reverb.rs):
pub struct ReverbParams {
    pub room_size: f32,
    pub decay_time: f32,
    pub pre_delay: f32,
    pub damping: f32,
    pub width: f32,
    pub wet: f32,
    pub dry: f32,
    pub early_reflection_mix: f32,
    pub late_reflection_mix: f32,
}
2. DSP Engine Implementation
ReverbProcessor struct (src/audio/dsp/reverb.rs):
- Internal buffers for delay lines (pre-allocated, fixed size)
- 4 parallel comb filters with different delay times
- 2 serial allpass filters
- Stereo cross-feedback
- Damping lowpass in feedback loop
- No heap allocations in process() method
- Denormal prevention (flush-to-zero or similar)
Fixed Base Parameters per mode:
- Studio: room_size=0.25, decay_time=0.8, etc.
- Stage: room_size=0.55, decay_time=1.8, etc.
- Stadium: room_size=0.85, decay_time=3.5, etc.
Processing method:
fn process(&mut self, input: &[f32], output: &mut [f32]) {
    // Apply reverb effect with current parameters
    // wet = base_wet * (intensity / 100.0)
    // Process stereo channels with cross-feedback
}
3. Backend Integration
MusicModel additions (src/ui/core.rs):
- Properties:
    pub reverb_mode: qt_property!(i32; NOTIFY reverb_mode_changed),
  pub reverb_amount: qt_property!(i32; NOTIFY reverb_amount_changed),
  - Methods:
    fn set_reverb_mode(&mut self, mode: i32) { /* update params + emit signal */ }
  fn set_reverb_amount(&mut self, amount: i32) { /* update wet level + emit signal */ }
  - DSP chain integration: Insert after Bass Enhancement, before Stereo Enhancer
- Thread safety: Use atomic parameter block
4. Config Updates
AppConfig additions (src/audio/config.rs):
pub reverb_enabled: bool,
pub reverb_mode: i32,    // 0=Studio, 1=Stage, 2=Stadium
pub reverb_amount: i32,  // 0-100 percentage
- Default values matching Studio mode at 50% intensity
- Save/load integration with existing config system
5. QML Components
Reverb Mode Selector (qml/ui/Dsp.qml):
- Based on BassModeSelector pattern
- ComboBox or 3-button selector: Studio/Stage/Stadium
- Bound to musicModel.reverb_mode
Reverb Intensity Slider (qml/ui/Dsp.qml):
- Horizontal slider 0-100
- Bound to musicModel.reverb_amount
- With value display showing "%"
- Reset button to 50%
Reverb Toggle Box (qml/ui/Dsp.qml):
- Standard FxToggleBox for enabling/disabling
- Controls enabled state of mode selector and intensity slider
6. DSP Chain Position
Modify audio processing pipeline in:
- src/audio/engine.rs or similar
- Order: Input → EQ → Compressor → Bass → Reverb → Stereo Enhancer → Limiter → Output
7. State Synchronization Approach
Initialization:
1. Load AppConfig from disk
2. Set MusicModel properties from config
3. Initialize ReverbProcessor with mode/amount from config
4. Emit all property signals after QML bindings established
Runtime Updates:
- QML → Backend: Slider/toggle → set_* methods → update params → emit signals
- Backend → QML: Property changes → automatic QML binding updates
- Preset loading: load_fx_preset() → set internal params → emit signals
Special Handling:
- Intensity slider only modifies wet parameter
- Mode changes reset all parameters to base values for that mode
- Room size and damp parameters exposed for advanced tuning (if needed)
8. Validation Checklist
Performance:
- Verify no heap allocations in audio callback
- Check for denormal prevention measures
- Profile CPU usage in release build
Audio Quality:
- Test for clipping at extreme settings
- Verify no pops/clicks when changing parameters
- Validate stereo imaging is preserved
- Check frequency response matches expected reverb curves
Integration:
- Preset save/restore includes reverb state
- UI correctly reflects backend state
- DSP chain order verified with signal tracer
- Bypass functionality works correctly
9. Implementation Order
1. Create ReverbProcessor with fixed parameters
2. Add backend properties and methods
3. Integrate into DSP chain
4. Update config system
5. Create QML components
6. Connect signals and test
7. Performance validation
8. Polish UI/UX
This plan addresses all requirements while maintaining existing architecture patterns. The implementation follows the established conventions for other DSP modules (Bass, Compressor, etc.) in the codebase.
```

---

# Surround Processor - Technical Documentation

## Overview
Surround adalah efek stereo enhancement yang menggunakan **Side Injection Method** untuk memperlebar stereO image tanpa mengubah sinyal asli (left/right). Berbeda dengan Mid-Side (M/S) matrix konvensional, metode ini mempertahankan klaritas asli lagu.

## Architecture

### Side Injection Method (Current Implementation)
```
Original Signal:     L, R
Side Signal:        S = L - R
Injection Gain:    G = (width - 1.0) * 0.5

Output:             L' = L + (S * G)
                    R' = R - (S * G)
```

**Key Characteristics:**
- `width = 1.0` → `G = 0.0` → **Bypass** (100% identical to OFF)
- `width = 2.0` → `G = 0.5` → **Maximum Wide**
- Original L/R signals **untouched** - only side difference is injected

### Comparison: M/S vs Side Injection

| Aspect | Mid-Side Matrix | Side Injection (Current) |
|--------|-------------------|---------------------------|
| Original Signal | Deconstructed (L+R, L-R) | Preserved (L, R untouched) |
| Bypass at 1.0 | Requires *0.5 reconstruction | Native (G=0 → L'=L, R'=R) |
| Phase Issues | Possible (filter on side) | Minimal (no mid reconstruction) |
| Transient Clarity | Can be "pengeng" | Crystal clear |

## Parameters

### UI Range: 1.0 - 2.0
- **1.0** = Bypass (identical to OFF)
- **1.5** = Medium Wide
- **2.0** = Maximum Wide

### Internal Mapping
```rust
// UI slider: 1.0 - 2.0 (direct, no conversion)
// In injection calculation:
let injection_gain = (self.current_width - 1.0) * 0.5;
```

### Preset Values (FX_PRESETS)
| Preset | Surround Width | Description |
|--------|---------------|-------------|
| LOONIX | 1.75 | Wide |
| BASS | 1.9 | Very Wide |
| ROCK | 1.65 | Wide |
| POP | 1.5 | Medium Wide |
| METAL | 1.8 | Very Wide |
| JAZZ | 1.6 | Wide |

## Signal Flow

### 1. High-Pass Filter (Bass Protection)
```rust
// Only processes the SIDE signal to preserve bass in center
if bass_safe > 0.5 {
    side = self.high_pass(side);
}
```
- **Cutoff**: 60Hz (preserves drum thump & fundamental bass)
- **Purpose**: Prevent bass frequencies from being widened (keep them mono/center)

### 2. Injection Gain Calculation
```rust
let injection_gain = (self.current_width - 1.0) * 0.5;
```
- `width = 1.0` → `injection_gain = 0.0` → **Bypass**
- `width = 1.5` → `injection_gain = 0.25` → Medium wide
- `width = 2.0` → `injection_gain = 0.5` → Maximum wide

### 3. Side Injection
```rust
let l = left + (side * injection_gain);
let r = right - (side * injection_gain);
```
- **Left**: Original + (Side * Gain)
- **Right**: Original - (Side * Gain)
- **Result**: Stereo width enhanced without touching original signals

### 4. Safety Net
```rust
output[i] = l.clamp(-1.0, 1.0);
output[i + 1] = r.clamp(-1.0, 1.0);
```
- Hard clamp (no RMS normalization) to prevent clipping

## Common Issues & Fixes

### Issue 1: "Sound Same as OFF"
**Cause**: `hp_coeff == 0.0` (high-pass filter not calculated)

**Fix in `surround.rs`**:
```rust
// Get current rate directly
let current_rate = samplerate::get_rate();

// Calculate hp_coeff only if rate valid (> 0.0)
// AND only if rate changed OR coeff never calculated (0.0)
if current_rate > 0.0 && (samplerate::consume_rate_changed() || self.hp_coeff == 0.0) {
    let hp_cutoff = 60.0;
    let rc = 1.0 / (2.0 * std::f32::consts::PI * hp_cutoff);
    let dt = 1.0 / current_rate;
    self.hp_coeff = rc / (rc + dt);
}
```

**Key**: Don't hardcode fallback rate (44100). Instead, let other modules (EQ, etc.) update the rate first, and only calculate when `hp_coeff == 0.0`.

### Issue 2: Toggle ON/OFF Doesn't Affect
**Cause**: `toggle_surround()` doing too much (updating width when toggling)

**Fix in `dspcontroller.rs`**:
```rust
pub fn toggle_surround(&mut self) {
    self.surround_active = !self.surround_active;
    crate::audio::dsp::surround::get_surround_enabled_arc()
        .store(self.surround_active, std::sync::atomic::Ordering::Relaxed);
    // Toggle ONLY does ON/OFF - no width update!
    self.surround_active_changed();
}
```

### Issue 3: Slider Doesn't Affect
**Cause**: `set_surround_width()` not updating atomic properly

**Fix in `dspcontroller.rs`**:
```rust
pub fn set_surround_width(&mut self, val: f64) {
    // UI: 1.0-2.0 (bypass→wide), same as engine
    self.surround_width = val;
    crate::audio::dsp::surround::get_surround_width_arc().store(
        (val as f32).to_bits(),
        std::sync::atomic::Ordering::Relaxed,
    );
    self.surround_width_changed();
}
```

### Issue 4: Display Shows "175%" Instead of "1.75"
**Cause**: `FxValueBox` component treating surround as percentage

**Fix in `Dsp.qml`**:
```qml
// Add property to FxValueBox:
property bool showSurround: false

// In display text logic:
} else if (rootItem.showSurround) {
    return sliderValue.toFixed(2);  // Shows "1.75" not "175%"
} else {
    return Math.round(sliderValue * 100) + "%";
}
```

**Apply to surround FxValueBox**:
```qml
FxValueBox {
    enabled: surrToggle.isOn && dspModel.dsp_enabled
    sliderValue: surrSlider.currentValue
    showSurround: true  // ADD THIS
    linkSlider: surrSlider
}
```

## Related Files

| File | Purpose |
|------|---------|
| `src/audio/dsp/surround.rs` | Main processor implementation |
| `src/ui/bridge/dspcontroller.rs` | UI controller (toggle, slider, preset loading) |
| `src/core/config/presets.rs` | Preset definitions (surround_width values) |
| `qml/ui/Dsp.qml` | UI components (slider, toggle, value display) |
| `src/audio/dsp/rack.rs` | DSP chain (SurroundProcessor included) |

## Quick Debug Checklist

1. **Toggle doesn't work**:
   - Check `toggle_surround()` → should ONLY toggle ON/OFF
   - Verify `SURROUND_ENABLED` atomic is updated

2. **No sound change**:
   - Check `hp_coeff == 0.0` → recalculate in `process()`
   - Verify `is_on` is true in `process()`
   - Check `injection_gain` calculation: `(width - 1.0) * 0.5`

3. **Display wrong**:
   - Verify `showSurround: true` in FxValueBox
   - Check `sliderRange: "surround"` in FxSliderBox

4. **Presets not loading**:
   - Check `load_preset()` and `load_user_fx_preset()`
   - Verify they call `set_surround_width()` with 1.0-2.0 range

## Summary

**Side Injection Method** is superior to M/S matrix because:
- ✅ **Zero coloration** at bypass (1.0 = identical to OFF)
- ✅ **Transient clarity** (original L/R untouched)
- ✅ **No phase issues** (no mid reconstruction)
- ✅ **Simple math** (just inject side difference)

**Key Rules:**
1. Toggle ONLY does ON/OFF
2. Slider updates width (1.0-2.0 range)
3. Never hardcode fallback rates (44100)
4. Always check `hp_coeff == 0.0` before applying high-pass
5. Use `showSurround: true` for QML display
