/* --- LOONIX-TUNES src/audio/dsp/crossfeed.rs --- */
use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

static CROSSFEED_AMOUNT_ARC: OnceLock<Mutex<Option<Arc<AtomicU32>>>> = OnceLock::new();

pub fn get_crossfeed_arc() -> Option<Arc<AtomicU32>> {
    let guard = CROSSFEED_AMOUNT_ARC.get_or_init(|| Mutex::new(None));
    guard.lock().unwrap().clone()
}

pub fn set_crossfeed_arc(arc: Arc<AtomicU32>) {
    let guard = CROSSFEED_AMOUNT_ARC.get_or_init(|| Mutex::new(None));
    *guard.lock().unwrap() = Some(arc);
}

pub struct Crossfeed {
    amount_bits: Arc<AtomicU32>,
    // Delay buffer kecil (sekitar 0.3ms atau 14-20 samples)
    delay_l: [f32; 32],
    delay_r: [f32; 32],
    write_pos: usize,
    // State untuk Low Pass Filter (biar suara bocoran gak terlalu tajam)
    lp_l: f32,
    lp_r: f32,
}

impl Crossfeed {
    pub fn new(amount: f32) -> Self {
        let arc = Arc::new(AtomicU32::new(amount.to_bits()));
        set_crossfeed_arc(arc.clone());
        Self {
            amount_bits: arc,
            delay_l: [0.0; 32],
            delay_r: [0.0; 32],
            write_pos: 0,
            lp_l: 0.0,
            lp_r: 0.0,
        }
    }
}

impl DspProcessor for Crossfeed {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let amount = f32::from_bits(self.amount_bits.load(Ordering::Relaxed));

        if amount <= 0.001 {
            output.copy_from_slice(input);
            return;
        }

        // Parameter Pro:
        // Delay sekitar 300 mikrodetik (14 samples @ 48kHz)
        let delay_samples = 14;
        // Low pass cut sekitar 700Hz untuk simulasi "head shadow"
        let filter_coeff = 0.15;
        // Reduksi volume sedikit agar total volume tidak naik saat mixing
        let gain_reduction = 1.0 - (amount * 0.15);

        for i in (0..input.len()).step_by(2) {
            let in_l = input[i];
            let in_r = input[i + 1];

            // 1. Simpan input ke delay buffer
            self.delay_l[self.write_pos] = in_l;
            self.delay_r[self.write_pos] = in_r;

            // 2. Ambil sample dari masa lalu (delay)
            let read_pos = (self.write_pos + 32 - delay_samples) % 32;
            let delayed_l = self.delay_l[read_pos];
            let delayed_r = self.delay_r[read_pos];

            // 3. Low Pass Filter pada sinyal delay (membuang treble)
            self.lp_l += filter_coeff * (delayed_l - self.lp_l);
            self.lp_r += filter_coeff * (delayed_r - self.lp_r);

            // 4. Mixing: L_out = L_in + (R_delayed_filtered * amount)
            // Pakai reduksi gain supaya tidak clipping
            output[i] = (in_l + (self.lp_r * amount * 0.8)) * gain_reduction;
            output[i + 1] = (in_r + (self.lp_l * amount * 0.8)) * gain_reduction;

            self.write_pos = (self.write_pos + 1) % 32;
        }
    }

    fn reset(&mut self) {
        self.delay_l.fill(0.0);
        self.delay_r.fill(0.0);
        self.lp_l = 0.0;
        self.lp_r = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
