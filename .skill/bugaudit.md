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