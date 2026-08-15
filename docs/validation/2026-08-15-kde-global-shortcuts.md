# KDE Global Shortcuts validation

- Date: 2026-08-15
- Session: KDE Plasma Wayland
- KWin: 6.7.4
- Portal interface: `org.freedesktop.portal.GlobalShortcuts`, version 2
- Application ID: `io.github.davidkodar.FluidVoiceLinux`
- Bound shortcut: `dictate_hold` → `Ctrl+Alt+D`

## Result

The host application successfully registered its app ID, created a portal session, displayed Plasma's binding UI, and received separate activation and deactivation signals.

Seven consecutive physical press/release cycles produced seven correctly ordered pairs:

```text
Activated { id: "dictate_hold", timestamp: 0ns }
Deactivated { id: "dictate_hold", timestamp: 0ns }
```

## Finding

KDE's portal emitted zero for the signal timestamp in this environment. Hold-to-talk state must therefore use signal ordering and a process-local monotonic clock for duration calculations. Portal timestamps may be retained as diagnostic metadata but must not be required for correctness.

The development binary also required a valid reverse-DNS application ID backed by an installed desktop entry before the host portal registry would accept it. Release and developer packaging must install `data/io.github.davidkodar.FluidVoiceLinux.desktop` before portal registration.

