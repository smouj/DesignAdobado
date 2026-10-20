---
title: DesignAdobado closes right after it starts
description: Steps for investigating a Windows startup close in a local DesignAdobado build; current release behavior is still being validated.
order: 10
---

> **Current status:** There is no tested DesignAdobado package yet. The driver notes below are
> inherited from the source project and have not been verified in a published Adobado build.

If a local DesignAdobado build flashes a spinner and closes, note your Windows version and graphics
card and report it in the [DesignAdobado issue tracker](https://github.com/smouj/DesignAdobado/issues).
The DirectX 12 option below is a diagnostic for local builds, not a confirmed release fix.

## Symptoms

- On Windows, DesignAdobado opens briefly with a spinner or a quick blue circle, then closes. No error message appears.
- Windows may record a crash that names a graphics driver file such as `atio6axx.dll` (AMD) or `igvk64.dll` (Intel).
- It happens with the installer and the portable version.

## Fix

1. If you built the app locally, start it again once and note whether the behavior changes.
2. If it still closes, you can test a local Windows build with the environment variable `WGPU_BACKEND` set to `dx12`. Start the app from a shell where it is set. Report whether this changes the result in the [issue tracker](https://github.com/smouj/DesignAdobado/issues).
3. Start DesignAdobado again.

Setting `WGPU_BACKEND` also turns off the automatic fallback. Other accepted values are `vulkan` and `gl`, and a comma-separated list lets the graphics library choose among them. Do not pick `vulkan` on Intel graphics or `gl` on AMD graphics: those are the drivers in the crash reports.

## Why this happens

DesignAdobado draws the canvas on the CPU and uses the graphics card only to show the interface. Some graphics drivers crash while the window is being set up, and a crash in a driver ends the program before DesignAdobado can react. One AMD laptop's Vulkan driver crashed on the first frame, which is why DirectX 12 is the default on Windows.

## Start the backend search over

DesignAdobado keeps its record of failed backends in a file called `gpu.json` in its settings folder, the one that holds `ui.json`, `prefs.json` and the `logs` folder (`%APPDATA%\DesignAdobado\logs\` is the log folder on Windows). Delete the file to start the search again. The list also starts over after every backend has failed, and with a new DesignAdobado version.

## Still stuck?

Send the log. See [Report a problem](get-help.md) for where it is and how to include it in an issue.
