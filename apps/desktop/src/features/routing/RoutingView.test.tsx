import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { useDeviceStore } from "../../stores/deviceStore";
import { device, phase3Capabilities, snapshot } from "../../test/fixtures";
import { RoutingView } from "./RoutingView";

afterEach(() => {
  useDeviceStore.setState({ snapshot: null, loading: true, error: null });
});

describe("RoutingView", () => {
  it("renders the routing matrix from capabilities", () => {
    useDeviceStore.setState({
      snapshot: snapshot(device()),
      loading: false,
      error: null
    });

    render(<RoutingView />);

    expect(screen.getByText("Mic")).toBeInTheDocument();
    expect(screen.getByText("Broadcast Mix")).toBeInTheDocument();
    expect(screen.getByLabelText("Mic to Headphones")).toBeEnabled();
    expect(screen.getByLabelText("Chat to Chat Mic")).toBeDisabled();
  });

  it("sends route toggles through the store", () => {
    const setRoutingRoute = vi.fn().mockResolvedValue(undefined);
    const selected = device();
    useDeviceStore.setState({
      snapshot: snapshot(selected),
      loading: false,
      error: null,
      setRoutingRoute
    });

    render(<RoutingView />);
    fireEvent.click(screen.getByLabelText("Music to Broadcast Mix"));

    expect(setRoutingRoute).toHaveBeenCalledWith(
      selected.identity.id,
      selected.sessionGeneration,
      { input: "music", output: "broadcastMix" },
      true
    );
  });

  it("disables routing when authoritative state is unavailable", () => {
    const selected = device("real:1", {
      ...phase3Capabilities(false),
      readableRouting: false,
      writableRouting: false
    });
    selected.routing = null;
    useDeviceStore.setState({
      snapshot: snapshot(selected),
      loading: false,
      error: null
    });

    render(<RoutingView />);

    expect(screen.getByLabelText("Mic to Headphones")).toBeDisabled();
    expect(screen.getByText(/authoritative profile state/i)).toBeInTheDocument();
  });
});
