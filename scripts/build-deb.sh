#!/bin/bash
set -e

# Определяем корень проекта (на уровень выше scripts/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

APP_NAME="BadWire"
BIN_NAME="badwire"
VERSION="1.0.0"
BUILD_DIR="$PROJECT_DIR/target/release"
DIST_DIR="$PROJECT_DIR/builds"
DEB_DIR="${BIN_NAME}_${VERSION}_amd64"

mkdir -p "$DIST_DIR"

if [ ! -f "$BUILD_DIR/$APP_NAME" ]; then
    echo "Error: Binary not found at $BUILD_DIR/$APP_NAME"
    echo "Run 'cargo build --release' from project root first"
    exit 1
fi

ICON48="$PROJECT_DIR/assets/icons/48x48/badwire.png"
ICON256="$PROJECT_DIR/assets/icons/256x256/badwire.png"
if [ ! -f "$ICON48" ] || [ ! -f "$ICON256" ]; then
    echo "Error: Icon files missing."
    exit 1
fi

DESKTOP_FILE="$PROJECT_DIR/packaging/badwire.desktop"
if [ ! -f "$DESKTOP_FILE" ]; then
    echo "Error: Desktop file not found"
    exit 1
fi

rm -rf "$DEB_DIR"

mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr/bin"
mkdir -p "$DEB_DIR/usr/share/applications"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/48x48/apps"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$DEB_DIR/usr/share/doc/$BIN_NAME"
mkdir -p "$DEB_DIR/usr/share/polkit-1/actions"
mkdir -p "$DEB_DIR/usr/lib/badwire"

# Бинарник
cp "$BUILD_DIR/$APP_NAME" "$DEB_DIR/usr/bin/$BIN_NAME"
chmod 755 "$DEB_DIR/usr/bin/$BIN_NAME"

# Иконки
for size in 16 22 32 48 64 256; do
    mkdir -p "$DEB_DIR/usr/share/icons/hicolor/${size}x${size}/apps"
    cp "$PROJECT_DIR/assets/icons/${size}x${size}/badwire.png" "$DEB_DIR/usr/share/icons/hicolor/${size}x${size}/apps/badwire.png"
    chmod 644 "$DEB_DIR/usr/share/icons/hicolor/${size}x${size}/apps/badwire.png"
done

# Desktop-файл с подстановкой pkexec
sed "s/^Exec=.*/Exec=\/usr\/bin\/$BIN_NAME/" "$DESKTOP_FILE" \
    > "$DEB_DIR/usr/share/applications/badwire.desktop"
chmod 644 "$DEB_DIR/usr/share/applications/badwire.desktop"

# Политики
cp "$PROJECT_DIR/packaging/polkit/badwire.policy" "$DEB_DIR/usr/share/polkit-1/actions/org.mrak77.badwire.run-tc.policy"
chmod 644 "$DEB_DIR/usr/share/polkit-1/actions/org.mrak77.badwire.run-tc.policy"
cp "$PROJECT_DIR/packaging/helper/badwire-tc-helper" "$DEB_DIR/usr/lib/badwire/badwire-tc-helper"
chmod 755 "$DEB_DIR/usr/lib/badwire/badwire-tc-helper"

# Управляющие файлы deb
CONTROL_DIR="$PROJECT_DIR/packaging/debian"
cp "$CONTROL_DIR/control" "$DEB_DIR/DEBIAN/control"
chmod 644 "$DEB_DIR/DEBIAN/control"
cp "$CONTROL_DIR/postinst" "$DEB_DIR/DEBIAN/postinst"
chmod 755 "$DEB_DIR/DEBIAN/postinst"

# Сборка
echo "Building .deb package..."
dpkg-deb --root-owner-group --build "$DEB_DIR"
mv "${DEB_DIR}.deb" "$DIST_DIR/"
rm -rf "$DEB_DIR"

echo "Package created: $DIST_DIR/${DEB_DIR}.deb"
