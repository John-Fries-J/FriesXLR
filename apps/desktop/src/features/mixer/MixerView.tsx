import { Cable, Music2, SlidersVertical, Volume2, VolumeX } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { EmptyState } from "../../components/EmptyState";
import { StatusPill } from "../../components/StatusPill";
import { useDeviceStore } from "../../stores/deviceStore";
import type { ChannelName, DeviceState, FaderState } from "../../types/backend";
import {
  formatChannel,
  formatDeviceModel,
  formatMuteFunction,
  formatMuteState
} from "../device/format";

const SEND_INTERVAL_MS = 90;

export function MixerView() {
  const snapshot = useDeviceStore((state) => state.snapshot);
  const setSelectedDevice = useDeviceStore((state) => state.setSelectedDevice);
  const devices = snapshot?.devices ?? [];

  if (devices.length === 0) {
    return <EmptyState />;
  }

  const selectedId = snapshot?.selectedDeviceId ?? devices[0]?.identity.id;
  const selectedDevice = devices.find((device) => device.identity.id === selectedId) ?? devices[0];

  return (
    <section className="mixer-view">
      <div className="section-heading mixer-heading">
        <div>
          <span className="eyebrow">Mixer</span>
          <h2>{formatDeviceModel(selectedDevice.identity.model)}</h2>
        </div>
        <div className="mixer-heading-actions">
          {devices.length > 1 && (
            <label className="device-select">
              <Cable size={16} />
              <select
                aria-label="Selected device"
                value={selectedDevice.identity.id}
                onChange={(event) => void setSelectedDevice(event.currentTarget.value)}
              >
                {devices.map((device) => (
                  <option key={device.identity.id} value={device.identity.id}>
                    {device.identity.serialNumber ?? device.identity.id}
                  </option>
                ))}
              </select>
            </label>
          )}
          <StatusPill tone={selectedDevice.status === "connected" ? "good" : "warn"}>
            {selectedDevice.status}
          </StatusPill>
        </div>
      </div>

      <div className="mixer-console">
        {selectedDevice.faders.map((fader) => (
          <MixerFader key={fader.name} device={selectedDevice} fader={fader} />
        ))}
      </div>
    </section>
  );
}

