/* --- loonixtunesv2/src/audio/audiobus.rs | Audio Bus --- */
// Audio signal path & buffer management

use ringbuf::traits::Split;
use ringbuf::{HeapProd, HeapRb};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// AudioBus handles the routing of audio signals through different stages
/// and manages the buffer flow between decoder, effects, and output.
pub struct AudioBus {
    // Producer side (decoder writes to this)
    producer: Arc<Mutex<HeapProd<f32>>>,

    // State flags
    is_running: Arc<AtomicBool>,
    is_paused: Arc<AtomicBool>,

    // Audio format info
    sample_rate: u32,
    channels: u16,
    buffer_size: usize,
}

impl AudioBus {
    /// Create a new AudioBus with specified buffer size
    pub fn new(buffer_size: usize) -> Self {
        let rb = HeapRb::<f32>::new(buffer_size);
        let (prod, _cons) = rb.split();

        Self {
            producer: Arc::new(Mutex::new(prod)),
            is_running: Arc::new(AtomicBool::new(false)),
            is_paused: Arc::new(AtomicBool::new(false)),
            sample_rate: 48000,
            channels: 2,
            buffer_size,
        }
    }

    /// Create with default buffer size (65536 samples)
    pub fn default() -> Self {
        Self::new(65536)
    }

    /// Get the producer for writing audio data
    pub fn get_producer(&self) -> Arc<Mutex<HeapProd<f32>>> {
        self.producer.clone()
    }

    /// Start the audio bus
    pub fn start(&self) {
        self.is_running.store(true, Ordering::SeqCst);
    }

    /// Stop the audio bus
    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    /// Pause the audio bus
    pub fn pause(&self) {
        self.is_paused.store(true, Ordering::SeqCst);
    }

    /// Resume the audio bus
    pub fn resume(&self) {
        self.is_paused.store(false, Ordering::SeqCst);
    }

    /// Check if the audio bus is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    /// Check if the audio bus is paused
    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::SeqCst)
    }

    /// Set audio format
    pub fn set_format(&mut self, sample_rate: u32, channels: u16) {
        self.sample_rate = sample_rate;
        self.channels = channels;
    }

    /// Get current sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get current channel count
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Get buffer size
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

/// AudioStage represents a stage in the audio processing pipeline
pub trait AudioStage {
    /// Process audio data
    fn process(&mut self, input: &[f32], output: &mut [f32]);

    /// Reset the stage state
    fn reset(&mut self);
}

/// AudioPipeline manages multiple audio stages in sequence
pub struct AudioPipeline {
    stages: Vec<Box<dyn AudioStage + Send + Sync>>,
}

impl AudioPipeline {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn add_stage(&mut self, stage: Box<dyn AudioStage + Send + Sync>) {
        self.stages.push(stage);
    }

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let num_stages = self.stages.len();
        if num_stages == 0 {
            output.copy_from_slice(input);
            return;
        }

        let len = input.len();
        let mut buffer_a = vec![0.0f32; len];
        let mut buffer_b = vec![0.0f32; len];
        buffer_a.copy_from_slice(input);

        for (i, stage) in self.stages.iter_mut().enumerate() {
            let is_last = i == num_stages - 1;
            if i % 2 == 0 {
                // Even index reads from buffer_a
                if is_last {
                    stage.process(&buffer_a, output);
                } else {
                    stage.process(&buffer_a, &mut buffer_b);
                }
            } else {
                // Odd index reads from buffer_b
                if is_last {
                    stage.process(&buffer_b, output);
                } else {
                    stage.process(&buffer_b, &mut buffer_a);
                }
            }
        }
    }

    pub fn reset_all(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.reset();
        }
    }
}

/* --- END --- */
