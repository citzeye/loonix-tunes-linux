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
