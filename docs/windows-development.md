# Windows Development

FriesXLR targets Windows 11 x64 first.

## Install Prerequisites

Install:

- Rust stable: https://rustup.rs
- Node.js 20 or newer: https://nodejs.org
- Microsoft Visual Studio Build Tools with the C++ desktop workload.
- WebView2 Runtime.
- Official TC-Helicon GoXLR Windows driver for real hardware discovery.

Restart the terminal after installing Rust or Visual Studio Build Tools.

## Build

```powershell
npm install
cargo test --workspace
npm run frontend:test
npm run frontend:build
npm run desktop:build
```

## Run

```powershell
npm run desktop:dev
```

If no GoXLR is attached, the app opens with an empty device state. Enable the
mock device in Settings to exercise the UI without hardware.

The development command uses Vite only as Tauri's dev-time asset server.
Production builds use bundled frontend assets from `apps/desktop/dist` and do
not require a browser, localhost page, internet access, or external hosting.

The Windows desktop shell owns tray operation, hide-on-close behavior,
single-instance foreground restore, launch-at-startup registration, and saved
window size, position, and maximized state.

## Logs

Logs are structured JSON files written under the FriesXLR application config
directory:

```text
%APPDATA%\FriesXLR\FriesXLR\config\logs
```

Set `RUST_LOG` before launching to increase verbosity:

```powershell
$env:RUST_LOG = "debug"
npm run desktop:dev
```

## Hardware Notes

Phase 1 uses read-only discovery. It does not perform firmware updates and does
not change mixer settings at startup.

Real hardware discovery requires the official driver API DLL, usually installed
at:

```text
C:\Program Files\TC-HELICON\GoXLR_Audio_Driver\W10_x64\goxlr_audioapi_x64.dll
```

The app first checks the driver CLSID registry entry used by the official driver
and falls back to that path if the registry lookup fails.
