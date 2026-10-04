# FriesXLR

FriesXLR is an unofficial open-source Windows desktop controller for
TC-Helicon GoXLR and GoXLR Mini devices. It is early in development.

It is not affiliated with TC-Helicon or Music Tribe, and it does not imitate or
redistribute the proprietary GoXLR application.

Phase 1 establishes the foundation:

- Tauri 2 desktop shell with React, TypeScript, Vite, and Zustand.
- Rust backend crates for protocol facts, device discovery, service state, model
  types, and configuration.
- Read-only Windows device discovery through the official TC-Helicon driver API
  when installed.
- GoXLR vs GoXLR Mini model detection from USB product IDs.
- Mock GoXLR Mini support for development.
- Backend-owned authoritative application snapshot delivered to the UI over
  Tauri IPC/events.
- System tray, hide-on-close behavior, single-instance launch handling, and
  persisted desktop window state.
- Structured JSON file logging.
- Basic tests, lint/build scripts, documentation, and GitHub Actions.

Phase 1 does not send mixer-setting commands at startup. Real fader positions,
mute state, and assignments are exposed only by the mock provider until the
read-only hardware-state path has been verified against real devices.

## Requirements

- Windows 11 x64.
- Official TC-Helicon GoXLR Windows driver for real hardware discovery.
- Rust stable toolchain.
- Node.js 20+.
- Microsoft WebView2 Runtime.

## Development

```powershell
npm install
npm run desktop:dev
```

Useful checks:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run frontend:lint
npm run frontend:test
npm run frontend:build
npm run desktop:build
```

## Repository Layout

```text
apps/
  desktop/                 Tauri + React desktop app
crates/
  goxlr-device/            Hardware discovery boundary and mock provider
  goxlr-model/             Shared authoritative state model
  goxlr-profile/           App configuration storage
  goxlr-protocol/          Small protocol constants and model mapping
  goxlr-service/           Backend service loop and snapshot events
docs/
  architecture.md
  windows-development.md
```

## Attribution

GoXLR hardware knowledge was studied from the MIT-licensed GoXLR Utility
project:

https://github.com/GoXLR-on-Linux/goxlr-utility

See [NOTICE.md](NOTICE.md) for details.
