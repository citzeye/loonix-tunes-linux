/* --- LOONIX-TUNES src/audio/dac.rs | The Specialist --- */
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Default)]
pub struct DacManager {
    pub exclusive_mode: Arc<AtomicBool>,
    pub current_sample_rate: u32,
}

impl DacManager {
    pub fn new() -> Self {
        Self {
            exclusive_mode: Arc::new(AtomicBool::new(false)),
            current_sample_rate: 48000,
        }
    }

    pub fn set_exclusive_mode(&mut self, enabled: bool) {
        self.exclusive_mode.store(enabled, Ordering::SeqCst);
    }

    #[cfg(target_os = "linux")]
    pub fn get_pipewire_properties(
        &mut self,
        track_sample_rate: u32,
    ) -> std::collections::HashMap<String, String> {
        let is_exclusive = self.exclusive_mode.load(Ordering::SeqCst);
        self.current_sample_rate = track_sample_rate;

        let mut props = std::collections::HashMap::new();
        props.insert("media.type".to_string(), "Audio".to_string());
        props.insert("media.category".to_string(), "Playback".to_string());
        props.insert("media.role".to_string(), "Music".to_string());
        props.insert("node.name".to_string(), "LoonixTunes".to_string());
        props.insert(
            "node.exclusive".to_string(),
            if is_exclusive { "true" } else { "false" }.to_string(),
        );
        props.insert("audio.rate".to_string(), track_sample_rate.to_string());
        props.insert("audio.channels".to_string(), "2".to_string());
        props.insert("audio.format".to_string(), "F32LE".to_string());

        if is_exclusive {
            println!(
                "[PIPEWIRE] VIP Exclusive Mode aktif. DAC dikunci di {}Hz.",
                track_sample_rate
            );
        } else {
            println!("[PIPEWIRE] Shared Mode jalan di {}Hz.", track_sample_rate);
        }

        props
    }

    #[cfg(target_os = "windows")]
    pub fn configure_stream(&mut self, track_sample_rate: u32) -> Result<(), String> {
        let is_exclusive = self.exclusive_mode.load(Ordering::SeqCst);
        self.current_sample_rate = track_sample_rate;

        if is_exclusive {
            println!(
                "[WINDOWS] WASAPI Exclusive Mode Request: {}Hz",
                track_sample_rate
            );
        }
        Ok(())
    }

    #[cfg(target_os = "android")]
    pub fn configure_stream(&mut self, track_sample_rate: u32) -> Result<(), String> {
        let is_exclusive = self.exclusive_mode.load(Ordering::SeqCst);
        self.current_sample_rate = track_sample_rate;

        if is_exclusive {
            println!(
                "[ANDROID] AAudio Exclusive/LowLatency Request: {}Hz",
                track_sample_rate
            );
        }
        Ok(())
    }
}
