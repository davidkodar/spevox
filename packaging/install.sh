#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
prefix=${PREFIX:-/usr/local}
destination=${DESTDIR:-}

if (( EUID == 0 )) && [[ -z "$destination" ]]; then
  echo "Run this installer as your normal user; it requests sudo only for system files." >&2
  exit 2
fi

cd "$project_dir"
if [[ ! -x target/release/fluidvoice-ui ]]; then
  command -v cargo >/dev/null || { echo "Cargo is required to build from source." >&2; exit 2; }
  [[ -f /usr/include/vulkan/vulkan.h ]] || { echo "Vulkan development headers are required." >&2; exit 2; }
  QMAKE=${QMAKE:-/usr/bin/qmake6} cargo build --release --locked -p fluidvoice-ui
fi

installer=(install)
if [[ -z "$destination" && ( "$prefix" == /usr || "$prefix" == /usr/* ) ]]; then
  installer=(sudo install)
fi

"${installer[@]}" -Dm755 target/release/fluidvoice-ui "$destination$prefix/bin/fluidvoice-ui"
"${installer[@]}" -Dm644 data/io.github.davidkodar.FluidVoiceLinux.desktop "$destination$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
"${installer[@]}" -Dm644 data/io.github.davidkodar.FluidVoiceLinux.metainfo.xml "$destination$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
"${installer[@]}" -Dm644 crates/fluidvoice-ui/assets/fluidvoice-app.png "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"
"${installer[@]}" -Dm644 LICENSE "$destination$prefix/share/licenses/fluidvoice-linux/LICENSE"

if [[ -z "$destination" ]] && command -v kbuildsycoca6 >/dev/null; then
  kbuildsycoca6 --noincremental
fi
echo "FluidVoice Linux installed under $destination$prefix"
