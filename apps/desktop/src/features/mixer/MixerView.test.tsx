import { render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { useDeviceStore } from "../../stores/deviceStore";
import { device, phase3Capabilities, snapshot } from "../../test/fixtures";
import type { DeviceCapabilities } from "../../types/backend";
import { MixerView } from "./MixerView";

const writableCapabilities: DeviceCapabilities = {
  ...phase3Capabilities(true)
};

const readOnlyCapabilities: DeviceCapabilities = {
  ...phase3Capabilities(false),
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
    const selected = device("real:1", readOnlyCapabilities);
    selected.identity.model = "goXlr";
    selected.identity.vendorId = 0x1220;
    selected.identity.productId = 0x8fe0;
    selected.identity.manufacturerName = "TC-Helicon";
    selected.identity.productName = "GoXLR";
    selected.identity.serialNumber = "REAL-1";
    selected.identity.driverInterface = "tusb";
    selected.identity.isMock = false;
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
