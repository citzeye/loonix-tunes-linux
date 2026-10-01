ini logic nya :
app di install : Loonix langsung jadi tema utama.
saat app di instal pasti dibuatkan theme.json, langsung isi dengan 8 tema built-in tapi jangan masukkan array warna, cukup masukkan nama dan status (status sedang aktif di gunakan atau tidak)
user buka PrefThemeEditor, user ubah nama, klik slot mana yang mau di ganti 1,2 atay 3. lalu klik save, lalu kirim semua ke theme.json dan aktifkan tema yang user baru edit.
saat aplikasi di tutup dan dibuka lagi, tinggal cari saja dari 11 tema mana yang memiliki status aktif.
kalau user klik toggle theme di Ui.qml atau melalui PrefAppearance.qml maka hanya kirim status aja menjadi akrif sekaligua meatikan theme lain yang memiliki status aktif sebelumnya.