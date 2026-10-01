---
name: audio-engine
description: Engine audio: timing, decoder, seek, dan A-B loop.
---
# Audio Engine Architecture – Loonix Tunes
**Version:** 2.1
**Status:** Current Implementation
**Principle:** Engine Core Isolation

---

## I. Signal Chain Architecture

### Current Signal Flow (After Refactor):
```
Source (Decoder)
    ↓
[Normalizer: RMS Gain Adjustment] ← Engine Core (ALWAYS ON)
    ↓
[DSP Rack] ← Toggleable via DSP ON/OFF              
  ├─ EqPreamp (with headroom check)
  ├─ EQ Processor
  ├─ Compressor
  ├─ BassBooster
  ├─ Reverb
  ├─ StereoEnhance
  ├─ Crystalizer
  ├─ SurroundProcessor
  ├─ StereoWidth
  ├─ PitchShifter
  ├─ MiddleClarity
  ├─ Crossfeed
  └─ (NO Limiter - moved to engine core)
    ↓
[Limiter] ← Engine Core (ALWAYS ON)
    ↓
[Volume & Balance Control]
    ↓
[Output to Device]
```

### Engine Core vs DSP Rack:

| Component | Location | Status | Description |
|:---|:---|:---|:---|
| **Normalizer** | Engine Core (`audiooutput.rs`) | **ALWAYS ON** | Applies RMS gain from scanner. Runs first in chain. |
| **Limiter** | Engine Core (`audiooutput.rs`) | **ALWAYS ON** | Prevents clipping. Runs last before volume. |
| **DSP Rack** | Toggleable (`rack.rs`) | **ON/OFF** | Contains all cosmetic effects. Bypassed when DSP OFF. |
| **EqPreamp** | DSP Rack | Toggleable | Volume adjustment with headroom management. |

---

## II. Engine Core Implementation

### Files:
- `src/audio/io/audiooutput.rs` - Main engine loop
- `src/audio/dsp/normalizer.rs` - Normalizer processor
- `src/audio/dsp/limiter.rs` - Limiter processor

### AudioOutput Struct Fields:
```rust
// Engine Core - Always runs
normalizer_enabled: Arc<AtomicBool>,
normalizer: Arc<Mutex<AudioNormalizer>>,
limiter_enabled: Arc<AtomicBool>,
limiter: Arc<Mutex<Limiter>>,

// DSP Rack - Toggleable
dsp_enabled: Arc<AtomicBool>,
dsp_chain: DspChain,
```

### Audio Callback Processing Order (F32 & I16 paths):
```rust
// 1. NORMALIZER (Engine Core - Always first)
if normalizer_enabled.load(Ordering::SeqCst) {
    norm.process(&read_buffer, &mut processed_buffer);
}

// 2. DSP RACK (Toggleable)
if dsp_enabled.load(Ordering::SeqCst) {
    dsp_chain.process(&processed_buffer, &mut processed_buffer);
}

// 3. LIMITER (Engine Core - Always last)
if limiter_enabled.load(Ordering::SeqCst) {
    limiter.process(&processed_buffer, &mut processed_buffer);
}

// 4. Volume & Balance (Always applied)
```

---

## III. Normalizer (Engine Core)

### Purpose:
Applies pre-computed RMS gain from scanner. Adjusts quiet tracks (boost) or loud tracks (cut) to target dBFS.

### Gain Calculation (Scanner):
- File: `src/core/library/scanner.rs`
- Calculates RMS loudness of entire track
- Applies constraints: max gain, true peak ceiling
- Returns linear gain factor stored in `NORMALIZER_GAIN` atomic

### Runtime Processing:
- File: `src/audio/dsp/normalizer.rs`
- Reads gain from global atomic `get_normalizer_gain_arc()`
- Smoothing applied to prevent sudden jumps
- Soft clipping as safety net

---

## IV. Limiter (Engine Core)

### Purpose:
Prevents clipping after DSP processing. Always active regardless of DSP on/off state.

### Implementation:
- File: `src/audio/dsp/limiter.rs`
- Stereo-linked peak limiting
- Attack: 2ms, Release: 50ms
- Threshold: -0.5 dBFS (0.95 linear)
- Hard clamp at ±1.0

---

## V. Key Design Decisions

### Why Normalizer & Limiter in Engine Core?
1. **User Protection**: Prevents clipping and protects hearing
2. **Consistency**: Volume normalization works even with DSP OFF
3. **Isolation**: Core audio processing separate from cosmetic effects

### Why EqPreamp in DSP Rack?
- It's a cosmetic effect (volume adjustment with EQ context)
- Should be toggleable with DSP on/off
- Includes headroom management to prevent clipping before EQ

### Headroom Management:
- EqPreamp checks peak after applying gain
- If `peak * gain > 0.5` (-6 dBFS), gain is reduced
- Ensures signal doesn't clip before reaching EQ

---

## VI. File Structure

```
src/audio/
├── io/
│   ├── audiooutput.rs    # Engine core loop (Normalizer → DSP → Limiter)
│   ├── audiobus.rs        # Signal routing
│   └── decoder.rs        # Source decoding
├── dsp/
│   ├── rack.rs            # DSP Rack (EqPreamp → FX, NO Normalizer/Limiter)
│   ├── chain.rs           # DSP Chain wrapper
│   ├── normalizer.rs      # Normalizer (used by engine core)
│   ├── limiter.rs         # Limiter (used by engine core)
│   ├── eqpreamp.rs        # EqPreamp with headroom check
│   └── [other processors]
└── engine/
    └── engine.rs          # FfmpegEngine, scanner integration
```

---

## VII. State Management

### Atomic Flags (Lock-free):
- `normalizer_enabled` - Engine core, always true by default
- `limiter_enabled` - Engine core, always true by default
- `dsp_enabled` - Toggleable via UI
- `NORMALIZER_GAIN` - Set by scanner, read by normalizer

### Initialization:
1. Scanner calculates track gain → stores in atomic
2. Engine loads track → normalizer reads gain from atomic
3. Audio callback processes: Normalizer → DSP Rack → Limiter

---

## VIII. Verification Checklist

- [x] Normalizer runs before DSP rack
- [x] Limiter runs after DSP rack
- [x] Both always active regardless of DSP on/off
- [x] Normalizer removed from DSP rack
- [x] Limiter removed from DSP rack
- [x] EqPreamp has headroom management
- [x] DSP bypass comment updated in rack.rs

---

# Audio Timing Architecture (clock.md)

## Status: AudioClock is SSoT for Timing

`clock.rs` is the **single source of truth** for all time calculations.
Only clock.rs divides samples to get seconds.

---

## Architecture (Separation of Concerns)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    KASTA TERTINGGI                                │
│              Audio Hardware (CPAL/WASAPI)                          │
│  - Gives callback at fixed rate (e.g., 48000 Hz)                  │
│  - THE source of real time                                       │
└─────────────────────────────────────────────────────────────────────────┘
                              ↓ callback
┌─────────────────────────────────────────────────────────────────────────┐
│                   KASTA MENENGAH                               │
│              Audio Thread                                        │
│  - samples_played.fetch_add(raw_samples)                         │
│  - NEVER divides or multiplies                                 │
│  - Just reports raw numbers                                  │
└─────────────────────────────────────────────────────────────────────────┘
                              ↓ read
┌─────────────────────────────────────────────────────────────────────────┐
│                  KASTA LOGIKA                                  │
│              Clock Struct (clock.rs)                           │
│  - ONLY place with time math                                    │
│  - Formula: Seconds = Samples / (SampleRate × Channels)           │
│  - Single Source of Truth (SSoT)                            │
└─────────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────────┐
│                      UI Timer                                 │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Double Taxing Warning

