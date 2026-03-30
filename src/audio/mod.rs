/* --- LOONIX-TUNES src/audio/mod.rs --- */

// Engine submodules
pub mod engine;

// Core audio functionality
pub mod audio_bus;
pub mod audio_output;
pub mod buffer;
pub mod dac;
pub mod decoder;
pub mod resample;

// DSP
pub mod dsp;
pub mod highres;

// Pre-scan loudness
pub mod scanner;

// Config & state
pub mod config;
pub mod metadata;

// UI components
pub mod playlist;
pub mod popup;

// Re-export key types
pub use self::audio_output::AudioOutput;
pub use self::decoder::DecoderControl;
pub use self::engine::{
    is_audio_file, AudioState, Engine, FfmpegEngine, MusicItem, OutputMode, ProAudioEngine,
};
pub use self::metadata::{read_track_metadata, TrackMetadata};
