/* --- LOONIX-TUNES src/audio/engine/seek.rs --- */
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SeekState {
    Idle = 0,
    Seeking = 1,
    Buffering = 2,
    Ready = 3,
}

impl From<u8> for SeekState {
    fn from(val: u8) -> Self {
        match val {
            1 => SeekState::Seeking,
            2 => SeekState::Buffering,
            3 => SeekState::Ready,
            _ => SeekState::Idle,
        }
    }
}

pub struct SeekController {
    state: AtomicU8,
    target_position: AtomicU64,
}

impl SeekController {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(SeekState::Idle as u8),
            target_position: AtomicU64::new(0),
        }
    }

    pub fn start_seek(&self, pos_ms: u64) {
        self.target_position.store(pos_ms, Ordering::SeqCst);
        self.set_state(SeekState::Seeking);
    }

    pub fn signal_buffer_ready(&self) {
        self.set_state(SeekState::Ready);
    }

    pub fn complete_seek(&self) {
        self.set_state(SeekState::Idle);
    }

    pub fn is_seeking(&self) -> bool {
        let current = self.get_state();
        current == SeekState::Seeking || current == SeekState::Buffering
    }

    pub fn is_buffer_ready(&self) -> bool {
        self.get_state() == SeekState::Ready
    }

    pub fn get_target_position(&self) -> u64 {
        self.target_position.load(Ordering::SeqCst)
    }

    pub fn get_state(&self) -> SeekState {
        self.state.load(Ordering::SeqCst).into()
    }

    pub fn set_state(&self, state: SeekState) {
        self.state.store(state as u8, Ordering::SeqCst);
    }
}
