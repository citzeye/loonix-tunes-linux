/* --- LOONIX-TUNES src/audio/mod.rs --- */

// Engine submodules
pub mod engine;

// Core audio functionality
pub mod audiobus;
pub mod audiooutput;
pub mod buffer;
pub mod decoder;
pub mod resample;

// DSP
pub mod dsp;

// Pre-scan loudness
pub mod scanner;

// Config & state
pub mod config;
pub mod metadata;
pub mod wireless;

// UI components
pub mod popup;

// System Media Controls (MPRIS on Linux)
#[cfg(target_os = "linux")]
pub mod sysmedia;

// Re-export key types
pub use self::audiooutput::AudioOutput;
pub use self::decoder::DecoderControl;
pub use self::engine::{is_audio_file, AudioState, Engine, FfmpegEngine, MusicItem, OutputMode};
pub use self::metadata::{read_track_metadata, TrackMetadata};
