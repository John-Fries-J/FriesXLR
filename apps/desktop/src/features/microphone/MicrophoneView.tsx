import { Cable, Mic2, ShieldAlert } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { EmptyState } from "../../components/EmptyState";
import { StatusPill } from "../../components/StatusPill";
import { useDeviceStore } from "../../stores/deviceStore";
import type {
  CompressorRatioOption,
  CompressorState,
  DeEsserState,
  DeviceState,
  EqBandCapability,
  EqBandState,
  MicrophoneState,
  NoiseGateState,
  TimeOption
} from "../../types/backend";
import {
  formatDeviceModel,
  formatFrequency,
  formatMicrophoneType,
  formatRatio
} from "../device/format";

export function MicrophoneView() {
  const snapshot = useDeviceStore((state) => state.snapshot);
  const setSelectedDevice = useDeviceStore((state) => state.setSelectedDevice);
  const devices = snapshot?.devices ?? [];

  if (devices.length === 0) {
    return <EmptyState />;
  }

  const selectedId = snapshot?.selectedDeviceId ?? devices[0]?.identity.id;
  const selectedDevice = devices.find((device) => device.identity.id === selectedId) ?? devices[0];

  return (
    <section className="microphone-view">
      <div className="section-heading mixer-heading">
        <div>
          <span className="eyebrow">Microphone</span>
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

      <MicrophonePanel device={selectedDevice} />
    </section>
  );
}

function MicrophonePanel({ device }: { device: DeviceState }) {
  const microphone = device.microphone;
  const canRead =
    device.capabilities.readableMicrophone &&
    microphone !== null &&
    microphone !== undefined;

  if (!canRead || !microphone) {
    return (
      <div className="mic-unavailable">
        <Mic2 size={22} />
        <strong>Microphone controls unavailable</strong>
        <span>Authoritative profile state is required before physical mic controls are active.</span>
      </div>
    );
  }

  return (
    <div className="mic-layout">
      <SetupSection device={device} microphone={microphone} />
      <EqualizerSection device={device} microphone={microphone} />
      <GateSection device={device} gate={microphone.gate} />
      <CompressorSection device={device} compressor={microphone.compressor} />
      <DeEsserSection device={device} deEsser={microphone.deEsser} />
    </div>
  );
}

function SetupSection({
  device,
  microphone
}: {
  device: DeviceState;
  microphone: MicrophoneState;
}) {
  const setMicrophoneType = useDeviceStore((state) => state.setMicrophoneType);
  const setMicrophoneGain = useDeviceStore((state) => state.setMicrophoneGain);
  const sessionGeneration = device.sessionGeneration;
  const canWrite =
    device.capabilities.writableMicrophone &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined;
  const activeType = microphone.setup.microphoneType;
  const gainRange = device.capabilities.microphoneGainRangeDb ?? { min: 0, max: 72 };
  const activeGain =
    microphone.setup.gains.find((gain) => gain.microphoneType === activeType)?.hardwareDb ?? 0;
  const [draftGain, setDraftGain] = useState(activeGain);

  useEffect(() => {
    setDraftGain(activeGain);
  }, [activeGain]);

  function commitGain(value: number) {
    if (canWrite && sessionGeneration !== null && sessionGeneration !== undefined) {
      void setMicrophoneGain(device.identity.id, sessionGeneration, activeType, value);
    }
  }

  return (
    <section className="mic-panel setup-panel">
      <div className="mic-panel-heading">
        <span className="eyebrow">Setup</span>
        <strong>{formatMicrophoneType(activeType)}</strong>
      </div>

      <div className="segmented-control">
        {device.capabilities.supportedMicrophoneTypes.map((type) => {
          const condenser = type === "condenser";
          const active = activeType === type;
          return (
            <button
              key={type}
              className={active ? "active" : ""}
              disabled={!canWrite}
              type="button"
              onClick={() => {
                if (sessionGeneration !== null && sessionGeneration !== undefined) {
                  void setMicrophoneType(
                    device.identity.id,
                    sessionGeneration,
                    type,
                    condenser
                  );
                }
              }}
            >
              {formatMicrophoneType(type)}
              {condenser && device.capabilities.phantomPowerSupported ? " +48V" : ""}
            </button>
          );
        })}
      </div>

      <label className="horizontal-control">
        <span>Gain</span>
        <input
          aria-label="Microphone gain"
          disabled={!canWrite}
          max={gainRange.max}
          min={gainRange.min}
          type="range"
          value={draftGain}
          onBlur={() => commitGain(draftGain)}
          onChange={(event) => setDraftGain(Number(event.currentTarget.value))}
          onPointerUp={() => commitGain(draftGain)}
        />
        <strong>{draftGain} dB</strong>
      </label>

      <div className="phantom-status">
        <ShieldAlert size={16} />
        <span>{microphone.setup.phantomPowerEnabled ? "+48V active" : "+48V off"}</span>
      </div>
    </section>
  );
}

