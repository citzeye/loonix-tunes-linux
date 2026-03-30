/* --- LOONIX-TUNES src/audio/dsp/chain.rs --- */

use crate::audio::dsp::DspProcessor;
use arc_swap::ArcSwap;
use std::cell::UnsafeCell;
use std::fmt;
use std::sync::Arc;

/// Thread-safe DSP Chain using ArcSwap for lock-free processing
pub struct DspChain {
    chain: Arc<ArcSwap<DspChainInner>>,
}

/// Inner chain with interior mutability for processors
struct DspChainInner {
    // UnsafeCell allows mutation through shared reference
    // Safety: We guarantee exclusive access via Guard (single audio thread)
    processors: UnsafeCell<Vec<Box<dyn DspProcessor + Send + Sync>>>,
}

// SAFETY: DspChainInner is Sync because we ensure exclusive access
// via Guard (single audio thread per chain instance)
unsafe impl Sync for DspChainInner {}

impl DspChain {
    pub fn new() -> Self {
        let inner = DspChainInner::new();
        Self {
            chain: Arc::new(ArcSwap::from_pointee(inner)),
        }
    }

    /// Process audio - lock-free, no mutex needed
    pub fn process(&self, input: &[f32], output: &mut [f32]) {
        // load() returns a Guard that keeps the inner alive
        let guard = self.chain.load();
        guard.process(input, output);
    }

    /// Replace entire effect chain atomically
    pub fn swap_chain(&self, new_rack: crate::audio::dsp::DspRack) {
        let new_inner = DspChainInner::from_rack(new_rack);
        // store() atomically replaces the inner, old inner dropped when no Guards reference it
        self.chain.store(Arc::new(new_inner));
    }

    /// Reset all processors in the chain
    pub fn reset(&self) {
        let guard = self.chain.load();
        guard.reset();
    }

    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        let guard = self.chain.load();
        guard.is_empty()
    }
}

impl Clone for DspChain {
    fn clone(&self) -> Self {
        Self {
            chain: self.chain.clone(),
        }
    }
}

impl Default for DspChain {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DspChain {
    fn drop(&mut self) {
        // ArcSwap will handle dropping when last reference is gone
        // No need for manual cleanup
    }
}

impl DspChainInner {
    pub fn new() -> Self {
        Self {
            processors: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn from_rack(rack: crate::audio::dsp::DspRack) -> Self {
        Self {
            processors: UnsafeCell::new(rack.processors),
        }
    }

    /// Process audio through the DSP chain using ping-pong buffering.
    /// Even-indexed processors read from buffer_a, odd-indexed from buffer_b.
    /// The last processor writes directly to output.
    ///
    /// # Safety
    /// Must have exclusive access (guaranteed by Guard)
    pub fn process(&self, input: &[f32], output: &mut [f32]) {
        // SAFETY: exclusive access via Guard
        let processors = unsafe { &mut *self.processors.get() };

        let num_processors = processors.len();
        if num_processors == 0 {
            output.copy_from_slice(input);
            return;
        }

        // Defensive check: ensure output slice matches input length
        if output.len() != input.len() {
            // This should never happen, but if it does, avoid panic by returning early.
            // We cannot process mismatched lengths, so we'll just copy input to output if possible.
            if output.len() >= input.len() {
                output[..input.len()].copy_from_slice(input);
            }
            return;
        }

        let len = input.len();

        if len <= 4096 {
            // Stack allocation for small buffers
            let mut buffer_a = [0.0f32; 4096];
            let mut buffer_b = [0.0f32; 4096];
            buffer_a[..len].copy_from_slice(input);

            for (i, processor) in processors.iter_mut().enumerate() {
                let is_last = i == num_processors - 1;

                if i % 2 == 0 {
                    // Index Genap (0: Crystalizer, 2: Surround)
                    if is_last {
                        processor.process(&buffer_a[..len], output);
                    } else {
                        processor.process(&buffer_a[..len], &mut buffer_b[..len]);
                    }
                } else {
                    // Index Ganjil (1: Limiter, 3: dst)
                    if is_last {
                        processor.process(&buffer_b[..len], output);
                    } else {
                        processor.process(&buffer_b[..len], &mut buffer_a[..len]);
                    }
                }
            }
        } else {
            // Defensive check: ensure output slice matches input length (already checked above)

            // Heap allocation for large buffers
            let mut buffer_a = vec![0.0f32; input.len()];
            let mut buffer_b = vec![0.0f32; input.len()];
            buffer_a.copy_from_slice(input);

            for (i, processor) in processors.iter_mut().enumerate() {
                let is_last = i == num_processors - 1;

                if i % 2 == 0 {
                    if is_last {
                        processor.process(&buffer_a, output);
                    } else {
                        processor.process(&buffer_a, &mut buffer_b);
                    }
                } else {
                    if is_last {
                        processor.process(&buffer_b, output);
                    } else {
                        processor.process(&buffer_b, &mut buffer_a);
                    }
                }
            }
        }
    }

    pub fn reset(&self) {
        // SAFETY: exclusive access via Guard
        let processors = unsafe { &mut *self.processors.get() };
        for processor in processors.iter_mut() {
            processor.reset();
        }
    }

    pub fn is_empty(&self) -> bool {
        // SAFETY: exclusive access via Guard
        let processors = unsafe { &*self.processors.get() };
        processors.is_empty()
    }
}

impl fmt::Debug for DspChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.chain.load();
        let processors = unsafe { &*guard.processors.get() };
        write!(f, "DspChain({} processors)", processors.len())
    }
}
