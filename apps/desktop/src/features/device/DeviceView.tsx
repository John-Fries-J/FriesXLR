import { Cpu, Fingerprint, RadioTower, Usb } from "lucide-react";
import { EmptyState } from "../../components/EmptyState";
import { StatusPill } from "../../components/StatusPill";
import { useDeviceStore } from "../../stores/deviceStore";
import type { DeviceState } from "../../types/backend";
import { FaderStrip } from "./FaderStrip";
import { formatDeviceModel, formatVersion } from "./format";

export function DeviceView() {
  const devices = useDeviceStore((state) => state.snapshot?.devices ?? []);

  if (devices.length === 0) {
    return <EmptyState />;
  }

  return (
    <section className="view-stack">
      {devices.map((device) => (
        <DevicePanel key={device.identity.id} device={device} />
      ))}
    </section>
  );
}

function DevicePanel({ device }: { device: DeviceState }) {
  return (
    <article className="device-panel">
      <div className="section-heading">
        <div>
          <span className="eyebrow">Device</span>
          <h2>{formatDeviceModel(device.identity.model)}</h2>
        </div>
        <StatusPill tone={device.status === "connected" ? "good" : "warn"}>
          {device.status}
        </StatusPill>
      </div>

      <dl className="device-grid">
        <Info icon={Usb} label="Product" value={device.identity.productName ?? "Unknown"} />
        <Info
          icon={Fingerprint}
          label="Serial"
          value={device.identity.serialNumber ?? "Not available"}
        />
        <Info
          icon={Cpu}
          label="Firmware"
          value={formatVersion(device.identity.firmwareVersion)}
        />
        <Info
          icon={RadioTower}
          label="Driver"
          value={driverLabel(device)}
        />
      </dl>

      <div className="fader-grid">
        {device.faders.map((fader) => (
          <FaderStrip key={fader.name} fader={fader} />
        ))}
      </div>
    </article>
  );
}

function Info({
  icon: Icon,
  label,
  value
}: {
  icon: typeof Usb;
  label: string;
  value: string;
}) {
  return (
    <div className="info-line">
      <Icon size={17} />
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function driverLabel(device: DeviceState) {
  const version = formatVersion(device.identity.driverVersion);
  const name = device.identity.driverInterface ?? "Unknown";
  return version === "Not available" ? name : `${name} ${version}`;
}

