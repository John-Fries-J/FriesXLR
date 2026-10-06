import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
  AppSnapshot,
  ChannelName,
  CompressorState,
  DeEsserState,
  EqBandId,
  FaderName,
  MicrophoneType,
  NoiseGateState,
  RoutingRoute
} from "../types/backend";

export const SNAPSHOT_EVENT = "friesxlr://snapshot";

export function getSnapshot() {
  return invoke<AppSnapshot>("get_snapshot");
}

export function setMockDeviceEnabled(enabled: boolean) {
  return invoke<AppSnapshot>("set_mock_device_enabled", { enabled });
}

export function setStartMinimized(enabled: boolean) {
  return invoke<AppSnapshot>("set_start_minimized", { enabled });
}

export function setLaunchAtStartupEnabled(enabled: boolean) {
  return invoke<AppSnapshot>("set_launch_at_startup_enabled", { enabled });
}

export function setSelectedDevice(deviceId: string | null) {
  return invoke<AppSnapshot>("set_selected_device", { deviceId });
}

export function setFaderVolume(
  deviceId: string,
  sessionGeneration: number,
  fader: FaderName,
  percent: number
) {
  return invoke<AppSnapshot>("set_fader_volume", {
    deviceId,
    sessionGeneration,
    fader,
    percent
  });
}

export function setFaderMute(
  deviceId: string,
  sessionGeneration: number,
  fader: FaderName,
  muted: boolean
) {
  return invoke<AppSnapshot>("set_fader_mute", {
    deviceId,
    sessionGeneration,
    fader,
    muted
  });
}

export function setFaderAssignment(
  deviceId: string,
  sessionGeneration: number,
  fader: FaderName,
  channel: ChannelName
) {
  return invoke<AppSnapshot>("set_fader_assignment", {
    deviceId,
    sessionGeneration,
    fader,
    channel
  });
}

export function setRoutingRoute(
  deviceId: string,
  sessionGeneration: number,
  route: RoutingRoute,
  enabled: boolean
) {
  return invoke<AppSnapshot>("set_routing_route", {
    deviceId,
    sessionGeneration,
    route,
    enabled
  });
}

export function setMicrophoneType(
  deviceId: string,
  sessionGeneration: number,
  microphoneType: MicrophoneType,
  confirmPhantomPower: boolean
) {
  return invoke<AppSnapshot>("set_microphone_type", {
    deviceId,
    sessionGeneration,
    microphoneType,
    confirmPhantomPower
  });
}

export function setMicrophoneGain(
  deviceId: string,
  sessionGeneration: number,
  microphoneType: MicrophoneType,
  gainDb: number
) {
  return invoke<AppSnapshot>("set_microphone_gain", {
    deviceId,
    sessionGeneration,
    microphoneType,
    gainDb
  });
}

export function setEqualizerBand(
  deviceId: string,
  sessionGeneration: number,
  bandId: EqBandId,
  frequencyTenthsHz: number,
  gainDb: number
) {
  return invoke<AppSnapshot>("set_equalizer_band", {
    deviceId,
    sessionGeneration,
    bandId,
    frequencyTenthsHz,
    gainDb
  });
}

export function setNoiseGate(
  deviceId: string,
  sessionGeneration: number,
  gate: NoiseGateState
) {
  return invoke<AppSnapshot>("set_noise_gate", {
    deviceId,
    sessionGeneration,
    gate
  });
}

export function setCompressor(
  deviceId: string,
  sessionGeneration: number,
  compressor: CompressorState
) {
  return invoke<AppSnapshot>("set_compressor", {
    deviceId,
    sessionGeneration,
    compressor
  });
}

export function setDeEsser(
  deviceId: string,
  sessionGeneration: number,
  deEsser: DeEsserState
) {
  return invoke<AppSnapshot>("set_de_esser", {
    deviceId,
    sessionGeneration,
    deEsser
  });
}

export function quitApplication() {
  return invoke<void>("quit_application");
}

export function subscribeToSnapshots(onSnapshot: (snapshot: AppSnapshot) => void) {
  return listen<AppSnapshot>(SNAPSHOT_EVENT, (event) => onSnapshot(event.payload));
}
