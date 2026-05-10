/* --- loonixtunesv2/src/audio/dsp/chain.rs | chain --- */

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
    // SAFETY: exclusive access via Guard (single audio thread)
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
        let guard = self.chain.load();
        guard.process(input, output);
    }

    /// Replace entire effect chain atomically
    pub fn swap_chain(&self, new_rack: crate::audio::dsp::DspRack) {
        let new_inner = DspChainInner::from_rack(new_rack);
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

    /// Update sample rate for all processors in the chain
    pub fn set_sample_rate(&self, rate: f32) {
        let guard = self.chain.load();
        guard.set_sample_rate(rate);
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
    fn drop(&mut self) {}
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

    /// Process audio using linear state-carrying approach.
    ///
    /// Why this works:
    /// - `output` starts as a copy of `input` (the "running state")
    /// - Each processor reads from temp_buffer (previous state) and writes to output (new state)
    /// - This eliminates buffer-shadowing that occurred with the ping-pong approach when
    ///   processors were bypassed or when the processor count was odd/even dependent
    pub fn process(&self, input: &[f32], output: &mut [f32]) {
        // SAFETY: exclusive access via Guard (single audio thread)
        let processors = unsafe { &mut *self.processors.get() };

        if processors.is_empty() {
            output.copy_from_slice(input);
            return;
        }

        let len = input.len();
        const MAX_BUFFER: usize = 8192;

        if len > MAX_BUFFER {
            output[..len].copy_from_slice(input);
            return;
        }

        output[..len].copy_from_slice(input);

        let mut temp_buffer = [0.0f32; MAX_BUFFER];

        for processor in processors.iter_mut() {
            temp_buffer[..len].copy_from_slice(&output[..len]);
            processor.process(&temp_buffer[..len], &mut output[..len]);
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

    /// Update sample rate for all processors in the chain
    pub fn set_sample_rate(&self, rate: f32) {
        // SAFETY: exclusive access via Guard
        let processors = unsafe { &mut *self.processors.get() };
        for processor in processors.iter_mut() {
            processor.set_sample_rate(rate);
        }
    }
}

impl fmt::Debug for DspChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.chain.load();
        let processors = unsafe { &*guard.processors.get() };
        write!(f, "DspChain({} processors)", processors.len())
    }
}
