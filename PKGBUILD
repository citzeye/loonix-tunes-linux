# File: PKGBUILD

pkgname=loonix-tunes
pkgver=1.0.0
pkgrel=1
pkgdesc="Music player for Loonix OS (Standalone Plugins)"
arch=('x86_64')
url="https://github.com/citz/loonix-tunes"
license=('GPL')
depends=('qt6-base' 'qt6-declarative' 'gstreamer' 'gst-plugins-base')
makedepends=('rust' 'cargo' 'calf' 'lsp-plugins-lv2')
source=("${pkgname}-${pkgver}::git+file://$PWD")
md5sums=('SKIP')

build() {
    cd "$srcdir/${pkgname}-${pkgver}"
    cargo build --release
}

package() {
    cd "$srcdir/${pkgname}-${pkgver}"
    
    # Bagian: Install binary
    install -Dm755 "target/release/loonix-tunes" "$pkgdir/usr/bin/loonix-tunes"

    # Bagian: Bundling Plugins (Production)
    mkdir -p "$pkgdir/usr/lib/loonix-tunes"
    cp -r plugins "$pkgdir/usr/lib/loonix-tunes/"

    # 2. LV2 Bundles (File suara/efek asli)
    mkdir -p "$pkgdir/usr/lib/loonix-tunes/plugins/lv2"
    
    # Copy Calf Bundle
    cp -r /usr/lib/lv2/calf.lv2 "$pkgdir/usr/lib/loonix-tunes/plugins/lv2/"
    # Copy LSP Bundle (Ganti nama sesuai folder aslinya di /usr/lib/lv2)
    cp -r /usr/lib/lv2/lsp-plugins.lv2 "$pkgdir/usr/lib/loonix-tunes/plugins/lv2/"

    # Pastikan file fisik, bukan symlink (Force overwrite jika perlu)
    cp /usr/lib/calf/libcalf.so "$pkgdir/usr/lib/loonix-tunes/plugins/lv2/calf.lv2/calf.so"
}

# Akhir Bagian