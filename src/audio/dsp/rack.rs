/* --- LOONIX-TUNES src/audio/dsp/rack.rs --- */

#![allow(non_snake_case)]

use std::sync::atomic::Ordering;

use crate::audio::dsp::{
    get_bass_enabled_arc, get_bass_freq_arc, get_bass_gain_arc, get_bass_q_arc,
    get_compressor_enabled_arc, get_compressor_threshold_arc, get_crossfeed_amount_arc,
    get_crossfeed_enabled_arc, get_crystal_amount_arc, get_crystal_enabled_arc,
    get_crystal_freq_arc, get_eq_band_arc, get_eq_bands_arc, get_eq_enabled_arc,
    get_limiter_enabled_arc, get_middle_amount_arc, get_middle_enabled_arc, get_mono_enabled_arc,
    get_mono_width_arc, get_pitch_enabled_arc, get_pitch_ratio_arc, get_preamp_enabled_arc,
    get_preamp_gain_arc, get_stereo_amount_arc, get_stereo_enabled_arc, get_surround_enabled_arc,
    get_surround_width_arc, AudioNormalizer, BassBooster, Compressor, Crossfeed, Crystalizer,
    DspProcessor, DspSettings, EqPreamp, EqProcessor, Limiter, MiddleClarity, PitchShifter, Reverb,
    StereoEnhance, StereoWidth, SurroundProcessor,
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

        type B = Box<dyn DspProcessor + Send + Sync>;

        // HAPUS SEMUA .store() DI SINI.
        // Biarkan processor mengambil nilai dari atomics saat audio berjalan (lewat sync_from_atomics).

        processors.push(Box::new(EqPreamp::new()) as B);
        processors.push(Box::new(AudioNormalizer::new(true, -14.0)) as B);

        // Gunakan new() tanpa melempar array gains kosong.
        processors.push(Box::new(EqProcessor::new()) as B);

        processors.push(Box::new(Compressor::new()) as B);
        processors.push(Box::new(BassBooster::new()) as B);
        processors.push(Box::new(Reverb::new()) as B);
        processors.push(Box::new(StereoEnhance::new()) as B);
        processors.push(Box::new(Crystalizer::new(48000.0)) as B);
        processors.push(Box::new(SurroundProcessor::new()) as B);
        processors.push(Box::new(StereoWidth::new()) as B);
        processors.push(Box::new(PitchShifter::new()) as B);
        processors.push(Box::new(MiddleClarity::new()) as B);
        processors.push(Box::new(Crossfeed::new()) as B);
        processors.push(Box::new(Limiter::new()) as B);

        processors
    }

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
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
