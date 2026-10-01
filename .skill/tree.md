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