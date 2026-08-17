#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$project_dir/Cargo.toml" | head -n 1)
stage="$project_dir/target/package/spevox-$version"
archive="$project_dir/target/package/spevox-$version-x86_64.tar.gz"

cd "$project_dir"
build_root="/usr/src/spevox"
cargo_home="${CARGO_HOME:-${HOME:-/tmp}/.cargo}"
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=${project_dir}=${build_root} --remap-path-prefix=${cargo_home}=/usr/src/cargo"
export CFLAGS="${CFLAGS:-} -ffile-prefix-map=${project_dir}=${build_root} -ffile-prefix-map=${cargo_home}=/usr/src/cargo"
export CXXFLAGS="${CXXFLAGS:-} -ffile-prefix-map=${project_dir}=${build_root} -ffile-prefix-map=${cargo_home}=/usr/src/cargo"
QMAKE=${QMAKE:-/usr/bin/qmake6} cargo build --release --locked -p spevox-ui
rm -rf "$stage"
install -Dm755 target/release/spevox "$stage/target/release/spevox"
install -Dm755 packaging/install.sh "$stage/packaging/install.sh"
install -Dm755 packaging/uninstall.sh "$stage/packaging/uninstall.sh"
install -Dm644 data/io.github.davidkodar.Spevox.desktop "$stage/data/io.github.davidkodar.Spevox.desktop"
install -Dm644 data/io.github.davidkodar.Spevox.metainfo.xml "$stage/data/io.github.davidkodar.Spevox.metainfo.xml"
install -Dm644 crates/spevox-ui/assets/spevox-app.png "$stage/crates/spevox-ui/assets/spevox-app.png"
install -Dm644 data/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png "$stage/data/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png"
install -Dm644 packaging/kwin-script/metadata.json "$stage/packaging/kwin-script/metadata.json"
install -Dm644 packaging/kwin-script/contents/code/main.js "$stage/packaging/kwin-script/contents/code/main.js"
install -Dm644 LICENSE "$stage/LICENSE"
install -Dm644 README.md "$stage/README.md"
install -Dm644 CREDITS.md "$stage/CREDITS.md"
install -Dm644 THIRD_PARTY_NOTICES.md "$stage/THIRD_PARTY_NOTICES.md"
install -Dm644 THIRD_PARTY_LICENSES.html "$stage/THIRD_PARTY_LICENSES.html"
tar -C "$(dirname "$stage")" -czf "$archive" "$(basename "$stage")"
(
    cd "$(dirname "$archive")"
    sha256sum "$(basename "$archive")" > "$(basename "$archive").sha256"
)
echo "$archive"
