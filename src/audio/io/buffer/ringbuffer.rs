/* --- loonixtunesv2/src/audio/buffer/ringbuffer.rs | Ringbuffer --- */

use ringbuf::traits::Consumer;
use ringbuf::traits::Split;
use ringbuf::{HeapCons, HeapProd, HeapRb};

pub struct RingBuffer {
    prod: HeapProd<f32>,
    cons: HeapCons<f32>,
    capacity: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        let rb = HeapRb::<f32>::new(capacity);
        let (prod, cons) = rb.split();

        Self {
            prod,
            cons,
            capacity,
        }
    }

    pub fn clear(&mut self) {
        while self.cons.try_pop().is_some() {}
    }

    pub fn get_producer(&mut self) -> &mut HeapProd<f32> {
        &mut self.prod
    }

    pub fn get_consumer(&mut self) -> &mut HeapCons<f32> {
        &mut self.cons
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
