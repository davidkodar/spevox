#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
source_dir="$project_dir/target/flatpak-source"
repo_dir="$project_dir/target/flatpak-repo"
build_dir="$project_dir/target/flatpak-build"
bundle="$project_dir/target/package/fluidvoice-linux.flatpak"

command -v flatpak-builder >/dev/null || { echo "flatpak-builder is required." >&2; exit 2; }
command -v cargo >/dev/null || { echo "Cargo is required to vendor Rust dependencies." >&2; exit 2; }

rm -rf "$source_dir" "$repo_dir" "$build_dir"
mkdir -p "$source_dir/.cargo" "$(dirname "$bundle")"
git -C "$project_dir" archive HEAD | tar -x -C "$source_dir"
(cd "$source_dir" && cargo vendor --locked vendor > .cargo/config.toml)
flatpak-builder --force-clean --repo="$repo_dir" "$build_dir" \
  "$project_dir/packaging/flatpak/io.github.davidkodar.FluidVoiceLinux.yml"
flatpak build-bundle "$repo_dir" "$bundle" io.github.davidkodar.FluidVoiceLinux
(
    cd "$(dirname "$bundle")"
    sha256sum "$(basename "$bundle")" > "$(basename "$bundle").sha256"
)
echo "$bundle"