function MixerFader({ device, fader }: { device: DeviceState; fader: FaderState }) {
  const setFaderVolume = useDeviceStore((state) => state.setFaderVolume);
  const setFaderMute = useDeviceStore((state) => state.setFaderMute);
  const setFaderAssignment = useDeviceStore((state) => state.setFaderAssignment);
  const sessionGeneration = device.sessionGeneration;
  const canReadAssignment =
    device.capabilities.readableFaderAssignments &&
    fader.assignedChannel !== null &&
    fader.assignedChannel !== undefined;
  const canReadVolume =
    device.capabilities.readableFaderVolumes &&
    fader.volume !== null &&
    fader.volume !== undefined;
  const canReadMuteState =
    device.capabilities.readableFaderMuteState &&
    fader.muteState !== null &&
    fader.muteState !== undefined;
  const canReadMuteButton =
    device.capabilities.readableFaderButtonState &&
    fader.muteButtonPressed !== null &&
    fader.muteButtonPressed !== undefined;
  const canWriteVolume =
    device.capabilities.writableFaderVolumes &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined;
  const canWriteMute =
    device.capabilities.writableFaderMuteState &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined &&
    fader.muted !== null &&
    fader.muted !== undefined;
  const canWriteAssignment =
    device.capabilities.writableFaderAssignments &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined;
  const [draftVolume, setDraftVolume] = useState(fader.volume?.percent ?? 0);
  const lastSentAt = useRef(0);
  const pendingVolume = useRef<number | null>(null);
  const timer = useRef<number | null>(null);

  useEffect(() => {
    setDraftVolume(fader.volume?.percent ?? 0);
  }, [fader.volume?.percent]);

  useEffect(() => {
    return () => {
      if (timer.current !== null) {
        window.clearTimeout(timer.current);
      }
    };
  }, []);

  function sendVolume(percent: number, final = false) {
    if (!canWriteVolume || sessionGeneration === null || sessionGeneration === undefined) {
      return;
    }

    pendingVolume.current = percent;

    const commit = () => {
      timer.current = null;
      const next = pendingVolume.current;
      if (next === null) {
        return;
      }
      pendingVolume.current = null;
      lastSentAt.current = Date.now();
      void setFaderVolume(device.identity.id, sessionGeneration, fader.name, next);
    };

    if (final) {
      if (timer.current !== null) {
        window.clearTimeout(timer.current);
        timer.current = null;
      }
      commit();
      return;
    }

    const elapsed = Date.now() - lastSentAt.current;
    if (elapsed >= SEND_INTERVAL_MS) {
      commit();
    } else if (timer.current === null) {
      timer.current = window.setTimeout(commit, SEND_INTERVAL_MS - elapsed);
    }
  }

  const muted = fader.muted === true;
  const assignmentValue = canReadAssignment ? (fader.assignedChannel ?? "") : "";
  const assignmentLabel = canReadAssignment ? formatChannel(fader.assignedChannel) : "Unavailable";
  const muteStateLabel = canReadMuteState ? formatMuteState(fader.muteState) : "Unavailable";
  const muteFunctionLabel = formatMuteFunction(fader.muteFunction);
  const muteButtonLabel = canReadMuteButton
    ? fader.muteButtonPressed
      ? "Button down"
      : "Button up"
    : "Button unavailable";
  const muteButtonText = canReadMuteState ? (muted ? "Muted" : "Mute") : "Mute unavailable";

  return (
    <article className="mixer-fader">
      <div className="mixer-fader-top">
        <span className="fader-letter">{fader.name}</span>
        <span className="fader-icon" aria-hidden="true">
          {muted ? <VolumeX size={18} /> : <Volume2 size={18} />}
        </span>
      </div>

      <div className="mixer-channel">
        <Music2 size={17} />
        <strong>{assignmentLabel}</strong>
      </div>

      <label className="vertical-fader">
        <SlidersVertical size={16} />
        <input
          aria-label={`Fader ${fader.name} volume`}
          disabled={!canWriteVolume || !canReadVolume}
          max={100}
          min={0}
          type="range"
          value={draftVolume}
          onBlur={() => sendVolume(draftVolume, true)}
          onChange={(event) => {
            const value = Number(event.currentTarget.value);
            setDraftVolume(value);
            sendVolume(value);
          }}
          onPointerUp={() => sendVolume(draftVolume, true)}
        />
      </label>

      <div className="mixer-readout">
        <strong>{canReadVolume ? `${fader.volume?.percent ?? 0}%` : "Unavailable"}</strong>
        <span>{muteStateLabel}</span>
        <span>{muteFunctionLabel}</span>
        <span>{muteButtonLabel}</span>
      </div>

      <button
        aria-label={`Fader ${fader.name} mute`}
        className={muted ? "mute-control active" : "mute-control"}
        disabled={!canWriteMute}
        type="button"
        onClick={() => {
          if (sessionGeneration !== null && sessionGeneration !== undefined) {
            void setFaderMute(device.identity.id, sessionGeneration, fader.name, !muted);
          }
        }}
      >
        {muted ? <VolumeX size={16} /> : <Volume2 size={16} />}
        <span>{muteButtonText}</span>
      </button>

      <label className="assignment-control">
        <span>Assignment</span>
        <select
          aria-label={`Fader ${fader.name} assignment`}
          disabled={!canWriteAssignment}
          value={assignmentValue}
          onChange={(event) => {
            const channel = event.currentTarget.value as ChannelName;
            if (sessionGeneration !== null && sessionGeneration !== undefined && channel) {
              void setFaderAssignment(device.identity.id, sessionGeneration, fader.name, channel);
            }
          }}
        >
          {!canReadAssignment && <option value="">Unavailable</option>}
          {device.capabilities.supportedAssignmentChannels.map((channel) => (
            <option key={channel} value={channel}>
              {formatChannel(channel)}
            </option>
          ))}
        </select>
      </label>
    </article>
  );
}
