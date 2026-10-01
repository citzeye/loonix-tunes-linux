# LOONIX-TUNES ULTIMATE SYSTEM INSTRUCTIONS
# Production Engineering Rules v2
# This must be production-grade reactive architecture.

## 1 Global Philosophy
Audio device adalah master clock
Engine adalah single authority
Decoder adalah worker
DSP adalah post-engine processor
Tidak ada blocking di audio callback
Tidak ada alokasi heap di audio thread
Semua thread harus punya shutdown path

---

## 2 Naming Convention (Revisi Rasional)
✅ File Naming
Type	Rule
.rs	lowercase only (audiooutput.rs)
.qml	PascalCase (MainWindow.qml)
.json	lowercase
.toml	lowercase

Tidak ada forced style di dalam kode Rust.
Ikuti konvensi Rust default.

✅ Rust Code Style (Ikuti Idiom Bahasa)
struct → PascalCase
enum → PascalCase
fn → snake_case
variable → snake_case
constant → SCREAMING_SNAKE_CASE
module → snake_case

Tidak ada #![allow(non_snake_case)] global.

---

## 3 Compiler Discipline
DILARANG:
#![allow(unused_imports)]
#![allow(dead_code)]

Global suppression dilarang.

Rule:

Warning harus = 0
Clippy clean
Allow hanya di scope kecil dan ada alasan

Compiler adalah safety net, bukan musuh.

---

## 4 Error Handling Policy
Runtime / Audio Thread:
Tidak boleh unwrap()
Tidak boleh expect()
Semua error → propagate atau fallback aman
Startup / Fatal Init:
expect() diperbolehkan jika failure = program tidak bisa lanjut

---

## 5 Thread Ownership Contract (WAJIB)
Semua thread harus punya:

should_stop: AtomicBool
Join handle disimpan
Shutdown sequence eksplisit
Shutdown Sequence Wajib:
engine.stop()
Set should_stop = true
Stop audio device
Join decoder thread
Join audio thread
Drop DSP
Exit clean

Tidak boleh ada detached thread.

---

## 6 Audio Engine Authority Model
Engine adalah satu-satunya yang boleh:
Mengubah is_playing
Mengubah seek_mode
Set samples_played
Reset DSP
Clear seek flag

Decoder:

Tidak boleh ubah state engine
Hanya kirim event

AudioOutput:

Tidak punya otoritas state
Hanya eksekusi output

---

## 7 Seek Contract (Final)
Urutan Wajib:
Engine.seek()
    is_playing = false
    set_seek_mode(true)
    audio.clear_buffer()
    control.request_seek()

Decoder:

flush codec
av_seek_frame
flush again
prebuffer until min
emit BufferReady(exact_sample)

Engine.on_buffer_ready():

samples_played = exact_sample
reset_dsp()
set_seek_mode(false)
clear_seek()
is_playing = true

Tidak boleh ada duplicate gate di audio loop.

Audio loop hanya check:

if seek_mode → output silence

---

## 8 Audio Thread Rules

Dilarang di audio callback:

Mutex lock
Arc clone berat
Allocation
Logging
Sleep
Blocking I/O

Audio callback harus deterministik.

---

## 9 DSP Contract (Revisi Lebih Fleksibel)
Linear DSP (EQ, Comp, Reverb)
Tidak boleh ubah sample count
Tidak boleh ubah timing
Harus realtime safe
Time-Domain DSP (Pitch, Stretch, Rubberband)

Pengecualian:

Boleh ubah sample count
Tapi harus expose latency
Harus sinkron dengan engine clock
Harus clear buffer saat seek

Kategori ini harus eksplisit, tidak implicit.

---

## 10 Clock Contract
Audio device = ground truth
Engine position = device sample counter
UI hanya baca engine
Seek target hanya menjadi authoritative setelah BufferReady

Tidak ada dual clock.

---

## 11 Ringbuffer Policy
Ringbuffer dibuat sekali
Tidak pernah recreate saat runtime
clear_buffer() hanya drain consumer
Flush tidak boleh recreate allocation

---

## 12 Fade / Crossfade Policy

Jika ada crossfade:

State harus dibaca di audio loop
Harus decrement frame counter per callback
Tidak boleh hanya set flag tanpa diproses

Crossfade tanpa processing = bug terselubung.

---

## 13 Lifecycle Model (Baru — Penting)

Track Change:

Stop playing
Flush decoder
Reset DSP
Clear buffer
Reset clock
Load decoder baru
Prebuffer
Play

Seek Spam:

Decoder harus break jika request berubah
Hanya request terakhir yang valid

---

## 14 UI Contract (QML)
Layout pakai Layout.*
Jangan mix anchors + Layout
Semua spacing konsisten
Semua color lewat theme

UI tidak boleh:

