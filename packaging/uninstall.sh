#!/usr/bin/env bash
set -euo pipefail

prefix=${PREFIX:-/usr/local}
destination=${DESTDIR:-}
files=(
  "$destination$prefix/bin/spevox"
  "$destination$prefix/share/applications/io.github.davidkodar.Spevox.desktop"
  "$destination$prefix/share/metainfo/io.github.davidkodar.Spevox.metainfo.xml"
  "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.Spevox.png"
  "$destination$prefix/share/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png"
  "$destination$prefix/share/icons/hicolor/256x256/apps/spevox-app.png"
  "$destination$prefix/share/icons/hicolor/512x512/apps/spevox-app.png"
  "$destination$prefix/share/licenses/spevox/LICENSE"
  "$destination$prefix/share/doc/spevox/README.md"
  "$destination$prefix/share/doc/spevox/CREDITS.md"
  "$destination$prefix/share/doc/spevox/THIRD_PARTY_NOTICES.md"
  "$destination$prefix/share/doc/spevox/THIRD_PARTY_LICENSES.html"
  "$destination$prefix/share/kwin/scripts/spevoxprofiles/metadata.json"
  "$destination$prefix/share/kwin/scripts/spevoxprofiles/contents/code/main.js"
  "$destination$prefix/bin/fluidvoice-ui"
  "$destination$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
  "$destination$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
  "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"
  "$destination$prefix/share/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png"
)

remover=(rm -f)
if [[ -z "$destination" && ( "$prefix" == /usr || "$prefix" == /usr/* ) ]]; then
  remover=(sudo rm -f)
fi
"${remover[@]}" "${files[@]}"
if [[ -z "$destination" ]]; then
  user_data_home=${XDG_DATA_HOME:-$HOME/.local/share}
  rm -f "$user_data_home/applications/io.github.davidkodar.Spevox.desktop"
  rm -f "$user_data_home/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
  if command -v kwriteconfig6 >/dev/null; then
    kwriteconfig6 --file kwinrc --group Plugins --key spevoxprofilesEnabled --delete || true
    qdbus6 org.kde.KWin /KWin reconfigure >/dev/null 2>&1 || true
  fi
  if command -v kbuildsycoca6 >/dev/null; then
    kbuildsycoca6 --noincremental || true
  fi
fi
echo "Spevox application files removed; user models and history were preserved."
