/* --- LOONIX-TUNES src/audio/buffer/shared_ring_buffer.rs --- */

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct SharedRingBuffer {
    buffer: Arc<Mutex<VecDeque<f32>>>,
    capacity: usize,
}

impl SharedRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            capacity,
        }
    }

    pub fn push(&self, samples: &[f32]) -> usize {
        if let Ok(mut buf) = self.buffer.lock() {
            let available = self.capacity - buf.len();
            let to_write = samples.len().min(available);
            for i in 0..to_write {
                buf.push_back(samples[i]);
            }
            to_write
        } else {
            0
        }
    }

    pub fn pop(&self, count: usize) -> Vec<f32> {
        if let Ok(mut buf) = self.buffer.lock() {
            let to_read = count.min(buf.len());
            (0..to_read)
                .map(|_| buf.pop_front().unwrap_or(0.0))
                .collect()
        } else {
            vec![0.0; count]
        }
    }

    pub fn available(&self) -> usize {
        if let Ok(buf) = self.buffer.lock() {
            buf.len()
        } else {
            0
        }
    }

    pub fn clear(&self) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Ok(buf) = self.buffer.lock() {
            buf.is_empty()
        } else {
            true
        }
    }

    pub fn get_arc(&self) -> Arc<Mutex<VecDeque<f32>>> {
        self.buffer.clone()
    }
}

impl Clone for SharedRingBuffer {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            capacity: self.capacity,
        }
    }
}
