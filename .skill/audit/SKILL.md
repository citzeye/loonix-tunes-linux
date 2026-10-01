---
name: audit
description: Panduan audit kode: bug, state, memori, unused, multi-OS, dan engine. Bisa dipakai project mana pun.
---
## audit.md

```
LOONIX-TUNES REALTIME AUDIO SYSTEM AUDIT REPORT
1. ARCHITECTURE DIAGRAM
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         AUDIO DATA FLOW                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  [File] → [Decoder + Resampler] → [RingBuffer] → [AudioCallback] → [Output]   │
│                                              ↓                                  │
│                                         [DspChain]                              │
│                                              ↓                                  │
│                                     [Volume/Balance]                           │
│                                              ↓                                  │
│                                  [OutputMode Processing]                       │
│                                              ↓                                  │
│                                       [Soft Clip]                              │
│                                              ↓                                  │
│                                   [cpal::Device::write]                        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         DSP CHAIN ORDER (rack.rs line 211-281)                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  STD MODE:                                                                    │
│  0. Preamp (Always ON)                                                         │
│  1. EQ                                                                        │
│  2. Crystalizer ←── SPATIAL (SHOULD BE AFTER COMPRESSOR!)                     │
│  3. Surround  ←── SPATIAL (SHOULD BE AFTER COMPRESSOR!)                       │
│  4. StereoWidth                                                               │
│  5. PitchShifter                                                              │
│  6. MiddleClarity ← SPATIAL                                                   │
│  7. StereoEnhance                                                            │
│  8. BassBooster (BiquadLowShelf)                                              │
│  9. Crossfeed                                                                 │
│  10. Compressor ← WRONG POSITION (SHOULD BE #2!)                              │
│  11. Reverb                                                                   │
│  12. Limiter (Always ON)                                                      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         THREAD STRUCTURE                                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  [UI Thread (QML)] ←──crossbeam/Atomics──→ [Engine Thread (Decoder)]         │
│                                                      ↓                         │
│                                              [cpal Audio Thread]               │
│                                                      ↓                         │
│                                              [DSP Chain - lock-free]           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
---
2. CRITICAL REALTIME VIOLATIONS (HIGH SEVERITY)
VIOLATION #1: DSP ORDER CONSTRAINT VIOLATION
Severity: CRITICAL  
Location: src/audio/dsp/rack.rs line 211-281  
Finding: DSP pipeline order violates mandatory constraint
Required: EQ → Compressor → Spatial → Limiter  
Actual: EQ → Crystalizer → Surround → StereoWidth → Pitch → MiddleClarity → StereoEnhance → BassBooster → Crossfeed → Compressor → Reverb → Limiter
Impact:
- Compressor at position 10 instead of position 2
- All spatial effects (Crystalizer, Surround, StereoWidth, MiddleClarity, StereoEnhance) running BEFORE compressor
- This violates the hardcoded architecture constraint
- May cause unexpected gain staging and audio artifacts
Code Evidence:
// rack.rs line 219-280
// 1. EQ Std
processors.push(Box::new(StdEqProcessor::with_bands(settings.eq_bands)));
// 2. Crystalizer Std - SPATIAL, should be after Compressor
let crystal = StdCrystalizer::new();
// ... more spatial effects ...
// 10. Compressor - WRONG POSITION, should be #2
let compressor = StdCompressor::new();
// 12. Limiter Std - Always ON
let limiter = StdLimiter::new();
---
VIOLATION #2: MUTEX IN AUDIO CALLBACK
Severity: HIGH  
Location: src/audio/audiooutput.rs line 554  
Finding: mode_shared.try_lock() called inside audio callback
Code Evidence:
// audiooutput.rs line 554-557
let current_mode = match mode_shared.try_lock() {
    Ok(m) => *m,
    Err(_) => OutputMode::Stereo,  // Fallback without lock
};
Analysis:
- This is a try_lock() which is non-blocking - acceptable fallback
- Uses try_lock not lock - partially compliant
- Could potentially cause brief audio artifact if mode changes during processing
- Recommendation: Acceptable as-is, but consider ArcSwap for OutputMode
---
VIOLATION #3: MUTEX FOR NORMALIZER IN AUDIO CALLBACK
Severity: HIGH  
Location: src/audio/audiooutput.rs line 578  
Finding: normalizer.try_lock() inside audio callback
Code Evidence:
// audiooutput.rs line 578-584
if normalizer_enabled.load(Ordering::SeqCst) {
    let safe_len = process_len.min(norm_input.len());
    norm_input[..safe_len].copy_from_slice(&processed_buffer[..safe_len]);
    if let Ok(mut norm) = normalizer.try_lock() {  // <-- MUTEX IN CALLBACK
        let norm_proc: &mut dyn crate::audio::dsp::DspProcessor = &mut *norm;
        norm_proc.process(&norm_input[..safe_len], &mut norm_output[..safe_len]);
        processed_buffer[..safe_len].copy_from_slice(&norm_output[..safe_len]);
    }
}
Impact:
- If lock fails, normalizer is skipped (silent fallback) - potential audio discontinuity
- Normalizer is a full DSP processor inside a Mutex
- This creates latency spike risk
---
3. AUDIO INTEGRITY VIOLATIONS (Sample Corruption Risk)
FINDING #1: DEFENSIVE LENGTH CHECK IN DSP CHAIN
Severity: MEDIUM  
Location: src/audio/dsp/chain.rs line 111-118
Code:
if output.len() != input.len() {
    if output.len() >= input.len() {
        output[..input.len()].copy_from_slice(input);
    }
    return;
}
Analysis: 
- This indicates potential for sample count mismatch
- The check exists but the handling could cause issues (partial copy, then return - losing frames)
- No evidence this is triggered in practice, but indicates architectural concern
---
FINDING #2: VEC ALLOCATION INSIDE DSP PROCESS LOOP
Severity: MEDIUM  
Location: src/audio/dsp/chain.rs line 151-152
Code:
// Heap allocation for large buffers
let mut buffer_a = vec![0.0f32; input.len()];
let mut buffer_b = vec![0.0f32; input.len()];
Analysis:
- Allocated inside process() method - violates "No memory allocation inside audio loop"
- Only triggered for buffers > 4096 samples
- For typical audio callback (~512-2048 samples), uses stack allocation (OK)
- Recommendation: Pre-allocate buffers at stream initialization
---
4. ATOMIC MISUSE FINDINGS
FINDING #1: LIMITER DEFAULT VALUE = FALSE
Severity: HIGH  
Location: src/audio/dsp/std/stdlimiter.rs line 10-14
Code:
static LIMITER_ENABLED: OnceLock<Arc<AtomicBool>> = OnceLock::new();
pub fn get_limiter_enabled_arc() -> Arc<AtomicBool> {
    LIMITER_ENABLED
        .get_or_init(|| Arc::new(AtomicBool::new(false)))  // DEFAULT: DISABLED
        .clone()
}
Analysis:
- Limiter default is false (disabled)
- But rack.rs line 277-280 sets it to true for Std Mode:
    // 12. Limiter Std - Always ON
  let limiter = StdLimiter::new();
  get_std_limiter_enabled().store(true, Ordering::Relaxed);
  - VERIFIED: This is correctly handled - atomic is set to true during rack build
- However, if toggle causes re-build without setting, could cause silent audio damage
---
FINDING #2: STEREO_AMOUNT DEFAULT = 1.0 (NOT 0.0)
Severity: MEDIUM  
Location: src/audio/dsp/std/stdstereoenhance.rs + dsp/mod.rs
Code in dsp/mod.rs:
stereo_enabled: false,
stereo_amount: 1.0,  // NOT 0.0 - could cause unexpected behavior
Analysis:
- If stereo is disabled but amount is 1.0, the processor may still pass through signal
- Check stdstereoenhance.rs process logic to verify this is handled
---
5. DSP EXECUTION GAPS (Effects Not Applied)
FINDING: DSP CHAIN SWAP CREATES NEW PROCESSORS EVERY TOGGLE
Severity: MEDIUM  
Location: src/audio/audiooutput.rs line 215-252  
Code:
pub fn update_dsp(&mut self, settings: &crate::audio::dsp::DspSettings) {
    // ... reads all atomic values ...
    let rack = crate::audio::dsp::DspRack::build_rack(true);  // CREATES NEW VECTOR
    self.dsp_chain.swap_chain(rack);  // ATOMIC REPLACE
}
Analysis:
- Each toggle calls build_rack(true) which creates new Box allocations
- Uses ArcSwap so old chain drops when no guards - lock-free but still allocates
- FIX APPLIED: Recent toggle fixes (BassBooster, Surround, etc.) removed unnecessary swap_chain calls - VERIFIED
---
6. MEMORY SAFETY ISSUES
ISSUE #1: UNSAFE CELL IN DSPCHAININNER
Severity: LOW (Justified Usage)  
Location: src/audio/dsp/chain.rs line 16-23
Code:
struct DspChainInner {
    processors: UnsafeCell<Vec<Box<dyn DspProcessor + Send + Sync>>>,
}
// SAFETY: DspChainInner is Sync because we ensure exclusive access
// via Guard (single audio thread per chain instance)
unsafe impl Sync for DspChainInner {}
Analysis:
- This is the correct pattern for audio processing
- Single audio thread (cpal callback) guarantees exclusive access
- Documented with SAFETY comment - VERIFIED
---
ISSUE #2: PRE-ALLOCATED BUFFERS IN AUDIOOUTPUT
Severity: NONE (Good Pattern)  
Location: src/audio/audiooutput.rs line 81-82
Code:
norm_input: Vec<f32>,
norm_output: Vec<f32>,
Analysis:
- Pre-allocated once during struct creation
- No allocation in callback - VERIFIED COMPLIANT
---
7. THREADING RISKS
FINDING: SHARED CONSUMER WITH MUTEX
Severity: MEDIUM  
Location: src/audio/audiooutput.rs line 74
Code:
shared_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
crossfade_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
Analysis:
- Consumer wrapped in Mutex - used in callback with try_lock
- Pattern: Try lock first, if fail, use fallback
- Acceptable but could be replaced with ArcSwap for lock-free operation
---
8. STATE & CLOCK SAFETY
FINDING #1: RESAMPLING IN DECODER THREAD (NOT DSP)
Severity: NONE (Correct Architecture)  
Location: src/audio/resample.rs + src/audio/decoder.rs
Analysis:
- Resampling happens in decoder thread, BEFORE ring buffer
- DSP never sees resampling - VERIFIED COMPLIANT
- DSP processes fixed-rate PCM
---
FINDING #2: RUBBERBAND FFI EXISTS BUT NOT IN MAIN CHAIN
Severity: LOW (Monitoring Required)  
Location: src/audio/dsp/std/stdrubberbandffi.rs
Analysis:
- Rubberband provides time-stretching (changes sample count)
- If used, would violate "no sample count change" constraint
- Not in standard/pro DSP chain builds - needs verification in engine
---
9. DSP ORDERING VERIFICATION
Stage
EQ
Compressor
Spatial (Surround, Crystalizer, etc.)
Limiter
---
10. INSTANCE OWNERSHIP VERIFICATION
Component	Owner
DspChain	AudioOutput
DspProcessors	DspChainInner (UnsafeCell)
Atomic Values	Global statics (OnceLock)
Normalizer	AudioOutput (Mutex)
VERIFIED: UI never mutates DSP struct directly - COMPLIANT
---
CONCRETE FIX RECOMMENDATIONS
PRIORITY 1: DSP ORDER FIX
File: src/audio/dsp/rack.rs  
Change: Reorder processors to: EQ → Compressor → (all Spatial) → Limiter
// Standard Mode - CORRECTED ORDER
// 0. Preamp (Always ON)
processors.push(Box::new(StdPreamp::new()));
// 1. EQ (First)
processors.push(Box::new(StdEqProcessor::with_bands(settings.eq_bands)));
// 2. COMPRESSOR (Second - moved from position 10)
let compressor = StdCompressor::new();
processors.push(Box::new(compressor));
// 3-8. ALL SPATIAL EFFECTS (After Compressor)
let crystal = StdCrystalizer::new();
let surround = StdSurroundProcessor::new();
let mono = StdStereoWidth::new();
let pitch = StdPitchShifter::new();
let middle = StdMiddleClarity::new();
let stereo = StdStereoEnhance::new();
let bass = StdBassBooster::new();
let crossfeed = StdCrossfeed::new();
// 9. Reverb (before Limiter)
let reverb = StdReverb::new();
// 10. Limiter (Last - always ON)
let limiter = StdLimiter::new();
PRIORITY 2: REMOVE MUTEX FROM NORMALIZER
File: src/audio/audiooutput.rs  
Change: Use ArcSwap for normalizer, or pre-process normalizer in engine thread
PRIORITY 3: PRE-ALLOCATE DSP BUFFERS
File: src/audio/dsp/chain.rs  
Change: Add buffer_a and buffer_b to DspChain struct, allocate once
---
VERIFIED COMPLIANT MODULES
1. src/ui/playerbridge.rs - Pure UI bridge, no audio thread access
2. src/audio/buffer/ring_buffer.rs - Lock-free ring buffer using ringbuf crate
3. src/audio/audiobus.rs - Audio routing, uses AtomicBool for state
4. src/audio/dsp/std/stdbassbooster.rs - Correctly reads atomic in process()
5. src/audio/resample.rs - Resampling in decoder thread (not DSP)
---
SUMMARY
Category
DSP Order
Mutex in Audio Thread
Atomic Misuse
Memory Allocation in Loop
Sample Count Alteration
Thread Safety
TOTAL CRITICAL
RECOMMENDED IMMEDIATE ACTION: Fix DSP ordering in rack.rs to comply with architecture constraint EQ → Compressor → Spatial → Limiter.
```

