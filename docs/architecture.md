# Architecture

FriesXLR is split into four layers:

1. Hardware/device layer: `crates/goxlr-device`
2. Application/service layer: `crates/goxlr-service`
3. Desktop IPC layer: `apps/desktop/src-tauri`
4. UI layer: `apps/desktop/src`

The UI never talks to USB hardware. It invokes Tauri commands and subscribes to
backend events. The service owns the authoritative `AppSnapshot`.

## Upstream Study

The following GoXLR Utility areas were studied before Phase 1 implementation:

- Workspace layout: `usb`, `daemon`, `ipc`, `types`, `profile`, `client`.
- USB constants and product IDs: `usb/src/lib.rs`.
- Command framing, command index reset, and response matching:
  `usb/src/commands.rs`, `usb/src/device/base.rs`.
- Linux/macOS libusb transport: `usb/src/device/libusb/device.rs`.
- Windows TUSBAUDIO driver transport and device notifications:
  `usb/src/device/tusb/device.rs`, `usb/src/device/tusb/tusbaudio.rs`.
- Device detection, load, serial/firmware read, and state status:
  `daemon/src/primary_worker.rs`, `daemon/src/device.rs`.
- IPC state and `DaemonStatus`: `ipc/src/lib.rs`, `ipc/src/device.rs`.
- Buttons, fader volumes, and hardware-originated events:
  `usb/src/buttonstate.rs`, `daemon/src/device.rs`.
- Routing and GoXLR Mini differences:
  `usb/src/routing.rs`, `types/src/lib.rs`, `daemon/src/device.rs`.
- Windows tray/runtime behavior: `daemon/src/tray/windows.rs`,
  `daemon/src/platform/windows.rs`.

## Phase 1 Hardware Safety

Real hardware discovery uses only driver enumeration and public device
properties. FriesXLR does not currently send GoXLR vendor control requests to
real devices.

The constants used for identifying devices are:

- Vendor ID: `0x1220`
- GoXLR product ID: `0x8fe0`
- GoXLR Mini product ID: `0x8fe4`

Those constants and the Windows TUSBAUDIO discovery function names are adapted
from GoXLR Utility under MIT attribution.

## State Flow

`AppService` periodically refreshes the configured provider set, produces a new
`AppSnapshot`, compares it with the current snapshot, and broadcasts a
`SnapshotChanged` event only when state changes.

The desktop app exposes:

- `get_snapshot`
- `set_mock_device_enabled`
- `friesxlr://snapshot` events

The frontend Zustand store is a cache of the backend snapshot, not a second
source of truth.

## Long-Term Service Direction

Phase 1 keeps the service in the Tauri backend process. The crate boundaries
allow the service to move later to a standalone process with local IPC while
preserving the same `goxlr-service` state model.

Planned extensions:

- Event-driven Windows device reconnect notifications.
- Verified read-only hardware state for faders, assignments, and mutes.
- Command queue with reconciliation against returned hardware state.
- Multi-device profile binding.
- Native local IPC for any future service split, without requiring a browser,
  web server, cloud service, or external frontend hosting.
- Crash recovery and service restart handling.
