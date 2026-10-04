import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AppSnapshot } from "../types/backend";

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

export function quitApplication() {
  return invoke<void>("quit_application");
}

export function subscribeToSnapshots(onSnapshot: (snapshot: AppSnapshot) => void) {
  return listen<AppSnapshot>(SNAPSHOT_EVENT, (event) => onSnapshot(event.payload));
}
