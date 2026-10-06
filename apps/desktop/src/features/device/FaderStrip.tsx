import { Mic2, Music2, Volume2, VolumeX } from "lucide-react";
import type { FaderState } from "../../types/backend";
import { formatChannel, formatMuteState } from "./format";

interface FaderStripProps {
  fader: FaderState;
}

export function FaderStrip({ fader }: FaderStripProps) {
  const hasVolume = fader.volume !== undefined && fader.volume !== null;
  const pct = fader.volume?.percent ?? 0;
  const Icon = fader.assignedChannel === "mic" ? Mic2 : Music2;
  const muteKnown = fader.muted !== undefined && fader.muted !== null;
  const isMuted = fader.muted === true;

  return (
    <div className="fader-strip">
      <div className="fader-head">
        <span>{fader.name}</span>
        {isMuted ? <VolumeX size={16} /> : <Volume2 size={16} />}
      </div>
      <div className="fader-channel">
        <Icon size={18} />
        <strong>{formatChannel(fader.assignedChannel)}</strong>
      </div>
      <div className="fader-rail" aria-hidden="true">
        <div className="fader-fill" style={{ height: `${hasVolume ? pct : 0}%` }} />
        <div className="fader-thumb" style={{ bottom: `${hasVolume ? pct : 0}%` }} />
      </div>
      <span className="fader-value">
        {hasVolume ? `${pct}%` : "Volume unavailable"}
        {" / "}
        {muteKnown ? formatMuteState(fader.muteState) : "Mute unavailable"}
      </span>
    </div>
  );
}
