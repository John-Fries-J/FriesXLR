import { create } from "zustand";
import {
  getSnapshot,
  quitApplication as quitApplicationCommand,
  setLaunchAtStartupEnabled as setLaunchAtStartupEnabledCommand,
  setMockDeviceEnabled as setMockDeviceEnabledCommand,
  setStartMinimized as setStartMinimizedCommand
} from "../services/backend";
import type { AppSnapshot } from "../types/backend";

interface DeviceStore {
  snapshot: AppSnapshot | null;
  loading: boolean;
  error: string | null;
  hydrate: () => Promise<void>;
  applySnapshot: (snapshot: AppSnapshot) => void;
  setMockDeviceEnabled: (enabled: boolean) => Promise<void>;
  setStartMinimized: (enabled: boolean) => Promise<void>;
  setLaunchAtStartupEnabled: (enabled: boolean) => Promise<void>;
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
