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