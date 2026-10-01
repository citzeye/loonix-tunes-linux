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