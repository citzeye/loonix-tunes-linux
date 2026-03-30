/* --- LOONIX-TUNES src/audio/dsp/rack.rs --- */

use crate::audio::dsp::{
    BassBooster, Crossfeed, Crystalizer, DspChain, DspProcessor, DspSettings, Limiter,
    MiddleClarity, PitchShifter, StereoEnhance, StereoWidth, SurroundProcessor,
};

/// DSP Rack Builder - builds DspChain with desired processors
pub struct DspRack {
    pub processors: Vec<Box<dyn DspProcessor + Send + Sync>>,
}

impl DspRack {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn DspProcessor + Send + Sync>) {
        self.processors.push(processor);
    }

    /// Build processors from settings
    /// Order: EQ → Crystalizer → Surround → Middle → Stereo → BassBooster → Compressor → Limiter
    pub fn build_processors(settings: &DspSettings) -> Vec<Box<dyn DspProcessor + Send + Sync>> {
        let mut processors: Vec<Box<dyn DspProcessor + Send + Sync>> = Vec::new();

        // 1. EQ always in chain
        processors.push(Box::new(crate::audio::dsp::EqProcessor::with_bands(
            settings.eq_bands,
        )));

        // 2. Crystalizer (harmonic saturation)
        let crystal_amount = if settings.crystal_enabled {
            settings.crystal_amount
        } else {
            0.0
        };
        processors.push(Box::new(Crystalizer::new(crystal_amount)));

        // 3. Phase manipulators (BEFORE BassBooster)
        if settings.surround_enabled {
            processors.push(Box::new(SurroundProcessor::with_width(
                settings.surround_width,
            )));
        }

        if settings.mono_enabled {
            processors.push(Box::new(StereoWidth::new(settings.mono_width)));
        }

        if settings.pitch_enabled {
            processors.push(Box::new(PitchShifter::new(settings.pitch_semitones)));
        }

        if settings.middle_enabled {
            processors.push(Box::new(MiddleClarity::new(settings.middle_amount)));
        }

        let stereo_amount = if settings.stereo_enabled {
            settings.stereo_amount
        } else {
            0.0
        };
        processors.push(Box::new(StereoEnhance::new(stereo_amount)));

        // 4. BassBooster AFTER all phase manipulators
        if settings.bass_enabled {
            processors.push(Box::new(BassBooster::with_params(
                48000.0,
                settings.bass_cutoff,
                settings.bass_gain,
                0.707,
            )));
        }

        if settings.crossfeed_enabled {
            processors.push(Box::new(Crossfeed::new(settings.crossfeed_amount)));
        }

        // 5. Dynamics (Compressor + Reverb + Limiter)
        processors.push(Box::new(crate::audio::dsp::Compressor::new()));
        processors.push(Box::new(crate::audio::dsp::reverb::Reverb::new()));
        processors.push(Box::new(Limiter::new()));

        processors
    }

    /// Build DspRack from settings
    pub fn build_chain(settings: &DspSettings) -> DspRack {
        let mut rack = DspRack::new();
        let processors = Self::build_processors(settings);
        for p in processors {
            rack.add_processor(p);
        }
        rack
    }

    /// Default chain with all effects enabled
    pub fn default_chain() -> DspChain {
        let chain = DspChain::new();

        // Create a rack with all processors
        let mut rack = DspRack::new();
        rack.add_processor(Box::new(crate::audio::dsp::EqProcessor::new()));
        rack.add_processor(Box::new(Crystalizer::new(0.20)));
        rack.add_processor(Box::new(SurroundProcessor::with_width(1.8)));
        rack.add_processor(Box::new(BassBooster::with_params(
            48000.0, 100.0, 6.0, 0.707,
        )));
        rack.add_processor(Box::new(crate::audio::dsp::Compressor::new()));
        rack.add_processor(Box::new(crate::audio::dsp::reverb::Reverb::new()));
        rack.add_processor(Box::new(Limiter::new()));

        chain.swap_chain(rack);
        chain
    }
}
