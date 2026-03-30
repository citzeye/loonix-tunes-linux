#!/bin/bash
# --- LOONIX-TUNES PACKAGER (LINUX EDITION) ---
set -e

PKG_NAME="loonix-tunes"
VERSION="1.0.1"
ARCH="x86_64"

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

# 1. Jalur Sumber (Sesuai Tree lo)
SOURCE_BINARY="./target/release/$PKG_NAME"
DESKTOP_FILE="./packaging/linux/loonix-tunes.desktop"
ICON_FILE="./packaging/linux/icon.png"

# 2. Bersihkan sisa paket lama
rm -f *.pkg.tar.zst
rm -rf pkg_temp

# 3. Buat folder struktur paket sesuai standar Linux
mkdir -p pkg_temp/usr/bin
mkdir -p pkg_temp/usr/share/applications
mkdir -p pkg_temp/usr/share/icons/hicolor/256x256/apps/

# 4. Copy File & Aset
echo "Menyiapkan binary dari: $SOURCE_BINARY"
if [ -f "$SOURCE_BINARY" ]; then
    cp -f "$SOURCE_BINARY" pkg_temp/usr/bin/
    chmod +x pkg_temp/usr/bin/$PKG_NAME
else
    echo "Error: Binary nggak ketemu! Lupa 'cargo build --release'?"
    exit 1
fi

echo "Memasukkan Shortcut & Ikon..."
cp -f "$DESKTOP_FILE" pkg_temp/usr/share/applications/
cp -f "$ICON_FILE" pkg_temp/usr/share/icons/hicolor/256x256/apps/loonix-tunes.png

# 5. Buat file .PKGINFO
INSTALLED_SIZE=$(du -sk pkg_temp | cut -f1)

cat <<EOF > pkg_temp/.PKGINFO
pkgname = $PKG_NAME
pkgver = $VERSION-1
pkgdesc = High-Res Music Player (PipeWire Native)
builddate = $(date +%s)
packager = citz
size = $((INSTALLED_SIZE * 1024))
arch = $ARCH
license = GPL
depend = qt6-base
depend = qt6-declarative
depend = ffmpeg
depend = pipewire
EOF

# 6. Kompresi menjadi .pkg.tar.zst
echo "Proses kompresi paket Arch Linux..."
cd pkg_temp
tar --zstd -cf "../$PKG_NAME-$VERSION-1-$ARCH.pkg.tar.zst" .PKGINFO usr/

# 7. Selesai
cd "$ROOT_DIR"
rm -rf pkg_temp

echo "-------------------------------------------"
echo "BERHASIL: $(ls $PKG_NAME-*.pkg.tar.zst)"
echo "Paket siap install di Loonix OS (Arch-based)"
echo "-------------------------------------------"