---
name: project-structure
description: Arsitektur folder dan peta proyek Loonix Tunes.
---
# Project Architecture & Folder Structure – Loonix Tunes
**Version:** 2.0  
**Design Pattern:** Domain-Driven Design (DDD) & Blind Engine Isolation  

Dokumen ini adalah panduan mutlak untuk tata letak file (Project Structure). Setiap modul memiliki batas domain yang ketat. **Dilarang keras menyilangkan dependensi yang melanggar aturan Leveling (terutama pada folder DSP).**

---

## 📂 Struktur Direktori Utama

```text
src/
├── audio/                 # 🎧 THE SOUND FACTORY: Murni Pemrosesan & Buffer Suara
│   │                      # Constraint: Dilarang import UI, Config, atau Preset ke folder ini.
│   │
│   ├── dsp/               # Level 1-3: The Blind Machine (Hanya menerima angka final f32/bool)
│   │   ├── bassbooster.rs
│   │   ├── biquad.rs
│   │   ├── chain.rs       # Level 3: Entry point dsp, urus routing limiter & preamp
│   │   ├── compressor.rs
│   │   ├── crossfeed.rs
│   │   ├── crystalizer.rs
│   │   ├── eq.rs
│   │   ├── eqpreamp.rs
│   │   ├── limiter.rs
│   │   ├── middleclarity.rs
│   │   ├── mod.rs
│   │   ├── normalizer.rs
│   │   ├── pitchshifter.rs
│   │   ├── preamp.rs
│   │   ├── rack.rs        # Level 2: Penampung semua unit efek & routing
│   │   ├── reverb.rs
│   │   ├── rubberbandffi.rs
│   │   ├── stereoenhance.rs
│   │   ├── stereowidth.rs
│   │   └── surround.rs
│   │
│   ├── engine/            # Level 4: The Driver (Waktu & Playback Logic)
│   │   ├── abrepeat.rs    # Logika A-B repeat (pembanding timestamp)
│   │   ├── clock.rs
│   │   ├── engine.rs      # Main loop audio
│   │   ├── library.rs     # Engine state library
│   │   ├── mod.rs
│   │   ├── scheduler.rs
│   │   └── seek.rs
│   │
│   └── io/                # Hardware, Stream & Konversi Sinyal
│       ├── audiobus.rs
│       ├── audiooutput.rs
│       ├── buffer/
│       │   ├── mod.rs
│       │   └── ringbuffer.rs
│       ├── decoder.rs
│       ├── mod.rs
│       └── resample.rs
│
├── core/                  # 🧠 THE BRAIN: Logika Bisnis, State, & Interaksi OS
│   │                      # Constraint: Tempat SSoT (Single Source of Truth) berada.
│   │
│   ├── config/            # Manajemen State & Penyimpanan
│   │   ├── appconfig.rs   # Global App settings
│   │   ├── dspconfig.rs   # Parser & handler untuk dsp.json (Volatile & File Sync)
│   │   └── presets.rs     # SSoT Hardcoded Konstanta untuk Built-in Presets (0-5)
│   │
│   ├── library/           # Manajemen Database Lagu & File
│   │   ├── favorites.rs
│   │   ├── library.rs     # Logika CRUD library
│   │   ├── metadata.rs    # Ekstraksi ID3 / Vorbis Comments
│   │   └── scanner.rs     # Crawler folder musik
│   │
│   └── services/          # Layanan Background & Integrasi OS
│       ├── fileservice.rs
│       ├── playback.rs
│       ├── sysmedia.rs    # MPRIS / Media Keys hardware bindings
│       └── wireless.rs    # Deteksi ganti output audio (Bluetooth/Jack)
│
├── ui/                    # 🖥️ THE FACE: Presentasi & Jembatan QML/Qt
│   │                      # Constraint: UI tidak memproses data, hanya memanggil core/audio.
│   │
│   ├── components/        # Elemen Visual Murni
│   │   ├── popup.rs
│   │   └── theme.rs
│   │
│   ├── bridge/            # QObject Bindings (Rust <-> C++/QML)
│   │   ├── core.rs        # Main Context Bridge
│   │   ├── dspcontroller.rs # Manajer Sinyal DSP & Protokol Snapshot
│   │   ├── playerbridge.rs
│   │   └── queue.rs
│   │
│   ├── reportbug.rs
│   └── updater.rs
│
└── main.rs                # 🚀 Entry Point: Inisialisasi Logger, Config, dan QGuiApplication

---

## tree.md

```
.
├── .backup
├── .cargo
│   └── config.toml
├── .directory
├── .gitattributes
├── .github
│   └── workflows
│       └── release.yml
├── .gitignore
├── .opencode
│   ├── .gitignore
│   ├── package-lock.json
│   ├── package.json
│   └── plans
├── assets
│   ├── default-config.toml
│   ├── eqpreset.json
│   ├── fonts
│   │   ├── KodeMono-VariableFont_wght.ttf
│   │   ├── Oswald-Regular.ttf
│   │   ├── SymbolsNerdFont-Regular.ttf
│   │   └── twemoji.ttf
│   ├── fxpreset.json
│   ├── LoonixTunes.png
│   └── qtquickcontrols2.conf
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── create_pkg.sh
├── LICENSE
├── loonix-tunes.sh
├── Output
├── packaging
│   ├── linux
│   │   ├── icon.png
│   │   └── loonix-tunes.desktop
│   └── windows
│       ├── avcodec-62.dll
│       ├── avdevice-62.dll
│       ├── avfilter-11.dll
│       ├── avformat-62.dll
│       ├── avutil-60.dll
│       ├── deploy.bat
│       ├── icon.ico
│       ├── include
│       │   ├── libavcodec
│       │   │   ├── ac3_parser.h
│       │   │   ├── adts_parser.h
│       │   │   ├── avcodec.h
│       │   │   ├── avdct.h
│       │   │   ├── bsf.h
│       │   │   ├── codec.h
│       │   │   ├── codec_desc.h
│       │   │   ├── codec_id.h
│       │   │   ├── codec_par.h
│       │   │   ├── d3d11va.h
│       │   │   ├── defs.h
│       │   │   ├── dirac.h
│       │   │   ├── dv_profile.h
│       │   │   ├── dxva2.h
│       │   │   ├── exif.h
│       │   │   ├── jni.h
│       │   │   ├── mediacodec.h
│       │   │   ├── packet.h
│       │   │   ├── qsv.h
│       │   │   ├── smpte_436m.h
│       │   │   ├── vdpau.h
│       │   │   ├── version.h
│       │   │   ├── version_major.h
│       │   │   ├── videotoolbox.h
│       │   │   └── vorbis_parser.h
│       │   ├── libavdevice
│       │   │   ├── avdevice.h
│       │   │   ├── version.h
│       │   │   └── version_major.h
│       │   ├── libavfilter
│       │   │   ├── avfilter.h
│       │   │   ├── buffersink.h
│       │   │   ├── buffersrc.h
│       │   │   ├── version.h
│       │   │   └── version_major.h
│       │   ├── libavformat
│       │   │   ├── avformat.h
│       │   │   ├── avio.h
│       │   │   ├── version.h
│       │   │   └── version_major.h
│       │   ├── libavutil
│       │   │   ├── adler32.h
│       │   │   ├── aes.h
│       │   │   ├── aes_ctr.h
│       │   │   ├── ambient_viewing_environment.h
│       │   │   ├── attributes.h
│       │   │   ├── audio_fifo.h
│       │   │   ├── avassert.h
│       │   │   ├── avconfig.h
│       │   │   ├── avstring.h
│       │   │   ├── avutil.h
│       │   │   ├── base64.h
│       │   │   ├── blowfish.h
│       │   │   ├── bprint.h
│       │   │   ├── bswap.h
│       │   │   ├── buffer.h
│       │   │   ├── camellia.h
│       │   │   ├── cast5.h
│       │   │   ├── channel_layout.h
│       │   │   ├── common.h
│       │   │   ├── container_fifo.h
│       │   │   ├── cpu.h
│       │   │   ├── crc.h
│       │   │   ├── csp.h
│       │   │   ├── des.h
│       │   │   ├── detection_bbox.h
│       │   │   ├── dict.h
│       │   │   ├── display.h
│       │   │   ├── dovi_meta.h
│       │   │   ├── downmix_info.h
│       │   │   ├── encryption_info.h
│       │   │   ├── error.h
│       │   │   ├── eval.h
│       │   │   ├── executor.h
│       │   │   ├── ffversion.h
│       │   │   ├── fifo.h
│       │   │   ├── file.h
│       │   │   ├── film_grain_params.h
│       │   │   ├── frame.h
│       │   │   ├── hash.h
│       │   │   ├── hdr_dynamic_metadata.h
│       │   │   ├── hdr_dynamic_vivid_metadata.h
│       │   │   ├── hmac.h
│       │   │   ├── hwcontext.h
│       │   │   ├── hwcontext_amf.h
│       │   │   ├── hwcontext_cuda.h
│       │   │   ├── hwcontext_d3d11va.h
│       │   │   ├── hwcontext_d3d12va.h
│       │   │   ├── hwcontext_drm.h
│       │   │   ├── hwcontext_dxva2.h
│       │   │   ├── hwcontext_mediacodec.h
│       │   │   ├── hwcontext_oh.h
│       │   │   ├── hwcontext_opencl.h
│       │   │   ├── hwcontext_qsv.h
│       │   │   ├── hwcontext_vaapi.h
│       │   │   ├── hwcontext_vdpau.h
│       │   │   ├── hwcontext_videotoolbox.h
│       │   │   ├── hwcontext_vulkan.h
│       │   │   ├── iamf.h
│       │   │   ├── imgutils.h
│       │   │   ├── intfloat.h
│       │   │   ├── intreadwrite.h
│       │   │   ├── lfg.h
│       │   │   ├── log.h
│       │   │   ├── lzo.h
│       │   │   ├── macros.h
│       │   │   ├── mastering_display_metadata.h
│       │   │   ├── mathematics.h
│       │   │   ├── md5.h
│       │   │   ├── mem.h
│       │   │   ├── motion_vector.h
│       │   │   ├── murmur3.h
│       │   │   ├── opt.h
│       │   │   ├── parseutils.h
│       │   │   ├── pixdesc.h
│       │   │   ├── pixelutils.h
│       │   │   ├── pixfmt.h
│       │   │   ├── random_seed.h
│       │   │   ├── rational.h
│       │   │   ├── rc4.h
│       │   │   ├── refstruct.h
│       │   │   ├── replaygain.h
│       │   │   ├── ripemd.h
│       │   │   ├── samplefmt.h
│       │   │   ├── sha.h
│       │   │   ├── sha512.h
│       │   │   ├── spherical.h
│       │   │   ├── stereo3d.h
│       │   │   ├── tdrdi.h
│       │   │   ├── tea.h
│       │   │   ├── threadmessage.h
│       │   │   ├── time.h
│       │   │   ├── timecode.h
│       │   │   ├── timestamp.h
│       │   │   ├── tree.h
│       │   │   ├── twofish.h
│       │   │   ├── tx.h
│       │   │   ├── uuid.h
│       │   │   ├── version.h
│       │   │   ├── video_enc_params.h
│       │   │   ├── video_hint.h
│       │   │   └── xtea.h
│       │   ├── libswresample
│       │   │   ├── swresample.h
│       │   │   ├── version.h
│       │   │   └── version_major.h
│       │   └── libswscale
│       │       ├── swscale.h
│       │       ├── version.h
│       │       └── version_major.h
│       ├── libfftw3-3.dll
│       ├── libfftw3f-3.dll
│       ├── libfftw3l-3.dll
│       ├── libs
│       │   ├── avcodec.lib
│       │   ├── avdevice.lib
│       │   ├── avfilter.lib
│       │   ├── avformat.lib
│       │   ├── avutil.lib
│       │   ├── fftw3.lib
│       │   ├── libfftw3-3.lib
│       │   ├── libfftw3f-3.lib
│       │   ├── libfftw3l-3.lib
│       │   ├── rubberband.lib
│       │   ├── samplerate.lib
│       │   ├── sndfile.lib
│       │   ├── swresample.lib
│       │   └── swscale.lib
│       ├── loonix-tunes-setup.iss
│       ├── release_windows
│       │   ├── avcodec-62.dll
│       │   ├── avdevice-62.dll
│       │   ├── avfilter-11.dll
│       │   ├── avformat-62.dll
│       │   ├── avutil-60.dll
│       │   ├── D3Dcompiler_47.dll
│       │   ├── fftw3.dll
│       │   ├── fftw3f.dll
│       │   ├── fftw3l.dll
│       │   ├── FLAC++.dll
│       │   ├── FLAC.dll
│       │   ├── generic
│       │   │   └── qtuiotouchplugin.dll
│       │   ├── iconengines
│       │   │   └── qsvgicon.dll
│       │   ├── imageformats
│       │   │   ├── qgif.dll
│       │   │   ├── qico.dll
│       │   │   ├── qjpeg.dll
│       │   │   └── qsvg.dll
│       │   ├── libfftw3-3.dll
│       │   ├── libfftw3f-3.dll
│       │   ├── libfftw3l-3.dll
│       │   ├── libmp3lame.dll
│       │   ├── loonix-tunes.exe
│       │   ├── mpg123.dll
│       │   ├── msvcp140.dll
│       │   ├── msvcp140_1.dll
│       │   ├── msvcp140_2.dll
│       │   ├── msvcp140_atomic_wait.dll
│       │   ├── msvcp140_codecvt_ids.dll
│       │   ├── networkinformation
│       │   │   └── qnetworklistmanager.dll
│       │   ├── ogg.dll
│       │   ├── opengl32sw.dll
│       │   ├── opus.dll
│       │   ├── out123.dll
│       │   ├── pkgconf-7.dll
│       │   ├── platforms
│       │   │   └── qwindows.dll
│       │   ├── qml
│       │   │   ├── QML
│       │   │   ├── Qt
│       │   │   ├── QtQml
│       │   │   └── QtQuick
│       │   ├── qmltooling
│       │   │   ├── qmldbg_debugger.dll
│       │   │   ├── qmldbg_inspector.dll
│       │   │   ├── qmldbg_local.dll
│       │   │   ├── qmldbg_messages.dll
│       │   │   ├── qmldbg_native.dll
│       │   │   ├── qmldbg_nativedebugger.dll
│       │   │   ├── qmldbg_preview.dll
│       │   │   ├── qmldbg_profiler.dll
│       │   │   ├── qmldbg_quickprofiler.dll
│       │   │   ├── qmldbg_server.dll
│       │   │   └── qmldbg_tcp.dll
│       │   ├── Qt6Core.dll
│       │   ├── Qt6Gui.dll
│       │   ├── Qt6LabsPlatform.dll
│       │   ├── Qt6Network.dll
│       │   ├── Qt6OpenGL.dll
│       │   ├── Qt6Qml.dll
│       │   ├── Qt6QmlMeta.dll
│       │   ├── Qt6QmlModels.dll
│       │   ├── Qt6QmlWorkerScript.dll
│       │   ├── Qt6Quick.dll
│       │   ├── Qt6QuickControls2.dll
│       │   ├── Qt6QuickControls2Basic.dll
│       │   ├── Qt6QuickControls2BasicStyleImpl.dll
│       │   ├── Qt6QuickControls2FluentWinUI3StyleImpl.dll
│       │   ├── Qt6QuickControls2Fusion.dll
│       │   ├── Qt6QuickControls2FusionStyleImpl.dll
│       │   ├── Qt6QuickControls2Imagine.dll
│       │   ├── Qt6QuickControls2ImagineStyleImpl.dll
│       │   ├── Qt6QuickControls2Impl.dll
│       │   ├── Qt6QuickControls2Material.dll
│       │   ├── Qt6QuickControls2MaterialStyleImpl.dll
│       │   ├── Qt6QuickControls2Universal.dll
│       │   ├── Qt6QuickControls2UniversalStyleImpl.dll
│       │   ├── Qt6QuickControls2WindowsStyleImpl.dll
│       │   ├── Qt6QuickEffects.dll
│       │   ├── Qt6QuickLayouts.dll
│       │   ├── Qt6QuickShapes.dll
│       │   ├── Qt6QuickTemplates2.dll
│       │   ├── Qt6Svg.dll
│       │   ├── Qt6Widgets.dll
│       │   ├── rubberband-3.dll
│       │   ├── samplerate.dll
│       │   ├── sleef.dll
│       │   ├── sleefdft.dll
│       │   ├── sleefquad.dll
│       │   ├── sndfile.dll
│       │   ├── soxr.dll
│       │   ├── styles
│       │   │   └── qmodernwindowsstyle.dll
│       │   ├── swresample-6.dll
│       │   ├── swscale-9.dll
│       │   ├── syn123.dll
│       │   ├── tls
│       │   │   ├── qcertonlybackend.dll
│       │   │   └── qschannelbackend.dll
│       │   ├── translations
│       │   │   ├── qt_ar.qm
│       │   │   ├── qt_bg.qm
│       │   │   ├── qt_ca.qm
│       │   │   ├── qt_cs.qm
│       │   │   ├── qt_da.qm
│       │   │   ├── qt_de.qm
│       │   │   ├── qt_en.qm
│       │   │   ├── qt_es.qm
│       │   │   ├── qt_fa.qm
│       │   │   ├── qt_fi.qm
│       │   │   ├── qt_fr.qm
│       │   │   ├── qt_gd.qm
│       │   │   ├── qt_he.qm
│       │   │   ├── qt_hr.qm
│       │   │   ├── qt_hu.qm
│       │   │   ├── qt_it.qm
│       │   │   ├── qt_ja.qm
│       │   │   ├── qt_ka.qm
│       │   │   ├── qt_ko.qm
│       │   │   ├── qt_lg.qm
│       │   │   ├── qt_lv.qm
│       │   │   ├── qt_nl.qm
│       │   │   ├── qt_nn.qm
│       │   │   ├── qt_pl.qm
│       │   │   ├── qt_pt_BR.qm
│       │   │   ├── qt_ru.qm
│       │   │   ├── qt_sk.qm
│       │   │   ├── qt_tr.qm
│       │   │   ├── qt_uk.qm
│       │   │   ├── qt_zh_CN.qm
│       │   │   └── qt_zh_TW.qm
│       │   ├── vcomp140.dll
│       │   ├── vcruntime140.dll
│       │   ├── vcruntime140_1.dll
│       │   ├── vcruntime140_threads.dll
│       │   ├── vorbis.dll
│       │   ├── vorbisenc.dll
│       │   ├── vorbisfile.dll
│       │   ├── yasm.dll
│       │   └── yasmstd.dll
│       ├── resource.rc
│       ├── samplerate.dll
│       ├── sndfile.dll
│       ├── swresample-6.dll
│       └── swscale-9.dll
├── PKGBUILD
├── qml
│   ├── qml.qrc
│   ├── ui
│   │   ├── contextmenu
│   │   │   ├── AppearanceContextMenu.qml
│   │   │   ├── PlaylistContextMenu.qml
│   │   │   └── TabContextMenu.qml
│   │   ├── Eq.qml
│   │   ├── Fx.qml
│   │   ├── Playlist.qml
│   │   ├── pref
│   │   │   ├── PrefAbout.qml
│   │   │   ├── PrefAppearance.qml
│   │   │   ├── PrefAudio.qml
│   │   │   ├── PrefButton.qml
│   │   │   ├── PrefCollapsibleSection.qml
│   │   │   ├── PrefDonate.qml
│   │   │   ├── PrefDropdown.qml
│   │   │   ├── PrefHardware.qml
│   │   │   ├── PrefLibrary.qml
│   │   │   ├── PrefSlider.qml
│   │   │   ├── PrefSwitch.qml
│   │   │   └── PrefTab.qml
│   │   ├── Pref.qml
│   │   ├── qmldir
│   │   ├── RenameDialog.qml
│   │   ├── Tab.qml
│   │   ├── TabCustom.qml
│   │   ├── TabFavorites.qml
│   │   ├── TabMusic.qml
│   │   ├── TabQueue.qml
│   │   ├── ThemeSlider.qml
│   │   └── TrackInfo.qml
│   └── Ui.qml
├── README.md
├── release_windows
│   ├── D3Dcompiler_47.dll
│   ├── generic
│   │   └── qtuiotouchplugin.dll
│   ├── iconengines
│   │   └── qsvgicon.dll
│   ├── imageformats
│   │   ├── qgif.dll
│   │   ├── qico.dll
│   │   ├── qjpeg.dll
│   │   └── qsvg.dll
│   ├── loonix-tunes.exe
│   ├── networkinformation
│   │   └── qnetworklistmanager.dll
│   ├── opengl32sw.dll
│   ├── platforms
│   │   └── qwindows.dll
│   ├── qml
│   │   ├── QML
│   │   │   ├── plugins.qmltypes
│   │   │   └── qmldir
│   │   ├── Qt
│   │   │   └── labs
│   │   │       └── platform
│   │   ├── QtQml
│   │   │   ├── Models
│   │   │   │   ├── modelsplugin.dll
│   │   │   │   ├── plugins.qmltypes
│   │   │   │   └── qmldir
│   │   │   ├── plugins.qmltypes
│   │   │   ├── qmldir
│   │   │   ├── qmlplugin.dll
│   │   │   ├── WorkerScript
│   │   │   │   ├── plugins.qmltypes
│   │   │   │   ├── qmldir
│   │   │   │   └── workerscriptplugin.dll
│   │   │   └── XmlListModel
│   │   └── QtQuick
│   │       ├── Controls
│   │       │   ├── Basic
│   │       │   ├── FluentWinUI3
│   │       │   ├── Fusion
│   │       │   ├── Imagine
│   │       │   ├── impl
│   │       │   ├── Material
│   │       │   ├── plugins.qmltypes
│   │       │   ├── qmldir
│   │       │   ├── qtquickcontrols2plugin.dll
│   │       │   ├── Universal
│   │       │   └── Windows
│   │       ├── Dialogs
│   │       │   └── quickimpl
│   │       ├── Effects
│   │       │   ├── effectsplugin.dll
│   │       │   ├── plugins.qmltypes
│   │       │   └── qmldir
│   │       ├── Layouts
│   │       │   ├── plugins.qmltypes
│   │       │   ├── qmldir
│   │       │   └── qquicklayoutsplugin.dll
│   │       ├── LocalStorage
│   │       ├── NativeStyle
│   │       │   ├── controls
│   │       │   ├── plugins.qmltypes
│   │       │   ├── qmldir
│   │       │   ├── qtquickcontrols2nativestyleplugin.dll
│   │       │   └── util
│   │       ├── Particles
│   │       ├── plugins.qmltypes
│   │       ├── qmldir
│   │       ├── qtquick2plugin.dll
│   │       ├── Shapes
│   │       │   ├── plugins.qmltypes
│   │       │   ├── qmldir
│   │       │   └── qmlshapesplugin.dll
│   │       ├── Templates
│   │       │   ├── plugins.qmltypes
│   │       │   ├── qmldir
│   │       │   └── qtquicktemplates2plugin.dll
│   │       ├── tooling
│   │       ├── VectorImage
│   │       └── Window
│   │           ├── qmldir
│   │           ├── quickwindow.qmltypes
│   │           └── quickwindowplugin.dll
│   ├── qmltooling
│   │   ├── qmldbg_debugger.dll
│   │   ├── qmldbg_inspector.dll
│   │   ├── qmldbg_local.dll
│   │   ├── qmldbg_messages.dll
│   │   ├── qmldbg_native.dll
│   │   ├── qmldbg_nativedebugger.dll
│   │   ├── qmldbg_preview.dll
│   │   ├── qmldbg_profiler.dll
│   │   ├── qmldbg_quickprofiler.dll
│   │   ├── qmldbg_server.dll
│   │   └── qmldbg_tcp.dll
│   ├── Qt6Core.dll
│   ├── Qt6Gui.dll
│   ├── Qt6LabsPlatform.dll
│   ├── Qt6Network.dll
│   ├── Qt6OpenGL.dll
│   ├── Qt6Qml.dll
│   ├── Qt6QmlMeta.dll
│   ├── Qt6QmlModels.dll
│   ├── Qt6QmlWorkerScript.dll
│   ├── Qt6Quick.dll
│   ├── Qt6QuickControls2.dll
│   ├── Qt6QuickControls2Basic.dll
│   ├── Qt6QuickControls2BasicStyleImpl.dll
│   ├── Qt6QuickControls2FluentWinUI3StyleImpl.dll
│   ├── Qt6QuickControls2Fusion.dll
│   ├── Qt6QuickControls2FusionStyleImpl.dll
│   ├── Qt6QuickControls2Imagine.dll
│   ├── Qt6QuickControls2ImagineStyleImpl.dll
│   ├── Qt6QuickControls2Impl.dll
│   ├── Qt6QuickControls2Material.dll
│   ├── Qt6QuickControls2MaterialStyleImpl.dll
│   ├── Qt6QuickControls2Universal.dll
│   ├── Qt6QuickControls2UniversalStyleImpl.dll
│   ├── Qt6QuickControls2WindowsStyleImpl.dll
│   ├── Qt6QuickEffects.dll
│   ├── Qt6QuickLayouts.dll
│   ├── Qt6QuickShapes.dll
│   ├── Qt6QuickTemplates2.dll
│   ├── Qt6Svg.dll
│   ├── Qt6Widgets.dll
│   ├── styles
│   │   └── qmodernwindowsstyle.dll
│   ├── tls
│   │   ├── qcertonlybackend.dll
│   │   └── qschannelbackend.dll
│   └── translations
│       ├── qt_ar.qm
│       ├── qt_bg.qm
│       ├── qt_ca.qm
│       ├── qt_cs.qm
│       ├── qt_da.qm
│       ├── qt_de.qm
│       ├── qt_en.qm
│       ├── qt_es.qm
│       ├── qt_fa.qm
│       ├── qt_fi.qm
│       ├── qt_fr.qm
│       ├── qt_gd.qm
│       ├── qt_he.qm
│       ├── qt_hr.qm
│       ├── qt_hu.qm
│       ├── qt_it.qm
│       ├── qt_ja.qm
│       ├── qt_ka.qm
│       ├── qt_ko.qm
│       ├── qt_lg.qm
│       ├── qt_lv.qm
│       ├── qt_nl.qm
│       ├── qt_nn.qm
│       ├── qt_pl.qm
│       ├── qt_pt_BR.qm
│       ├── qt_ru.qm
│       ├── qt_sk.qm
│       ├── qt_tr.qm
│       ├── qt_uk.qm
│       ├── qt_zh_CN.qm
│       └── qt_zh_TW.qm
├── src
│   ├── audio
│   │   ├── audio_bus.rs
│   │   ├── audio_output.rs
│   │   ├── buffer
│   │   │   ├── mod.rs
│   │   │   ├── ring_buffer.rs
│   │   │   └── shared_ring_buffer.rs
│   │   ├── config.rs
│   │   ├── dac.rs
│   │   ├── decoder.rs
│   │   ├── dsp
│   │   │   ├── abrepeat.rs
│   │   │   ├── bassbooster.rs
│   │   │   ├── biquad.rs
│   │   │   ├── chain.rs
│   │   │   ├── compressor.rs
│   │   │   ├── crossfeed.rs
│   │   │   ├── crystalizer.rs
│   │   │   ├── eq.rs
│   │   │   ├── limiter.rs
│   │   │   ├── middleclarity.rs
│   │   │   ├── mod.rs
│   │   │   ├── normalizer.rs
│   │   │   ├── pitchshifter.rs
│   │   │   ├── rack.rs
│   │   │   ├── reverb.rs
│   │   │   ├── rubberband_ffi.rs
│   │   │   ├── stereoenhance.rs
│   │   │   ├── stereowidth.rs
│   │   │   └── surround.rs
│   │   ├── engine
│   │   │   ├── clock.rs
│   │   │   ├── engine.rs
│   │   │   ├── library.rs
│   │   │   ├── mod.rs
│   │   │   ├── scheduler.rs
│   │   │   ├── seek.rs
│   │   │   └── state.rs
│   │   ├── highres.rs
│   │   ├── metadata.rs
│   │   ├── mod.rs
│   │   ├── playlist.rs
│   │   ├── popup.rs
│   │   ├── resample.rs
│   │   └── scanner.rs
│   ├── dbus_service.rs
│   ├── main.rs
│   └── ui
│       ├── core.rs
│       ├── mod.rs
│       ├── playerbridge.rs
│       ├── playlistcontextmenu.rs
│       ├── theme.rs
│       └── updater.rs
└── SS
    ├── 1.png
    ├── 2.png
    ├── 3.png
    └── 4.png
```

---

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