function EqualizerSection({
  device,
  microphone
}: {
  device: DeviceState;
  microphone: MicrophoneState;
}) {
  const capabilities = device.capabilities.eqBands;
  const bands = capabilities
    .map((capability) => ({
      capability,
      state: microphone.equalizer.bands.find((band) => band.id === capability.id)
    }))
    .filter((band): band is { capability: EqBandCapability; state: EqBandState } =>
      Boolean(band.state)
    );

  return (
    <section className="mic-panel eq-panel">
      <div className="mic-panel-heading">
        <span className="eyebrow">EQ</span>
        <strong>{bands.length} bands</strong>
      </div>
      <EqGraph bands={bands.map((band) => band.state)} capabilities={capabilities} />
      <div className="eq-band-list">
        {bands.map(({ capability, state }) => (
          <EqBandControl
            key={state.id}
            capability={capability}
            device={device}
            state={state}
          />
        ))}
      </div>
    </section>
  );
}

function EqGraph({
  bands,
  capabilities
}: {
  bands: EqBandState[];
  capabilities: EqBandCapability[];
}) {
  const points = useMemo(() => {
    if (bands.length === 0) {
      return "";
    }
    const minFreq = Math.min(...capabilities.map((band) => band.minFrequencyTenthsHz));
    const maxFreq = Math.max(...capabilities.map((band) => band.maxFrequencyTenthsHz));
    const logMin = Math.log10(minFreq);
    const logMax = Math.log10(maxFreq);

    return bands
      .map((band) => {
        const x = ((Math.log10(band.frequencyTenthsHz) - logMin) / (logMax - logMin)) * 100;
        const y = 50 - (band.gainDb / 9) * 40;
        return `${x.toFixed(2)},${y.toFixed(2)}`;
      })
      .join(" ");
  }, [bands, capabilities]);

  return (
    <svg aria-label="EQ response" className="eq-graph" viewBox="0 0 100 100" role="img">
      <line x1="0" x2="100" y1="50" y2="50" />
      <polyline points={points} />
      {bands.map((band) => {
        const index = bands.indexOf(band);
        const [x, y] = points.split(" ")[index]?.split(",") ?? ["0", "50"];
        return <circle key={band.id} cx={x} cy={y} r="2.4" />;
      })}
    </svg>
  );
}

function EqBandControl({
  capability,
  device,
  state
}: {
  capability: EqBandCapability;
  device: DeviceState;
  state: EqBandState;
}) {
  const setEqualizerBand = useDeviceStore((store) => store.setEqualizerBand);
  const sessionGeneration = device.sessionGeneration;
  const canWrite =
    device.capabilities.writableMicrophone &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined;
  const [frequency, setFrequency] = useState(state.frequencyTenthsHz);
  const [gain, setGain] = useState(state.gainDb);

  useEffect(() => {
    setFrequency(state.frequencyTenthsHz);
    setGain(state.gainDb);
  }, [state.frequencyTenthsHz, state.gainDb]);

  function commit(nextFrequency = frequency, nextGain = gain) {
    if (canWrite && sessionGeneration !== null && sessionGeneration !== undefined) {
      void setEqualizerBand(
        device.identity.id,
        sessionGeneration,
        state.id,
        nextFrequency,
        nextGain
      );
    }
  }

  return (
    <div className="eq-band-control">
      <strong>{capability.label}</strong>
      <label>
        <span>Freq</span>
        <input
          aria-label={`${capability.label} frequency`}
          disabled={!canWrite}
          max={capability.maxFrequencyTenthsHz}
          min={capability.minFrequencyTenthsHz}
          step={capability.frequencyStepTenthsHz}
          type="range"
          value={frequency}
          onBlur={() => commit()}
          onChange={(event) => setFrequency(Number(event.currentTarget.value))}
          onPointerUp={() => commit()}
        />
        <em>{formatFrequency(frequency)}</em>
      </label>
      <label>
        <span>Gain</span>
        <input
          aria-label={`${capability.label} gain`}
          disabled={!canWrite}
          max={capability.maxGainDb}
          min={capability.minGainDb}
          type="range"
          value={gain}
          onBlur={() => commit()}
          onChange={(event) => setGain(Number(event.currentTarget.value))}
          onPointerUp={() => commit()}
        />
        <em>{gain} dB</em>
      </label>
    </div>
  );
}