---

## auditengine.md

```
ROLE: Senior Realtime Audio Systems Auditor

You are auditing a production Rust-based MP3 player called Loonix-Tunes.

Architecture constraints (MANDATORY):

1. DSP Pipeline order must be:
   EQ → Compressor → Spatial → Limiter.

2. DSP MUST NOT change sample count under any circumstances.

3. Audio thread must be realtime-safe:
   - No Mutex
   - No blocking
   - No memory allocation inside audio loop
   - No Vec::new, Box::new, format!, clone of large buffers

4. UI ↔ Audio communication must use:
   - Arc<AtomicF32>
   - Arc<AtomicBool>
   - lock-free ring buffers
   - crossbeam channels for non-realtime messages only

5. No panic:
   - No unwrap()
   - No expect()
   - Proper Result handling

6. No QML polling timers for audio state.

----------------------------------------

Your Task:

Perform a FULL SYSTEM AUDIT of the following directories:

src/audio/
src/audio/dsp/
src/audio/engine/
src/audio/buffer/
src/audio/audiobus.rs
src/audio/audiooutput.rs
src/ui/playerbridge.rs

----------------------------------------

Audit must verify:

A) DSP Execution Integrity
- Is DspChain::process() always called in audio callback?
- Is the processed buffer overwritten after DSP?
- Are all DSP modules actually modifying samples?
- Are Atomic parameters properly loaded inside process()?

B) Atomic Migration Correctness
- Are there cached fields that no longer update?
- Are Atomic default values incorrect (0.0 causing silent DSP)?
- Are Atomics cloned correctly across threads?
- Is there any stale DSP instance used by audio thread?

C) Thread Safety
- Any Mutex in audio callback?
- Any allocation inside process loop?
- Any blocking channel usage inside callback?
- Any dynamic DSP creation inside callback?

D) State & Clock Safety
- Does any DSP alter buffer length?
- Any resampling inside DSP?
- Any rubberband/time-stretch altering sample count post-decode?

E) DSP Ordering
- Confirm EQ → Compressor → Spatial → Limiter
- Confirm limiter is last stage before output

F) Instance Ownership
- Is DSP instance owned only by audio thread?
- Does UI ever mutate DSP struct directly?
- Is Arc<Atomic> the only shared mutable state?

----------------------------------------

Output Format Required:

1. Architecture Diagram (based on real code)
2. Critical Realtime Violations (HIGH severity)
3. Audio Integrity Violations (sample corruption risk)
4. Atomic Misuse Findings
5. DSP Execution Gaps (effects not applied)
6. Memory Safety Issues
7. Threading Risks
8. Concrete Fix Recommendations (code-level)

Do NOT give generic advice.
Base all conclusions on real observed code patterns.

If a subsystem is correct, explicitly state:
"VERIFIED: No violation found in this module."

This is a production system audit.
Be strict and exhaustive.
```

