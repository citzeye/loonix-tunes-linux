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
