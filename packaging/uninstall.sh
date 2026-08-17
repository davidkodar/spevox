#!/usr/bin/env bash
set -euo pipefail

prefix=${PREFIX:-/usr/local}
destination=${DESTDIR:-}
files=(
  "$destination$prefix/bin/fluidvoice-ui"
  "$destination$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
  "$destination$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
  "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"
  "$destination$prefix/share/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png"
  "$destination$prefix/share/licenses/fluidvoice-linux/LICENSE"
  "$destination$prefix/share/doc/fluidvoice-linux/README.md"
  "$destination$prefix/share/doc/fluidvoice-linux/CREDITS.md"
  "$destination$prefix/share/doc/fluidvoice-linux/THIRD_PARTY_NOTICES.md"
  "$destination$prefix/share/doc/fluidvoice-linux/THIRD_PARTY_LICENSES.html"
  "$destination$prefix/share/kwin/scripts/fluidvoiceprofiles/metadata.json"
  "$destination$prefix/share/kwin/scripts/fluidvoiceprofiles/contents/code/main.js"
)

remover=(rm -f)
if [[ -z "$destination" && ( "$prefix" == /usr || "$prefix" == /usr/* ) ]]; then
  remover=(sudo rm -f)
fi
"${remover[@]}" "${files[@]}"
if [[ -z "$destination" ]]; then
  user_data_home=${XDG_DATA_HOME:-$HOME/.local/share}
  rm -f "$user_data_home/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
  if command -v kwriteconfig6 >/dev/null; then
    kwriteconfig6 --file kwinrc --group Plugins --key fluidvoiceprofilesEnabled --delete || true
    qdbus6 org.kde.KWin /KWin reconfigure >/dev/null 2>&1 || true
  fi
  if command -v kbuildsycoca6 >/dev/null; then
    kbuildsycoca6 --noincremental || true
  fi
fi
echo "FluidVoice Linux application files removed; user models and history were preserved."