If `/(sample_rate * channels)` appears in MORE THAN ONE place → **BUG**:

| File | Should Divide? | 
|------|----------------|
| audiooutput.rs | ❌ NO - just add raw samples |
| engine.rs | ❌ NO - just pass to Clock |
| bridge.rs | ❌ NO - just display |
| clock.rs | ✅ YES - ONLY place! |

---

## Implementation

### audiooutput.rs - The Reporter
```rust
// Add TOTAL samples (NOT frames!)
samples_played.fetch_add(samples_per_write as u64, Ordering::SeqCst);
// samples_per_write = frames * channels = 48000 * 2 = 96000
```

### engine.rs - The Messenger
```rust
// Just pass to Clock, let Clock do math
let live_samples = audiooutput.get_samples_played();
// Pass to Clock (via Clock's method)
self.audio_clock.sync_from_absolute(live_samples);
```

### clock.rs - The Interpreter (ONLY PLACE)
```rust
// Samples / (Rate × Channels) = Seconds
pub fn sync_from_absolute(&mut self, total_samples: u64) {
    self.position_ms = (total_samples * 1000) / (self.sample_rate as u64 * self.channels as u64);
}
```

---

## Formula Verification

```
Given:
- sample_rate = 48000 Hz
- channels = 2 (Stereo)
- 1 second of audio = 48000 frames = 96000 samples (L+R)

Calculation:
seconds = 96000 / (48000 × 2)
        = 96000 / 96000
        = 1.0 second ✅

Not:
96000 / 48000 = 2.0 ❌ (wrong - double speed)
```

---

## Track Change Reset

When new track starts:
```rust
// In start_audiooutput()
audio_clock.reset();
audiooutput.reset_samples_played(0);
```

When playback starts:
```rust
// In InitialBufferReady event
audio_clock.reset();
audiooutput.reset_samples_played(0);
```

---

## References

- `src/audio/io/audiooutput.rs` - samples_played atomic + fetch_add
- `src/audio/engine/clock.rs` - AudioClock with division logic
- `src/audio/engine/engine.rs` - passes to Clock
- `src/audio/samplerate.rs` - rate management

---

## audio/engine/seek.md

