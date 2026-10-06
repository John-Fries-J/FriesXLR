import { render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { useDeviceStore } from "../../stores/deviceStore";
import type { AppSnapshot, DeviceCapabilities, DeviceState } from "../../types/backend";
import { MixerView } from "./MixerView";

const writableCapabilities: DeviceCapabilities = {
  readableFaderAssignments: true,
  readableFaderVolumes: true,
  readableFaderMuteState: true,
  readableFaderButtonState: true,
  writableFaderVolumes: true,
  writableFaderMuteState: true,
  writableFaderAssignments: true,
  supportedAssignmentChannels: ["mic", "chat", "music", "system"]
};

const readOnlyCapabilities: DeviceCapabilities = {
  readableFaderAssignments: false,
  readableFaderVolumes: true,
  readableFaderMuteState: false,
  readableFaderButtonState: true,
  writableFaderVolumes: false,
  writableFaderMuteState: false,
  writableFaderAssignments: false,
  supportedAssignmentChannels: ["mic", "chat", "music", "system"]
};

afterEach(() => {
  useDeviceStore.setState({ snapshot: null, loading: true, error: null });
});

describe("MixerView", () => {
  it("renders all four faders for the selected device", () => {
    useDeviceStore.setState({
      snapshot: snapshot(device("mock:1", writableCapabilities)),
      loading: false,
      error: null
    });

    render(<MixerView />);

    expect(screen.getByText("A")).toBeInTheDocument();
    expect(screen.getByText("B")).toBeInTheDocument();
    expect(screen.getByText("C")).toBeInTheDocument();
    expect(screen.getByText("D")).toBeInTheDocument();
    expect(screen.getByLabelText("Fader A volume")).toBeEnabled();
    expect(screen.getByLabelText("Fader A mute")).toBeEnabled();
    expect(screen.getByLabelText("Fader A assignment")).toBeEnabled();
  });

  it("renders unavailable controls without fake values", () => {
    const selected = device("real:1", readOnlyCapabilities, true);
    selected.faders[0].assignedChannel = null;
    selected.faders[0].volume = null;
    selected.faders[0].muted = null;

    useDeviceStore.setState({
      snapshot: snapshot(selected),
      loading: false,
      error: null
    });

    render(<MixerView />);

    expect(screen.getAllByText("Unavailable").length).toBeGreaterThan(0);
    expect(screen.getByLabelText("Fader A volume")).toBeDisabled();
    expect(screen.getByLabelText("Fader A mute")).toBeDisabled();
    expect(screen.getByLabelText("Fader A assignment")).toBeDisabled();
    expect(screen.getAllByText("Button up").length).toBeGreaterThan(0);
  });

  it("renders a selector for multiple devices", () => {
    const first = device("mock:1", writableCapabilities);
    const second = device("mock:2", writableCapabilities);
    second.identity.serialNumber = "MOCK-2";

    useDeviceStore.setState({
      snapshot: snapshot(first, [first, second], second.identity.id),
      loading: false,
      error: null
    });

    render(<MixerView />);

    const selector = screen.getByLabelText("Selected device");
    expect(selector).toHaveValue("mock:2");
    expect(screen.getByText("MOCK-1")).toBeInTheDocument();
    expect(screen.getByText("MOCK-2")).toBeInTheDocument();
  });
});

function snapshot(
  device: DeviceState,
  devices: DeviceState[] = [device],
  selectedDeviceId = device.identity.id
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

function device(
  id: string,
  capabilities: DeviceCapabilities,
  real = false
): DeviceState {
  return {
    identity: {
      id,
      model: real ? "goXlr" : "goXlrMini",
      vendorId: real ? 0x1220 : null,
      productId: real ? 0x8fe0 : null,
      manufacturerName: real ? "TC-Helicon" : "FriesXLR",
      productName: real ? "GoXLR" : "Mock GoXLR Mini",
      serialNumber: real ? "REAL-1" : "MOCK-1",
      firmwareVersion: null,
      driverInterface: real ? "tusb" : "mock",
      driverVersion: null,
      isMock: !real
    },
    status: "connected",
    capabilities,
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
