/* --- loonixtunesv2/src/audio/mod.rs | Audio Module --- */

// Audio IO submodules
pub mod io;

// Engine submodules
pub mod engine;

// DSP
pub mod dsp;

// Presets
pub mod presets;

// Pre-scan loudness
pub mod scanner;

// Config & state
pub mod config;
pub mod metadata;

// System Media Controls (MPRIS on Linux)
#[cfg(target_os = "linux")]
pub mod sysmedia;
pub mod wireless;

// Re-export key types
pub use self::io::audiobus::AudioBus;
pub use self::io::audiooutput::AudioOutput;
pub use self::io::buffer::ringbuffer::RingBuffer;
pub use self::io::decoder::DecoderControl;
pub use self::io::resample::StereoResampler;
pub use crate::audio::engine::{is_audio_file, AudioState, Engine, FfmpegEngine, MusicItem, OutputMode};
pub use self::metadata::{read_track_metadata, TrackMetadata};