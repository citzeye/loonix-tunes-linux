🎯 FULL PRODUCTION REFACTOR
Centralized Config System → config.rs
🎯 OBJECTIVE

Refactor total sistem konfigurasi agar:

src/audio/config.rs menjadi single source of truth
Default config → hardcoded di Default::default()
User config → ~/.config/loonix-tunes/config.json
Startup logic:
Jika user config ada → load & override default
Jika tidak ada → pakai default
Remove obsolete Pro flags:
highres_enabled
dac_exclusive_mode
Zero warning
Clippy clean
Tidak ada unwrap di runtime path
🔥 HARD RULES (WAJIB)
❌ Tidak boleh unwrap() di runtime
❌ Tidak boleh panic kecuali fatal startup
❌ Tidak boleh global allow unused/dead_code
❌ Tidak boleh baca file assets lama
❌ Tidak boleh sentuh Git
❌ Tidak boleh delete file permanent
✅ Semua error harus propagate atau fallback
✅ Config loading hanya terjadi saat startup atau reload
✅ Audio thread tidak boleh menyentuh file I/O
🧠 ARCHITECTURE DESIGN
1️⃣ Config Location

User config path:

Linux:

~/.config/loonix-tunes/config.json

Implement via:

dirs::config_dir()

Folder harus auto-create jika belum ada.

2️⃣ Config Loading Strategy
Flow:
AppConfig::load()
    ↓
if user_config_exists:
    read file
    deserialize
    merge with default
else:
    return Default::default()

Tidak boleh panic jika file corrupt.
Fallback ke default + log warning.

🧱 STRUCT DESIGN
AppConfig (FINAL)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub volume: f32,
    pub balance: f32,
    pub theme: String,
    pub eq_enabled: bool,
    pub eq_bands: [f32; 10],
    pub fx_enabled: bool,
    pub reverb_amount: f32,
    pub compressor_threshold: f32,
}

❌ REMOVE:

highres_enabled
dac_exclusive_mode
🏗 Default Implementation

Default harus pure hardcoded:

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            volume: 1.0,
            balance: 0.0,
            theme: "dark".to_string(),
            eq_enabled: false,
            eq_bands: [0.0; 10],
            fx_enabled: false,
            reverb_amount: 0.0,
            compressor_threshold: -12.0,
        }
    }
}

❌ Tidak boleh load file
❌ Tidak boleh panggil fungsi eksternal

💾 Load Implementation
impl AppConfig {
    pub fn load() -> Self {
        let default = Self::default();

        match Self::load_user_config() {
            Ok(cfg) => cfg,
            Err(_) => default,
        }
    }
}
Safe User Load
fn load_user_config() -> Result<Self, ConfigError>

Rules:

Jika file tidak ada → return Err(NotFound)
Jika JSON invalid → return Err(ParseError)
Tidak unwrap
Tidak panic
💾 Save Implementation
pub fn save(&self) -> Result<(), ConfigError>
Serialize JSON pretty
Atomic write (write temp → rename)
Create directory if missing
Tidak panic
🧹 REMOVE LEGACY

Hapus total:

load_fx_config()
load_eq_presets()

Jika masih dipakai di tempat lain → refactor caller.

Jangan delete file permanent.
Jika ada file obsolete .rs → pindah ke .backup/

🧠 DSPCONFIG INTEGRATION DECISION
Jangan hapus DspConfigManager dan DspStateView

Mereka bukan duplicate config.
Mereka adalah:

runtime state sync layer
bukan persistent storage

Tapi:

dspconfig.rs tidak boleh lagi baca file.

Refactor agar:

DspConfigManager::new(app_config: &AppConfig)

Jadi pure runtime layer.

🔄 Reload Logic

Saat user tekan reload config:

engine.pause()
load config baru
apply ke dsp manager
engine.resume()

Tidak boleh recreate ringbuffer.
Tidak boleh restart audio device.
Tidak boleh spawn thread baru.

Patuh lifecycle rule.

🧪 TESTING CHECKLIST

Sebelum dianggap selesai:

App start tanpa config file
App start dengan config valid
App start dengan config corrupt
Delete config saat app running → reload
Rapid reload spam
Close app while saving

Tidak boleh panic.
Tidak boleh leak thread.

📁 FILES TO MODIFY
src/audio/config.rs
src/core/dspconfig.rs (refactor dependency)
src/main.rs (call load())
🎯 ACCEPTANCE CRITERIA
cargo check → 0 warning
cargo clippy → clean
Tidak ada unwrap runtime
Tidak ada file asset dependency
Config pusat hanya di config.rs
User override bekerja
Audio thread untouched
Engine authority tetap utuh