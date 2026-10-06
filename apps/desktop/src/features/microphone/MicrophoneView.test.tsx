import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { useDeviceStore } from "../../stores/deviceStore";
import { device, phase3Capabilities, snapshot } from "../../test/fixtures";
import { MicrophoneView } from "./MicrophoneView";

afterEach(() => {
  useDeviceStore.setState({ snapshot: null, loading: true, error: null });
});

describe("MicrophoneView", () => {
  it("renders microphone setup and processing controls", () => {
    useDeviceStore.setState({
      snapshot: snapshot(device()),
      loading: false,
      error: null
    });

    render(<MicrophoneView />);

    expect(screen.getAllByText("Dynamic").length).toBeGreaterThan(0);
    expect(screen.getByLabelText("Microphone gain")).toBeEnabled();
    expect(screen.getByLabelText("EQ response")).toBeInTheDocument();
    expect(screen.getByLabelText("90 Hz gain")).toBeEnabled();
    expect(screen.getAllByLabelText("Threshold")[0]).toBeEnabled();
    expect(screen.getByLabelText("De-esser amount")).toBeEnabled();
  });

  it("sends microphone type and gain changes", () => {
    const setMicrophoneType = vi.fn().mockResolvedValue(undefined);
    const setMicrophoneGain = vi.fn().mockResolvedValue(undefined);
    const selected = device();
    useDeviceStore.setState({
      snapshot: snapshot(selected),
      loading: false,
      error: null,
      setMicrophoneType,
      setMicrophoneGain
    });

    render(<MicrophoneView />);
    fireEvent.click(screen.getByText("Condenser +48V"));
    const gain = screen.getByLabelText("Microphone gain");
    fireEvent.change(gain, { target: { value: "40" } });
    fireEvent.pointerUp(gain);

    expect(setMicrophoneType).toHaveBeenCalledWith(
      selected.identity.id,
      selected.sessionGeneration,
      "condenser",
      true
    );
    expect(setMicrophoneGain).toHaveBeenCalledWith(
      selected.identity.id,
      selected.sessionGeneration,
      "dynamic",
      40
    );
  });

  it("disables microphone controls when state is unavailable", () => {
    const selected = device("real:1", {
      ...phase3Capabilities(false),
      readableMicrophone: false,
      writableMicrophone: false
    });
    selected.microphone = null;
    useDeviceStore.setState({
      snapshot: snapshot(selected),
      loading: false,
      error: null
    });

    render(<MicrophoneView />);

    expect(screen.getByText(/microphone controls unavailable/i)).toBeInTheDocument();
  });
});
