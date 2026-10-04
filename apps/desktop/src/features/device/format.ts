import type { ChannelName, DeviceModel, VersionNumber } from "../../types/backend";

export function formatDeviceModel(model: DeviceModel) {
  switch (model) {
    case "goXlr":
      return "GoXLR";
    case "goXlrMini":
      return "GoXLR Mini";
    default:
      return "Unknown";
  }
}

export function formatChannel(channel?: ChannelName | null) {
  if (!channel) {
    return "Unknown";
  }

  const labels: Record<ChannelName, string> = {
    mic: "Mic",
    lineIn: "Line In",
    console: "Console",
    system: "System",
    game: "Game",
    chat: "Chat",
    sample: "Sample",
    music: "Music",
    headphones: "Headphones",
    micMonitor: "Mic Monitor",
    lineOut: "Line Out",
    unknown: "Unknown"
  };

  return labels[channel];
}

export function formatVersion(version?: VersionNumber | null) {
  if (!version) {
    return "Not available";
  }

  const parts = [version.major, version.minor];
  if (version.patch !== undefined && version.patch !== null) {
    parts.push(version.patch);
  }
  if (version.build !== undefined && version.build !== null) {
    parts.push(version.build);
  }

  return parts.join(".");
}

