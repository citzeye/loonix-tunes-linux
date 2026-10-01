---
name: loonix-rule
description: Aturan wajib Loonix Tunes. Baca sebelum menyunting kode apa pun.
---
# LoonixRule - Aturan Wajib Loonix Tunes (Updated 2026)

Semua aturan ini **WAJIB** diikuti oleh AI saat bekerja dengan proyek Loonix Tunes. Aturan ini telah disesuaikan dengan struktur folder `src/audio/`.

---

## 1. Core Architecture (Nested Audio Structure)

Proyek ini menggunakan struktur nested di dalam `src/audio/`.

| File / Folder           | Peran Utama                                                                           |
| :---------------------- | :------------------------------------------------------------------------------------ |
| `src/audio/engine/`     | **Player Engine**. Jantung aplikasi, koordinasi playback, clock, sinkronisasi thread. |
| `src/ffmpeg_decoder.rs` | **FFmpeg Engine**. Hanya bertugas merubah file menjadi PCM data.                      |
| `src/audio_output.rs`   | **Hardware Interface**. Mengirim audio ke PipeWire/CPAL.                              |
| `src/dsp/`              | **DSP Rack**. Tempat pemrosesan EQ, Reverb, Compressor, Limiter.                      |
| `src/audio_bus.rs`      | **Routing**. Jalur data PCM antar komponen.                                           |

---

## 2. Audio Clock System (Master Source)

Tujuan: Mencegah _drift_ pada _progress bar_ dan memastikan seek yang instan.

- **Rule Utama**: Audio device adalah master clock. Bukan UI, bukan timer QML.
- **Position Tracking**: `samples_played` di `engine/` harus disimpan. Setiap buffer dikirim ke output, counter bertambah.
- **Rumus Posisi**: $position\_ms = (samples\_played * 1000) / sample\_rate$.
- **UI Update**: Backend mengirim `position_ms` ke QML setiap **30ms – 60ms**. UI hanya bertugas menampilkan nilai tersebut.

---

## 3. Instant Seek Procedure

Gunakan urutan ini di `engine/` dan `ffmpeg_decoder.rs` untuk performa instan tanpa delay:

1. **Pause & Mute**: Hentikan aliran audio agar tidak ada suara _glitch_.
2. **Flush Pipeline**:
   - Bersihkan _ring buffer_ di `engine/`.
   - Panggil `.reset()` pada modul di `dsp/` (untuk buang sisa reverb/delay).
3. **FFmpeg Jump**:
   - Gunakan `av_seek_frame()` dengan flag `AVSEEK_FLAG_BACKWARD` di `ffmpeg_decoder.rs`.
   - **Wajib**: Panggil `avcodec_flush_buffers()` untuk membuang sisa frame lama.
4. **Reset Clock**: Set ulang `samples_played` berdasarkan posisi baru.
5. **Refill**: Isi buffer minimal 100ms sebelum _unmute_.

---

## 4. DSP & Effect Rack Rules

Semua pemrosesan efek dilakukan di `src/dsp/` setelah audio didecode.

- **Pipeline Order**: EQ -> Compressor -> Spatial Effects -> Limiter.
- **No Sample Alteration**: DSP dilarang mengubah jumlah _sample count_ agar tidak merusak kalkulasi waktu (clock).
- **Realtime Safe**: Dilarang melakukan alokasi memori (`Box`, `Vec` baru) atau menggunakan `Mutex` yang berat di dalam loop audio.

---

## 5. UI Sync & Communication

UI (`qml/Ui.qml`) dilarang keras melakukan kalkulasi waktu sendiri.

- **Slider Logic**:
  - `value` pada slider harus di-bind ke properti `position_ms` dari Rust.
  - Saat user melakukan _drag_, kirim sinyal `seekTo(ms)` ke backend.
- **Isolasi**: UI hanya mengirim perintah (Play, Pause, Seek, SetFX), semua logika audio tetap di Rust.

---

## 6. Development & Safety

1. **Backup**: Selalu backup file ke folder `.backup/` sebelum modifikasi besar.
2. **No Internet**: Dilarang fetch data dari URL eksternal; semua library harus _offline capable_.
3. **Thread Safety**: Minimal gunakan 3 thread (Decode, Output, Control/UI Sync).
