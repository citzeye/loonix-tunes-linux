---
name: native-controls
description: Kontrol media native MPRIS/SMTC, mute sistem, dan samplerate.
---
# TASK: Implement Native Media Controls (MPRIS/SMTC) using `souvlaki`

## Context
You are working on `loonix-tunes`, a native desktop audio player built with Rust (backend) and Qt6/QML (frontend). The project uses a custom audio engine (FFmpeg -> PipeWire/VST3). We need to implement system-level media controls so the app can be controlled via the Linux Top Panel (MPRIS) and Windows Media Controls (SMTC).

## Objective
Integrate the `souvlaki` crate to handle bi-directional media control and metadata broadcasting.

## Architectural Requirements

### 1. Dependency & Module Setup
- Add `souvlaki` to `Cargo.toml`.
- Create a new module `src/audio/sysmedia.rs` to encapsulate all `souvlaki` logic. Keep the main engine file clean.

### 2. Bi-Directional Synchronization
**A. Player to System (Broadcasting State):**
- When a track changes, update the MPRIS metadata (Title, Artist, Album, Cover Art URL, Duration).
- When playback state changes (Play, Pause, Stop), update the MPRIS playback status.
- When the user scrubs the timeline, update the MPRIS position.

**B. System to Player (Handling Inputs):**
- Listen for events from the system (e.g., user clicks "Next" on their keyboard media keys or Linux top bar).
- Handle `souvlaki::MediaControlEvent` (Play, Pause, Toggle, Next, Previous).

### 3. Threading & Concurrency (CRITICAL)
- The UI (Qt) and Audio Engine run on their own threads. MPRIS integration MUST NOT block the audio rendering thread or the Qt main event loop.
- Implement an MPSC channel (using `std::sync::mpsc` or `crossbeam-channel`) to send media control events from the `souvlaki` event loop back to the main application state / audio engine.
- Ensure the `souvlaki` MediaControls instance is kept alive for the duration of the app but is gracefully dropped during object destruction to prevent memory leaks.

### 4. Error Handling
- D-Bus connection can fail on some Linux setups. Wrap the `souvlaki` initialization in proper `Result` types. If it fails to initialize, log a warning but DO NOT crash the application. The player must still function perfectly without MPRIS.

## Execution Steps
1. Write the implementation for `sys_media.rs`.
2. Show how to integrate it into the main state/engine struct.
3. Show where and how to spawn the event listener thread.
4. Ensure code adheres to strict Rust safety standards (no unwraps on external system calls).

Please provide the code implementation and explain the threading architecture you chose.

---

## audio/systemmute.md

```
PROJECT: Loonix-Tunes
FILE TO MODIFY: src/audio/systemmute.rs
LANGUAGE: Rust
AUDIO STACK: PipeWire (via pipewire-pulse compatibility layer)
LIBRARY: libpulse-binding (threaded mainloop API)

GOAL:
Implement a production-grade, event-driven system mute monitor using libpulse-binding with PipeWire (pipewire-pulse). 
This must be:
- Fully event-driven (NO polling)
- NO pactl
- NO spawning processes
- NO recreating mainloop/context repeatedly
- NO hardcoded sink names
- Proper lock discipline (threaded mainloop rules)
- Clean shutdown support
- Correct default sink detection
- Stable under device change (USB/Bluetooth/HDMI switch)

ARCHITECTURE REQUIREMENTS:

1) Use libpulse_binding::mainloop::threaded::Mainloop
2) Use a single Mainloop and single Context for the entire monitor lifetime
3) Call mainloop.start() once
4) Wait for Context State::Ready using mainloop.wait()
5) Subscribe to InterestMaskSet::SINK
6) On Sink event (Changed/New/Removed), re-query default sink mute state
7) To query mute:
   - Call get_server_info()
   - Extract default_sink_name
   - Call get_sink_info_by_name(default_sink_name)
   - Store info.mute into AtomicBool
8) NO hardcoded sink name
9) NO polling loop for querying mute
10) Thread should remain alive with light sleep loop (1s)
11) On shutdown:
    - lock mainloop
    - context.disconnect()
    - unlock
    - mainloop.stop()

THREADING RULES (CRITICAL):

- In threaded mainloop API, callbacks are executed in the internal Pulse thread.
- DO NOT call mainloop.lock() inside subscribe callback.
- All context operations that require sync must be done under proper lock discipline.
- Do not double-lock (deadlock risk).
- Do not spawn new mainloop inside query function.
- Do not use thread::park().

GLOBAL STATE:

static SYSTEM_MUTED: AtomicBool (Ordering::SeqCst)
static MONITOR_RUNNING: AtomicBool (Ordering::SeqCst)

PUBLIC API REQUIRED:

pub fn isSystemMuted() -> bool
pub fn startMuteMonitor()

EXPECTED BEHAVIOR:

When user mutes system via:
- pavucontrol
- keyboard mute key
- device change

Then:
- Subscribe callback triggers
- Default sink is re-queried
- SYSTEM_MUTED updated correctly
- No crash
- No freeze
- No deadlock

DEBUG LOGGING:

Add debug logs:
- On monitor start
- On subscribe event (print facility + operation)
- When querying default sink
- When mute state updated
- On shutdown

DO NOT:

- Use pactl
- Use polling-based mute check
- Recreate context per event
- Hardcode ALSA sink name
- Cache default sink permanently
- Block mainloop thread improperly

FINAL IMPLEMENTATION STRUCTURE SHOULD LOOK LIKE:

---------------------------------------------------------

#![allow(non_snake_case)]

use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::subscribe::{Facility, InterestMaskSet, Operation};
use libpulse_binding::context::{Context, FlagSet, State};
use libpulse_binding::mainloop::threaded::Mainloop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static SYSTEM_MUTED: AtomicBool = AtomicBool::new(false);
static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn isSystemMuted() -> bool {
    SYSTEM_MUTED.load(Ordering::SeqCst)
}

pub fn startMuteMonitor() {
    if MONITOR_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    thread::spawn(move || {
        run_monitor();
    });
}

fn run_monitor() {
    // create mainloop
    // create context
    // connect
    // start mainloop
    // wait for Ready
    // initial query_default_sink_mute
    // set_subscribe_callback
    // subscribe InterestMaskSet::SINK
    // unlock
    // keep thread alive while MONITOR_RUNNING true
    // clean shutdown sequence
}

fn query_default_sink_mute(context: &Context) {
    // get_server_info
    // extract default_sink_name
    // get_sink_info_by_name
    // update SYSTEM_MUTED
}

---------------------------------------------------------

TESTING STEPS AFTER IMPLEMENTATION:

1) Build project
2) Run app
3) Mute via pavucontrol
4) Confirm logs show:
   - Sink event triggered
   - Mute status updated true
5) Unmute
6) Confirm status updated false
7) Plug USB DAC or switch Bluetooth device
8) Confirm monitor still tracks correct default sink

OUTPUT EXPECTATION:

Return full corrected src/audio/systemmute.rs file content.
No explanations.
No commentary.
Only final Rust code.

This must compile cleanly with current libpulse-binding and work under PipeWire 1.6.x (pipewire-pulse).
```
