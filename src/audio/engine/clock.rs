/* --- LOONIX-TUNES src/audio/engine/clock.rs --- */

/// Audio clock based on sample counter (not system timer)
pub struct AudioClock {
    sample_rate: u32,
    channels: u32,
    samples_played: u64,
    position_ms: u64,
    is_running: bool,
}

impl AudioClock {
    pub fn new(sample_rate: u32, channels: u32) -> Self {
        Self {
            sample_rate,
            channels,
            samples_played: 0,
            position_ms: 0,
            is_running: false,
        }
    }

    pub fn reset(&mut self) {
        self.samples_played = 0;
        self.position_ms = 0;
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn update(&mut self, samples: u64) {
        if self.is_running {
            self.samples_played += samples;
            if self.sample_rate > 0 {
                self.position_ms =
                    (self.samples_played * 1000) / (self.sample_rate as u64 * self.channels as u64);
            }
        }
    }

    pub fn set_position(&mut self, position_ms: u64) {
        self.position_ms = position_ms;
        self.samples_played = (position_ms * self.sample_rate as u64 * self.channels as u64) / 1000;
    }

    pub fn get_position_ms(&self) -> u64 {
        self.position_ms
    }

    pub fn get_position_seconds(&self) -> f64 {
        self.position_ms as f64 / 1000.0
    }

    pub fn get_samples_played(&self) -> u64 {
        self.samples_played
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}
