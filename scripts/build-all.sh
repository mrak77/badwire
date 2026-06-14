#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

APP_NAME="BadWire"
BIN_NAME="badwire"
VERSION="1.0.0"
DIST_DIR="builds"
DEB_FILE="${BIN_NAME}_${VERSION}_amd64.deb"
ARCHIVE_FILE="${BIN_NAME}-${VERSION}.tar.gz"
PKGBUILD_FILE="PKGBUILD"

echo "=== Building $APP_NAME ==="

mkdir -p "$DIST_DIR"

# 1. Бинарник
if [ ! -f "target/release/$APP_NAME" ]; then
    echo "Building release binary..."
    cargo build --release
else
    echo "Binary already exists."
fi

# 2. Deb-пакет
if [ -f "scripts/build-deb.sh" ]; then
    echo "Building .deb package..."
    bash scripts/build-deb.sh
else
    echo "WARNING: scripts/build-deb.sh not found, skipping .deb"
fi

# 3. Архив исходников (исключая лишнее)
echo "Creating source archive $ARCHIVE_FILE..."
git archive --format=tar.gz --prefix="${BIN_NAME}-${VERSION}/" --output="$DIST_DIR/$ARCHIVE_FILE" HEAD

# 4. Вычисляем SHA256 архива
SHA256=$(sha256sum "$DIST_DIR/$ARCHIVE_FILE" | awk '{print $1}')
echo "SHA256 of source archive: $SHA256"

# 5. Обновляем PKGBUILD
if [ -f "$PKGBUILD_FILE" ]; then
    echo "Updating PKGBUILD..."
    sed -i "s/^pkgver=.*/pkgver=$VERSION/" "$PKGBUILD_FILE"
    if grep -q '^sha256sums=' "$PKGBUILD_FILE"; then
        sed -i "s/^sha256sums=.*/sha256sums=('$SHA256')/" "$PKGBUILD_FILE"
    else
        sed -i "/^source=/a sha256sums=('$SHA256')" "$PKGBUILD_FILE"
    fi
else
    echo "WARNING: PKGBUILD not found in project root, creating minimal one..."
    cat > "$PKGBUILD_FILE" <<EOF
# Maintainer: mrak77 <pb.mrak@yandex.ru>
pkgname=$BIN_NAME
pkgver=$VERSION
pkgrel=1
pkgdesc="Network impairment tool using tc-netem"
arch=('x86_64')
url="https://github.com/mrak77/$BIN_NAME"
license=('GPL-3.0')
options=(!debug)
depends=('gtk3' 'iproute2' 'polkit')
makedepends=('cargo')
source=("${BIN_NAME}-\${pkgver}.tar.gz::https://github.com/yourrepo/$BIN_NAME/archive/refs/tags/v\${pkgver}.tar.gz")
sha256sums=('$SHA256')

build() {
    cd "\$srcdir/${BIN_NAME}-\${pkgver}"
    cargo build --release --locked
}

package() {
    cd "\$srcdir/${BIN_NAME}-\${pkgver}"
    install -Dm755 "target/release/$APP_NAME" "\$pkgdir/usr/bin/$BIN_NAME"

    for size in 16 22 32 48 64 256; do
        install -Dm644 "assets/icons/${size}x${size}/badwire.png" "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/badwire.png"
    done

    install -Dm644 "packaging/${BIN_NAME}.desktop" "\$pkgdir/usr/share/applications/${BIN_NAME}.desktop"
    install -Dm755 "packaging/helper/badwire-tc-helper" "$pkgdir/usr/lib/badwire/badwire-tc-helper"
    install -Dm644 "packaging/polkit/badwire.policy" "$pkgdir/usr/share/polkit-1/actions/org.mrak77.badwire.run-tc.policy"
}
EOF
fi

echo ""
echo "=== Done ==="
echo "Source archive: $DIST_DIR/$ARCHIVE_FILE"
echo "Deb package:    $DIST_DIR/$DEB_FILE (if built)"
echo "PKGBUILD:       $PKGBUILD_FILE (updated with SHA256)"
echo ""
echo "For AUR submission, upload $ARCHIVE_FILE to GitHub release and update PKGBUILD source URL."