function GateSection({ device, gate }: { device: DeviceState; gate: NoiseGateState }) {
  const setNoiseGate = useDeviceStore((state) => state.setNoiseGate);
  const sessionGeneration = device.sessionGeneration;
  const canWrite =
    device.capabilities.writableMicrophone &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined;
  const thresholdRange = device.capabilities.gateThresholdRangeDb ?? { min: -59, max: 0 };
  const attenuationRange = device.capabilities.gateAttenuationRangePercent ?? {
    min: 0,
    max: 100
  };
  const [draft, setDraft] = useState(gate);

  useEffect(() => setDraft(gate), [gate]);

  function commit(next: NoiseGateState) {
    setDraft(next);
    if (canWrite && sessionGeneration !== null && sessionGeneration !== undefined) {
      void setNoiseGate(device.identity.id, sessionGeneration, next);
    }
  }

  return (
    <section className="mic-panel dynamics-panel">
      <div className="mic-panel-heading">
        <span className="eyebrow">Gate</span>
        <button
          className={draft.enabled ? "small-toggle active" : "small-toggle"}
          disabled={!canWrite}
          type="button"
          onClick={() => commit({ ...draft, enabled: !draft.enabled })}
        >
          {draft.enabled ? "On" : "Off"}
        </button>
      </div>
      <RangeControl
        disabled={!canWrite}
        label="Threshold"
        max={thresholdRange.max}
        min={thresholdRange.min}
        suffix=" dB"
        value={draft.thresholdDb}
        onCommit={(value) => commit({ ...draft, thresholdDb: value })}
      />
      <RangeControl
        disabled={!canWrite}
        label="Attenuation"
        max={attenuationRange.max}
        min={attenuationRange.min}
        suffix="%"
        value={draft.attenuationPercent}
        onCommit={(value) => commit({ ...draft, attenuationPercent: value })}
      />
      <TimeSelect
        disabled={!canWrite}
        label="Attack"
        options={device.capabilities.gateTimeOptions}
        value={draft.attack}
        onChange={(value) => commit({ ...draft, attack: value })}
      />
      <TimeSelect
        disabled={!canWrite}
        label="Release"
        options={device.capabilities.gateTimeOptions}
        value={draft.release}
        onChange={(value) => commit({ ...draft, release: value })}
      />
    </section>
  );
}

function CompressorSection({
  device,
  compressor
}: {
  device: DeviceState;
  compressor: CompressorState;
}) {
  const setCompressor = useDeviceStore((state) => state.setCompressor);
  const sessionGeneration = device.sessionGeneration;
  const canWrite =
    device.capabilities.writableMicrophone &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined;
  const thresholdRange = device.capabilities.compressorThresholdRangeDb ?? { min: -40, max: 0 };
  const makeupRange = device.capabilities.compressorMakeupGainRangeDb ?? { min: -6, max: 24 };
  const [draft, setDraft] = useState(compressor);

  useEffect(() => setDraft(compressor), [compressor]);

  function commit(next: CompressorState) {
    setDraft(next);
    if (canWrite && sessionGeneration !== null && sessionGeneration !== undefined) {
      void setCompressor(device.identity.id, sessionGeneration, next);
    }
  }

  return (
    <section className="mic-panel dynamics-panel">
      <div className="mic-panel-heading">
        <span className="eyebrow">Compressor</span>
        <strong>{formatRatio(draft.ratio.ratioTenths)}</strong>
      </div>
      <RangeControl
        disabled={!canWrite}
        label="Threshold"
        max={thresholdRange.max}
        min={thresholdRange.min}
        suffix=" dB"
        value={draft.thresholdDb}
        onCommit={(value) => commit({ ...draft, thresholdDb: value })}
      />
      <RatioSelect
        disabled={!canWrite}
        options={device.capabilities.compressorRatioOptions}
        value={draft.ratio}
        onChange={(value) => commit({ ...draft, ratio: value })}
      />
      <TimeSelect
        disabled={!canWrite}
        label="Attack"
        options={device.capabilities.compressorAttackOptions}
        value={draft.attack}
        onChange={(value) => commit({ ...draft, attack: value })}
      />
      <TimeSelect
        disabled={!canWrite}
        label="Release"
        options={device.capabilities.compressorReleaseOptions}
        value={draft.release}
        onChange={(value) => commit({ ...draft, release: value })}
      />
      <RangeControl
        disabled={!canWrite}
        label="Makeup"
        max={makeupRange.max}
        min={makeupRange.min}
        suffix=" dB"
        value={draft.makeupGainDb}
        onCommit={(value) => commit({ ...draft, makeupGainDb: value })}
      />
    </section>
  );
}

