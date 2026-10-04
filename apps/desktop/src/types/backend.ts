export type DeviceModel = "goXlr" | "goXlrMini" | "unknown";
export type ConnectionStatus = "connected" | "disconnected";
export type LogLevel = "error" | "warn" | "info" | "debug" | "trace";
export type FaderName = "A" | "B" | "C" | "D";

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

export interface FaderState {
  name: FaderName;
  assignedChannel?: ChannelName | null;
  volume?: number | null;
  muted?: boolean | null;
}

export interface DeviceState {
  identity: DeviceIdentity;
  status: ConnectionStatus;
  faders: FaderState[];
  lastSeenEpochMs: number;
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
}
