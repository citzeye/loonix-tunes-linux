/* --- loonixtunesv2/src/core/services/mod.rs | Services Module --- */

pub use self::fileservice::FileService;
pub use self::fileservice::get_file_service;
pub use self::playback::PlaybackController;

#[cfg(target_os = "linux")]
pub use self::sysmedia::SysMediaManager;
#[cfg(target_os = "linux")]
pub use self::wireless::WirelessManager;

mod fileservice;
mod playback;
#[cfg(target_os = "linux")]
mod sysmedia;
#[cfg(target_os = "linux")]
pub mod wireless;