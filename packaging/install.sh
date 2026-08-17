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
command -v ldd >/dev/null || {
  echo "ldd is required to verify the prebuilt binary's runtime libraries." >&2
  exit 2
}
if [[ ! -x target/release/fluidvoice-ui ]]; then
  command -v cargo >/dev/null || { echo "Cargo is required to build from source." >&2; exit 2; }
  [[ -f /usr/include/vulkan/vulkan.h ]] || { echo "Vulkan development headers are required." >&2; exit 2; }
  QMAKE=${QMAKE:-/usr/bin/qmake6} cargo build --release --locked -p fluidvoice-ui
fi

if missing_libraries=$(ldd target/release/fluidvoice-ui 2>/dev/null | awk '/not found/ { print $1 }') && [[ -n "$missing_libraries" ]]; then
  echo "FluidVoice cannot be installed because required runtime libraries are missing:" >&2
  while IFS= read -r library; do
    [[ -n "$library" ]] && printf '  %s\n' "$library" >&2
  done <<< "$missing_libraries"
  echo "Install the matching Qt 6, PipeWire, Vulkan loader, and C/C++ runtime packages, then retry." >&2
  exit 2
fi

installer=(install)
if [[ -z "$destination" && ( "$prefix" == /usr || "$prefix" == /usr/* ) ]]; then
  installer=(sudo install)
fi

"${installer[@]}" -Dm755 target/release/fluidvoice-ui "$destination$prefix/bin/fluidvoice-ui"
"${installer[@]}" -Dm644 data/io.github.davidkodar.FluidVoiceLinux.desktop "$destination$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
"${installer[@]}" -Dm644 data/io.github.davidkodar.FluidVoiceLinux.metainfo.xml "$destination$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
"${installer[@]}" -Dm644 crates/fluidvoice-ui/assets/fluidvoice-app.png "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"
"${installer[@]}" -Dm644 data/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png "$destination$prefix/share/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png"
"${installer[@]}" -Dm644 LICENSE "$destination$prefix/share/licenses/fluidvoice-linux/LICENSE"
"${installer[@]}" -Dm644 README.md "$destination$prefix/share/doc/fluidvoice-linux/README.md"
"${installer[@]}" -Dm644 CREDITS.md "$destination$prefix/share/doc/fluidvoice-linux/CREDITS.md"
"${installer[@]}" -Dm644 THIRD_PARTY_NOTICES.md "$destination$prefix/share/doc/fluidvoice-linux/THIRD_PARTY_NOTICES.md"
"${installer[@]}" -Dm644 THIRD_PARTY_LICENSES.html "$destination$prefix/share/doc/fluidvoice-linux/THIRD_PARTY_LICENSES.html"
"${installer[@]}" -Dm644 packaging/kwin-script/metadata.json "$destination$prefix/share/kwin/scripts/fluidvoiceprofiles/metadata.json"
"${installer[@]}" -Dm644 packaging/kwin-script/contents/code/main.js "$destination$prefix/share/kwin/scripts/fluidvoiceprofiles/contents/code/main.js"

if [[ -z "$destination" ]] && command -v kbuildsycoca6 >/dev/null; then
  kbuildsycoca6 --noincremental
fi
if [[ -z "$destination" ]] && ! command -v ffmpeg >/dev/null; then
  echo "Note: install ffmpeg to enable audio/video file transcription." >&2
fi
echo "FluidVoice Linux installed under $destination$prefix"