```
cara kerja engine.rs dalam mengelola seekbar.

# Logical Sequence for Instant Seek (Loonix Tunes)

Sistem harus menangani dua fase: "The Jump" (saat klik) dan "The Refill" (setelah klik dilepas).

### PHASE 1: The Jump (Saat User Klik/Release Slider)

1. **Signal Pause**: Segera set `is_playing = false` agar audio thread berhenti menarik data dari ring buffer.
2. **Flush Hardware**: Perintahkan CPAL/Pipewire untuk "drain" atau kirim buffer kosong agar tidak ada suara "pop".
3. **Clear Engine Buffer**: Kosongkan `RingBuffer` di Rust agar data lagu di posisi lama hilang total.
4. **Command Decoder**: Kirim pesan `Seek(target_ms)` ke thread FFmpeg.

### PHASE 2: The Decoder (Di src/decoder.rs)

1. **FFmpeg Seek**: `av_seek_frame` ke target.
2. **Decoder Flush**: `avcodec_flush_buffers` (Wajib! Jika tidak, decoder masih menyimpan sisa frame lama dan tidak mau memproses frame baru).
3. **Primary Decode**: Decoder harus langsung men-decode minimal 5-10 frame di posisi baru dan mengirimnya ke `RingBuffer`.

### PHASE 3: The Resume (Sinkronisasi Akhir)

1. **Sync Samples**: Update `samples_played = (target_ms * sample_rate * 2) / 1000`.
2. **Wait for Data**: Tunggu sampai `RingBuffer.len() > threshold` (misal 100ms data).
3. **Start Playback**: Set `is_playing = true`.

# Troubleshooting Silent Audio after Seek

Jika UI jalan tapi suara hilang, cek hal berikut di `src/engine.rs` dan `src/decoder.rs`:

1. **Decoder Deadlock**: Apakah thread decoder berhenti bekerja setelah seek?
   - _Solusi_: Pastikan loop decoder tidak blocking saat menerima command seek.
2. **Buffer Underrun**: Apakah audio thread mencoba membaca buffer yang masih kosong?
   - _Solusi_: Berikan logika "Pre-roll". Jangan aktifkan audio output sebelum buffer terisi minimal 4096 samples.
3. **Time Base Mismatch**: Apakah nilai `target_ms` sudah dikonversi dengan benar ke `AVStream.time_base`?
   - _Solusi_: Gunakan `av_rescale_q` untuk akurasi posisi.
4. **FX State**: Apakah modul di `src/fx.rs` (seperti Compressor atau Limiter) "macet" karena input tiba-tiba hilang?
   - _Solusi_: Tambahkan fungsi `fx.reset()` untuk menormalkan state DSP setelah seek.

FLOW YANG SEHARUSNYA ADA (INI YANG LU HARUS BANGUN)
request_seek()
→ set SEEKING

decoder:
→ flush
→ av_seek_frame

→ masuk DECODING phase
→ decode frame by frame
→ discard sampai mendekati target
→ mulai push ke buffer
→ hit min_buffer_samples

→ set READY

audio:
→ resume playback
→ reset clock
→ disable seek_mode

## CHECKING

You are a senior audio engine engineer.

Your task is to audit a Rust-based audio player project (decoder + engine + audio output + DSP pipeline) and verify whether the SEEK implementation matches professional-grade behavior similar to PotPlayer.

You have full access to the codebase. Do not guess. Trace actual execution flow across threads.

---

STEP 1 — TRACE SEEK FLOW (MANDATORY)

Find and analyze:

- engine.seek() or equivalent entry point
- decoder thread loop
- audio output callback
- state synchronization (atomic flags / mutex / state enum)

Build a REAL execution flow like:

seek() → flags set → decoder reacts → buffer fill → audio resumes

Reject analysis if flow is incomplete.

---

STEP 2 — VERIFY CRITICAL SEEK PROPERTIES

Check each item strictly:

1. CONTINUOUS AUDIO CALLBACK

- Audio device MUST NOT stop during seek
- Callback must continue running at all times

2. EXPLICIT SEEK MODE IN AUDIO THREAD

- There MUST be a condition like:
  if seek_mode { output silence }
- If silence depends on empty buffer → FAIL

3. NO FAKE BUFFER FILL

- Reject if code pushes artificial silence into buffer during seek
- Buffer must contain only decoded audio

4. PROPER DECODER SEEK HANDLING

- Must call:
  av_seek_frame + decoder.flush()
- Then enter PRE-DECODE phase

5. PREBUFFER BEFORE READY

- Decoder must fill buffer until min_buffer_samples BEFORE signaling ready
- If READY is triggered immediately after seek → FAIL

6. STATE MACHINE VALIDITY

- Must have at least:
  IDLE → SEEKING → DECODING → READY
- If states are skipped or unused → FAIL

7. CLEAN TRANSITION OUT OF SEEK

- seek_mode must be disabled ONLY AFTER buffer is ready
- samples_played must reset at correct timing

8. NO RACE CONDITIONS

- Check ordering of:
  - seek flags
  - buffer_ready
  - seek_mode
- Detect early reset or conflicting writes

---

STEP 3 — DETECT COMMON FAILURE PATTERNS

Explicitly scan for:

- pushing silence into ring buffer
- calling READY before decoding frames
- clearing seek flags too early
- empty buffer being treated as silence
- missing DECODING phase
- audio callback depending on buffer state instead of explicit mode

---

STEP 4 — VALIDATE TIMING & CLOCK

- Check if samples_played / playback clock:
  - freezes during seek
  - resumes correctly after
- Detect drift or stuck timer issues

---

STEP 5 — FINAL VERDICT

Output STRICT result:

PASS → if behavior matches professional player standard

FAIL → if any critical issue found

---

STEP 6 — IF FAIL, PROVIDE FIX PLAN

Give:

1. Root cause (not symptom)
2. Exact file + function responsible
3. What must be changed (not suggestion, but directive)
4. Missing stage (e.g., PREBUFFER, SEEK MODE, etc.)

---

IMPORTANT RULES

- Do NOT give generic advice
- Do NOT summarize code
- Do NOT assume behavior — verify it
- Be harsh and precise
- Treat this as production-critical audio system

Goal: achieve glitch-free, instant-response seeking identical to high-end media players.

Flow verify must be correct:
Step Action
1 seek() → audio_output.set_seek_mode(true)
2 seek() → control.request_seek(target_ms)
3 Decoder: av_seek_frame() + flush()
4 Decoder: recreate Resampler
5 Decoder: prebuffer 9600 samples
6 Decoder: send_event(DecoderEvent::SeekComplete)
7 Engine: on_seek_complete() → set_seek_mode(false)
8 Engine: set_samples_played(exact)
9 Engine: trigger_delayed_resume() (2 frames)
10 Engine: trigger_crossfade()
11 Callback: processes buffer, fade-in, DSP clean
Fixed: Added audio_output.set_seek_mode(false) in step 7 - previously missing.

---

# GOLD STANDARD: Professional Seek Implementation

**Version 2.0 | Date: March 19, 2026**

---

## 📋 GOLD STANDARD RULES

### 1. SAMPLE RATE UNITS MUST BE CONSISTENT

**Golden Rule:** `sample_rate = 48000` means **48,000 FRAMES per second**, NOT samples!

For stereo audio:

- 1 frame = 2 samples (L + R)
- 48,000 frames/second = 96,000 samples/second

**NEVER** mix frames and samples in calculations!

---

### 2. FRAME vs SAMPLE COUNTING

| Variable                | Unit           | Notes                          |
| ----------------------- | -------------- | ------------------------------ |
| `samples_played`        | **SAMPLES**    | Counts individual L+R values   |
| `sample_rate`           | **FRAMES/SEC** | Hardware playback rate         |
| `num_frames_to_write`   | **FRAMES**     | CPAL buffer units              |
| `samples_read`          | **SAMPLES**    | Ring buffer items (f32 values) |
| `total_decoded_samples` | **SAMPLES**    | Decoder's pushed samples       |

---

## 🛠️ CODE FIXES APPLIED

### FIX 1: Ring Buffer Size (engine.rs:108)

**Problem:** Ring buffer was sized in FRAMES, but HeapRb<f32> expects SAMPLES.

**Before:**

```rust
let buffer_size = (sample_rate * buffer_ms / 1000) as usize;
// 48000 * 500 / 1000 = 24000 FRAMES
```

**After:**

```rust
let sample_rate = 48000; // frames per second
let channels = 2;
let buffer_ms = 500;
let buffer_size = (sample_rate * channels * buffer_ms / 1000) as usize;
// 48000 * 2 * 500 / 1000 = 48000 SAMPLES
```

**Why:** `HeapRb<f32>::new()` expects f32 item count, not frame count.

---

### FIX 2: Audio Callback Sample Counting (audio_output.rs:332)

**Problem:** Counting FRAMES instead of SAMPLES.

**Before:**

```rust
samples_played.fetch_add(num_frames_to_write as u64, Ordering::SeqCst);
```

**After:**

```rust
if samples_read > 0 {
    samples_played.fetch_add(samples_read as u64, Ordering::SeqCst);
}
```

**Why:** `samples_played` must count SAMPLES (f32 values), not frames.

---

### FIX 3: Position Calculation (engine.rs:302)

**Problem:** Dividing samples by frame rate.

**Before:**

```rust
self.samples_played as f64 / self.sample_rate as f64
```

**After:**

```rust
self.samples_played as f64 / (self.sample_rate as f64 * 2.0)
```

**Why:** `sample_rate` is frames/sec, `samples_played` is samples. Need to divide by 2.

---

### FIX 4: Position MS Calculation (engine.rs:337)

**Problem:** Integer division causing wrong results.

**Before:**

```rust
(self.samples_played * 1000) / self.sample_rate
```

**After:**

```rust
((self.samples_played as f64 * 1000.0) / (self.sample_rate as f64 * 2.0)) as u64
```

**Why:** Use f64 division to avoid rounding errors.

---

### FIX 5: Seek Target Calculation (engine.rs:360)

**Problem:** Calculating target in FRAMES instead of SAMPLES.

**Before:**

```rust
let target_samples = (seconds * self.sample_rate as f64) as u64;
```

**After:**

```rust
let target_samples = (seconds * self.sample_rate as f64 * 2.0) as u64;
```

**Why:** When seeking to 120 seconds at 48kHz:

- **Before:** 120 × 48000 = 5,760,000 (FRAMES) ❌
- **After:** 120 × 48000 × 2 = 11,520,000 (SAMPLES) ✅

---

### FIX 6: On Seek Complete (engine.rs:386)

**Problem:** Calculating exact samples in FRAMES instead of SAMPLES.

**Before:**

```rust
let exact_samples = (target_ms * self.sample_rate) / 1000;
```

**After:**

```rust
let exact_samples = ((target_ms as f64 * self.sample_rate as f64 * 2.0) / 1000.0) as u64;
```

**Why:** Match the unit used in `samples_played`.

---

### FIX 7: Duration Calculation (engine.rs:331)

**Problem:** Duration was calculated in FRAMES.

**Before:**

```rust
(duration_ms * self.sample_rate) / 1000
```

**After:**

```rust
(duration_ms * self.sample_rate * 2) / 1000
```

**Why:** Duration must be in SAMPLES to match `samples_played`.

---

### FIX 8: Clock Set Position (clock.rs:44)

**Problem:** Same frame/sample confusion.

**Before:**

```rust
self.samples_played = (position_ms * self.sample_rate as u64) / 1000;
```

**After:**

```rust
self.samples_played = (position_ms * self.sample_rate as u64 * 2) / 1000;
```

**Why:** Convert position_ms to SAMPLES, not frames.

---

### FIX 9: Prebuffer Size (decoder.rs:49)

**Problem:** Prebuffer was too small (100ms).

**Before:**

```rust
min_buffer_samples: AtomicU64::new(9600),  // ~100ms
```

**After:**

```rust
min_buffer_samples: AtomicU64::new(96000), // ~1 second
```

**Why:** 1 second of buffer ensures stable playback.

---

### FIX 10: Reset samples_played on New Track (engine.rs:474)

**Problem:** `samples_played` accumulated across tracks.

**Before:**

```rust
pub fn play(&mut self, path: &str) {
    engine.stop();
    // samples_played keeps old value!
    engine.start_audio_output(path.to_string());
}
```

**After:**

```rust
pub fn play(&mut self, path: &str) {
    engine.stop();
    engine.samples_played = 0;  // Reset for new track
    engine.start_audio_output(path.to_string());
}
```

**Why:** Each track must start at position 0.

---

## ✅ VERIFICATION CHECKLIST

| Check                               | Status | Code Location       |
| ----------------------------------- | ------ | ------------------- |
| Ring buffer sized in SAMPLES        | ✅     | engine.rs:108       |
| samples_played counts SAMPLES       | ✅     | audio_output.rs:332 |
| Position calc divides by (rate\*2)  | ✅     | engine.rs:302       |
| Seek target multiplies by 2         | ✅     | engine.rs:360       |
| on_seek_complete uses exact_samples | ✅     | engine.rs:386       |
| Duration calc multiplies by 2       | ✅     | engine.rs:331       |
| Clock set_position multiplies by 2  | ✅     | clock.rs:44         |
| Prebuffer size = 96,000             | ✅     | decoder.rs:49       |
| samples_played reset on new track   | ✅     | engine.rs:474       |

---

## 🎯 TARGET AKHIR

Seek flow final:
UI
 → Engine.seek()
   → state = Seeking
   → audio.set_seek_mode(true)
   → ringbuffer.clear()
   → control.request_seek()

Decoder
 → flush
 → av_seek_frame
 → prebuffer sampai min
 → send_event(BufferReady)

Engine.on_buffer_ready()
 → set samples_played exact
 → fx.reset_all()
 → audio.set_seek_mode(false)
 → control.clear_seek()
 → state = Playing

 ---


 ===============================
PHASE 1 — FIX OWNERSHIP SEEK
===============================
🔥 Objective:

Decoder tidak lagi mematikan seek flag.

✅ 1.1 Hapus ini dari decoder.rs

HAPUS:

control.is_seeking.store(false, Ordering::SeqCst);

Decoder hanya boleh:

control.seeking_state.store(SEEK_STATE_READY, Ordering::SeqCst);
control.send_event(DecoderEvent::BufferReady);
✅ 1.2 Engine yang clear seek

Di engine.rs tambahkan:

pub fn on_buffer_ready(&mut self) {
    let target_ms = self.control.seek_request.load(Ordering::SeqCst);

    // 1. set exact samples
    let exact_samples =
        ((target_ms as f64 * self.sample_rate as f64 * 2.0) / 1000.0) as u64;

    self.samples_played.store(exact_samples, Ordering::SeqCst);

    // 2. reset DSP
    self.fx.reset_all();

    // 3. disable seek mode
    self.audio_output.set_seek_mode(false);

    // 4. clear seek flags (ENGINE authority)
    self.control.clear_seek();

    // 5. resume state
    self.state = EngineState::Playing;
}
===============================
PHASE 2 — HARD SEEK MODE DI AUDIO THREAD
===============================
🔥 Objective:

Audio callback tidak boleh tergantung ringbuffer kosong.

✅ 2.1 Tambah atomic seek_mode di audio_output
pub struct AudioOutput {
    seek_mode: AtomicBool,
}

Setter:

pub fn set_seek_mode(&self, enabled: bool) {
    self.seek_mode.store(enabled, Ordering::SeqCst);
}
✅ 2.2 Callback wajib begini
if self.seek_mode.load(Ordering::Acquire) {
    output.fill(0.0);
    return;
}

JANGAN ADA:

if ringbuffer.is_empty()

Itu bug.

===============================
PHASE 3 — RINGBUFFER SAFETY
===============================
🔥 Objective:

Tidak ada data lama bercampur.

✅ 3.1 Clear consumer sebelum seek

Di engine.seek():

pub fn seek(&mut self, target_ms: u64) {
    self.state = EngineState::Seeking;

    self.audio_output.set_seek_mode(true);

    // CLEAR CONSUMER SIDE
    self.ring_consumer.clear();

    // request decoder
    self.control.request_seek(target_ms);
}

Decoder tidak boleh clear producer.
Engine clear consumer.

===============================
PHASE 4 — RACE PROTECTION DI DECODER
===============================
🔥 Objective:

Seek spam tidak corrupt buffer.

✅ 4.1 Tambah check di prebuffer loop

Ubah:

while buffered < min {

Menjadi:

while buffered < min {
    if control.should_stop.load(Ordering::SeqCst) {
        return;
    }

    // Jika ada seek baru masuk
    let new_target = control.seek_request.load(Ordering::SeqCst);
    if new_target != target_ms {
        break;
    }

Kalau tidak ini bisa kacau saat user drag slider cepat.

===============================
PHASE 5 — STATE MACHINE STRICT
===============================

Tambahkan enum:

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EngineState {
    Idle,
    Seeking,
    Decoding,
    Ready,
    Playing,
}

Flow wajib:

Playing
→ Seeking
→ (Decoder working)
→ Ready
→ Playing

Tidak boleh langsung Seeking → Playing.

===============================
PHASE 6 — DELAYED RESUME (Pulse Safety)
===============================

Pulse punya latency queue.

Tambahkan counter di audio thread:

fade_in_remaining: AtomicU32,

Saat keluar seek:

self.fade_in_remaining.store(256, Ordering::SeqCst);

Di callback:

let fade_left = self.fade_in_remaining.load(Ordering::Acquire);

if fade_left > 0 {
    let frames = fade_left.min(output.len() as u32);
    for i in 0..frames as usize {
        let factor = i as f32 / frames as f32;
        output[i] *= factor;
    }

    self.fade_in_remaining
        .fetch_sub(frames, Ordering::SeqCst);
}

Ini hilangkan pop total.

===============================
PHASE 7 — CLOCK FREEZE
===============================
🔥 Objective:

Slider tidak mental.

Di engine position getter:

pub fn position_ms(&self) -> u64 {
    if self.state == EngineState::Seeking {
        return self.control.seek_request.load(Ordering::SeqCst);
    }

    let samples = self.samples_played.load(Ordering::SeqCst);

    ((samples as f64 * 1000.0)
        / (self.sample_rate as f64 * 2.0)) as u64
}

Clock tidak boleh increment saat Seeking.

===============================
FINAL EXPECTED BEHAVIOR
===============================

Saat seek:

Audio callback tetap jalan
Output silence
Decoder prebuffer
Engine set exact clock
DSP reset
Fade in 256 frame
Resume playback
Slider stabil

Tidak ada:

pop
stop tiba-tiba
slider balik
underrun panic
📌 PRIORITY ORDER IMPLEMENTASI
Phase 1 (ownership fix) ← paling penting
Phase 2 (hard seek_mode)
Phase 3 (ring clear)
Phase 7 (clock freeze)
Phase 4 (race protection)
Phase 6 (fade-in polish)

---

## ⚠️ SEKARANG KITA CARI TITIK LEMAH YANG MASIH MUNGKIN

Aku akan cek 4 hal yang biasanya masih jadi sumber pop atau slider glitch walaupun flow terlihat benar.

1️⃣ is_playing = false Saat Seek

Ini perlu hati-hati.

Kalau audio callback kamu pakai:

if !is_playing {
    output.fill(0.0);
    return;
}

Maka kamu punya DUA gate:

is_playing
seek_mode

Itu redundant dan bisa race.

Idealnya:

Audio callback hanya peduli:

seek_mode

Bukan is_playing.

Kalau is_playing dipakai juga, pastikan:

is_playing tidak dimatikan sebelum seek_mode aktif
tidak ada branch lain yang treat !is_playing sebagai STOP state

Kalau tidak, bisa muncul:

playback tidak resume
state stuck

Kalau callback hanya pakai seek_mode → aman.

2️⃣ clear_buffer() Timing

Kamu bilang:

audio.clear_buffer()

Pertanyaannya:

Apakah ini clear CONSUMER saja atau recreate ringbuffer?

Kalau kamu recreate ringbuffer instance → bahaya.

Yang benar:

consumer.discard_all()

Decoder producer tetap hidup.

Kalau kamu recreate ringbuffer object saat audio thread masih pegang reference → itu undefined behavior.

Pastikan hanya drain consumer side.

3️⃣ Pulse Latency Residual

Sekarang kamu tidak cork stream lagi (bagus).

Tapi Pulse punya internal latency.

Kalau seek_mode dimatikan dan langsung output data,
kadang masih ada 1 callback cycle berisi silence.

Ini bisa bikin:

satu tick UI delay
kadang perceived micro glitch

Solusi profesional:

Saat on_buffer_ready():

set_seek_mode(false)
fade_in = 256 frame

Kalau belum ada fade-in ringan, itu satu-satunya polish yang masih kurang.

Tanpa fade-in sistem tetap benar,
tapi belum “high-end player feel”.

4️⃣ Clock Freeze Validation

Kamu belum sebut ini:

Saat Seeking,
apakah position_ms() tetap pakai samples_played lama?

Kalau iya → slider akan jalan terus walaupun audio silent.

Pastikan:

if state == Seeking {
    return seek_target_ms;
}

Kalau ini tidak ada,
slider bisa “karet” walaupun audio tidak error.

---

1️⃣ Dual Gate: is_running / is_playing vs seek_mode
Status: ⚠️ Harus dirapikan

Sekarang logic kamu:

if !is_running {
    output_silence();
    continue;
}

if is_seeking {
    output_silence();
    sleep(2ms);
    continue;
}

Saat seek:

is_playing = false
seek_mode = true

Artinya dua gate aktif sekaligus.

Ini tidak crash, tapi ini masalah desain:

Dua source of truth
Perilaku berbeda (sleep vs no sleep)
Sulit reasoning
Bisa bikin edge case saat stop → seek → play cepat
🎯 Fix Profesional

Audio thread hanya punya satu authority untuk silence:

if seek_mode.load(Ordering::Acquire) {
    output.fill(0.0);
    return;
}

Stop state harus beda konsepnya dari seek.

Struktur yang benar:
match engine_state {
    EngineState::Playing => process_audio(),
    EngineState::Seeking => output_silence(),
    EngineState::Idle => output_silence(),
}

Kalau kamu tetap pakai bool:

Hapus is_playing dari audio callback
Biarkan engine state yang menentukan

Audio callback tidak boleh punya interpretasi sendiri.

2️⃣ clear_buffer() — ✅ Sudah Benar

Karena kamu:

flush_requested.store(true)

Dan drain consumer saja → aman.

Selama ringbuffer tidak di-recreate → ini production safe.

3️⃣ Fade-in — ❌ Ini Yang Sekarang Paling Lemah

Kamu punya:

crossfade_frames.store(2400);

Tapi tidak ada code yang pakai itu.

Artinya saat seek complete:

Seek mode dimatikan
Buffer mulai dibaca
Audio muncul langsung full amplitude

Kalau frame pertama bukan zero-crossing → klik kecil.

🎯 Implementasi Fade-In yang Benar

Tambahkan di push_loop setelah ambil data dari buffer:

let fade_left = self.crossfade_frames.load(Ordering::Acquire);

if fade_left > 0 {
    let frames = fade_left.min(output.len() as u32);

    for i in 0..frames as usize {
        let factor = i as f32 / frames as f32;
        output[i] *= factor;
    }

    self.crossfade_frames
        .fetch_sub(frames, Ordering::SeqCst);
}

2400 samples @ 48k stereo = 2400/2 = 1200 frames
1200 / 48000 ≈ 25ms fade

Itu cukup halus dan tidak terasa delay.

🔥 Bahkan Lebih Bersih

Jangan pakai crossfade name.

Ganti jadi:

seek_fade_remaining

Karena ini bukan crossfade antar lagu.

4️⃣ Clock Freeze — ✅ Acceptable

Sekarang:

if is_playing {
    return audio_samples;
}
return samples_played;

Saat seek:

is_playing = false
samples_played sudah di-set exact

Slider langsung lompat → benar.

Ini lebih natural daripada freeze.

Tidak perlu ubah.

🎯 FINAL EVALUATION
Area	Status
Ownership seek	✅ Solid
Decoder authority	✅ Clean
Buffer handling	✅ Correct
Pulse stability	✅ Safe
Dual gate logic	⚠️ Minor cleanup
Fade-in	❌ Missing polish
🏁 Real Final Verdict

Core engine kamu sekarang:

🟢 PRODUCTION STABLE

Tidak ada lagi structural race condition.

Yang tersisa hanya polish untuk:

feel lebih halus
hilangkan micro click
🔥 Urutan Cleanup Terbaik
Rapikan dual gate → audio callback hanya patuh ke seek_mode / state
Implement fade-in 20–30ms
Rename crossfade ke seek_fade

Setelah itu, seek kamu sudah masuk kelas high-end desktop player. 

---

📋 DIAGNOSIS: Shutdown Lifecycle Bug
🔍 Root Cause Analysis
File: src/main.rs:122-124
engine.load_file("qrc:/qml/Ui.qml".into());
engine.exec();
// ❌ NO CLEANUP - app exits without shutdown!
🐛 Bug Chain
Window Close
  ↓
main() exits after engine.exec()
  ↓
MusicModel/FfmpegEngine/AudioOutput DROPS (implicit)
  ↓
AudioOutput.Drop() sends AudioCommand::Exit
  ↓
BUT: push_loop_owned still running!
  ↓
AudioOutput.Drop() calls thread.join()
  ↓
DEADLOCK or audio keeps playing until Ctrl+C
❌ Issues Found
Issue	File	Problem
No shutdown	main.rs:122-124	No cleanup after engine.exec()
No Drop for Engine	engine.rs	Engine doesn't call stop() on drop
should_stop never set	engine.rs:370-390	Engine.stop() sets decoder_control.should_stop, but only when called
Zombie system threads	wireless.rs:109, systemcheck.rs:15	Infinite loops with no stop flag
No decoder JoinHandle	decoder.rs:109	Decoder thread can't be joined
---
📝 FIX PLAN
Phase 1: Add Engine Drop (Minimal Fix)
File: src/audio/engine/engine.rs
Add impl Drop for Engine:
impl Drop for Engine {
    fn drop(&mut self) {
        // Ensure clean shutdown when Engine is dropped
        self.stop();
    }
}
Phase 2: Add aboutToQuit Handler
File: src/main.rs
Add Qt shutdown signal connection:
engine.load_file("qrc:/qml/Ui.qml".into());
// Connect Qt's aboutToQuit signal for proper cleanup
QObject::connect(&engine, signal!(quit()), || {
    // Call cleanup on all audio resources
});
engine.exec();
// Explicit cleanup after Qt event loop exits
// Drop all boxed objects in reverse order
drop(boxed_sysmedia);
drop(boxed_bridge);
drop(boxed_popup);
drop(boxed_theme);
drop(boxed_model);
Phase 3: Stop All System Threads
File: src/audio/wireless.rs, src/audio/systemcheck.rs
Add static STOP flag:
static SHOULD_STOP: AtomicBool = AtomicBool::new(false);
pub fn stop_system_check() {
    SHOULD_STOP.store(true, Ordering::SeqCst);
}
Update loops to check flag:
loop {
    if SHOULD_STOP.load(Ordering::SeqCst) {
        break;
    }
    // ... existing logic
}
Phase 4: Add Decoder Thread Handle
File: src/audio/decoder.rs
Store JoinHandle:
pub struct DecoderHandle {
    pub thread_handle: Option<JoinHandle<()>>,
    pub control: Arc<DecoderControl>,
}
impl DecoderHandle {
    pub fn spawn(...) -> Self { ... }
    pub fn stop(&self) {
        self.control.should_stop.store(true, Ordering::SeqCst);
    }
}
Update Engine to store and join decoder:
pub struct Engine {
    decoder_handle: Option<DecoderHandle>,
    // ...
}
---
## 🎯 Execution Order
1. **Phase 1** — `impl Drop for Engine` (Quick fix, 5 min)
2. **Phase 2** — Qt shutdown handler in main.rs (Critical)
3. **Phase 3** — Stop system check threads (Minor)
4. **Phase 4** — Decoder JoinHandle (Optional, nice-to-have)
---
Estimasi: 15-20 menit untuk semua phase.

---

## ✅ SEEK FLOW — CURRENT CORRECT FLOW (Verified Working)

### Normal Seek Flow:

```
Engine.seek(target_ms)
  → set_seek_mode(true)           // audio callback outputs silence
  → control.request_seek(target_ms)
  → flush.store(true)              // drain ring buffer

