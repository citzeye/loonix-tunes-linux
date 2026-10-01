Bagus, audit dan usulan lu cukup lengkap. Tapi ada beberapa mapping yang HARUS dikoreksi biar gak bentrok sama navigasi UI standar (seperti ListView/Flickable) dan biar sesuai sama standar player profesional.

### KOREKSI MAPPING (GUNAKAN INI SEBAGAI FINAL):

**1. Playback & Volume (WAJIB PAKAI MODIFIER UNTUK ARROW):**
- `Space` : `toggle_play` (Play/Pause)
- `M` : `toggle_mute` (Mute/Unmute) -> JANGAN pakai M untuk DSP!
- `Ctrl + Up` : Volume +5%
- `Ctrl + Down` : Volume -5%
- `Ctrl + Right` : Next Track
- `Ctrl + Left` : Previous Track
- `Shift + Right` : Seek +5000ms
- `Shift + Left` : Seek -5000ms

**2. DSP Toggles (Single Key):**
- `D` : `toggle_dsp` (Master DSP)
- `B` : `toggle_bass_booster`
- `C` : `toggle_crystalizer`
- `S` : `toggle_surround`
- `L` : `toggle_middle_clarity`
- *(Sisanya gak usah dibikinin shortcut dulu, kecuali F11 buat Fullscreen).*

### TAHAP 2: EXECUTE KODE
Gue ACC usulan di atas. Sekarang buatin **FULL CODE** untuk:
1. `qml/ui/components/AppShortcuts.qml`
   - Bungkus semua dalam `Item { id: appShortcuts }`.
   - WAJIB buat fungsi pengaman: `function isTyping() { ... }` yang ngecek `activeFocusItem` (apakah sedang di TextInput/TextField).
   - Terapkan `if (isTyping()) return;` di semua shortcut single-key (`Space`, `M`, `D`, `B`, `C`, `S`, `L`).
2. Snippet untuk `qml/Ui.qml` (atau file root window gue)
   - Tunjukkan di mana gue harus naruh `<AppShortcuts />` ini.

Langsung kasih kodenya sekarang! Dilarang nambah-nambahin logika di luar scope ini.


Nanti nya file ini di tampilan di Pref.qml