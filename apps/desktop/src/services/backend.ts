import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AppSnapshot, ChannelName, FaderName } from "../types/backend";

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

export function quitApplication() {
  return invoke<void>("quit_application");
}

export function subscribeToSnapshots(onSnapshot: (snapshot: AppSnapshot) => void) {
  return listen<AppSnapshot>(SNAPSHOT_EVENT, (event) => onSnapshot(event.payload));
}
