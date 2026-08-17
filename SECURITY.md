# Security policy

FluidVoice Linux processes microphone audio, dictated text, optional AI-provider
credentials, and locally retained history. Security and privacy reports are
therefore handled privately until a fix is available.

## Supported versions

The project is currently a public prerelease preview. Only the newest published prerelease
and the current `main` branch receive security fixes. Older prereleases should
be upgraded rather than patched independently.

## Reporting a vulnerability

Use GitHub's **Security → Advisories → New draft security advisory** for this
repository. Do not open a normal issue containing exploit details, credentials,
recordings, transcripts, or other private data.

Include the affected version or commit, impact, reproduction steps, relevant
environment details, and a minimal sanitized proof of concept when possible.
Please allow time to reproduce and correct the issue before public disclosure.

Reports about FluidVoice for macOS belong to the upstream project and should not
be submitted here unless the same defect is demonstrably present in this Linux
port.

## Scope expectations

Useful reports include unintended network transmission, unsafe local API
exposure, credential disclosure, command or path injection, insecure package or
model verification, privilege-boundary errors, and access to another user's
private application data. General support questions and model-quality concerns
without a security impact belong in the normal issue tracker.
