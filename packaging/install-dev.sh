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
if ! command -v cargo >/dev/null; then
  echo "Cargo was not found. Install Rust 1.85 or newer and run this script without sudo." >&2
  exit 2
fi
if [[ ! -f /usr/include/vulkan/vulkan.h ]]; then
  echo "Vulkan development headers are required to build GPU acceleration." >&2
  echo "On Arch/CachyOS, install them with: sudo pacman -S --needed vulkan-headers" >&2
  exit 2
fi
if ! command -v secret-tool >/dev/null; then
  echo "Note: secret-tool was not found; install libsecret before saving cloud AI API keys." >&2
fi
if ! command -v ffmpeg >/dev/null; then
  echo "Note: FFmpeg was not found; file transcription will be limited to 16-bit PCM WAV." >&2
fi
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
"${install_command[@]}" -Dm644 packaging/kwin-script/metadata.json \
  "$destination$prefix/share/kwin/scripts/fluidvoiceprofiles/metadata.json"
"${install_command[@]}" -Dm644 packaging/kwin-script/contents/code/main.js \
  "$destination$prefix/share/kwin/scripts/fluidvoiceprofiles/contents/code/main.js"

if [[ -z "$destination" ]]; then
  # KDE gives per-user desktop entries priority. Replace the early development
  # entry that used Icon=audio-input-microphone so it cannot shadow the
  # correctly packaged application identity and artwork.
  user_data_home=${XDG_DATA_HOME:-$HOME/.local/share}
  install -Dm644 data/io.github.davidkodar.FluidVoiceLinux.desktop \
    "$user_data_home/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
  if command -v kbuildsycoca6 >/dev/null; then
    kbuildsycoca6 --noincremental
  fi
fi

echo "FluidVoice Linux installed under $destination$prefix"