---

## auditmemory.md

```
Prompt Mode: Senior Rust-Qt FFI Auditor

"This is V2 of the application. Please pay extra attention to any potential leftover states or unclosed loops from the V1 to V2 refactoring process. I need to conduct a strict Memory Management & Leak Audit.

Even though Rust guarantees memory safety, I know memory leaks can still occur, especially across FFI boundaries and in audio processing.

Before we look at unused code or debug prints, please act as a Senior Systems Architect and analyze my project for potential memory pitfalls. Tell me exactly which files you need to see based on these 4 critical audit vectors:

The QML/C++ Bridge (qmetaobject-rs): Are there places where QString, QVariantList, or QVariantMap are created in loops or continuous updates (like progress bars) without being properly dropped or reused?

Audio Buffers & Engine: My audio engine uses FFMPEG and DSP chains. I need you to check my decoding loop and ring buffers. Are audio frames or float arrays accumulating in memory?

Concurrency (Arc<Mutex<T>>): I use Arc<Mutex<...>> heavily for bridging the Playback Controller, Audio Engine, and QML. Are there potential Reference Cycles (where I should be using Weak pointers instead), or lock contentions causing memory bloat?

Unbounded Collections: Are there any Vec, HashMap, or channels (mpsc) in my core engine or UI state that grow indefinitely over time?

Do not give me generic Rust advice. Tell me which specific files (audio/engine.rs, ui/core.rs, etc.) you want to inspect first to start this memory audit."

"Stop cutting your analysis short. Your previous audit plan stopped after only 1 file (scanner.rs). That is unacceptable for a full C++/Rust FFI audio application.

Look at the file tree I provided. I explicitly need you to identify the specific files handling the qmetaobject bridge, the FFmpeg ring buffers, and the UI tick update loops. Provide the FULL list covering all 4 vectors I asked for, without stopping."
```

