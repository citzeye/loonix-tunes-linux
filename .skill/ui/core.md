lokasi: src/ui/core.rs

ROLE
core.rs adalah Qt bridge layer.
Dia adalah pintu masuk QML ke Rust.

core.rs bukan logic utama.
core.rs bukan DSP.
core.rs bukan engine.
core.rs bukan tempat konversi rumit.

core.rs hanya wrapper + signal dispatcher.

============================================================
TANGGUNG JAWAB

core.rs boleh:

- expose Q_PROPERTY
- expose qt_method
- emit qt_signal
- panggil fungsi di src/core/
- panggil fungsi di src/ui/dsp.rs (wrapper dsp)
- forward nilai dari QML ke layer logic
- forward hasil logic ke QML

core.rs tidak boleh:

- punya logika DSP
- punya atomic
- punya Arc audio state
- clamp parameter DSP
- konversi dB berat
- hitung preset logic
- load/save config langsung
- akses file system langsung
- proses audio buffer

============================================================
STRUKTUR YANG BENAR

core.rs harus terlihat seperti ini:

pub struct MusicModel {
    // Q_PROPERTY fields
}

impl MusicModel {

    // QML wrapper method
    pub fn toggle_compressor(&mut self) {
        self.app.toggle_compressor();
        self.compressor_active_changed();
    }

}

Artinya:
core.rs hanya memanggil app/core logic layer.

Tidak boleh ada:

crate::audio::dsp::compressor::get_compressor_threshold_arc().store(...)

Kalau ada ini di core.rs → arsitektur salah.

============================================================
RULE: SINGLE RESPONSIBILITY

core.rs = UI adapter.

Dia tidak boleh tahu:
- bagaimana compressor bekerja
- bagaimana rack menyimpan atomic
- bagaimana engine memproses audio

Dia hanya tahu:
- ada method toggle_compressor()
- ada property compressor_threshold

============================================================
Q_PROPERTY RULE

Setiap property harus:

1. Punya storage di struct
2. Punya NOTIFY signal
3. Emit signal ketika berubah

Contoh benar:

self.compressor_threshold = val;
self.compressor_threshold_changed();

Contoh salah:

update atomic tapi tidak update property.

============================================================
WRAPPER STYLE RULE

Semua method di core.rs harus:

Tipis.
Maksimal 3–6 baris.

Kalau satu method lebih dari 15 baris,
itu bukan wrapper lagi.

============================================================
RESET RULE

Reset di core.rs:

- Panggil logic reset di layer lain
- Update property
- Emit signal

Tidak boleh:
- tulis ulang nilai DSP langsung
- clamp manual tanpa logic layer

============================================================
ANTI-PATTERN YANG HARUS DIHINDARI

1. core.rs punya preset logic besar
2. core.rs baca file config
3. core.rs simpan Arc<AtomicF32>
4. core.rs konversi UI 0–1 ke dB besar
5. core.rs tahu default snapshot detail

Kalau ada ini → refactor ke src/core/.

============================================================
DEPENDENCY DIRECTION

QML
  ↓
core.rs (ui bridge)
  ↓
src/core/*
  ↓
audio engine
  ↓
dsp

Tidak boleh ada panah balik dari dsp ke core.rs.

============================================================
NAMING RULE

File: core.rs
Semua method lowercase.
Tidak pakai underscore untuk nama file baru.
Tidak duplikasi nama mirip seperti:
corebridge.rs
coreui.rs
modelcore.rs

Semua wrapper tetap di core.rs.

============================================================
KESIMPULAN

core.rs harus:

- Tipis
- Bersih
- Tidak pintar
- Hanya jembatan

Kalau core.rs mulai “pintar”,
itu tanda arsitektur mulai bocor.

END