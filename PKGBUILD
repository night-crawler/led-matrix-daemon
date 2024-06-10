# Maintainer: Igor Kalishevskii <lilo.panic@gmail.com>
pkgname=led_matrix_daemon
pkgver=0.1.0
pkgrel=1
pkgdesc="A daemon that renders images on a LED matrix connected to a Framework 16 Laptop"
arch=('x86_64')
url="https://github.com/night-crawler/$pkgname"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust', 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/night-crawler/$pkgname/archive/$pkgver.tar.gz")
sha512sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=nightly
  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  install -Dm644 "$srcdir/$pkgname-$pkgver/led_matrix_daemon.service" "$pkgdir/usr/lib/systemd/system/$pkgname.service"
  install -Dm644 "$srcdir/$pkgname-$pkgver/test_data/config.toml" "$pkgdir/etc/led_matrix/daemon.toml"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE"
}
