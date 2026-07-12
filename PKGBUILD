# Maintainer: Parch Linux Team <team@parchlinux.com>

pkgname=parch-tweaks
pkgver=0.1.1
pkgrel=1
pkgdesc='Configuration tool for Parch Linux'
arch=('x86_64')
url='https://github.com/parchlinux/parch-tweaks'
license=('GPL-3.0-or-later')
depends=('gtk4' 'libadwaita' 'pacman')
makedepends=('cargo' 'git')
source=("git+https://github.com/parchlinux/parch-tweaks.git")
b2sums=('SKIP')

pkgver() {
    cd "$pkgname"
    git describe --long --tags | sed 's/^v//;s/[^-]*-g\([0-9a-f]*\)/\1/;s/-/./g'
}

prepare() {
    cd "$pkgname"
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$pkgname"
    cargo build --frozen --release --bin parch-tweaks --bin ptt
}

package() {
    cd "$pkgname"

    install -Dm755 "target/release/parch-tweaks" "$pkgdir/usr/bin/parch-tweaks"
    install -Dm755 "target/release/ptt" "$pkgdir/usr/bin/ptt"

    install -Dm644 "data/parch-tweaks.desktop" "$pkgdir/usr/share/applications/parch-tweaks.desktop"
    install -Dm644 "data/icons/parch-tweaks.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/parch-tweaks.svg"
    install -Dm644 "data/parch-tweaks.metainfo.xml" "$pkgdir/usr/share/metainfo/parch-tweaks.metainfo.xml"
}
