/*
 * SPDX-FileCopyrightText: 2026 David Bolin
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

const service = "io.github.davidkodar.FluidVoiceLinux.Profiles";
const path = "/Profiles";
const iface = "io.github.davidkodar.FluidVoiceLinux.Profiles";

function reportActiveApplication(window) {
    if (!window || !window.normalWindow)
        return;
    const resourceClass = String(window.resourceClass || "").slice(0, 256);
    const title = String(window.caption || "").slice(0, 512);
    if (resourceClass.length === 0)
        return;
    callDBus(service, path, iface, "ActiveApplication", resourceClass, title);
}

workspace.windowActivated.connect(reportActiveApplication);
reportActiveApplication(workspace.activeWindow);
