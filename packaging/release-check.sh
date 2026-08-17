#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
smoke_root="$(mktemp -d -t spevox-release-check.XXXXXX)"
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
if command -v cargo-deny >/dev/null; then
    cargo deny check advisories licenses sources
fi
if cargo about --version >/dev/null 2>&1; then
    cargo about generate about.hbs --workspace --locked --fail --output-file "${smoke_root}/THIRD_PARTY_LICENSES.html"
    cmp THIRD_PARTY_LICENSES.html "${smoke_root}/THIRD_PARTY_LICENSES.html"
fi
qmllint crates/spevox-ui/qml/Main.qml
find packaging -type f -name '*.sh' -exec bash -n {} +
# Keep localhost mock-provider integration tests deterministic. They are tiny,
# while serial execution avoids transient socket races on heavily loaded CI.
QMAKE="${QMAKE:-/usr/bin/qmake6}" dbus-run-session -- cargo test --workspace --locked -- --test-threads=1
QMAKE="${QMAKE:-/usr/bin/qmake6}" cargo build --release --locked -p spevox-ui

if command -v appstreamcli >/dev/null; then
    # The repository remains private until the public launch. Validate the
    # metadata itself without treating intentionally unreachable project URLs
    # as a release-candidate failure.
    appstreamcli validate --no-net data/io.github.davidkodar.Spevox.metainfo.xml
fi
python3 -c 'import pathlib, yaml; yaml.safe_load(pathlib.Path("packaging/flatpak/io.github.davidkodar.Spevox.yml").read_text())'

DESTDIR="${smoke_root}/install" PREFIX=/usr ./packaging/install.sh
test -x "${smoke_root}/install/usr/bin/spevox"
test -f "${smoke_root}/install/usr/share/applications/io.github.davidkodar.Spevox.desktop"
test -f "${smoke_root}/install/usr/share/metainfo/io.github.davidkodar.Spevox.metainfo.xml"
test -f "${smoke_root}/install/usr/share/icons/hicolor/256x256/apps/io.github.davidkodar.Spevox.png"
test -f "${smoke_root}/install/usr/share/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png"
test -f "${smoke_root}/install/usr/share/doc/spevox/README.md"
test -f "${smoke_root}/install/usr/share/doc/spevox/CREDITS.md"
test -f "${smoke_root}/install/usr/share/doc/spevox/THIRD_PARTY_NOTICES.md"
test -f "${smoke_root}/install/usr/share/doc/spevox/THIRD_PARTY_LICENSES.html"

./packaging/package-tarball.sh
version=$(awk -F '"' '/^version = "/ { print $2; exit }' Cargo.toml)
archive="target/package/spevox-${version}-$(uname -m).tar.gz"
test -s "${archive}"
archive_listing="${smoke_root}/archive-contents.txt"
tar -tzf "${archive}" > "${archive_listing}"
grep -q '/target/release/spevox$' "${archive_listing}"
grep -q '/data/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png$' "${archive_listing}"
grep -q '/README.md$' "${archive_listing}"
grep -q '/CREDITS.md$' "${archive_listing}"
grep -q '/THIRD_PARTY_NOTICES.md$' "${archive_listing}"
grep -q '/THIRD_PARTY_LICENSES.html$' "${archive_listing}"
archive_install="${smoke_root}/archive-install"
archive_extract="${smoke_root}/archive-extract"
mkdir -p "${archive_extract}"
tar -xzf "${archive}" -C "${archive_extract}"
extracted_root="${archive_extract}/spevox-${version}"
test -x "${extracted_root}/packaging/install.sh"
DESTDIR="${archive_install}" PREFIX=/usr "${extracted_root}/packaging/install.sh"
test -x "${archive_install}/usr/bin/spevox"
test -f "${archive_install}/usr/share/icons/hicolor/512x512/apps/io.github.davidkodar.Spevox.png"
test -f "${archive_install}/usr/share/doc/spevox/CREDITS.md"
test -f "${archive_install}/usr/share/doc/spevox/THIRD_PARTY_NOTICES.md"
test -f "${archive_install}/usr/share/doc/spevox/THIRD_PARTY_LICENSES.html"
(cd "$(dirname "${archive}")" && sha256sum --check "$(basename "${archive}").sha256")
if strings "${extracted_root}/target/release/spevox" | grep -Fq -- "${repo_root}"; then
    echo "release binary contains the local repository path" >&2
    exit 1
fi
echo "Release candidate validation passed."
