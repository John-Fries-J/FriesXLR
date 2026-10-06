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
- Phase 3 routing and microphone processing:
  `usb/src/routing.rs`, `usb/src/commands.rs`, `usb/src/device/base.rs`,
  `types/src/lib.rs`, `ipc/src/lib.rs`, `ipc/src/device.rs`,
  `daemon/src/device.rs`, `daemon/src/mic_profile.rs`,
  `daemon/src/profile.rs`, `profile/src/mic_profile.rs`,
  `profile/src/microphone/mic_setup.rs`,
  `profile/src/microphone/equalizer.rs`,
  `profile/src/microphone/equalizer_mini.rs`,
  `profile/src/microphone/gate.rs`,
  `profile/src/microphone/compressor.rs`,
  `profile/src/components/mixer.rs`.
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

## Phase 2A Read-Only Mixer State

The Windows physical session opens a persistent TUSBAUDIO handle, activates the
vendor pipe, and reads serial number, firmware version, and the lightweight
`GetButtonStates` status. `GetButtonStates` is the only direct hardware status
query used during normal read-only mixer updates. It provides:

- four physical fader raw positions
- currently pressed button bits, including the four fader mute buttons
- encoder deltas, which FriesXLR currently ignores

When the installed TUSBAUDIO DLL exposes device notification functions,
FriesXLR registers for input-change notifications and only rereads
`GetButtonStates` after a button/fader interrupt. If those notification
functions are unavailable or fail, FriesXLR falls back to a 50 ms poll of
`GetButtonStates`. The fallback is intentionally limited to that small status
command; FriesXLR does not query full device/profile state every 50 ms.

GoXLR Utility models fader assignment, mute function, and latched fader mute
state from its loaded profile state. Its direct hardware input monitoring uses
`GetButtonStates` for physical fader positions and button down/up transitions.
FriesXLR therefore keeps direct physical fader assignment, mute function, and
latched mute state unknown until a verified profile/state source is available.
This avoids inventing defaults or issuing unverified protocol requests.

## State Flow

`AppService` periodically refreshes the configured provider set, produces a new
`AppSnapshot`, compares it with the current snapshot, and broadcasts a
`SnapshotChanged` event only when state changes.

The desktop app exposes:

- `get_snapshot`
- `set_mock_device_enabled`
- fader write commands
- routing write commands
- microphone setup, gain, EQ, gate, compressor, and de-esser write commands
- `friesxlr://snapshot` events

The frontend Zustand store is a cache of the backend snapshot, not a second
source of truth.

## Phase 3 Routing And Microphone State

`DeviceState` now keeps mixer faders, routing, and microphone processing as
separate nested domains. `DeviceCapabilities` carries endpoint, route,
microphone type, gain range, EQ band, gate, compressor, and de-esser
availability so the frontend does not branch on GoXLR vs GoXLR Mini directly.

The verified routing command is GoXLR Utility's `SetRouting` command:

- command id: `(0x804 << 12) | stereo_input_id`
- stereo input ids: mic `0x02/0x03`, line in `0x04/0x05`, console
  `0x06/0x07`, system `0x08/0x09`, game `0x0a/0x0b`, chat `0x0c/0x0d`,
  music `0x0e/0x0f`, sample `0x10/0x11`
- enabled route byte: `0x20` at the destination channel position
- base payload length: 22 bytes; Mix 2 capable payload length: 26 bytes
- FriesXLR models the upstream-invalid Chat -> Chat Mic route as unsupported

The verified microphone setup command is GoXLR Utility's
`SetMicrophoneParameters` command (`0x80b << 12`). It sends repeated
little-endian parameter id + four-byte value records. Mic gain is represented in
hardware dB and encoded in the upper two bytes of the four-byte value. The
verified mic type parameter is `MicType`; GoXLR Utility sends `1` for condenser
and `0` otherwise. FriesXLR therefore requires explicit condenser/phantom
confirmation before selecting condenser mode.

GoXLR Mini EQ is written through microphone parameters. Full-size GoXLR EQ,
gate, compressor, and de-esser values are written through the verified
`SetEffectParameters` command (`0x801 << 12`), while shared gate/compressor
values also have microphone-parameter encodings used by GoXLR Utility. The
conversion helpers live in `crates/goxlr-protocol`; React receives meaningful
units from the model and does not convert raw protocol payloads itself.

FriesXLR still does not have a verified active profile source for physical
devices. Because upstream derives routing, fader assignment, fader mute
configuration, and most microphone processing state from the active profile and
mic profile, physical Phase 3 controls remain capability-disabled until that
state can be read safely. The mock provider exposes writable routing and
microphone processing through the same service/session path used by real
hardware.

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
