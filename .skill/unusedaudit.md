# 🧹 Unused Code Audit Guide (Loonix-Tunes)

Dokumen ini adalah *checklist* untuk membersihkan "Dead Code" (kode zombie/tidak terpakai) setelah migrasi arsitektur ke sistem Lazy Update (Atomics) dan perombakan QML.

Lakukan audit ini secara berkala agar *binary size* tetap kecil dan *codebase* tidak berantakan.

## 🛠️ Tahap 1: Rust Backend Audit (The Compiler Way)

Rust punya *tooling* bawaan yang sangat agresif terhadap *dead code*. Gunakan ini sebagai langkah pertama:

### 1. Jalankan Cargo Check & Clippy
Jalankan perintah ini di terminal:
`cargo check`

ATAU untuk analisa yang lebih dalam:
`cargo clippy -- -D warnings`

cari warning seperti:

- unused import
- unused variable
- value assigned but never read
- fields are never read
- function/method never used
- struct never constructed
- associated items never used
- dead_code

Tugas kamu:

1. Untuk SETIAP item yang kena warning:
   - Jelaskan apakah benar-benar tidak digunakan (safe to delete),
   - Atau sebenarnya bagian dari flow arsitektur tapi tidak terhubung (harusnya dipakai tapi belum dikoneksikan).

2. Untuk kasus "value assigned is never read":
   - Analisa apakah:
     a) memang redundant assignment,
     b) state machine bug (harusnya dibaca tapi ketimpa),
     c) logic reconnect / retry yang tidak lengkap.

3. Untuk "fields are never read":
   - Cek apakah field:
     a) bagian dari future feature,
     b) state yang seharusnya dipakai tapi lupa wiring,
     c) memang dead field yang bisa dihapus.

4. Untuk function/method yang tidak pernah dipanggil:
   - Telusuri apakah:
     a) memang legacy code,
     b) intended untuk dipanggil dari layer lain tapi tidak pernah di-hook,
     c) bagian dari desain modular yang belum selesai.

5. Jangan hanya menyarankan:
   - "hapus saja"
   - atau "prefix dengan _"

Saya ingin:
- Analisis berbasis arsitektur
- Deteksi bug tersembunyi akibat state yang tidak pernah terbaca
- Identifikasi reconnect/state logic yang salah (karena banyak reconnect_attempts warning)

Output format WAJIB:

Untuk setiap item:

[ITEM]
Lokasi:
Jenis warning:
Analisa:
Status: (DELETE / FIX LOGIC / CONNECT MISSING FLOW / KEEP)

Jika FIX atau CONNECT:
Tunjukkan secara konkret bagian mana yang seharusnya membaca/menggunakan item tersebut. 
Jika kamu menemukan pola state yang di-set tapi tidak pernah digunakan untuk decision making,
anggap itu sebagai kemungkinan bug desain dan jelaskan dampaknya terhadap runtime behavior. 

---
*Loonix-Tunes Development Routine - Run this checklist before merging major feature branches.*