---

## bugaudit.md

```
You are performing a full playback pipeline audit on a Rust-based audio engine.

Goal:
Find the root cause of intermittent premature end-of-track (EOT) where:
- First track sometimes plays fully.
- Second track sometimes cuts early.
- Seeking near the end allows full playback.
- Decoder EOF logs are correct.
- VBR files involved.
- No panics.
- No obvious deadlocks.

Architecture summary:

Backend:
- FFmpeg decoder → produces interleaved f32 samples
- Optional resampler (flush emits delayed frames)
- RingBuffer (producer = decoder thread, consumer = audio callback)
- CPAL output callback pulls from ring buffer
- DSP (EQ, reverb, comp) happens AFTER engine output (not in decoder thread)
- Optional VST3 after DSP
- Then soundcard

Engine EOT logic:
1. Decoder sends DecoderEvent::EndOfTrack { total_samples }
2. Engine sets decoder_eof = true
3. update_tick():
   if decoder_eof && is_playing {
       if audio_output.is_truly_buffer_empty() {
           end_of_track = true
           decoder_eof = false
       }
   }

Starvation detection:
- Audio callback tracks samples_read_from_consumer
- If samples_read_from_consumer == 0 for 3 consecutive callbacks:
  empty_callback_count >= 3
- is_truly_buffer_empty() returns that condition

Duration logic:
- Metadata duration used initially (can be inaccurate for VBR)
- After EOF:
  true_duration_ms = total_samples * 1000 / (sample_rate * channels)
- Engine overwrites duration with decoded value

Observed behavior:
- Sometimes playback ends early.
- Sometimes only on second track.
- Sometimes only without seek.
- Logs show:
  "Estimating duration from bitrate, this may be inaccurate"
  and normal EOF logs.

Your task:

1. Trace the entire lifecycle of a track:
   - load
   - play
   - decode
   - EOF
   - buffer drain
   - end_of_track
   - auto-next
   - next track init

2. Look for:
   - Race conditions between decoder thread and audio callback
   - Ring buffer underflow misinterpreted as EOT
   - Duration mismatch affecting state machine
   - Incorrect reset between tracks
   - Atomic ordering mistakes
   - Channel/sample miscalculation
   - Callback starvation false positives
   - Improper resampler flush handling
   - State leakage across tracks

3. Specifically verify:
   - All flags reset on stop() and next track
   - empty_callback_count resets correctly
   - decoder_eof cannot trigger before final samples enqueued
   - No scenario where producer stops before flushing resampler
   - Ring buffer len() consistency under concurrency

4. Do NOT suggest superficial fixes.
   Identify the precise failure mechanism.

5. If multiple plausible causes exist,
   rank them by probability and explain why.

Output format:
- Root cause candidates (ranked)
- Supporting reasoning
- Exact code-level vulnerability pattern
- Concrete fix (minimal, deterministic, architecture-safe)
```

