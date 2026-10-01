# Normalizer Architecture (Loonix-Tunes)

## Overview

Normalizer di Loonix-Tunes menggunakan sistem **Fixed Gain per Track** dengan **Smoothing Transitions**. Gain dihitung sekali saat track di-load, lalu di-apply dengan transisi mulus saat berganti track.

---

## Arsitektur DSP Chain

```
Input → Preamp → Normalizer → EQ → ... → Limiter → Output
```

### Urutan Processing:
1. **Preamp** - Gain adjustment manual (headroom control)
2. **Normalizer** - Gain dari scanner (per-track, smooth transition)
3. **EQ** - 10-band biquad
4. **Spatial FX** - Crystalizer, Surround, StereoWidth, dll
5. **Limiter** - Final stage protection

---

## Alur Kerja

### 1. Saat Track Change (engine.rs)

```
Track change
    ↓
set_normalizer_gain(1.0)      ← langsung set gain=1.0 agar bisa langsung play
    ↓
start_audiooutput()
    ↓
spawn scanner thread           ← hitung gain di background (non-blocking)
    ↓
scanner selesai → store gain ← hasil di-store ke gain_arc atomic
```

### 2. Saat DSP Processing (setiap sample)

```
Audio callback (audiooutput.rs)
    ↓
dsp_chain.process(input, output)
    ↓
Normalizer.process():
    - baca gain_arc (dari scanner)
    - baca smoothing_arc (dari UI)
    - apply: current_gain += (target - current_gain) * smoothing
    - apply: soft_clip(input[i] * current_gain)
```

---

## Scanner (scanner.rs)

### Fungsi Utama

```rust
pub fn calculate_track_gain(path: &str, params: &ScanParams) -> f32
```

### Parameter:

| Parameter | Default | Range | Keterangan |
|-----------|---------|-------|------------|
| target_lufs | -14.0 | -24.0 s/d -10.0 | Standar streaming modern |
| true_peak_dbtp | -1.5 | -3.0 s/d 0.0 | Batas plafon biar nggak clipping |
| max_gain_db | 12.0 | 0 s/d +12.0 | Max gain boost yang diizinkan |

### Constraints:

1. **Target LUFS** - Rata-rata loudness track disesuaikan ke target
2. **Max Gain** - Gain nggak boleh melebihi ini
3. **True Peak Ceiling** - Kalau projected peak > ceiling, gain direduce

### Caching:

Hasil scan di-cache di `GAIN_CACHE` (static OnceLock<Mutex<HashMap>>). Track yang sama nggak perlu di-scan ulang.

---

## Normalizer (normalizer.rs)
Refactor file normalizer.rs agar menjadi track gain applicator yang stabil dan transparan, bukan dynamic limiter.

Tujuan:
Gain hanya dipakai di awal lagu untuk menyamakan volume antar track.
Bukan untuk compress / react ke transient.

🎯 Objective

Ubah AudioNormalizer menjadi:

Fixed gain applicator
Dengan short fade-in smoothing
Tanpa transient detection
Tanpa dynamic gain chasing
Tanpa per-sample reactivity

Ini bukan compressor.

1️⃣ Hapus Instant Attack Logic

Hapus seluruh bagian ini dari process():

let input_abs = input[i].abs();
if input_abs > self.current_gain * 2.0 {
    self.current_gain = (target).max(input_abs * 1.05);
} else {
    self.current_gain += (target - self.current_gain) * smoothing;
}

Tidak boleh ada deteksi transient.

2️⃣ Gain Behavior Baru

Implementasi baru:

fixed_gain adalah hasil dari scanner
current_gain mulai dari 1.0 saat lagu baru
Smooth menuju fixed_gain selama beberapa ratus milidetik
Setelah itu stabil dan tidak berubah

Gunakan ini:

self.current_gain += (self.fixed_gain - self.current_gain) * smoothing;

Itu saja.

Tidak ada kondisi lain.

3️⃣ Soft Clip Tetap Dipakai

Pertahankan soft_clip().

Tapi tambahkan komentar:

// Safety clipper to prevent overshoot after normalization.
// Should rarely engage if scanner peak constraint works correctly.
4️⃣ Default Smoothing Lebih Stabil

Ubah default smoothing:

0.002 → 0.0015

Transition sedikit lebih halus.

5️⃣ Tambahkan Reset Logic yang Benar

Pastikan reset() mengembalikan:

self.current_gain = 1.0;

Bukan ke fixed_gain.

Karena fade harus terjadi setiap lagu baru.

6️⃣ Jangan Ubah
Jangan ubah atomic sharing
Jangan ubah Arc structure
Jangan tambahkan dependency
Jangan ubah public API
7️⃣ Tambahkan Komentar di Atas Struct

Track-level gain applicator.
Applies precomputed RMS gain from scanner.
Not a compressor. Not a limiter.

## Catatan Performance

- Scanner jalan di dedicated thread, nggak block audio thread
- Normalizer menggunakan lock-free atomics
- Gain calculation cuma sekali per track (di-cache)
- Soft clip pakai `tanh()` yang optimized di Rust stdlib

STATUS VERIFIKASI NORMALIZER: MATCH (Sesuai Arsitektur)

1. Workflow Perpindahan Track (engine.rs)
   - Status: SESUAI
   - Logic: engine.set_normalizer_gain(1.0) dipanggil langsung saat load agar lagu bunyi duluan, baru kemudian scanner thread dispawn secara non-blocking. Hasil scan di-store ke atomic gain_arc.

2. DSP Processing & Smoothing (normalizer.rs)
   - Status: SESUAI
   - Rumus: current_gain += (target - current_gain) * smoothing.
   - Konstanta: Slow (0.001), Balanced (0.002), Fast (0.005) cocok dengan rencana.
   - Verifikasi Transisi: Untuk preset Balanced (0.002), waktu tempuh mencapai 95% target adalah ~31ms pada 48kHz. Akurat.

3. Perlindungan Output (Limiter/Clip)
   - Status: SESUAI
   - Logic: Menggunakan soft_clip dengan (0.99 * sample.tanh()).clamp(-0.99, 0.99). Ini "safety net" yang pas buat nahan lonjakan gain hasil normalisasi.

4. Scanner & Caching (scanner.rs)
   - Status: SESUAI
   - Parameter: Target -14.0 LUFS, True Peak -1.5 dBTP, dan Max Gain 12.0 dB sudah ter-hardcode sebagai default params.
   - Caching: Menggunakan GAIN_CACHE (OnceLock<Mutex<HashMap>>) sehingga track yang sama tidak perlu di-scan ulang.

5. Urutan DSP Chain
   - Status: SESUAI
   - Posisi: Input -> Preamp -> Normalizer -> EQ -> ... -> Limiter. Normalizer diletakkan sebelum EQ agar tone shaping tidak mengacaukan perhitungan loudness awal.