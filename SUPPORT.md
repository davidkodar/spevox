# Support

FluidVoice Linux is an unofficial Linux port in public prerelease preview. Support is
best-effort and currently targets KDE Plasma on Wayland.

## Where to ask

- Use the bug-report template for reproducible application defects.
- Use the feature-request template for focused Linux improvements.
- Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).
- Ask the upstream FluidVoice project about its macOS application, subscriptions,
  services, and proprietary components.

Before reporting a bug, test the newest prerelease, review the README setup and
diagnostic guidance, and search existing issues. Include the FluidVoice version,
Linux distribution, Plasma and KWin versions, selected speech engine, compute
backend, and sanitized logs needed to reproduce the problem.

Never post API keys, full transcripts, retained recordings, model-provider
tokens, personal paths, or other sensitive data. Replace private dictated text
with a minimal synthetic example that demonstrates the same behavior.

## Current boundaries

The supported installation path is the package attached to the newest GitHub
prerelease or a local build using its matching `PKGBUILD`. Flatpak remains a
developer preview. Hardware-specific speech runtimes, third-party AI services,
and external model servers may require their own vendor support when the failure
cannot be reproduced in FluidVoice itself.
