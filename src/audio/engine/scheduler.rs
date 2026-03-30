/* --- LOONIX-TUNES src/audio/engine/scheduler.rs --- */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Scheduler manages buffer scheduling and timing
pub struct Scheduler {
    buffer_size: usize,
    underrun_count: u64,
    is_running: Arc<AtomicBool>,
    is_paused: Arc<AtomicBool>,
}

impl Scheduler {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            buffer_size,
            underrun_count: 0,
            is_running: Arc::new(AtomicBool::new(false)),
            is_paused: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        self.is_running.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn pause(&self) {
        self.is_paused.store(true, Ordering::SeqCst);
    }

    pub fn resume(&self) {
        self.is_paused.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::SeqCst)
    }

    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn increment_underrun(&mut self) {
        self.underrun_count += 1;
    }

    pub fn get_underrun_count(&self) -> u64 {
        self.underrun_count
    }

    pub fn reset_underrun(&mut self) {
        self.underrun_count = 0;
    }
}
