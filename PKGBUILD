# Maintainer: citz <citz@loonix>
pkgname=loonix-tunes
pkgver=1.0.2
pkgrel=1
pkgdesc="Native desktop music player - Rust back end, Qt6 front end"
arch=('x86_64')
url="https://github.com/citzeye/loonix-tunes-linux"
license=('GPL')
depends=('qt6-base' 'qt6-declarative')
makedepends=('rust' 'cargo')

# Mencegah error trait CppTrait pada qmetaobject
options=(!lto)

# Kosongkan source agar makepkg tidak download dari internet
source=()

# Fungsi untuk ambil versi otomatis dari Cargo.toml
pkgver() {
    cd "$startdir"
    grep '^version =' Cargo.toml | head -n1 | cut -d '"' -f2
}

build() {
    # Langsung masuk ke folder tempat kamu berada sekarang
    cd "$startdir"
    
    # Pastikan Qt6 yang terdeteksi oleh build script qmetaobject
    export PATH="/usr/lib/qt6/bin:$PATH"
    
    echo "Starting compilation in $startdir..."
    cargo build --release --locked
}

package() {
    cd "$startdir"
    
    # Install Binary
    install -Dm755 "target/release/loonix-tunes" "$pkgdir/usr/bin/loonix-tunes"
    
    # Install Desktop Entry
    install -Dm644 "packaging/linux/loonix-tunes.desktop" \
        "$pkgdir/usr/share/applications/loonix-tunes.desktop"
    
    # Install Icon
    install -Dm644 "assets/LoonixTunes.png" \
        "$pkgdir/usr/share/icons/hicolor/256x256/apps/loonix-tunes.png"
}