import { Cable, SlidersHorizontal } from "lucide-react";
import { EmptyState } from "../../components/EmptyState";
import { StatusPill } from "../../components/StatusPill";
import { useDeviceStore } from "../../stores/deviceStore";
import { formatDeviceModel } from "./format";

export function DashboardView() {
  const snapshot = useDeviceStore((state) => state.snapshot);
  const devices = snapshot?.devices ?? [];

  if (devices.length === 0) {
    return <EmptyState />;
  }

  const primary =
    devices.find((device) => device.identity.id === snapshot?.selectedDeviceId) ?? devices[0];

  return (
    <section className="dashboard">
      <div className="dashboard-hero">
        <div>
          <span className="eyebrow">Primary device</span>
          <h2>{formatDeviceModel(primary.identity.model)}</h2>
          <p>{primary.identity.serialNumber ?? "Serial unavailable"}</p>
        </div>
        <StatusPill tone="good">connected</StatusPill>
      </div>

      <div className="metric-row">
        <div className="metric">
          <Cable size={18} />
          <span>Devices</span>
          <strong>{devices.length}</strong>
        </div>
        <div className="metric">
          <SlidersHorizontal size={18} />
          <span>Readable faders</span>
          <strong>
            {primary.faders.filter((fader) => fader.volume !== null && fader.volume !== undefined)
              .length}
          </strong>
        </div>
      </div>
    </section>
  );
}
