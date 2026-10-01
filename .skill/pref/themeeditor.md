🧠 Logic Flow: The "Melek" System
Entry Scan: User klik edit -> QML tanya: "Gue lagi megang apa?" -> Rust jawab: "Lo lagi megang tema [X], ini data lengkapnya (Warna + Nama)."

State Loading: QML terima data -> Langsung sebar ke semua picker dan placeholder.

The "Smart Save" Chain:

User klik Save -> Scan semua input UI.

Kirim ke Rust: "Ini data barunya, namanya [Nama], warnanya [Warna]."

Rust Logic: Scan config.json.

Ada nama yang sama? Langsung tindas (Overwrite).

Nggak ada? Kirim sinyal balik: "Bray, nama ini belum ada di data, mau bikin baru?"

Post-Save Check: "Apakah tema yang baru gue save ini adalah tema yang lagi nempel di layar?" -> Kalau iya, Trigger Refresh.


🛠️ Backend (Rust): src/ui/theme.rs
Kita bikin fungsi smart_save yang nggak peduli profile, dia peduli Nama dan State.

Rust
#[invokable]
pub fn smart_save_theme(&mut self, current_editor_name: String, colors: QVariantMap) -> String {
    let mut color_map: HashMap<String, String> = HashMap::new();
    for (k, v) in &colors {
        color_map.insert(k.to_string(), v.to_qstring().to_string());
    }

    // 1. SCAN CONFIG: Cari apakah nama di editor sudah ada di database custom_themes
    let theme_exists = self.custom_themes.iter().any(|t| t.name == current_editor_name);

    if theme_exists {
        // 2. REPLACE LOGIC: Update warnanya saja
        if let Some(pos) = self.custom_themes.iter().position(|t| t.name == current_editor_name) {
            self.custom_themes[pos].colors = color_map;
        }
        
        self.save_config();
        self.custom_themes_changed();

        // 3. ACTIVE CHECK: Kalau tema yang di-save lagi aktif, REFRESH UI
        if current_editor_name == self.current_theme.to_string() {
            self.set_theme(current_editor_name);
        }
        
        return "SUCCESS".into();
    } else {
        // 4. NEW THEME LOGIC: Kirim pesan ke QML buat tanya User
        // Di sini kita gak langsung save, tapi minta konfirmasi lewat return string
        return "CONFIRM_NEW".into();
    }
}

// Fungsi paksa save (panggil kalau user klik 'OKE' pas ditanya mau save nama baru)
#[invokable]
pub fn force_save_new_theme(&mut self, name: String, colors: QVariantMap, target_slot: i32) {
    let mut color_map: HashMap<String, String> = HashMap::new();
    for (k, v) in &colors {
        color_map.insert(k.to_string(), v.to_qstring().to_string());
    }

    let idx = target_slot as usize;
    if idx < self.custom_themes.len() {
        self.custom_themes[idx].name = name.clone();
        self.custom_themes[idx].colors = color_map;
        self.save_config();
        self.custom_themes_changed();
        self.set_theme(name); // Langsung aktifin
    }
}
🎨 UI (QML): ThemeEditor.qml
UI sekarang cuma lapor: "Ini data yang gue punya sekarang."

QML
// Tombol SAVE yang "mikir"
Rectangle {
    id: saveBtn
    // ... styling ...
    MouseArea {
        anchors.fill: parent
        onClicked: {
            var scannedColors = scanCurrentEditorColors();
            var result = theme.smart_save_theme(themeNameInput.text, scannedColors);

            if (result === "CONFIRM_NEW") {
                // TAMPILIN DIALOG KE USER
                confirmDialog.open(); 
            } else {
                root.themeEditorVisible = false;
            }
        }
    }
}

// Dialog konfirmasi (bukan judol bray, murni logic)
MessageDialog {
    id: confirmDialog
    title: "Save New Theme?"
    text: "Nama '" + themeNameInput.text + "' nggak ada di config. Mau ganti nama di profile ini?"
    buttons: MessageDialog.Ok | MessageDialog.Cancel
    onAccepted: {
        theme.force_save_new_theme(
            themeNameInput.text, 
            scanCurrentEditorColors(), 
            themeEditorRoot.selectedSlotIndex
        );
        root.themeEditorVisible = false;
    }
}



1.No More Ghost Updates: Karena ada current_theme check di akhir proses save, nggak akan ada lagi kejadian "Warna udah di-save tapi layar nggak berubah".
2.Double-Blind Check: Rust nggak cuma nerima data, dia nge-cek integritas data ke config dulu sebelum beraksi.
3.Flexible Entry: Mau lo klik kanan dari tema aktif, atau lo buka lewat menu "Create", Rust bakal Scan Starter Colors dulu (get_editor_starter_colors) buat mastiin placeholder nggak kosong atau item.