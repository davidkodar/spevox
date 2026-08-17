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
if [[ ! -x target/release/spevox ]]; then
  command -v cargo >/dev/null || { echo "Cargo is required to build from source." >&2; exit 2; }
  [[ -f /usr/include/vulkan/vulkan.h ]] || { echo "Vulkan development headers are required." >&2; exit 2; }
  QMAKE=${QMAKE:-/usr/bin/qmake6} cargo build --release --locked -p spevox-ui
fi

if missing_libraries=$(ldd target/release/spevox 2>/dev/null | awk '/not found/ { print $1 }') && [[ -n "$missing_libraries" ]]; then
  echo "Spevox cannot be installed because required runtime libraries are missing:" >&2
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

"${installer[@]}" -Dm755 target/release/spevox "$destination$prefix/bin/spevox"
"${installer[@]}" -Dm644 data/io.github.davidkodar.Spevox.desktop "$destination$prefix/share/applications/io.github.davidkodar.Spevox.desktop"
"${installer[@]}" -Dm644 data/io.github.davidkodar.Spevox.metainfo.xml "$destination$prefix/share/metainfo/io.github.davidkodar.Spevox.metainfo.xml"
"${installer[@]}" -Dm644 crates/spevox-ui/assets/spevox-app.png "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.Spevox.png"
"${installer[@]}" -Dm644 data/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png "$destination$prefix/share/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png"
"${installer[@]}" -Dm644 LICENSE "$destination$prefix/share/licenses/spevox/LICENSE"
"${installer[@]}" -Dm644 README.md "$destination$prefix/share/doc/spevox/README.md"
"${installer[@]}" -Dm644 CREDITS.md "$destination$prefix/share/doc/spevox/CREDITS.md"
"${installer[@]}" -Dm644 THIRD_PARTY_NOTICES.md "$destination$prefix/share/doc/spevox/THIRD_PARTY_NOTICES.md"
"${installer[@]}" -Dm644 THIRD_PARTY_LICENSES.html "$destination$prefix/share/doc/spevox/THIRD_PARTY_LICENSES.html"
"${installer[@]}" -Dm644 packaging/kwin-script/metadata.json "$destination$prefix/share/kwin/scripts/spevoxprofiles/metadata.json"
"${installer[@]}" -Dm644 packaging/kwin-script/contents/code/main.js "$destination$prefix/share/kwin/scripts/spevoxprofiles/contents/code/main.js"

if [[ -z "$destination" ]]; then
  legacy_files=(
    "$prefix/bin/fluidvoice-ui"
    "$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
    "$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
    "$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"
    "$prefix/share/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png"
  )
  legacy_remover=(rm -f)
  if [[ "$prefix" == /usr || "$prefix" == /usr/* ]]; then
    legacy_remover=(sudo rm -f)
  fi
  "${legacy_remover[@]}" "${legacy_files[@]}"
  user_data_home=${XDG_DATA_HOME:-$HOME/.local/share}
  rm -f "$user_data_home/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
fi

if [[ -z "$destination" ]] && command -v kbuildsycoca6 >/dev/null; then
  kbuildsycoca6 --noincremental
fi
if [[ -z "$destination" ]] && ! command -v ffmpeg >/dev/null; then
  echo "Note: install ffmpeg to enable audio/video file transcription." >&2
fi
echo "Spevox installed under $destination$prefix"
