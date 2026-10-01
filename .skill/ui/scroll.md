📜 LOONIX-TUNES SCROLLBAR STANDARD OPERATING PROCEDURE

Dokumen ini berisi standar teknis dan visual untuk implementasi ScrollBar di seluruh halaman Loonix-Tunes.

## 1. THE GOLDEN RULES (PRINSIP UTAMA)
VERTICAL ONLY: DILARANG keras menggunakan ScrollBar.horizontal. 
Jika konten meluber ke samping, perbaiki Responsive Layout-nya (gunakan elide: Text.ElideRight atau Layout.fillWidth), jangan tambahkan scrollbar horizontal.

INSIDE SCOPE: ScrollBar.vertical WAJIB didefinisikan di DALAM kurung kurawal { } milik komponen parent (seperti ListView, Flickable, atau GridView).

DYNAMIC SIZING: Batang gulir (handle) harus memiliki tinggi yang dinamis sesuai dengan rasio konten yang terlihat.

## 2. HIGH-END SCROLLBAR TEMPLATE
Gunakan template kode di bawah ini untuk setiap halaman baru atau perbaikan:
QMLScrollBar.vertical:
```
ScrollBar {
    id: vBar
    width: 6
    policy: ScrollBar.AsNeeded
    
    // Background dibuat transparan agar tidak mengganggu visual list
    background: Rectangle { 
        color: "transparent" 
    }

    contentItem: Rectangle {
        implicitWidth: 6
        // Menghitung tinggi dinamis berdasarkan rasio konten vs tinggi view
        implicitHeight: Math.max(30, parent.height * parent.visibleArea.heightRatio)
        radius: 3
        
        // Interaktivitas Warna: Accent saat normal/klik, Hover saat disentuh
        color: vBar.pressed ? theme.colormap.playeraccent : 
               vBar.hovered ? theme.colormap.headerhover : 
               theme.colormap.playeraccent
        
        // Redup saat idle (0.5), Terang saat aktif/sedang scroll (1.0)
        opacity: vBar.active ? 1.0 : 0.5
        
        // Animasi transisi halus
        Behavior on color { ColorAnimation { duration: 150 } }
        Behavior on opacity { NumberAnimation { duration: 150 } }
    }
}
```

## 3. CHECKLIST IMPLEMENTASI
Saat membuat halaman baru, pastikan poin-poin ini terpenuhi:[ ] ID Consistency: Gunakan id: vBar jika hanya ada satu list di file tersebut. 
Jika ada lebih, gunakan prefix unik (misal: prefVBar).[ ] Parent Reference: Pastikan parent.height merujuk langsung ke ListView atau Flickable yang menaunginya.[ ] Z-Order: Jika scrollbar tertutup elemen lain, tambahkan z: 1 di dalam blok ScrollBar.[ ] No Overlap: Jika scrollbar menutupi teks, tambahkan rightMargin pada elemen anak list tersebut.

## 4. TROUBLESHOOTING (PEMECAHAN MASALAH)
MasalahPenyebab UmumSolusiScrollBar Gak MunculDitaruh di luar kurung kurawal ListView.Pindahkan kode ke dalam blok ListView/Flickable.
Batang (Handle) 0pxGagal membaca heightRatio.
Pastikan parent memiliki height yang jelas dan clip: true.ScrollBar Horizontal MunculcontentWidth lebih besar dari width.
Hapus ScrollBar.horizontal dan set width anak-anaknya ke parent.width.Update Nama/Data LagQML Binding tidak reaktif.
Gunakan trik refreshTicker dan Connections.

## 5. REAKTIVITAS (REFRESH TICKER)
Jika list data berubah di Rust tapi ScrollBar tidak memperbarui posisinya, gunakan mekanisme ini:
QML// Di root element
```
property int refreshTicker: 0

Connections {
    target: musicModel
    function onData_changed() { refreshTicker++ }
}
```
```
// Di binding scrollbar
implicitHeight: (refreshTicker, Math.max(30, parent.height * parent.visibleArea.heightRatio))
```