import type {
  ChannelName,
  DeviceModel,
  EqBandId,
  FaderMuteState,
  MicrophoneType,
  MuteFunction,
  RoutingInput,
  RoutingOutput,
  VersionNumber
} from "../../types/backend";

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

export function formatMuteState(state?: FaderMuteState | null) {
  switch (state) {
    case "unmuted":
      return "Unmuted";
    case "mutedToX":
      return "Muted to X";
    case "mutedToAll":
      return "Muted";
    case "unknown":
      return "Unknown";
    default:
      return "Unavailable";
  }
}

export function formatMuteFunction(muteFunction?: MuteFunction | null) {
  switch (muteFunction) {
    case "all":
      return "Mute all";
    case "toStream":
      return "Mute to stream";
    case "toVoiceChat":
      return "Mute to voice chat";
    case "toPhones":
      return "Mute to phones";
    case "toLineOut":
      return "Mute to line out";
    case "toStream2":
      return "Mute to stream 2";
    case "toStreams":
      return "Mute to streams";
    case "unknown":
      return "Unknown";
    default:
      return "Unavailable";
  }
}

export function formatRoutingInput(input: RoutingInput) {
  const labels: Record<RoutingInput, string> = {
    microphone: "Mic",
    chat: "Chat",
    music: "Music",
    game: "Game",
    console: "Console",
    lineIn: "Line In",
    system: "System",
    sample: "Sample"
  };

  return labels[input];
}

export function formatRoutingOutput(output: RoutingOutput) {
  const labels: Record<RoutingOutput, string> = {
    headphones: "Headphones",
    broadcastMix: "Broadcast Mix",
    chatMic: "Chat Mic",
    sampler: "Sampler",
    lineOut: "Line Out",
    streamMix2: "Stream Mix 2"
  };

  return labels[output];
}

export function formatMicrophoneType(type: MicrophoneType) {
  const labels: Record<MicrophoneType, string> = {
    dynamic: "Dynamic",
    condenser: "Condenser",
    jack: "3.5mm"
  };

  return labels[type];
}

export function formatFrequency(tenthsHz: number) {
  const hz = tenthsHz / 10;
  if (hz >= 1000) {
    return `${Number((hz / 1000).toFixed(1))} kHz`;
  }
  return `${Number(hz.toFixed(1))} Hz`;
}

export function formatRatio(ratioTenths: number) {
  return `${Number((ratioTenths / 10).toFixed(1))}:1`;
}

export function formatEqBand(id: EqBandId) {
  const labels: Record<EqBandId, string> = {
    eq31Hz: "31 Hz",
    eq63Hz: "63 Hz",
    eq125Hz: "125 Hz",
    eq250Hz: "250 Hz",
    eq500Hz: "500 Hz",
    eq1KHz: "1 kHz",
    eq2KHz: "2 kHz",
    eq4KHz: "4 kHz",
    eq8KHz: "8 kHz",
    eq16KHz: "16 kHz",
    miniEq90Hz: "90 Hz",
    miniEq250Hz: "250 Hz",
    miniEq500Hz: "500 Hz",
    miniEq1KHz: "1 kHz",
    miniEq3KHz: "3 kHz",
    miniEq8KHz: "8 kHz"
  };

  return labels[id];
}
