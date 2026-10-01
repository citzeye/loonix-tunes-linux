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