Mengakses decoder
Mengakses audio device
Mengubah engine internals

UI hanya kirim intent.

Di UI bridge, sedikit duplication boleh selama:
-deterministic
-explicit sync

---

## 15 Testing Policy

Sebelum release:

Seek spam test
Rapid play/pause test
Rapid track change test
Close app while playing test
Ctrl+C test
Memory leak test (valgrind)

Audio engine gagal biasanya di lifecycle, bukan di playback biasa.

---

## 16 Strict Separation

Pisahkan secara mental:

Audio Core
DSP Layer
VST Host
UI

Jangan campur rule UI di audio spec.

## 17 Strict Git & Environment Isolation (ANTI-DATA LOSS)

AI DILARANG KERAS menyentuh infrastruktur Git. Fokus kerja AI hanya pada konten file di dalam disk lokal.

### ✅ MANDATORY:
1. AI hanya boleh melakukan pembacaan (read) dan penulisan (write/edit) pada file fisik di direktori lokal.
2. Jika terjadi error atau inkonsistensi, AI wajib bertanya kepada user, BUKAN mencoba memperbaiki lewat command Git.
3. AI harus berasumsi bahwa kondisi file di lokal saat ini adalah "Kebenaran Tunggal" (Single Source of Truth), meskipun berbeda dengan history Git atau remote repository.

### ❌ STRICTLY PROHIBITED:
1. DILARANG menjalankan command Git apapun, termasuk namun tidak terbatas pada: `git checkout`, `git reset`, `git pull`, `git rebase`, `git stash`, atau `git clean`.
2. DILARANG mencoba menyinkronkan (sync) file dengan remote repository secara otomatis.
3. DILARANG menghapus atau menimpa file berdasarkan data dari branch lain tanpa konfirmasi eksplisit dari user di setiap barisnya.
4. DILARANG melakukan destruksi file (deletion) secara massal dengan alasan "merapikan" struktur repo.

PENYALAHGUNAAN COMMAND GIT ADALAH PELANGGARAN VITAL. AI tidak memiliki izin untuk memanipulasi riwayat versi (version control history). Kerja AI berhenti di level file editing.

## 18 File Backup Policy (ANTI-DATA LOSS)

### MANDATORY RULE:
DILARANG menghapus file `.qml` atau `.rs` secara permanent.

### PROSEDUR:
1. Jika ada file yang ingin dihapus dari aplikasi:
   - Pindahkan ke folder `.backup/` di root project
   - Jangan hapus dari filesystem

2. Jika ada kesamaan nama di folder `.backup/`:
   - Rename file dengan menambahkan timestamp di akhir nama
   - Format: `<filename>_<YYYYMMDD>_<HHMMSS>.bak`
   - Contoh: `EqPopup.qml` → `EqPopup_20260415_180930.bak`

3. Koneksi backend (import, mod.rs, qml.qrc) harus dihapus dari aplikasi, tapi file aslinya di-backup.

### CONTOH:
```
# Sebelum: File tidak dipakai di aplikasi
# Backend: mod.rs, main.rs, qml.qrc sudah dihapus koneksinya

# Action: Move ke .backup/
mv src/audio/playlist.rs .backup/playlist_20260415_180930.bak
mv qml/ui/pref/PrefAudio.qml .backup/PrefAudio_20260415_180930.bak
```

### OWNERSHIP:
Owner project yang decide kapan file di-hapus permanent.
AI hanya backup, tidak pernah delete permanent.


### FOLDER TREE
 Loonix citz loonix-tunes-linux  
