# Phase 3 Manual Hardware Checklist

These checks require real GoXLR or GoXLR Mini hardware. Do not mark them passed
from CI, mock devices, or code inspection alone.

## Routing

- Compare every routing path shown in FriesXLR with the original GoXLR app or
  the known current device/profile state.
- Toggle one routing path at a time and confirm the backend refreshes to the
  hardware/profile-authoritative state.
- Verify headphones routing.
- Verify broadcast mix routing.
- Verify chat mic routing.
- Verify line out routing where the selected device exposes line out.
- Verify sampler routing.
- Confirm unsupported routes, including Chat -> Chat Mic, cannot be enabled.

## Microphone Setup

- Read the current microphone type before changing controls.
- Change microphone type to Dynamic and verify it does not enable phantom power.
- Change microphone type to 3.5mm and verify it does not enable phantom power.
- Change microphone type to Condenser only with an appropriate test microphone
  or safe load, and verify phantom-power behavior deliberately.
- Confirm changing microphone type does not overwrite unrelated gain values.
- Change microphone gain in small increments and compare against the original
  app/current device state.

## Processing

- Compare EQ band frequency and gain values with the original app/current
  device state.
- Make a small EQ gain change and confirm it applies.
- Make a small EQ frequency change and confirm it applies.
- Test gate threshold, attenuation, attack, release, and enabled state.
- Test compressor threshold, ratio, attack, release, and makeup gain.
- Test de-esser amount if supported by the selected model.

## Lifecycle

- Restart FriesXLR and verify routing and microphone state persisted on the
  device/profile and is read back before controls become active.
- Unplug and reconnect the device, then verify state is refreshed and stale
  session commands are rejected.
- Hide to tray, reopen, and verify state is unchanged.
- Drag gain/EQ/dynamics controls rapidly and verify the final selected value is
  sent and reconciled without command flooding.
