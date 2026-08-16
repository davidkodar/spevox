#!/usr/bin/env bash
set -euo pipefail

version=${1:?usage: make-update-manifest.sh VERSION ARCHIVE SHA256 OUTPUT}
archive=${2:?archive name required}
sha256=${3:?sha256 required}
output=${4:?output path required}
channel=stable
[[ "$version" == *-* ]] && channel=beta

cat > "$output" <<EOF
{
  "schema": 1,
  "channel": "$channel",
  "version": "${version#v}",
  "published_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "artifact": "$archive",
  "sha256": "$sha256",
  "release_url": "https://github.com/davidkodar/fluidvoice-linux/releases/tag/$version"
}
EOF