Decoder (next iteration):
  → detects is_seeking && IDLE
  → av_seek_frame() + decoder.flush()
  → recreate resampler
  → prebuffer until min_buffer_samples
  → set seeking_state = READY
  → is_seeking stays TRUE (Engine decides when to clear)

Engine.on_buffer_ready():
  → samples_played = exact_samples
  → audio_clock.reset()
  → reset_dsp()
  → set_output_state(Running)
  → set_seek_mode(false)
  → trigger_seek_fade()
  → control.clear_seek()
  → playback_state = Playing

Audio callback:
  → pops from buffer FIRST (Black Hole pattern)
  → if seek_mode: fill with silence
  → else: process DSP, output audio
```

### AB Loop Seek Flow:

```
Audio callback detects current >= B:
  → samples_played.store(A)
  → seek_mode.store(true)
  → flush.store(true)
  → ab_loop_seek_sample.store(A)

Decoder detects ab_loop_seek_sample > 0:
  → is_seeking.store(true)
  → av_seek_frame(A) + flush
  → recreate resampler
  → prebuffer until min
  → total_decoded_samples = A    // prevent check_loop re-trigger
  → seeking_state = READY
  → is_seeking = false            // UNIFIED atomic → audio sees seek_mode=false
  → send BufferReady { samples_played: A }

