#!/usr/bin/env bash
set -euo pipefail

prefix=${PREFIX:-/usr/local}
destination=${DESTDIR:-}
files=(
  "$destination$prefix/bin/fluidvoice-ui"
  "$destination$prefix/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
  "$destination$prefix/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
  "$destination$prefix/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"
  "$destination$prefix/share/licenses/fluidvoice-linux/LICENSE"
)

remover=(rm -f)
if [[ -z "$destination" && ( "$prefix" == /usr || "$prefix" == /usr/* ) ]]; then
  remover=(sudo rm -f)
fi
"${remover[@]}" "${files[@]}"
if [[ -z "$destination" ]] && command -v kbuildsycoca6 >/dev/null; then
  kbuildsycoca6 --noincremental || true
fi
echo "FluidVoice Linux application files removed; user models and history were preserved."
