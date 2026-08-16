#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
prefix=${PREFIX:-/usr/local}
destination=${DESTDIR:-}

if (( EUID == 0 )); then
  echo "Do not run this script with sudo." >&2
  echo "Run it as your normal user; it will request sudo only when installing files." >&2
  exit 2
fi

cd "$project_dir"
QMAKE=${QMAKE:-/usr/bin/qmake6} cargo build --release -p fluidvoice-ui

install_command=(install)
if [[ -z "$destination" && "$prefix" == /usr/* ]]; then
  install_command=(sudo install)
fi

"${install_command[@]}" -Dm755 target/release/fluidvoice-ui "$destination$prefix/bin/fluidvoice-ui"
"${install_command[@]}" -Dm644 data/io.github.davidkodar.FluidVoiceLinux.desktop \
  "$destination$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
"${install_command[@]}" -Dm644 data/io.github.davidkodar.FluidVoiceLinux.metainfo.xml \
  "$destination$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
"${install_command[@]}" -Dm644 crates/fluidvoice-ui/assets/fluidvoice-app.png \
  "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"
"${install_command[@]}" -Dm644 LICENSE "$destination$prefix/share/licenses/fluidvoice-linux/LICENSE"

if [[ -z "$destination" ]] && command -v kbuildsycoca6 >/dev/null; then
  kbuildsycoca6 --noincremental
fi

echo "FluidVoice Linux installed under $destination$prefix"
