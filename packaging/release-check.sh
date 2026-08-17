#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
smoke_root="$(mktemp -d -t fluidvoice-release-check.XXXXXX)"
trap 'rm -rf -- "${smoke_root}"' EXIT
cd "${repo_root}"

test "${ALLOW_DIRTY:-0}" = 1 || test -z "$(git status --porcelain)" || {
    echo "release check requires a clean worktree" >&2
    exit 1
}

cargo fmt --all --check
QMAKE="${QMAKE:-/usr/bin/qmake6}" cargo clippy --workspace --all-targets --locked -- -D warnings
if command -v cargo-audit >/dev/null; then
    cargo audit
fi
qmllint crates/fluidvoice-ui/qml/Main.qml
find packaging -type f -name '*.sh' -exec bash -n {} +
# Keep localhost mock-provider integration tests deterministic. They are tiny,
# while serial execution avoids transient socket races on heavily loaded CI.
QMAKE="${QMAKE:-/usr/bin/qmake6}" dbus-run-session -- cargo test --workspace --locked -- --test-threads=1
QMAKE="${QMAKE:-/usr/bin/qmake6}" cargo build --release --locked -p fluidvoice-ui

if command -v appstreamcli >/dev/null; then
    # The repository remains private until the public launch. Validate the
    # metadata itself without treating intentionally unreachable project URLs
    # as a release-candidate failure.
    appstreamcli validate --no-net data/io.github.davidkodar.FluidVoiceLinux.metainfo.xml
fi
python3 -c 'import pathlib, yaml; yaml.safe_load(pathlib.Path("packaging/flatpak/io.github.davidkodar.FluidVoiceLinux.yml").read_text())'

DESTDIR="${smoke_root}/install" PREFIX=/usr ./packaging/install.sh
test -x "${smoke_root}/install/usr/bin/fluidvoice-ui"
test -f "${smoke_root}/install/usr/share/applications/io.github.davidkodar.FluidVoiceLinux.desktop"
test -f "${smoke_root}/install/usr/share/metainfo/io.github.davidkodar.FluidVoiceLinux.metainfo.xml"
test -f "${smoke_root}/install/usr/share/icons/hicolor/256x256/apps/io.github.davidkodar.FluidVoiceLinux.png"

./packaging/package-tarball.sh
version=$(awk -F '"' '/^version = "/ { print $2; exit }' Cargo.toml)
archive="target/package/fluidvoice-linux-${version}-$(uname -m).tar.gz"
test -s "${archive}"
archive_listing="${smoke_root}/archive-contents.txt"
tar -tzf "${archive}" > "${archive_listing}"
grep -q '/target/release/fluidvoice-ui$' "${archive_listing}"
sha256sum "${archive}"
echo "Release candidate validation passed."
