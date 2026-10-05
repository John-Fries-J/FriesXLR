import { create } from "zustand";
import {
  getSnapshot,
  quitApplication as quitApplicationCommand,
  setFaderAssignment as setFaderAssignmentCommand,
  setFaderMute as setFaderMuteCommand,
  setFaderVolume as setFaderVolumeCommand,
  setLaunchAtStartupEnabled as setLaunchAtStartupEnabledCommand,
  setMockDeviceEnabled as setMockDeviceEnabledCommand,
  setSelectedDevice as setSelectedDeviceCommand,
  setStartMinimized as setStartMinimizedCommand
} from "../services/backend";
import type { AppSnapshot, ChannelName, FaderName } from "../types/backend";

interface DeviceStore {
  snapshot: AppSnapshot | null;
  loading: boolean;
  error: string | null;
  hydrate: () => Promise<void>;
  applySnapshot: (snapshot: AppSnapshot) => void;
  setSelectedDevice: (deviceId: string | null) => Promise<void>;
  setMockDeviceEnabled: (enabled: boolean) => Promise<void>;
  setStartMinimized: (enabled: boolean) => Promise<void>;
  setLaunchAtStartupEnabled: (enabled: boolean) => Promise<void>;
  setFaderVolume: (
    deviceId: string,
    sessionGeneration: number,
    fader: FaderName,
    percent: number
  ) => Promise<void>;
  setFaderMute: (
    deviceId: string,
    sessionGeneration: number,
    fader: FaderName,
    muted: boolean
  ) => Promise<void>;
  setFaderAssignment: (
    deviceId: string,
    sessionGeneration: number,
    fader: FaderName,
    channel: ChannelName
  ) => Promise<void>;
  quitApplication: () => Promise<void>;
}

export const useDeviceStore = create<DeviceStore>((set) => ({
  snapshot: null,
  loading: true,
  error: null,
  hydrate: async () => {
    set({ loading: true, error: null });
    try {
      const snapshot = await getSnapshot();
      set({ snapshot, loading: false });
    } catch (error) {
      set({ error: errorMessage(error), loading: false });
    }
  },
  applySnapshot: (snapshot) => set({ snapshot, loading: false, error: null }),
  setSelectedDevice: async (deviceId) => {
    set({ error: null });
    try {
      const snapshot = await setSelectedDeviceCommand(deviceId);
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setMockDeviceEnabled: async (enabled) => {
    set({ error: null });
    try {
      const snapshot = await setMockDeviceEnabledCommand(enabled);
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setStartMinimized: async (enabled) => {
    set({ error: null });
    try {
      const snapshot = await setStartMinimizedCommand(enabled);
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setLaunchAtStartupEnabled: async (enabled) => {
    set({ error: null });
    try {
      const snapshot = await setLaunchAtStartupEnabledCommand(enabled);
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setFaderVolume: async (deviceId, sessionGeneration, fader, percent) => {
    set({ error: null });
    try {
      const snapshot = await setFaderVolumeCommand(
        deviceId,
        sessionGeneration,
        fader,
        percent
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setFaderMute: async (deviceId, sessionGeneration, fader, muted) => {
    set({ error: null });
    try {
      const snapshot = await setFaderMuteCommand(
        deviceId,
        sessionGeneration,
        fader,
        muted
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setFaderAssignment: async (deviceId, sessionGeneration, fader, channel) => {
    set({ error: null });
    try {
      const snapshot = await setFaderAssignmentCommand(
        deviceId,
        sessionGeneration,
        fader,
        channel
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  quitApplication: async () => {
    set({ error: null });
    try {
      await quitApplicationCommand();
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  }
}));

function errorMessage(error: unknown) {
  if (typeof error === "string") {
    return error;
  }

  if (error && typeof error === "object" && "message" in error) {
    return String(error.message);
  }

  return "Backend request failed";
}
