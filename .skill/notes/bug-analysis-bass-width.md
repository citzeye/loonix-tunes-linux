Dan ada 2 penyebab teknis utama di sini.

🔎 Penyebab 1 — Width Dikali 2.0 (Over-Expansion)
self.current_width = target_width * 2.0;

Ini agresif banget.

Kalau UI kirim:

0.75

DSP jadi:

1.5

Side signal dikali 1.5 → high frequency (yang dominan di side) ikut naik.

Karena:

High freq biasanya lebih stereo
Bass biasanya mono-ish

Jadi makin lebar → makin bright.

🔎 Penyebab 2 — HPF Cuma 1st Order (6 dB/oct)
let hp_cutoff = 250.0;

High-pass 1st order itu lemah banget.

Artinya:

Upper mids & highs lolos full power
Cuma bass doang yang dikurangi

Jadi efeknya:

yang diboost cuma high region.

Itu sebabnya lo denger pedes di 2k+.

🔎 Penyebab 3 (Yang Paling Bahaya)

Mid tidak dinormalisasi ulang.

Formula lo:

L = mid + widened_side
R = mid - widened_side

Kalau side naik → total RMS naik → perceived brightness naik.

Walau di clamp, tonal balance berubah.

🎯 Fix Profesional (Bukan Tambal Sulam)

Kita perbaiki 3 hal:

✅ 1. Jangan Kali 2.0

Ganti:

self.current_width = target_width * 2.0;

Jadi:

self.current_width = target_width;

Atau kalau mau soft expansion:

self.current_width = 1.0 + (target_width - 0.5) * 1.2;

Lebih smooth.

✅ 2. Compensate Gain Setelah Widening

Tambahkan normalization:

let norm = 1.0 / (1.0 + self.current_width.abs() * 0.5);
output[i] = ((mid + widened_side) * norm).clamp(-1.0, 1.0);
output[i + 1] = ((mid - widened_side) * norm).clamp(-1.0, 1.0);

Ini penting.

Kalau gak → high selalu naik.

✅ 3. Optional: Kurangi High-Pass Cutoff

Kalau mau lebih natural:

let hp_cutoff = 120.0;

Karena 250 Hz terlalu tinggi.
Itu udah masuk low-mid.

🔥 Versi Stabil yang Aman (Recommended)

Ganti block processing jadi:

let mid = (left_in + right_in) * 0.5;
let side = (left_in - right_in) * 0.5;

let side_filtered = if self.current_bass_safe > 0.5 {
    self.high_pass(side)
} else {
    side
};

let widened_side = side_filtered * self.current_width;

// energy compensation
let norm = 1.0 / (1.0 + self.current_width.abs() * 0.5);

output[i] = ((mid + widened_side) * norm).clamp(-1.0, 1.0);
output[i + 1] = ((mid - widened_side) * norm).clamp(-1.0, 1.0);
🧠 Kenapa Ini Fix Pedes?

Karena sekarang:

Width tidak over-scale
Total energy dikontrol
High freq tidak otomatis naik level

Lo tetap dapet widening, tapi gak nambah brightness.

⚠️ Catatan Penting

Stereo widener = selalu meningkatkan side.

Side = biasanya high content.

Jadi tanpa normalization → selalu jadi bright.

Itu bukan bug.
Itu sifat algoritma mid/side.

Kalau lo mau versi lebih profesional lagi, kita bisa bikin:

psychoacoustic widener
frequency-dependent widening
atau dynamic width

Tapi untuk sekarang, implement fix di atas dulu.