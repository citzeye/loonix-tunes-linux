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