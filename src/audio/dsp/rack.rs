/* --- loonixtunesv2/src/audio/dsp/rack.rs | DSP Rack --- */

use crate::audio::dsp::{
    AudioNormalizer, BassBooster, Compressor, Crossfeed, Crystalizer, DspProcessor, DspSettings,
    EqPreamp, EqProcessor, Limiter, MiddleClarity, PitchShifter, Reverb, StereoEnhance,
    StereoWidth, SurroundProcessor,
};

pub struct DspRack {
    pub processors: Vec<Box<dyn DspProcessor + Send + Sync>>,
}

impl DspRack {
    const MAX_BUFFER: usize = 8192;

    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn DspProcessor + Send + Sync>) {
        self.processors.push(processor);
    }

    pub fn build_rack(_is_pro: bool) -> Self {
        let settings = DspSettings::default();
        let processors = Self::build_processors(&settings);
        Self { processors }
    }

    pub fn build_processors(_settings: &DspSettings) -> Vec<Box<dyn DspProcessor + Send + Sync>> {
        let mut processors: Vec<Box<dyn DspProcessor + Send + Sync>> = Vec::new();

        // LANGSUNG PUSH SAJA. Rust akan melakukan "Unsizing Coercion" secara otomatis
        // karena tipe Vec sudah didefinisikan di atas.

        processors.push(Box::new(EqPreamp::new()));
        processors.push(Box::new(AudioNormalizer::new(true, -14.0)));

        processors.push(Box::new(EqProcessor::new()));
        processors.push(Box::new(Compressor::new()));
        processors.push(Box::new(BassBooster::new()));
        processors.push(Box::new(Reverb::new()));
        processors.push(Box::new(StereoEnhance::new()));
        processors.push(Box::new(Crystalizer::new(48000.0)));
        processors.push(Box::new(SurroundProcessor::new()));
        processors.push(Box::new(StereoWidth::new()));
        processors.push(Box::new(PitchShifter::new()));
        processors.push(Box::new(MiddleClarity::new()));
        processors.push(Box::new(Crossfeed::new()));
        processors.push(Box::new(Limiter::new()));

        processors
    }

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        // DSP BYPASS: If bypass is enabled (DSP OFF), skip ALL cosmetic processing
        // The Cosmetics (Rack) are bypassed, but Core Chain (Preamp/Normalizer/Limiter) still run
        if crate::audio::dsp::is_dsp_bypass() {
            output[..input.len()].copy_from_slice(input);
            return;
        }

        if self.processors.is_empty() {
            output.copy_from_slice(input);
            return;
        }

        let len = input.len();

        if len > Self::MAX_BUFFER {
            output[..len].copy_from_slice(input);
            return;
        }

        output[..len].copy_from_slice(input);

        let mut temp_buffer = [0.0f32; Self::MAX_BUFFER];

        for processor in self.processors.iter_mut() {
            temp_buffer[..len].copy_from_slice(&output[..len]);
            processor.process(&temp_buffer[..len], &mut output[..len]);
        }
    }
}
