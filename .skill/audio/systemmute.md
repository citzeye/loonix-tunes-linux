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