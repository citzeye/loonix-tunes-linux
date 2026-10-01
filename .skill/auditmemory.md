Prompt Mode: Senior Rust-Qt FFI Auditor

"This is V2 of the application. Please pay extra attention to any potential leftover states or unclosed loops from the V1 to V2 refactoring process. I need to conduct a strict Memory Management & Leak Audit.

Even though Rust guarantees memory safety, I know memory leaks can still occur, especially across FFI boundaries and in audio processing.

Before we look at unused code or debug prints, please act as a Senior Systems Architect and analyze my project for potential memory pitfalls. Tell me exactly which files you need to see based on these 4 critical audit vectors:

The QML/C++ Bridge (qmetaobject-rs): Are there places where QString, QVariantList, or QVariantMap are created in loops or continuous updates (like progress bars) without being properly dropped or reused?

Audio Buffers & Engine: My audio engine uses FFMPEG and DSP chains. I need you to check my decoding loop and ring buffers. Are audio frames or float arrays accumulating in memory?

Concurrency (Arc<Mutex<T>>): I use Arc<Mutex<...>> heavily for bridging the Playback Controller, Audio Engine, and QML. Are there potential Reference Cycles (where I should be using Weak pointers instead), or lock contentions causing memory bloat?

Unbounded Collections: Are there any Vec, HashMap, or channels (mpsc) in my core engine or UI state that grow indefinitely over time?

Do not give me generic Rust advice. Tell me which specific files (audio/engine.rs, ui/core.rs, etc.) you want to inspect first to start this memory audit."

"Stop cutting your analysis short. Your previous audit plan stopped after only 1 file (scanner.rs). That is unacceptable for a full C++/Rust FFI audio application.

Look at the file tree I provided. I explicitly need you to identify the specific files handling the qmetaobject bridge, the FFmpeg ring buffers, and the UI tick update loops. Provide the FULL list covering all 4 vectors I asked for, without stopping."