import type {
  AppSnapshot,
  CompressorState,
  DeviceCapabilities,
  DeviceState,
  MicrophoneState,
  NoiseGateState,
  RoutingState
} from "../types/backend";

export function phase3Capabilities(writable = true): DeviceCapabilities {
  return {
    readableFaderAssignments: true,
    readableFaderVolumes: true,
    readableFaderMuteState: true,
    readableFaderButtonState: true,
    writableFaderVolumes: writable,
    writableFaderMuteState: writable,
    writableFaderAssignments: writable,
    supportedAssignmentChannels: ["mic", "chat", "music", "system"],
    readableRouting: writable,
    writableRouting: writable,
    supportedRoutingInputs: [
      "microphone",
      "chat",
      "music",
      "game",
      "console",
      "lineIn",
      "system",
      "sample"
    ],
    supportedRoutingOutputs: ["headphones", "broadcastMix", "chatMic", "sampler", "lineOut"],
    supportedRoutes: [
      { input: "microphone", output: "headphones" },
      { input: "microphone", output: "broadcastMix" },
      { input: "microphone", output: "chatMic" },
      { input: "chat", output: "headphones" },
      { input: "music", output: "headphones" },
      { input: "music", output: "broadcastMix" }
    ],
    readableMicrophone: writable,
    writableMicrophone: writable,
    supportedMicrophoneTypes: ["dynamic", "condenser", "jack"],
    microphoneGainRangeDb: { min: 0, max: 72 },
    phantomPowerSupported: true,
    eqBands: [
      {
        id: "miniEq90Hz",
        label: "90 Hz",
        defaultFrequencyTenthsHz: 900,
        minFrequencyTenthsHz: 300,
        maxFrequencyTenthsHz: 900,
        frequencyStepTenthsHz: 10,
        minGainDb: -9,
        maxGainDb: 9,
        configurableFrequency: true,
        configurableWidth: false
      },
      {
        id: "miniEq250Hz",
        label: "250 Hz",
        defaultFrequencyTenthsHz: 1600,
        minFrequencyTenthsHz: 1000,
        maxFrequencyTenthsHz: 3000,
        frequencyStepTenthsHz: 10,
        minGainDb: -9,
        maxGainDb: 9,
        configurableFrequency: true,
        configurableWidth: false
      }
    ],
    gateControls: ["enabled", "threshold", "attenuation", "attack", "release"],
    gateThresholdRangeDb: { min: -59, max: 0 },
    gateAttenuationRangePercent: { min: 0, max: 100 },
    gateTimeOptions: [
      { index: 0, millis: 10 },
      { index: 19, millis: 200 }
    ],
    compressorControls: ["threshold", "ratio", "attack", "release", "makeupGain"],
    compressorThresholdRangeDb: { min: -40, max: 0 },
    compressorMakeupGainRangeDb: { min: -6, max: 24 },
    compressorRatioOptions: [
      { index: 0, ratioTenths: 10 },
      { index: 9, ratioTenths: 40 }
    ],
    compressorAttackOptions: [
      { index: 0, millis: 0 },
      { index: 1, millis: 2 }
    ],
    compressorReleaseOptions: [
      { index: 0, millis: 0 },
      { index: 9, millis: 100 }
    ],
    deEsserSupported: writable,
    deEsserRangePercent: { min: 0, max: 100 }
  };
}

export function routingState(): RoutingState {
  return {
    routes: [
      { route: { input: "microphone", output: "headphones" }, enabled: true },
      { route: { input: "microphone", output: "broadcastMix" }, enabled: true },
      { route: { input: "microphone", output: "chatMic" }, enabled: true },
      { route: { input: "chat", output: "headphones" }, enabled: true },
      { route: { input: "music", output: "headphones" }, enabled: true },
      { route: { input: "music", output: "broadcastMix" }, enabled: false }
    ]
  };
}

export function noiseGate(): NoiseGateState {
  return {
    enabled: false,
    thresholdDb: -30,
    attenuationPercent: 100,
    attack: { index: 0, millis: 10 },
    release: { index: 19, millis: 200 }
  };
}

export function compressor(): CompressorState {
  return {
    thresholdDb: -10,
    ratio: { index: 9, ratioTenths: 40 },
    attack: { index: 1, millis: 2 },
    release: { index: 9, millis: 100 },
    makeupGainDb: 0
  };
}

export function microphoneState(): MicrophoneState {
  return {
    setup: {
      microphoneType: "dynamic",
      gains: [
        { microphoneType: "dynamic", hardwareDb: 35 },
        { microphoneType: "condenser", hardwareDb: 30 },
        { microphoneType: "jack", hardwareDb: 20 }
      ],
      phantomPowerEnabled: false
    },
    equalizer: {
      bands: [
        { id: "miniEq90Hz", frequencyTenthsHz: 900, gainDb: 0 },
        { id: "miniEq250Hz", frequencyTenthsHz: 1600, gainDb: 3 }
      ]
    },
    gate: noiseGate(),
    compressor: compressor(),
    deEsser: { amountPercent: 25 }
  };
}

export function device(
  id = "mock:1",
  capabilities: DeviceCapabilities = phase3Capabilities()
): DeviceState {
  return {
    identity: {
      id,
      model: "goXlrMini",
      vendorId: null,
      productId: null,
      manufacturerName: "FriesXLR",
      productName: "Mock GoXLR Mini",
      serialNumber: "MOCK-1",
      firmwareVersion: null,
      driverInterface: "mock",
      driverVersion: null,
      isMock: true
    },
    status: "connected",
    capabilities,
    routing: routingState(),
    microphone: microphoneState(),
    sessionGeneration: 1,
    lastSeenEpochMs: 1,
    faders: [
      {
        name: "A",
        assignedChannel: "mic",
        volume: { raw: 128, percent: 50 },
        muteState: "unmuted",
        muted: false,
        muteButtonPressed: false
      },
      {
        name: "B",
        assignedChannel: "chat",
        volume: { raw: 64, percent: 25 },
        muteState: "unmuted",
        muted: false,
        muteButtonPressed: false
      },
      {
        name: "C",
        assignedChannel: "music",
        volume: { raw: 200, percent: 78 },
        muteState: "mutedToAll",
        muted: true,
        muteButtonPressed: true
      },
      {
        name: "D",
        assignedChannel: "system",
        volume: { raw: 255, percent: 100 },
        muteState: "unmuted",
        muted: false,
        muteButtonPressed: false
      }
    ]
  };
}

export function snapshot(
  primary: DeviceState = device(),
  devices: DeviceState[] = [primary],
  selectedDeviceId = primary.identity.id
): AppSnapshot {
  return {
    settings: {
      mockDeviceEnabled: true,
      showTrayIcon: true,
      startMinimized: false,
      launchAtStartup: false,
      logLevel: "info",
      configPath: null
    },
    service: {
      running: true,
      lastError: null
    },
    devices,
    selectedDeviceId
  };
}
