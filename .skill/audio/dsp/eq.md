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
