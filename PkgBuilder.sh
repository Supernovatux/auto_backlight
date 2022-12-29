#!/bin/zsh
cargo build --profile=release    
version=$(target/release/auto-backlight -V | awk  '{print $2}')
tar -cvf auto-backlight-$version-x86_64.tar.gz completions target/release/auto-backlight LICENSE 90-auto-backlight.rules -I "gzip --best"
shasum=$(sha512sum ./auto-backlight-$version-x86_64.tar.gz | awk '{print $1}')
sed -i "s/sha512sums=.*/sha512sums=(\"$shasum\")/" PKGBUILD
sed -i "s/pkgver=.*/pkgver=$version/" PKGBUILD