---

## stateaudit.md

```
URGENT CODE AUDIT: STATE PERSISTENCE & UI SYNCHRONIZATION

CONTEXT: > We have just unified the configuration into a single Arc<Mutex<AppConfig>> in src/audio/config.rs. Now, we need to audit every function in the backend that handles "Saving" or "Updating" state to ensure they follow the new architecture.

TASK:
Check every setter function and "Save" method in the following files:

src/ui/theme.rs (ThemeManager)

src/ui/core.rs (MusicModel)

src/audio/config.rs (AppConfig)

AUDIT CRITERIA (The "Smart Save" Rules):

Rule 1: Use Shared Config. > Ensure NO function uses local variables or confy. They must lock the shared Arc<Mutex<AppConfig>>, update the value, and then call cfg.save().

Rule 2: Immediate UI Refresh (Smart Apply).
In theme.rs, when set_custom_theme_colors or names are called, check if the index being edited is the Active Theme. If YES, it MUST trigger self.set_theme() immediately so the UI reflects changes without a restart.

Rule 3: Audio State Persistence.
In core.rs, ensure functions like set_volume, set_balance, and EQ fader updates are correctly writing to the shared AppConfig. Check if the AppConfig implementation of save() is actually being called after these updates.

Rule 4: Metadata & Playlists.
Ensure that adding custom_folders or updating favorites follows the same SSOT pattern.

Rule 5: Signal Notification.
Every setter must emit its corresponding qt_signal (e.g., colormap_changed, music_state_changed, etc.) after saving, so QML stays in sync.

REQUIRED OUTPUT:
Identify any functions that are still "hardcoded", missing a .save() call, or failing to notify the UI. Provide the corrected code for those specific functions.
```