Engine.on_buffer_ready():
  → samples_played = A
  → audio_clock.reset()
  → reset_dsp()
  → set_output_state(Running)
  → set_seek_mode(false)
  → trigger_seek_fade()
  → control.clear_seek()
```

---

## 🔧 CRITICAL FIXES APPLIED (Seek & AB Loop Deadlock)

### Fix 1: Unified Seek Atomic (seek_mode = is_seeking)

**Problem:** AudioOutput had `seek_mode`, Decoder had `is_seeking` as separate atomics.
Race: Engine might clear seek_mode before decoder clears is_seeking.

**Fix:** Single `Arc<AtomicBool>` shared between AudioOutput and DecoderControl:
```rust
// engine.rs start_audiooutput():
let seek_is_seeking = Arc::new(AtomicBool::new(false));
let control = Arc::new(DecoderControl::new(
    ab_loop_seek_sample.clone(),
    seek_is_seeking.clone(),  // unified atomic
));
audiooutput.set_seek_mode_arc(seek_is_seeking.clone());
```

### Fix 2: Black Hole Pattern (Audio Callback Pops During Seek)

**Problem:** Audio callback outputs silence WITHOUT popping from ring buffer during seek_mode=true.
Deadlock: Ring buffer fills → decoder's push_output() blocks forever → BufferReady never sent.

**Fix:** Audio callback ALWAYS pops from buffer first, then gates with silence:
```rust
// ALWAYS pop from ring buffer first — prevents decoder deadlock
if let Ok(mut c) = consumer.try_lock() {
    if let Some(ref mut cons) = *c {
        let samples_read = cons.pop_slice(&mut read_buffer);
        // ... update empty_count
    }
}