.
├── .directory
├── .gitattributes
├── .github
│   └── workflows
│       └── release.yml
├── .gitignore
├── .opencode
│   ├── .gitignore
│   ├── opencode.json
│   ├── package-lock.json
│   ├── package.json
│   ├── plans
│   └── plugins
│       └── graphify.js
├── AGENTS.md
├── assets
│   ├── fonts
│   │   ├── KodeMono-VariableFont_wght.ttf
│   │   ├── Oswald-Regular.ttf
│   │   ├── SymbolsNerdFont-Regular.ttf
│   │   └── twemoji.ttf
│   ├── images
│   │   ├── kofiqrcode.png
│   │   └── saweriaqrcode.png
│   ├── LoonixTunes.png
│   └── qtquickcontrols2.conf
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── create_pkg.sh
├── LICENSE
├── loonix-tunes.sh
├── packaging
│   └── linux
│       ├── icon.png
│       └── loonix-tunes.desktop
├── PKGBUILD
├── qml
│   ├── qml.qrc
│   ├── ui
│   │   ├── components
│   │   │   ├── RenameDialog.qml
│   │   │   ├── ThemeSlider.qml
│   │   │   └── TrackInfo.qml
│   │   ├── contextmenu
│   │   │   ├── AppearanceContextMenu.qml
│   │   │   ├── PlaylistContextMenu.qml
│   │   │   └── TabContextMenu.qml
│   │   ├── Dsp.qml
│   │   ├── Playlist.qml
│   │   ├── pref
│   │   │   ├── PrefAbout.qml
│   │   │   ├── PrefAppearance.qml
│   │   │   ├── PrefButton.qml
│   │   │   ├── PrefCollapsibleSection.qml
│   │   │   ├── PrefDonate.qml
│   │   │   ├── PrefDropdown.qml
│   │   │   ├── PrefLibrary.qml
│   │   │   ├── PrefReportBug.qml
│   │   │   ├── PrefSlider.qml
│   │   │   ├── PrefSwitch.qml
│   │   │   ├── PrefTab.qml
│   │   │   └── PrefThemeEditor.qml
│   │   ├── Pref.qml
│   │   ├── qmldir
│   │   └── tabs
│   │       ├── Tab.qml
│   │       ├── TabCustom.qml
│   │       ├── TabFavorites.qml
│   │       ├── TabMusic.qml
│   │       └── TabQueue.qml
│   └── Ui.qml
├── README.md
├── src
│   ├── audio
│   │   ├── config.rs
│   │   ├── dsp
│   │   │   ├── bassbooster.rs
│   │   │   ├── biquad.rs
│   │   │   ├── chain.rs
│   │   │   ├── compressor.rs
│   │   │   ├── crossfeed.rs
│   │   │   ├── crystalizer.rs
│   │   │   ├── eq.rs
│   │   │   ├── eqpreamp.rs
│   │   │   ├── limiter.rs
│   │   │   ├── middleclarity.rs
│   │   │   ├── mod.rs
│   │   │   ├── normalizer.rs
│   │   │   ├── pitchshifter.rs
│   │   │   ├── preamp.rs
│   │   │   ├── rack.rs
│   │   │   ├── reverb.rs
│   │   │   ├── rubberbandffi.rs
│   │   │   ├── stereoenhance.rs
│   │   │   ├── stereowidth.rs
│   │   │   └── surround.rs
│   │   ├── engine
│   │   │   ├── abrepeat.rs
│   │   │   ├── clock.rs
│   │   │   ├── engine.rs
│   │   │   ├── library.rs
│   │   │   ├── mod.rs
│   │   │   ├── scheduler.rs
│   │   │   └── seek.rs
│   │   ├── io
│   │   │   ├── audiobus.rs
│   │   │   ├── audiooutput.rs
│   │   │   ├── buffer
│   │   │   │   ├── mod.rs
│   │   │   │   └── ringbuffer.rs
│   │   │   ├── decoder.rs
│   │   │   ├── mod.rs
│   │   │   └── resample.rs
│   │   ├── metadata.rs
│   │   ├── mod.rs
│   │   ├── presets.rs
│   │   ├── scanner.rs
│   │   ├── sysmedia.rs
│   │   └── wireless.rs
│   ├── core
│   │   ├── config
│   │   │   ├── appconfig.rs
│   │   │   ├── dspconfig.rs
│   │   │   ├── mod.rs
│   │   │   └── presets.rs
│   │   ├── dspconfig.rs
│   │   ├── library
│   │   │   ├── favorites.rs
│   │   │   ├── fileservice.rs
│   │   │   ├── library.rs
│   │   │   ├── metadata.rs
│   │   │   ├── mod.rs
│   │   │   ├── playback.rs
│   │   │   └── scanner.rs
│   │   ├── mod.rs
│   │   └── services
│   │       ├── fileservice.rs
│   │       ├── mod.rs
│   │       ├── playback.rs
│   │       ├── sysmedia.rs
│   │       └── wireless.rs
│   ├── main.rs
│   └── ui
│       ├── bridge
│       │   ├── core.rs
│       │   ├── dspcontroller.rs
│       │   ├── mod.rs
│       │   ├── playerbridge.rs
│       │   └── queue.rs
│       ├── components
│       │   ├── mod.rs
│       │   ├── popup.rs
│       │   └── theme.rs
│       ├── dspcontroller.rs.bak
│       ├── mod.rs
│       ├── reportbug.rs
│       └── updater.rs
└── SS
    ├── 1.png
    ├── 2.png
    ├── 3.png
    └── 4.png

// openrouter/free	200K	                      ✅	Auto-routing
// qwen/qwen3-coder:free	256K	                ✅	Coding
// qwen/qwen3-next-80b-a3b:free	262K	          ✅	Reasoning
// kimi/k2.5:free	262K	                         ✅	General
// deepseek/deepseek-r1:free	64K	             ✅	Reasoning
// meta-llama/llama-3.3-70b-instruct:free	128K	 ✅	General