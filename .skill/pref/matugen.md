# Panduan Integrasi Matugen (Wallpaper Sync) ke Loonix

Berikut adalah arsitektur dan langkah-langkah untuk mengintegrasikan Matugen ke dalam Loonix-Tunes menggunakan prinsip *Single Source of Truth* (SSOT). Alur utamanya: **Ambil Warna ➔ Lempar ke UI ➔ Gembok di Config.**

---

## 1. Siapkan "Wadah" di `AppConfig` (`src/audio/config.rs`)

Kita butuh *flag* untuk menentukan apakah user sedang menggunakan mode tema manual atau mode *Wallpaper Sync*, beserta wadah untuk menyimpan warna terakhir dari wallpaper.

Tambahkan *field* berikut di dalam `struct AppConfig`:

```rust
// Tambahin di struct AppConfig
pub use_wallpaper_theme: bool,
pub matugen_colors: HashMap<String, String>, // Buat nyimpen warna terakhir dari wallpaper
```

Jangan lupa *update* implementasi `Default`-nya (di sekitar baris `impl Default for AppConfig`):

```rust
// Di dalam Self { ... }
use_wallpaper_theme: false,
matugen_colors: HashMap::new(),
```

---

## 2. Mesin Penarik Warna di `ThemeManager` (`src/ui/theme.rs`)

Kita akan menambahkan dua fungsi yang bisa dipanggil dari QML (`sync_with_wallpaper` dan `set_loonix_manual`), plus satu fungsi internal untuk membaca file JSON *output* dari Matugen (`~/.cache/matugen/colors.json`).

Tambahkan fungsi-fungsi ini di dalam `impl ThemeManager`:

```rust
use std::process::Command;
use serde_json::Value;

// --- 1. FUNGSI INTERNAL BUAT BACA MATUGEN ---
fn fetch_matugen_colors(&self) -> Option<HashMap<String, String>> {
    // Path default Matugen (Sesuaikan kalau path di sistem lo beda)
    let cache_path = dirs::home_dir()?.join(".cache/matugen/colors.json");
    
    let json_str = std::fs::read_to_string(cache_path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&json_str).ok()?;
    
    // Ambil skema warna (misal kita ambil yang dark mode)
    let colors = v["colors"]["dark"].as_object()?; 
    
    let mut map = HashMap::new();
    
    // MAPPING: Cocokin format nama Matugen ke key Loonix
    if let Some(surface) = colors["surface"].as_str() { c!(map, { "bgmain", surface }); }
    if let Some(surface_variant) = colors["surface_variant"].as_str() { c!(map, { "bgoverlay", surface_variant, "headerbg", surface_variant }); }
    if let Some(primary) = colors["primary"].as_str() { 
        c!(map, { 
            "playeraccent", primary, 
            "playlistactive", primary, 
            "eqactive", primary, 
            "fxactive", primary, 
            "tabhover", primary 
        }); 
    }
    if let Some(on_surface) = colors["on_surface"].as_str() { 
        c!(map, { 
            "playertitle", on_surface, 
            "tabtext", on_surface, 
            "playlisttext", on_surface, 
            "eqtext", on_surface 
        }); 
    }
    if let Some(outline) = colors["outline"].as_str() { c!(map, { "graysolid", outline, "tabborder", outline }); }
    if let Some(tertiary) = colors["tertiary"].as_str() { c!(map, { "playerhover", tertiary, "eqmix", tertiary }); }
    
    Some(map)
}

// --- 2. TRIGGER DARI QML: NYALAIN SYNC WALLPAPER ---
#[qt_method]
pub fn sync_with_wallpaper(&mut self) {
    if let Some(new_colors) = self.fetch_matugen_colors() {
        // 1. Terapin ke QML (UI Langsung Berubah)
        let qmap: QVariantMap = new_colors.iter()
            .map(|(k, v)| (QString::from(k.clone()), QVariant::from(QString::from(v.clone()))))
            .collect();
        
        self.colormap = qmap;
        self.current_raw_colors = new_colors.clone();
        self.colormap_changed();
        
        // 2. Simpan ke Config (Biar gak ilang pas restart)
        if let Some(ref config) = self.config {
            if let Ok(mut cfg) = config.lock() {
                cfg.use_wallpaper_theme = true;
                cfg.matugen_colors = new_colors; // Simpan warnanya di config
                cfg.save();
            }
        }
    } else {
        println!("Gagal baca Matugen JSON! Pastikan ~/.cache/matugen/colors.json ada.");
    }
}

// --- 3. TRIGGER DARI QML: BALIK KE LOONIX MANUAL ---
#[qt_method]
pub fn set_loonix_manual(&mut self) {
    // 1. Update status di config
    if let Some(ref config) = self.config {
        if let Ok(mut cfg) = config.lock() {
            cfg.use_wallpaper_theme = false;
            cfg.save();
        }
    }
    
    // 2. Panggil ulang tema manual yang terakhir aktif
    let current = self.current_theme.to_string();
    self.set_theme(current);
}
```

---

## 3. Modifikasi Fungsi Load di Saat Startup

Buka `set_config` di `theme.rs` dan ubah cara *load* tema agar Rust bisa mengecek status `use_wallpaper_theme` pada saat aplikasi dibuka.

```rust
pub fn set_config(&mut self, config: Arc<Mutex<AppConfig>>) {
    let cfg = config.lock().unwrap();
    
    let theme_name = if cfg.theme.is_empty() { "Default".to_string() } else { cfg.theme.clone() };
    let custom_themes = cfg.custom_themes.clone();
    
    // Ambil data wallpaper engine
    let use_wallpaper = cfg.use_wallpaper_theme;
    let matugen_saved = cfg.matugen_colors.clone();
    
    drop(cfg); // Lepas lock sebelum manggil fungsi lain
    
    self.custom_themes = custom_themes;
    self.config = Some(config);

    // LOGIKA MELEK: Load Matugen atau Load Manual?
    if use_wallpaper && !matugen_saved.is_empty() {
        // Apply warna wallpaper yang udah kesimpen di config
        let qmap: QVariantMap = matugen_saved.iter()
            .map(|(k, v)| (QString::from(k.clone()), QVariant::from(QString::from(v.clone()))))
            .collect();
        self.colormap = qmap;
        self.current_raw_colors = matugen_saved;
        self.colormap_changed();
    } else {
        // Apply tema manual
        self.set_theme(theme_name);
    }
}
```

---

## 4. UI/UX di Pref Theme (`ThemeEditor.qml` / `PrefAppearance.qml`)

Tambahkan interaksi pada *toggle* atau tombol di QML agar bisa memanggil *backend* Rust yang sudah dibuat.

```qml
// Di dalam MouseArea loonixToggle:
onClicked: {
    wallSyncToggle.active = false
    active = true
    theme.set_loonix_manual() // Mematikan sync dan balik ke manual
}

// Di dalam MouseArea wallSyncToggle:
onClicked: {
    if (!active) {
        active = true
        loonixToggle.active = false
        theme.sync_with_wallpaper() // Menarik warna Matugen
    }
}
```

---


## ⚠️ Catatan Tambahan
* **Check Availability:** Saat aplikasi berjalan, sebaiknya buat fungsi untuk mengecek apakah `matugen` tersedia di `$PATH`. Jika tidak ada, disable *toggle* Matugen di UI untuk mencegah *error*.
* **Auto-Update (Opsional):** Bisa dikembangkan lebih lanjut dengan menambahkan *file watcher* di Rust untuk memantau perubahan *file* wallpaper. Jadi saat user mengganti wallpaper, Loonix otomatis mengganti temanya tanpa perlu interaksi klik.