// Gating: replace with silence if seeking/paused
if is_seeking || is_paused {
    read_buffer.fill(0.0);
}
```
Applied to BOTH f32 and i16 audio callback paths in audiooutput.rs.

### Fix 3: on_buffer_ready() Completes the Seek

**Problem:** on_buffer_ready() didn't reset audio_clock or set OutputState::Running.

**Fix:** Added to on_buffer_ready():
```rust
self.audio_clock.reset();  // Reset SSoT clock
audiooutput.set_output_state(OutputState::Running);  // Ensure hardware running
```

### Fix 4: Prebuffer Deadlock Guard

**Problem:** If ring buffer fills during prebuffer, push_output() sleeps 1ms forever.

**Fix:** Added vacant_len() check in prebuffer loop (decoder.rs):
```rust
while buffered < min {
    // Break if ring buffer is completely full
    if producer.vacant_len() == 0 {
        break;
    }
    // ... decode and push
}
```
Requires: `use ringbuf::traits::Observer;` in decoder.rs.

### Fix 5: total_decoded_samples Reset After AB Prebuffer

**Problem:** After AB seek prebuffer, check_loop() immediately re-triggers.

**Fix:** Reset to A after prebuffer:
```rust
// After prebuffer loop completes:
total_decoded_samples = ab_seek_sample;  // NOT A + prebuffer
```

### Fix 6: sync_ab_loop_atomics() Ordering

**Activating:** write A, write B, write active=true (LAST)
**Deactivating:** write active=false (FIRST), write A=0, write B=0

### Fix 7: ab_loop_seek_sample Reset in start_audiooutput()

**Problem:** ab_loop_seek_sample could retain value from previous track.

**Fix:** Reset in start_audiooutput() before decoder spawn:
```rust
ab_loop_seek_sample.store(0, Ordering::SeqCst);
```

---

## 📋 FINAL VERIFICATION CHECKLIST

| Check                               | Status | Code Location             |
| ----------------------------------- | ------ | ------------------------- |
| Unified seek_mode/is_seeking        | ✅     | engine.rs:242, audiooutput.rs:1037 |
| Audio callback pops during seek      | ✅     | audiooutput.rs:630-647, 857-874 |
| on_buffer_ready() resets clock       | ✅     | engine.rs:751             |
| on_buffer_ready() sets Running      | ✅     | engine.rs:761             |
| prebuffer vacant_len() guard         | ✅     | decoder.rs:382-386        |
| total_decoded_samples reset after prebuffer | ✅ | decoder.rs:406       |
| sync_ab_loop_atomics() correct       | ✅     | engine.rs:855-870         |
| ab_loop_seek_sample reset on new track | ✅   | engine.rs:223             |
| flush set on AB trigger              | ✅     | audiooutput.rs:796        |
| Decoder doesn't clear is_seeking     | ✅     | decoder.rs:408            |
| check_loop() runs every iteration    | ✅     | decoder.rs:344-351        |

---

## 🎵 EXPECTED RUNTIME BEHAVIOR

### Normal Seek:
1. Slider dragged → audio outputs silence
2. Decoder seeks + prebuffers in background
3. BufferReady sent → engine resets clock + DSP
4. Audio resumes smoothly with fade-in

### AB Loop:
1. Playback reaches B → audio callback detects, triggers seek to A
2. Decoder seeks to A, prebuffers
3. BufferReady sent → engine resets position to A
4. Audio continues seamlessly from A
5. No silence gaps, no deadlock

### Timer Behavior:
- Timer shows correct position during playback
- Timer snaps to A immediately when AB loop triggers
- Timer resets to 0:00 on new track
- No timer drift or "rubber banding"

---

END OF SPEC
```

---

# A-B Loop — Engine Integration Specification

Status: Production Stable (All critical bugs fixed)
Scope: Engine-level looping behavior integrated with FFmpeg decoder and UI bridge
Architecture Reference: .skill/audio/engine/abloop.md

---------------------------------------------------------------------

1. PURPOSE

A-B Loop allows the user to:

1) Mark point A (current playback position)
2) Mark point B (current playback position, must be > A)
3) Automatically loop between A and B
4) Reset when:
   - User toggles third time
   - Track changes
   - Playback engine reinitializes

This feature is ENGINE-SIDE. UI must not implement logic.

---------------------------------------------------------------------

2. ARCHITECTURAL LAYERING

A-B Loop is strictly an engine-layer feature.

It must NOT:
- Live in UI
- Live in QML timers
- Depend on update_tick()
- Modify playback_state directly

Correct ownership:

