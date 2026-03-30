/* --- LOONIX-TUNES src/audio/highres.rs --- */

pub struct HighResProcessor {
    pub enabled: bool,
    // Threshold di mana soft-clipping mulai kerja (misal di -1.0 dB atau 0.89)
    threshold: f64,
    ceiling: f64,
}

impl HighResProcessor {
    pub fn new() -> Self {
        Self {
            enabled: false,
            threshold: 0.90, // Mulai melengkung di 90% volume
            ceiling: 1.0,    // Batas maksimal gak boleh lewat 0 dB
        }
    }

    pub fn process(&self, input: &[f32], output: &mut [f32]) {
        if !self.enabled {
            output.copy_from_slice(input);
            return;
        }

        // Pake iterator + SIMD optimization dari Rust
        input
            .iter()
            .zip(output.iter_mut())
            .for_each(|(in_sample, out_sample)| {
                let s = *in_sample as f64;

                // Panggil fungsi soft_clip
                let processed = self.soft_clip(s);

                *out_sample = processed as f32;
            });
    }

    #[inline(always)]
    fn soft_clip(&self, x: f64) -> f64 {
        let abs_x = x.abs();

        if abs_x <= self.threshold {
            x
        } else {
            let sign = if x > 0.0 { 1.0 } else { -1.0 };
            sign * (self.threshold
                + (self.ceiling - self.threshold)
                    * ((abs_x - self.threshold) / (self.ceiling - self.threshold)).tanh())
        }
    }

    pub fn toggle(&mut self, state: bool) {
        self.enabled = state;
    }
}
