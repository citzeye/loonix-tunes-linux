/* --- LOONIX-TUNES src/audio/engine/state.rs --- */

#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Seeking,
}

pub struct EngineState {
    state: PlaybackState,
    volume: f32,
    balance: f32,
    duration_ms: u64,
    position_ms: u64,
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            state: PlaybackState::Stopped,
            volume: 1.0,
            balance: 0.0,
            duration_ms: 0,
            position_ms: 0,
        }
    }

    pub fn get_state(&self) -> &PlaybackState {
        &self.state
    }

    pub fn set_state(&mut self, state: PlaybackState) {
        self.state = state;
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, PlaybackState::Playing)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.state, PlaybackState::Paused)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.state, PlaybackState::Stopped)
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_balance(&self) -> f32 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance.clamp(-1.0, 1.0);
    }

    pub fn get_duration(&self) -> u64 {
        self.duration_ms
    }

    pub fn set_duration(&mut self, duration_ms: u64) {
        self.duration_ms = duration_ms;
    }

    pub fn get_position(&self) -> u64 {
        self.position_ms
    }

    pub fn set_position(&mut self, position_ms: u64) {
        self.position_ms = position_ms;
    }
}