---

## multiosaudit.md

```
System Audit Request: Cross-Platform Compatibility (Linux, Windows, Android)

"Gue mau lo melakukan audit menyeluruh terhadap seluruh codebase Loonix-tunes. Aplikasi ini harus berjalan di Linux (PipeWire), Windows (WASAPI), dan Android (AAudio). Cek setiap file .rs dan .qml untuk mendeteksi potensi 'leak' platform-specific yang bisa bikin build rusak di OS lain.

Audit checklist:

Cargo.toml Dependencies: Cari crate yang hanya jalan di Linux (seperti alsa, pipewire tanpa wrapper, atau library sistem .so). Pastikan mereka dibungkus dalam [target.'cfg(target_os = "linux")'.dependencies].

Conditional Compilation: Cek apakah fungsi low-level (Audio Engine, File System, DAC Management) sudah dibungkus dengan atribut #[cfg(target_os = "linux")], #[cfg(windows)], atau #[cfg(target_os = "android")].

Hardcoded Paths: Cari string path seperti ~/Music, /home/..., atau C:\.... Pastikan semua path management menggunakan std::path::PathBuf atau crate dirs supaya dinamis mengikuti OS.

Audio Backend: Pastikan logic inisialisasi audio tidak memaksa memanggil driver Linux saat berjalan di Windows/Android.

QML Resources: Cek apakah ada font atau path asset yang menggunakan format Linux-only.

Thread & Process: Cek jika ada penggunaan perintah shell Linux (std::process::Command) yang tidak akan jalan di Windows CMD/PowerShell.

Jika lo nemu fungsi atau file yang 'bocor' (Linux-only tapi gak dibungkus cfg), jangan cuma kasih tahu, tapi berikan solusi refactor menggunakan Conditional Compilation atau Abstraction Layer agar build tetap sukses di 3 OS tersebut."

Kenapa Prompt Ini Penting?
Dependency Leak: Seringkali kita nambahin alsa di [dependencies] biasa. Pas di Windows, Cargo bakal nyari file .h ALSA dan langsung error biarpun kodenya nggak dipanggil. Prompt ini maksa AI buat benerin Cargo.toml.

Pathing: Di Linux itu /, di Windows itu \. Kalau lo pake hardcoded string, file picker lo bakal mati total di salah satu OS.

Shell Commands: Kalau lo iseng manggu Command::new("ls"), di Windows bakal error karena perintahnya dir.
```

