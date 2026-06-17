# Maintainer: mrak77 <pb.mrak@yandex.ru>
pkgname=badwire
pkgver=1.0.1
pkgrel=1
pkgdesc="BadWire is a simple GUI for simulating network problems with tc-netem"
arch=('x86_64')
url="https://github.com/mrak77/badwire"
license=('GPL-3.0')
options=(!debug)
depends=('gtk3' 'iproute2' 'polkit')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/mrak77/badwire/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('1eebc65226e937a53c8a3b03bd7d8beb6adc574bbe1e1a062c35e8de71f088b3')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/BadWire" "$pkgdir/usr/bin/badwire"

    for size in 16 22 32 48 64 256; do
        install -Dm644 "assets/icons/${size}x${size}/badwire.png" "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/badwire.png"
    done

    install -Dm644 "packaging/badwire.desktop" "$pkgdir/usr/share/applications/badwire.desktop"
    install -Dm755 "packaging/helper/badwire-tc-helper" "$pkgdir/usr/lib/badwire/badwire-tc-helper"
    install -Dm644 "packaging/polkit/badwire.policy" "$pkgdir/usr/share/polkit-1/actions/org.mrak77.badwire.run-tc.policy"
}