function DeEsserSection({ device, deEsser }: { device: DeviceState; deEsser: DeEsserState }) {
  const setDeEsser = useDeviceStore((state) => state.setDeEsser);
  const sessionGeneration = device.sessionGeneration;
  const canWrite =
    device.capabilities.writableMicrophone &&
    device.capabilities.deEsserSupported &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined;
  const range = device.capabilities.deEsserRangePercent ?? { min: 0, max: 100 };
  const [amount, setAmount] = useState(deEsser.amountPercent);

  useEffect(() => setAmount(deEsser.amountPercent), [deEsser.amountPercent]);

  function commit(value: number) {
    if (canWrite && sessionGeneration !== null && sessionGeneration !== undefined) {
      void setDeEsser(device.identity.id, sessionGeneration, { amountPercent: value });
    }
  }

  return (
    <section className="mic-panel de-esser-panel">
      <div className="mic-panel-heading">
        <span className="eyebrow">De-Esser</span>
        <strong>{amount}%</strong>
      </div>
      <input
        aria-label="De-esser amount"
        disabled={!canWrite}
        max={range.max}
        min={range.min}
        type="range"
        value={amount}
        onBlur={() => commit(amount)}
        onChange={(event) => setAmount(Number(event.currentTarget.value))}
        onPointerUp={() => commit(amount)}
      />
    </section>
  );
}

function RangeControl({
  disabled,
  label,
  max,
  min,
  suffix,
  value,
  onCommit
}: {
  disabled: boolean;
  label: string;
  max: number;
  min: number;
  suffix: string;
  value: number;
  onCommit: (value: number) => void;
}) {
  const [draft, setDraft] = useState(value);

  useEffect(() => setDraft(value), [value]);

  return (
    <label className="horizontal-control compact">
      <span>{label}</span>
      <input
        aria-label={label}
        disabled={disabled}
        max={max}
        min={min}
        type="range"
        value={draft}
        onBlur={() => onCommit(draft)}
        onChange={(event) => setDraft(Number(event.currentTarget.value))}
        onPointerUp={() => onCommit(draft)}
      />
      <strong>
        {draft}
        {suffix}
      </strong>
    </label>
  );
}

function TimeSelect({
  disabled,
  label,
  options,
  value,
  onChange
}: {
  disabled: boolean;
  label: string;
  options: TimeOption[];
  value: TimeOption;
  onChange: (value: TimeOption) => void;
}) {
  return (
    <label className="select-control">
      <span>{label}</span>
      <select
        aria-label={label}
        disabled={disabled}
        value={value.index}
        onChange={(event) => {
          const next = options.find((option) => option.index === Number(event.currentTarget.value));
          if (next) {
            onChange(next);
          }
        }}
      >
        {options.map((option) => (
          <option key={option.index} value={option.index}>
            {option.millis} ms
          </option>
        ))}
      </select>
    </label>
  );
}

function RatioSelect({
  disabled,
  options,
  value,
  onChange
}: {
  disabled: boolean;
  options: CompressorRatioOption[];
  value: CompressorRatioOption;
  onChange: (value: CompressorRatioOption) => void;
}) {
  return (
    <label className="select-control">
      <span>Ratio</span>
      <select
        aria-label="Ratio"
        disabled={disabled}
        value={value.index}
        onChange={(event) => {
          const next = options.find((option) => option.index === Number(event.currentTarget.value));
          if (next) {
            onChange(next);
          }
        }}
      >
        {options.map((option) => (
          <option key={option.index} value={option.index}>
            {formatRatio(option.ratioTenths)}
          </option>
        ))}
      </select>
    </label>
  );
}
