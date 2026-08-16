#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
prefix=${PREFIX:-/usr/local}
destination=${DESTDIR:-}

cd "$project_dir"
QMAKE=${QMAKE:-/usr/bin/qmake6} cargo build --release -p fluidvoice-ui

install -Dm755 target/release/fluidvoice-ui "$destination$prefix/bin/fluidvoice-ui"
install -Dm644 data/io.github.davidkodar.FluidVoiceLinux.desktop \
  "$destination$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
install -Dm644 data/io.github.davidkodar.FluidVoiceLinux.metainfo.xml \
  "$destination$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
install -Dm644 crates/fluidvoice-ui/assets/fluidvoice-app.png \
  "$destination$prefix/share/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png"
install -Dm644 LICENSE "$destination$prefix/share/licenses/fluidvoice-linux/LICENSE"

echo "FluidVoice Linux installed under $destination$prefix"
