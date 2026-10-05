import { describe, expect, it } from "vitest";
import {
  formatChannel,
  formatDeviceModel,
  formatMuteFunction,
  formatMuteState,
  formatVersion
} from "./format";

describe("device formatting", () => {
  it("formats device models", () => {
    expect(formatDeviceModel("goXlr")).toBe("GoXLR");
    expect(formatDeviceModel("goXlrMini")).toBe("GoXLR Mini");
  });

  it("formats optional versions", () => {
    expect(formatVersion(null)).toBe("Not available");
    expect(formatVersion({ major: 1, minor: 3, patch: 40, build: 7 })).toBe("1.3.40.7");
  });

  it("formats known channels", () => {
    expect(formatChannel("lineIn")).toBe("Line In");
    expect(formatChannel(undefined)).toBe("Unknown");
  });

  it("formats mute states", () => {
    expect(formatMuteState("mutedToAll")).toBe("Muted");
    expect(formatMuteState(null)).toBe("Unavailable");
  });

  it("formats mute functions", () => {
    expect(formatMuteFunction("all")).toBe("Mute all");
    expect(formatMuteFunction("toVoiceChat")).toBe("Mute to voice chat");
    expect(formatMuteFunction(null)).toBe("Unavailable");
  });
});
