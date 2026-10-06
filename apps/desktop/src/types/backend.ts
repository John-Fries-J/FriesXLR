export type DeviceModel = "goXlr" | "goXlrMini" | "unknown";
export type ConnectionStatus = "connected" | "disconnected";
export type LogLevel = "error" | "warn" | "info" | "debug" | "trace";
export type FaderName = "A" | "B" | "C" | "D";
export type RoutingInput =
  | "microphone"
  | "chat"
  | "music"
  | "game"
  | "console"
  | "lineIn"
  | "system"
  | "sample";

export type RoutingOutput =
  | "headphones"
  | "broadcastMix"
  | "chatMic"
  | "sampler"
  | "lineOut"
  | "streamMix2";

export type MicrophoneType = "dynamic" | "condenser" | "jack";
export type EqBandId =
  | "eq31Hz"
  | "eq63Hz"
  | "eq125Hz"
  | "eq250Hz"
  | "eq500Hz"
  | "eq1KHz"
  | "eq2KHz"
  | "eq4KHz"
  | "eq8KHz"
  | "eq16KHz"
  | "miniEq90Hz"
  | "miniEq250Hz"
  | "miniEq500Hz"
  | "miniEq1KHz"
  | "miniEq3KHz"
  | "miniEq8KHz";

export type GateControl = "enabled" | "threshold" | "attenuation" | "attack" | "release";
export type CompressorControl =
  | "threshold"
  | "ratio"
  | "attack"
  | "release"
  | "makeupGain";

export type ChannelName =
  | "mic"
  | "lineIn"
  | "console"
  | "system"
  | "game"
  | "chat"
  | "sample"
  | "music"
  | "headphones"
  | "micMonitor"
  | "lineOut"
  | "unknown";

export type MuteFunction =
  | "all"
  | "toStream"
  | "toVoiceChat"
  | "toPhones"
  | "toLineOut"
  | "toStream2"
  | "toStreams"
  | "unknown";

export type FaderMuteState = "unmuted" | "mutedToX" | "mutedToAll" | "unknown";

export interface VersionNumber {
  major: number;
  minor: number;
  patch?: number | null;
  build?: number | null;
}

export interface DeviceIdentity {
  id: string;
  model: DeviceModel;
  vendorId?: number | null;
  productId?: number | null;
  manufacturerName?: string | null;
  productName?: string | null;
  serialNumber?: string | null;
  firmwareVersion?: VersionNumber | null;
  driverInterface?: string | null;
  driverVersion?: VersionNumber | null;
  isMock: boolean;
}

export interface FaderVolume {
  raw: number;
  percent: number;
}

export interface RoutingRoute {
  input: RoutingInput;
  output: RoutingOutput;
}

export interface RoutingRouteState {
  route: RoutingRoute;
  enabled: boolean;
}

export interface RoutingState {
  routes: RoutingRouteState[];
}

export interface ValueRangeI16 {
  min: number;
  max: number;
}

export interface ValueRangeU16 {
  min: number;
  max: number;
}

export interface TimeOption {
  index: number;
  millis: number;
}

export interface CompressorRatioOption {
  index: number;
  ratioTenths: number;
}

export interface EqBandCapability {
  id: EqBandId;
  label: string;
  defaultFrequencyTenthsHz: number;
  minFrequencyTenthsHz: number;
  maxFrequencyTenthsHz: number;
  frequencyStepTenthsHz: number;
  minGainDb: number;
  maxGainDb: number;
  configurableFrequency: boolean;
  configurableWidth: boolean;
}

export interface MicrophoneGainState {
  microphoneType: MicrophoneType;
  hardwareDb: number;
}

export interface MicrophoneSetupState {
  microphoneType: MicrophoneType;
  gains: MicrophoneGainState[];
  phantomPowerEnabled: boolean;
}

export interface EqBandState {
  id: EqBandId;
  frequencyTenthsHz: number;
  gainDb: number;
}

export interface EqualizerState {
  bands: EqBandState[];
}

export interface NoiseGateState {
  enabled: boolean;
  thresholdDb: number;
  attenuationPercent: number;
  attack: TimeOption;
  release: TimeOption;
}

export interface CompressorState {
  thresholdDb: number;
  ratio: CompressorRatioOption;
  attack: TimeOption;
  release: TimeOption;
  makeupGainDb: number;
}

export interface DeEsserState {
  amountPercent: number;
}

export interface MicrophoneState {
  setup: MicrophoneSetupState;
  equalizer: EqualizerState;
  gate: NoiseGateState;
  compressor: CompressorState;
  deEsser: DeEsserState;
}

export interface DeviceCapabilities {
  readableFaderAssignments: boolean;
  readableFaderVolumes: boolean;
  readableFaderMuteState: boolean;
  readableFaderButtonState: boolean;
  writableFaderVolumes: boolean;
  writableFaderMuteState: boolean;
  writableFaderAssignments: boolean;
  supportedAssignmentChannels: ChannelName[];
  readableRouting: boolean;
  writableRouting: boolean;
  supportedRoutingInputs: RoutingInput[];
  supportedRoutingOutputs: RoutingOutput[];
  supportedRoutes: RoutingRoute[];
  readableMicrophone: boolean;
  writableMicrophone: boolean;
  supportedMicrophoneTypes: MicrophoneType[];
  microphoneGainRangeDb?: ValueRangeU16 | null;
  phantomPowerSupported: boolean;
  eqBands: EqBandCapability[];
  gateControls: GateControl[];
  gateThresholdRangeDb?: ValueRangeI16 | null;
  gateAttenuationRangePercent?: ValueRangeU16 | null;
  gateTimeOptions: TimeOption[];
  compressorControls: CompressorControl[];
  compressorThresholdRangeDb?: ValueRangeI16 | null;
  compressorMakeupGainRangeDb?: ValueRangeI16 | null;
  compressorRatioOptions: CompressorRatioOption[];
  compressorAttackOptions: TimeOption[];
  compressorReleaseOptions: TimeOption[];
  deEsserSupported: boolean;
  deEsserRangePercent?: ValueRangeU16 | null;
}

export interface FaderState {
  name: FaderName;
  assignedChannel?: ChannelName | null;
  volume?: FaderVolume | null;
  muteState?: FaderMuteState | null;
  muteFunction?: MuteFunction | null;
  muted?: boolean | null;
  muteButtonPressed?: boolean | null;
}

export interface DeviceState {
  identity: DeviceIdentity;
  status: ConnectionStatus;
  capabilities: DeviceCapabilities;
  faders: FaderState[];
  routing?: RoutingState | null;
  microphone?: MicrophoneState | null;
  lastSeenEpochMs: number;
  sessionGeneration?: number | null;
}

export interface AppSettingsSummary {
  mockDeviceEnabled: boolean;
  showTrayIcon: boolean;
  startMinimized: boolean;
  launchAtStartup: boolean;
  logLevel: LogLevel;
  configPath?: string | null;
}

export interface ServiceStatus {
  running: boolean;
  lastError?: string | null;
}

export interface AppSnapshot {
  settings: AppSettingsSummary;
  service: ServiceStatus;
  devices: DeviceState[];
  selectedDeviceId?: string | null;
}
