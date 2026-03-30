# File: PKGBUILD

pkgname=loonix-tunes
pkgver=1.0.0
pkgrel=1
pkgdesc="Music player for Loonix OS"
arch=('x86_64')
url="https://github.com/citz/loonix-tunes"
license=('GPL')
depends=('qt5-base' 'qt5-declarative' 'qt5-multimedia' 'hicolor-icon-theme' 'gstreamer' 'gst-plugins-base')
source=("${pkgname}-${pkgver}::git+file://$PWD")
md5sums=('SKIP')

build() {
    cd "$srcdir/${pkgname}-${pkgver}"
    
    # Paksa pake Qt5
    export QT_SELECT=qt5
    export PKG_CONFIG_PATH=/usr/lib/pkgconfig
    
    cargo build --release
}

package() {
    cd "$srcdir/${pkgname}-${pkgver}"
    
    # 1. Install Binary
    install -Dm755 "target/release/loonix-tunes" "$pkgdir/usr/bin/loonix-tunes"

    # 2. Install Desktop File
    install -Dm644 "packaging/linux/loonix-tunes.desktop" "$pkgdir/usr/share/applications/loonix-tunes.desktop"

    # 3. Install Icon
    install -Dm644 "packaging/linux/icon.png" "$pkgdir/usr/share/icons/hicolor/256x256/apps/loonix-tunes.png"
}