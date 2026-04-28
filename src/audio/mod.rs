/* --- loonixtunesv2/src/audio/mod.rs | mod --- */

// Audio IO submodules
pub mod io;

// Engine submodules
pub mod engine;

// DSP
pub mod dsp;

// Config & state
pub mod config;

// System Media Controls (MPRIS on Linux)
#[cfg(target_os = "linux")]
pub use crate::core::services::sysmedia;

// Re-export key types
pub use self::io::audiobus::AudioBus;
pub use self::io::audiooutput::AudioOutput;
pub use self::io::buffer::ringbuffer::RingBuffer;
pub use self::io::decoder::DecoderControl;
pub use self::io::resample::StereoResampler;
pub use crate::audio::engine::{is_audio_file, AudioState, Engine, FfmpegEngine, MusicItem, OutputMode};