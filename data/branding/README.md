# Spevox branding

The Spevox icon and wordmark in this directory are original project artwork,
introduced with Spevox 0.6.0. They depict a speech waveform flowing into an
abstract “S” and two written-text lines.

Copyright © 2026 David Bolin. Distributed with Spevox under GPL-3.0-only.

- `spevox-icon.svg` is the flat application-icon master.
- `spevox-symbolic.svg` is the transparent monochrome tray-icon master.
- `spevox-logo.svg` is the horizontal wordmark master.
- The PNG files are reproducible raster exports for desktop metadata, QML, and
  documentation.

The runtime-sized application and tray assets are generated from the master
icon and live under `crates/spevox-ui/assets/`; the freedesktop 512 px copy is
installed from `data/icons/hicolor/`.