FfmpegEngine
 └── abloop: Arc<Mutex<ABLoop>>
 └── ab_loop_a: Arc<AtomicU64>   (Point A in ms)
 └── ab_loop_b: Arc<AtomicU64>   (Point B in ms)
 └── ab_loop_active: Arc<AtomicBool>
 └── ab_loop_seek_sample: Arc<AtomicU64> (seek target for AB loop)

Reason:
- Shared between UI thread and decoder thread
- Persistent across playback lifetime
- Resettable on track change
- Atomic flags for lock-free audio callback access

---------------------------------------------------------------------

3. OWNERSHIP MODEL

Struct Location:
src/audio/engine/engine.rs

pub struct FfmpegEngine {
    ...
    ab_loop: Arc<Mutex<ABLoop>>,
    ab_loop_a: Arc<AtomicU64>,
    ab_loop_b: Arc<AtomicU64>,
    ab_loop_active: Arc<AtomicBool>,
    ab_loop_seek_sample: Arc<AtomicU64>,
}

Rules:

- Created once during engine initialization
- Cloned and passed into decoder thread via Arc
- Never recreated inside toggle handler
- Never instantiated inside UI
- Atomic flags cloned into AudioOutput for audio callback access

---------------------------------------------------------------------

4. UI → ENGINE FLOW

4.1 QML

Button calls:

MusicModel.toggle_abLoop()

4.2 Bridge Layer
File: src/ui/bridge/core.rs

pub fn toggle_abLoop(&mut self) {
    self.playback_controller.lock().unwrap().toggle_abLoop();
}

Bridge must:
- Not mutate playback state
- Not call pause()
- Not perform seek

4.3 Service Layer
File: src/core/services/playback.rs

pub fn toggle_abLoop(&mut self) {
    let pos = self.ffmpeg.lock().unwrap().get_position_immut();
    let mut ab = self.ffmpeg.lock().unwrap().abLoop.lock().unwrap();
    ab.toggle(pos);
}

Rules:
- Read position from engine (source of truth)
- Operate on shared abLoop instance
- Must NOT pause playback
- Must NOT modify playback_state

---------------------------------------------------------------------

5. PLAYBACK LOOP INTEGRATION (CRITICAL)

File:
src/audio/io/decoder.rs (decoder main loop)
src/audio/io/audiooutput.rs (audio callback AB check)

### 5.1 Decoder-Side Loop Check (Brake)

Inside decoder main loop:

// check_loop() runs EVERY iteration as brake to prevent decode-past-B
if let Ok(ab) = ab_loop.lock() {
    if let Some(seek_to) = ab.check_loop(total_decoded_samples) {
        // DON'T call trigger_seek() here - just break and let audio callback handle
        // Set ab_loop_seek_sample for audio callback to detect
        control.ab_loop_seek_sample.store(seek_to, Ordering::SeqCst);
    }
}

Rules:
- check_loop() must run every iteration
- Seek must not stop decoder loop
- Seek must not change playback_state
- Loop must continue after seek
- check_loop() acts as BRAKE: prevents decoder from decoding past point B
- Decoder resets total_decoded_samples = A after prebuffer to prevent immediate re-trigger

### 5.2 Audio Callback AB Detection (Primary Trigger)

File: src/audio/io/audiooutput.rs
Location: Audio callback (f32 and i16 paths)

// Check AB loop in audio callback - GUARD prevents re-entrant seeking
if ab_loop_active.load(Ordering::SeqCst) && !seek_mode.load(Ordering::SeqCst) {
    let current = samples_played.load(Ordering::SeqCst);
    let b = ab_loop_b.load(Ordering::SeqCst);
    if current >= b {
        let a = ab_loop_a.load(Ordering::SeqCst);
        samples_played.store(a, Ordering::SeqCst);
        seek_mode.store(true, Ordering::SeqCst);
        flush.store(true, Ordering::SeqCst);
        ab_loop_seek_sample.store(a, Ordering::SeqCst);
    }
}

Rules:
- MUST have !seek_mode guard to prevent re-entrant AB seek loops
- Audio callback sets flush=true to drain ring buffer before decoder prebuffers
- Samples_played reset to A immediately (SSoT clock stays correct)
- ab_loop_seek_sample signals decoder to seek

---------------------------------------------------------------------

6. POSITION SOURCE OF TRUTH

There must be exactly ONE playback position source:

AudioOutput.samples_played (AtomicU64)

Engine must derive position from this.

Forbidden:
- Caching duplicate position
- Maintaining parallel counters
- Updating position only inside getter

If position is stale:
- B comparison fails
- Loop never triggers

---------------------------------------------------------------------

7. TRACK CHANGE RESET

File:
engine.rs

Inside:

pub fn start_audiooutput(...)

Reset all AB loop atomics BEFORE decoder spawn:

// Push reset state to atomics so AudioOutput sees clean state
ab_loop_active.store(false, Ordering::SeqCst);
ab_loop_a.store(0, Ordering::SeqCst);
ab_loop_b.store(0, Ordering::SeqCst);
ab_loop_seek_sample.store(0, Ordering::SeqCst);  // CRITICAL: prevent ghost seek

Also in load():

if let Ok(mut ab) = self.ab_loop.lock() {
    ab.reset();
}
// After start_audiooutput, reset AudioOutput atomics too
if let Some(ref mut audiooutput) = engine.audiooutput {
    audiooutput.reset_ab_loop();  // Resets stream_ab_loop_* fields
}

Reset must occur BEFORE playback resumes.

---------------------------------------------------------------------

8. STATE MACHINE

States:

Off
ASet
Active

Transitions:

Off -> Click -> ASet
ASet -> Click (pos > A) -> Active
ASet -> Click (pos <= A) -> Off
Active -> Click -> Off

Playback rule:

If state == Active AND position >= B:
    seek(A)

No other transitions allowed.

---------------------------------------------------------------------

9. UI SYNCHRONIZATION

UI properties:

- ab_state
- ab_point_a
- ab_point_b

Bridge must periodically sync engine → UI:

self.sync_abLoop();

sync_abLoop() must:
- Read from shared engine instance
- Emit *_changed() only when value changes

UI must NOT calculate A/B logic.

---------------------------------------------------------------------

10. THREADING MODEL

Two threads:

1) UI Thread
   - toggle_abLoop()
   - sync_abLoop()

2) Decoder Thread
   - check_loop() as brake
   - Prebuffer after AB seek
   - Send BufferReady when ready

3) Audio Thread (CPAL/audio callback)
   - AB loop detection (primary trigger)
   - Drains ring buffer even during seek (Black Hole pattern)
   - Sets flush + seek_mode + ab_loop_seek_sample

Shared via:

Arc<Mutex<ABLoop>>         (for ABLoop struct access)
Arc<AtomicU64>              (for ab_loop_a, ab_loop_b, ab_loop_seek_sample)
Arc<AtomicBool>             (for ab_loop_active)
Arc<AtomicBool>             (for unified seek_mode/is_seeking)

No duplicate state allowed.

---------------------------------------------------------------------

11. SEEK MODE UNIFICATION (CRITICAL FIX)

File: src/audio/engine/engine.rs, src/audio/io/decoder.rs, src/audio/io/audiooutput.rs

PROBLEM: AudioOutput had `seek_mode`, Decoder had `is_seeking` — two separate atomics.
RACE CONDITION: Engine might set seek_mode=false before decoder sets is_seeking=false, or vice versa.

FIX: Unified into single `Arc<AtomicBool>`:

// In start_audiooutput():
let seek_is_seeking = Arc::new(AtomicBool::new(false));

// Pass to DecoderControl:
let control = Arc::new(DecoderControl::new(
    ab_loop_seek_sample.clone(),
    seek_is_seeking.clone(),  // <-- unified atomic
));

// Pass to AudioOutput:
audiooutput.set_seek_mode_arc(seek_is_seeking.clone());

