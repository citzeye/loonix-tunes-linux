<div align="center">
  <img src="packaging/linux/icon.png" width="400" alt="Loonix-Tunes Logo">
  <h1>Loonix-Tunes</h1>
  <p><b>A high-performance, native Linux audio player crafted for uncompromising sound quality.</b></p>
  <p>Built with Rust & Qt/QML</p>
</div>

---

## 📸 Screenshot Showroom
| | |
|:---:|:---:|
| ![Main UI](SS/1.png) | ![Playlist View](SS/2.png) |
| ![DSP Controls](SS/3.png) | ![Compact Mode](SS/4.png) |


## 🎵 About The Project
Loonix-Tunes is an elegant, fast, and feature-rich audio player designed specifically for Linux users who care about audio processing and system efficiency. By combining a blazingly fast Rust audio engine with a beautiful, hardware-accelerated Qt interface, Loonix-Tunes delivers a premium listening experience without bloating your system.

## ✨ Key Features
* **Native & Efficient:** Compiled to machine code. Snappy UI, minimal RAM footprint, and instant startup times.
* **Advanced DSP Suite:** Built-in studio-grade effects including Crystalizer, Bass Booster, Compressor, Reverb, and a robust Equalizer.
* **Pro-Audio Ready:** Native support for VST3 plugins to take your audio processing to the next level.
* **High-Res Audio Engine:** Custom pipeline handling pristine audio decoding, resampling, and gapless playback.
* **Time & Pitch Shifting:** Powered by the industry-standard Rubberband library for high-quality audio stretching.
* **Desktop Integration:** Full MPRIS/DBus support for seamless media key and desktop widget controls across Linux desktop environments.

## 🚀 Installation
Head over to the [Releases](../../releases) page to download the latest version for Linux.

We provide two simple ways to run Loonix-Tunes on any Linux distribution:
1. **AppImage:** Download the `.AppImage` file, make it executable (`chmod +x`), and run it directly.
2. **Portable Tarball:** Download the `.tar.gz` file, extract it, and run the included `./loonix-tunes.sh` script. No system-wide installation required.


## 🛠️ Building from Source

If you prefer to compile Loonix-Tunes yourself, ensure you have the Rust toolchain, Qt6, and FFmpeg development headers installed on your Linux machine.

```bash
git clone [https://github.com/citzeye/loonix-tunes-linux.git](https://github.com/citzeye/loonix-tunes-linux.git)
cd loonix-tunes-linux
cargo build --release
```

🤝 Issues, and feature requests are welcome! Feel free to check the issues page.

Created with ❤️ by citzeye


