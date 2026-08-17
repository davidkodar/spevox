#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$project_dir/Cargo.toml" | head -n 1)
stage="$project_dir/target/package/fluidvoice-linux-$version"
archive="$project_dir/target/package/fluidvoice-linux-$version-x86_64.tar.gz"

cd "$project_dir"
QMAKE=${QMAKE:-/usr/bin/qmake6} cargo build --release --locked -p fluidvoice-ui
rm -rf "$stage"
install -Dm755 target/release/fluidvoice-ui "$stage/target/release/fluidvoice-ui"
install -Dm755 packaging/install.sh "$stage/packaging/install.sh"
install -Dm755 packaging/uninstall.sh "$stage/packaging/uninstall.sh"
install -Dm644 data/io.github.davidkodar.FluidVoiceLinux.desktop "$stage/data/io.github.davidkodar.FluidVoiceLinux.desktop"
install -Dm644 data/io.github.davidkodar.FluidVoiceLinux.metainfo.xml "$stage/data/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
install -Dm644 crates/fluidvoice-ui/assets/fluidvoice-app.png "$stage/crates/fluidvoice-ui/assets/fluidvoice-app.png"
install -Dm644 data/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png "$stage/data/icons/hicolor/512x512/apps/io.github.davidkodar.FluidVoiceLinux.png"
install -Dm644 packaging/kwin-script/metadata.json "$stage/packaging/kwin-script/metadata.json"
install -Dm644 packaging/kwin-script/contents/code/main.js "$stage/packaging/kwin-script/contents/code/main.js"
install -Dm644 LICENSE "$stage/LICENSE"
install -Dm644 README.md "$stage/README.md"
install -Dm644 THIRD_PARTY_NOTICES.md "$stage/THIRD_PARTY_NOTICES.md"
tar -C "$(dirname "$stage")" -czf "$archive" "$(basename "$stage")"
sha256sum "$archive" > "$archive.sha256"
echo "$archive"
