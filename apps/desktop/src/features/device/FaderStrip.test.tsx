import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { FaderStrip } from "./FaderStrip";

describe("FaderStrip", () => {
  it("renders read-only unavailable state", () => {
    render(<FaderStrip fader={{ name: "A", assignedChannel: null, volume: null, muted: null }} />);

    expect(screen.getByText(/Volume unavailable/)).toBeInTheDocument();
    expect(screen.getByText(/Mute unavailable/)).toBeInTheDocument();
  });

  it("renders channel and percentage", () => {
    render(
      <FaderStrip
        fader={{
          name: "B",
          assignedChannel: "chat",
          volume: { raw: 128, percent: 50 },
          muteState: "unmuted",
          muted: false
        }}
      />
    );

    expect(screen.getByText("Chat")).toBeInTheDocument();
    expect(screen.getByText(/50%/)).toBeInTheDocument();
  });
});
