System Audit Request: Cross-Platform Compatibility (Linux, Windows, Android)

"Gue mau lo melakukan audit menyeluruh terhadap seluruh codebase Loonix-tunes. Aplikasi ini harus berjalan di Linux (PipeWire), Windows (WASAPI), dan Android (AAudio). Cek setiap file .rs dan .qml untuk mendeteksi potensi 'leak' platform-specific yang bisa bikin build rusak di OS lain.

Audit checklist:

Cargo.toml Dependencies: Cari crate yang hanya jalan di Linux (seperti alsa, pipewire tanpa wrapper, atau library sistem .so). Pastikan mereka dibungkus dalam [target.'cfg(target_os = "linux")'.dependencies].

Conditional Compilation: Cek apakah fungsi low-level (Audio Engine, File System, DAC Management) sudah dibungkus dengan atribut #[cfg(target_os = "linux")], #[cfg(windows)], atau #[cfg(target_os = "android")].

Hardcoded Paths: Cari string path seperti ~/Music, /home/..., atau C:\.... Pastikan semua path management menggunakan std::path::PathBuf atau crate dirs supaya dinamis mengikuti OS.

Audio Backend: Pastikan logic inisialisasi audio tidak memaksa memanggil driver Linux saat berjalan di Windows/Android.

QML Resources: Cek apakah ada font atau path asset yang menggunakan format Linux-only.

Thread & Process: Cek jika ada penggunaan perintah shell Linux (std::process::Command) yang tidak akan jalan di Windows CMD/PowerShell.

Jika lo nemu fungsi atau file yang 'bocor' (Linux-only tapi gak dibungkus cfg), jangan cuma kasih tahu, tapi berikan solusi refactor menggunakan Conditional Compilation atau Abstraction Layer agar build tetap sukses di 3 OS tersebut."

Kenapa Prompt Ini Penting?
Dependency Leak: Seringkali kita nambahin alsa di [dependencies] biasa. Pas di Windows, Cargo bakal nyari file .h ALSA dan langsung error biarpun kodenya nggak dipanggil. Prompt ini maksa AI buat benerin Cargo.toml.

Pathing: Di Linux itu /, di Windows itu \. Kalau lo pake hardcoded string, file picker lo bakal mati total di salah satu OS.

Shell Commands: Kalau lo iseng manggu Command::new("ls"), di Windows bakal error karena perintahnya dir.