Now both AudioOutput.seek_mode and DecoderControl.is_seeking point to the SAME atomic.
When decoder sets is_seeking=false, audio callback immediately sees seek_mode=false.

---------------------------------------------------------------------

12. BLACK HOLE PATTERN (CRITICAL FIX)

File: src/audio/io/audiooutput.rs

PROBLEM: During seek_mode=true, audio callback output silence WITHOUT popping from ring buffer.
DEADLOCK: Ring buffer fills up → decoder's push_output() blocks (sleeps 1ms forever) →
         BufferReady never sent → seek_mode stays true → ETERNAL SILENCE.

FIX: Audio callback ALWAYS pops from ring buffer FIRST, then gates with silence:

// ALWAYS pop from ring buffer first — prevents decoder deadlock
// Even during seek/pause, we drain the buffer so decoder can push
if let Ok(mut c) = consumer.try_lock() {
    if let Some(ref mut cons) = *c {
        let samples_read = cons.pop_slice(&mut read_buffer);
        // ... update empty_count ...
    }
}

// Gating: replace with silence if seeking/paused
// Decoder can still push into ring buffer (preventing deadlock)
if is_seeking || is_paused {
    read_buffer.fill(0.0);
}

This ensures:
- Ring buffer never stays full during seek
- Decoder can always push new data
- BufferReady sent → seek_mode cleared → audio resumes

Applied to BOTH f32 and i16 audio callback paths.

---------------------------------------------------------------------

13. PREBUFFER DEADLOCK GUARD (CRITICAL FIX)

File: src/audio/io/decoder.rs

Added guard in AB loop prebuffer loop:

while buffered < min {
    // ... stop checks ...

    // Break if ring buffer is completely full — prevents push_output deadlock
    // Audio callback will drain buffer (even during seek) to make room
    if producer.vacant_len() == 0 {
        break;
    }

    // ... decode and push ...
}

Requires: use ringbuf::traits::Observer; for vacant_len() method.

---------------------------------------------------------------------

14. DECODER AB SEEK FLOW (VERIFIED CORRECT)

When AB loop triggers (audio callback detects current >= B):

1. Audio callback: sets seek_mode=true, flush=true, ab_loop_seek_sample=A
2. Decoder: detects ab_loop_seek_sample > 0 (swapped from atomic)
3. Decoder: sets is_seeking=true, seeking_state=SEEK_STATE_DECODING
4. Decoder: av_seek_frame to position A
5. Decoder: flush codecs (decoder.flush() x2)
6. Decoder: recreate resampler for new position
7. Decoder: prebuffer loop until min_buffer_samples
   - total_decoded_samples reset to A after prebuffer (prevents check_loop re-trigger)
8. Decoder: sets seeking_state=SEEK_STATE_READY
9. Decoder: sets is_seeking=false (unified atomic → audio callback sees seek_mode=false)
10. Decoder: sends BufferReady { samples_played: total_decoded_samples }
11. Engine: on_buffer_ready() called:
    - Sets samples_played = exact_samples
    - Resets audio_clock.reset()
    - Resets DSP (audiooutput.reset_dsp())
    - Sets output_state = Running (audiooutput.set_output_state(Running))
    - Sets seek_mode = false (audiooutput.set_seek_mode(false))
    - Triggers seek fade (audiooutput.trigger_seek_fade())
    - Clears seek flags (control.clear_seek())
    - Sets playback_state = Playing
12. Audio callback: reads seek_mode=false → outputs audio normally

---------------------------------------------------------------------

15. ON_BUFFER_READY FIX (CRITICAL FIX)

File: src/audio/engine/engine.rs

Added to on_buffer_ready():

pub fn on_buffer_ready(&mut self) {
    // STEP 1: Get exact position from decoder
    let target_ms = ...;
    let exact_samples = ...;

    // STEP 2: Set samples_played EXACT
    self.samples_played = exact_samples;
    self.audio_clock.reset();  // <-- ADDED: Reset SSoT clock

    // STEP 3: Reset DSP
    if let Some(ref mut audiooutput) = self.audiooutput {
        audiooutput.reset_dsp();
        audiooutput.set_output_state(OutputState::Running);  // <-- ADDED: Ensure hardware running
    }

    // STEP 4: Audio - clear seek mode
    if let Some(ref mut audiooutput) = self.audiooutput {
        audiooutput.set_seek_mode(false);
        audiooutput.trigger_seek_fade();
    }

    // STEP 5: Clear seek flags
    if let Some(ref control) = self.decoder_control {
        control.clear_seek();
    }

    // STEP 6: Set state = Playing
    self.playback_state = PlaybackState::Playing;
}

---------------------------------------------------------------------

16. SYNC_AB_LOOP_ATOMICS ORDERING (CRITICAL FIX)

File: src/audio/engine/engine.rs

Correct ordering for atomic writes:

Activating AB loop:
1. Write ab_loop_a (Point A)
2. Write ab_loop_b (Point B)
3. Write ab_loop_active = true  (LAST - audio callback checks this first)

Deactivating AB loop:
1. Write ab_loop_active = false (FIRST - audio callback stops checking)
2. Write ab_loop_a = 0
3. Write ab_loop_b = 0

This prevents race where audio callback reads active=true but A/B not yet written.

---------------------------------------------------------------------

17. FAILURE MODES

Typical causes of failure:

1) check_loop() not called or removed
2) Position source stale (samples_played not updated)
3) Multiple ABLoop instances
4) Seek stops decoder loop
5) Reset not called on track change
6) Playback state modified during seek
7) Unified seek atomic NOT used (race condition)
8) Audio callback does NOT pop during seek (deadlock)
9) flush not set on AB trigger (old data not drained)
10) total_decoded_samples not reset after prebuffer (immediate re-trigger)

---------------------------------------------------------------------

18. PROHIBITED FIXES

- Adding QML timers
- Forcing playback_state = Playing
- Calling pause() suppression hacks
- Recreating ABLoop inside toggle
- Calling seek() from UI layer
- Adding conditional hacks in toggle_abLoop()
- Using separate seek_mode and is_seeking (must be unified)
- NOT popping from ring buffer during seek (causes deadlock)

All logic must remain inside engine layer.

---------------------------------------------------------------------

19. VALIDATION CHECKLIST

Before marking complete:

[x] Only one ABLoop instance exists
[x] Decoder calls check_loop() every iteration (as brake)
[x] Seek does not pause playback
[x] Position derived from atomic sample counter
[x] Reset called on track change (BEFORE decoder spawn)
[x] No playback_state mutation by ABLoop
[x] UI reflects state only
[x] seek_mode and is_seeking are SAME atomic (unified)
[x] Audio callback pops during seek (Black Hole pattern)
[x] flush set on AB trigger
[x] total_decoded_samples reset after prebuffer
[x] sync_ab_loop_atomics() correct ordering
[x] on_buffer_ready() resets clock + sets Running state
[x] Prebuffer loop has vacant_len() guard
[x] ab_loop_seek_sample reset in start_audiooutput()

---------------------------------------------------------------------

20. EXPECTED RUNTIME BEHAVIOR

Log example:

A-B Loop: Point A set at 12.50s
A-B Loop: Active! Looping from 12.50s to 18.00s
Looping to 12.50s
Looping to 12.50s
...
A-B Loop: Off

Playback must never auto-pause.
Audio must be continuous (no silence gaps during AB loop).
Timer must reset to A when loop triggers.

---------------------------------------------------------------------

21. KNOWN LIMITATIONS

- For very short ranges (< 500ms), AB loop may not work smoothly due to decoder latency
- Timer position during seek may show brief flicker (BufferReady processing delay)
- crossfade_frames must be implemented for click-free transition (polish, not critical)

---------------------------------------------------------------------

END OF SPEC
