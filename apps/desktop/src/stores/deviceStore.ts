import { create } from "zustand";
import {
  getSnapshot,
  quitApplication as quitApplicationCommand,
  setCompressor as setCompressorCommand,
  setDeEsser as setDeEsserCommand,
  setEqualizerBand as setEqualizerBandCommand,
  setFaderAssignment as setFaderAssignmentCommand,
  setFaderMute as setFaderMuteCommand,
  setFaderVolume as setFaderVolumeCommand,
  setLaunchAtStartupEnabled as setLaunchAtStartupEnabledCommand,
  setMicrophoneGain as setMicrophoneGainCommand,
  setMicrophoneType as setMicrophoneTypeCommand,
  setMockDeviceEnabled as setMockDeviceEnabledCommand,
  setNoiseGate as setNoiseGateCommand,
  setRoutingRoute as setRoutingRouteCommand,
  setSelectedDevice as setSelectedDeviceCommand,
  setStartMinimized as setStartMinimizedCommand
} from "../services/backend";
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
  setRoutingRoute: (
    deviceId: string,
    sessionGeneration: number,
    route: RoutingRoute,
    enabled: boolean
  ) => Promise<void>;
  setMicrophoneType: (
    deviceId: string,
    sessionGeneration: number,
    microphoneType: MicrophoneType,
    confirmPhantomPower: boolean
  ) => Promise<void>;
  setMicrophoneGain: (
    deviceId: string,
    sessionGeneration: number,
    microphoneType: MicrophoneType,
    gainDb: number
  ) => Promise<void>;
  setEqualizerBand: (
    deviceId: string,
    sessionGeneration: number,
    bandId: EqBandId,
    frequencyTenthsHz: number,
    gainDb: number
  ) => Promise<void>;
  setNoiseGate: (
    deviceId: string,
    sessionGeneration: number,
    gate: NoiseGateState
  ) => Promise<void>;
  setCompressor: (
    deviceId: string,
    sessionGeneration: number,
    compressor: CompressorState
  ) => Promise<void>;
  setDeEsser: (
    deviceId: string,
    sessionGeneration: number,
    deEsser: DeEsserState
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
  setRoutingRoute: async (deviceId, sessionGeneration, route, enabled) => {
    set({ error: null });
    try {
      const snapshot = await setRoutingRouteCommand(
        deviceId,
        sessionGeneration,
        route,
        enabled
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setMicrophoneType: async (
    deviceId,
    sessionGeneration,
    microphoneType,
    confirmPhantomPower
  ) => {
    set({ error: null });
    try {
      const snapshot = await setMicrophoneTypeCommand(
        deviceId,
        sessionGeneration,
        microphoneType,
        confirmPhantomPower
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setMicrophoneGain: async (deviceId, sessionGeneration, microphoneType, gainDb) => {
    set({ error: null });
    try {
      const snapshot = await setMicrophoneGainCommand(
        deviceId,
        sessionGeneration,
        microphoneType,
        gainDb
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setEqualizerBand: async (
    deviceId,
    sessionGeneration,
    bandId,
    frequencyTenthsHz,
    gainDb
  ) => {
    set({ error: null });
    try {
      const snapshot = await setEqualizerBandCommand(
        deviceId,
        sessionGeneration,
        bandId,
        frequencyTenthsHz,
        gainDb
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setNoiseGate: async (deviceId, sessionGeneration, gate) => {
    set({ error: null });
    try {
      const snapshot = await setNoiseGateCommand(deviceId, sessionGeneration, gate);
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setCompressor: async (deviceId, sessionGeneration, compressor) => {
    set({ error: null });
    try {
      const snapshot = await setCompressorCommand(
        deviceId,
        sessionGeneration,
        compressor
      );
      set({ snapshot });
    } catch (error) {
      set({ error: errorMessage(error) });
    }
  },
  setDeEsser: async (deviceId, sessionGeneration, deEsser) => {
    set({ error: null });
    try {
      const snapshot = await setDeEsserCommand(deviceId, sessionGeneration, deEsser);
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