---

# 🧹 Unused Code Audit Guide (Loonix-Tunes)

Dokumen ini adalah *checklist* untuk membersihkan "Dead Code" (kode zombie/tidak terpakai) setelah migrasi arsitektur ke sistem Lazy Update (Atomics) dan perombakan QML.

Lakukan audit ini secara berkala agar *binary size* tetap kecil dan *codebase* tidak berantakan.

## 🛠️ Tahap 1: Rust Backend Audit (The Compiler Way)

Rust punya *tooling* bawaan yang sangat agresif terhadap *dead code*. Gunakan ini sebagai langkah pertama:

### 1. Jalankan Cargo Check & Clippy
Jalankan perintah ini di terminal:
`cargo check`

ATAU untuk analisa yang lebih dalam:
`cargo clippy -- -D warnings`

cari warning seperti:

- unused import
- unused variable
- value assigned but never read
- fields are never read
- function/method never used
- struct never constructed
- associated items never used
- dead_code

Tugas kamu:

1. Untuk SETIAP item yang kena warning:
   - Jelaskan apakah benar-benar tidak digunakan (safe to delete),
   - Atau sebenarnya bagian dari flow arsitektur tapi tidak terhubung (harusnya dipakai tapi belum dikoneksikan).

2. Untuk kasus "value assigned is never read":
   - Analisa apakah:
     a) memang redundant assignment,
     b) state machine bug (harusnya dibaca tapi ketimpa),
     c) logic reconnect / retry yang tidak lengkap.

3. Untuk "fields are never read":
   - Cek apakah field:
     a) bagian dari future feature,
     b) state yang seharusnya dipakai tapi lupa wiring,
     c) memang dead field yang bisa dihapus.

4. Untuk function/method yang tidak pernah dipanggil:
   - Telusuri apakah:
     a) memang legacy code,
     b) intended untuk dipanggil dari layer lain tapi tidak pernah di-hook,
     c) bagian dari desain modular yang belum selesai.

5. Jangan hanya menyarankan:
   - "hapus saja"
   - atau "prefix dengan _"

Saya ingin:
- Analisis berbasis arsitektur
- Deteksi bug tersembunyi akibat state yang tidak pernah terbaca
- Identifikasi reconnect/state logic yang salah (karena banyak reconnect_attempts warning)

Output format WAJIB:

Untuk setiap item:

[ITEM]
Lokasi:
Jenis warning:
Analisa:
Status: (DELETE / FIX LOGIC / CONNECT MISSING FLOW / KEEP)

Jika FIX atau CONNECT:
Tunjukkan secara konkret bagian mana yang seharusnya membaca/menggunakan item tersebut. 
Jika kamu menemukan pola state yang di-set tapi tidak pernah digunakan untuk decision making,
anggap itu sebagai kemungkinan bug desain dan jelaskan dampaknya terhadap runtime behavior. 

---
*Loonix-Tunes Development Routine - Run this checklist before merging major feature branches.*
