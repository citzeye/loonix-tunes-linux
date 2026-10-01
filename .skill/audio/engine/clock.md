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