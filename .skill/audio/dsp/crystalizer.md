Act as an expert Rust audio DSP and Qt/QML developer. I am building a music player with a native Rust audio backend and a QML frontend. I already have the base `.rs` and `.qml` files ready. 

I need you to write the implementation for a "Crystalizer" native DSP effect. 

Here are the strict architectural and DSP requirements:

1. **The DSP Algorithm (Crystalizer):**
   - The effect should act as a harmonic exciter + high-shelf boost.
   - Split the incoming audio into a "dry" and "wet" path.
   - On the wet path: Apply a High-Pass Filter (cutoff around 3kHz - 5kHz).
   - Then, apply a soft-clipping/saturation function (e.g., using `tanh`) to the high-passed signal to generate high-frequency harmonics (sparkle).
   - Finally, mix the wet signal back with the dry signal.

2. **Rust Backend Implementation (`crystalizer.rs` or similar):**
   - Create a `Crystalizer` struct.
   - It must have a `process(&mut [f32])` or `process_sample(sample: f32) -> f32` method to process audio buffers in real-time.
   - Expose an "Amount" or "Intensity" parameter (range 0.0 to 1.0).
   - **CRITICAL:** The audio thread must be lock-free. Use `std::sync::atomic::AtomicF32` (or `AtomicU32` with bit-casting) to receive parameter updates from the QML GUI thread. Do NOT use `Mutex` for audio parameters.

3. **QML Frontend Integration (`Ui.qml` SECTION SLIDER CONTROL ):**
   - Provide a clean QML implementation using a `Slider` to control the Crystalizer's "Amount".
   - Show the exact C++/Rust boilerplate or Qt bindings needed to connect this QML slider's `onValueChanged` signal to update the `Atomic` variable in the Rust backend without blocking the UI or Audio threads.

Please provide the minimal, modular, and highly performant code for both the Rust DSP logic and the QML UI binding.