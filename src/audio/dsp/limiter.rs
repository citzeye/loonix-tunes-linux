/* --- LOONIX-TUNES src/audio/dsp/limiter.rs --- */

use crate::audio::dsp::DspProcessor;

pub struct Limiter {
    threshold_lin: f32,
    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,
}

impl Limiter {
    pub fn new() -> Self {
        let sample_rate = 48000.0;
        let attack_ms = 2.0; // Jangan 0 ms! Kasih waktu 2ms biar bass gak terpotong kasar
        let release_ms = 50.0; // Release lebih lama biar gak 'pumping' (naik-turun)

        Self {
            threshold_lin: 10.0_f32.powf(-0.5 / 20.0), // -0.5 dB
            envelope: 0.0,
            attack_coeff: (-1.0_f32 / (attack_ms * 0.001 * sample_rate)).exp(),
            release_coeff: (-1.0_f32 / (release_ms * 0.001 * sample_rate)).exp(),
        }
    }
}

impl DspProcessor for Limiter {
    #[inline(always)]
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let safe_len = input.len() - (input.len() % 2);

        for i in (0..safe_len).step_by(2) {
            let l = input[i];
            let r = input[i + 1];

            // 1. STEREO LINKING: Cari peak paling kenceng antara Kiri & Kanan
            let peak = l.abs().max(r.abs());

            // 2. ENVELOPE FOLLOWER (Mencegah Bass Pecah)
            if peak > self.envelope {
                // Attack: Envelope naik mengejar peak
                self.envelope = peak + self.attack_coeff * (self.envelope - peak);
            } else {
                // Release: Envelope turun pelan-pelan
                self.envelope = peak + self.release_coeff * (self.envelope - peak);
            }

            // 3. CALCULATE GAIN REDUCTION
            let mut gain = 1.0;
            if self.envelope > self.threshold_lin {
                gain = self.threshold_lin / self.envelope;
            }

            // 4. APPLY GAIN & SAFETY SOFT-CLIP
            // Clamp -0.99 to 0.99 untuk jaminan gak digital clipping
            output[i] = (l * gain).clamp(-0.99, 0.99);
            output[i + 1] = (r * gain).clamp(-0.99, 0.99);
        }
    }

    fn reset(&mut self) {
        self.envelope = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
