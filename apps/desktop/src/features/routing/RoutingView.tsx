import { Cable, Check, Minus } from "lucide-react";
import { EmptyState } from "../../components/EmptyState";
import { StatusPill } from "../../components/StatusPill";
import { useDeviceStore } from "../../stores/deviceStore";
import type { DeviceState, RoutingRoute } from "../../types/backend";
import {
  formatDeviceModel,
  formatRoutingInput,
  formatRoutingOutput
} from "../device/format";

export function RoutingView() {
  const snapshot = useDeviceStore((state) => state.snapshot);
  const setSelectedDevice = useDeviceStore((state) => state.setSelectedDevice);
  const devices = snapshot?.devices ?? [];

  if (devices.length === 0) {
    return <EmptyState />;
  }

  const selectedId = snapshot?.selectedDeviceId ?? devices[0]?.identity.id;
  const selectedDevice = devices.find((device) => device.identity.id === selectedId) ?? devices[0];

  return (
    <section className="routing-view">
      <div className="section-heading mixer-heading">
        <div>
          <span className="eyebrow">Routing</span>
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

      <RoutingMatrix device={selectedDevice} />
    </section>
  );
}

function RoutingMatrix({ device }: { device: DeviceState }) {
  const setRoutingRoute = useDeviceStore((state) => state.setRoutingRoute);
  const routing = device.routing;
  const sessionGeneration = device.sessionGeneration;
  const canRead = device.capabilities.readableRouting && routing !== null && routing !== undefined;
  const canWrite =
    device.capabilities.writableRouting &&
    sessionGeneration !== null &&
    sessionGeneration !== undefined &&
    canRead;
  const supportedRoutes = new Set(
    device.capabilities.supportedRoutes.map((route) => routeKey(route))
  );
  const routeStates = new Map(
    routing?.routes.map((state) => [routeKey(state.route), state.enabled]) ?? []
  );

  return (
    <div className="routing-matrix-wrap">
      <table className="routing-matrix">
        <thead>
          <tr>
            <th scope="col">Source</th>
            {device.capabilities.supportedRoutingOutputs.map((output) => (
              <th key={output} scope="col">
                {formatRoutingOutput(output)}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {device.capabilities.supportedRoutingInputs.map((input) => (
            <tr key={input}>
              <th scope="row">{formatRoutingInput(input)}</th>
              {device.capabilities.supportedRoutingOutputs.map((output) => {
                const route = { input, output };
                const supported = supportedRoutes.has(routeKey(route));
                const enabled = routeStates.get(routeKey(route)) === true;
                const disabled = !supported || !canWrite;
                const label = `${formatRoutingInput(input)} to ${formatRoutingOutput(output)}`;

                return (
                  <td key={output}>
                    <button
                      aria-label={label}
                      className={enabled ? "route-cell active" : "route-cell"}
                      disabled={disabled}
                      type="button"
                      onClick={() => {
                        if (
                          sessionGeneration !== null &&
                          sessionGeneration !== undefined &&
                          supported
                        ) {
                          void setRoutingRoute(
                            device.identity.id,
                            sessionGeneration,
                            route,
                            !enabled
                          );
                        }
                      }}
                    >
                      {supported ? (
                        enabled ? (
                          <Check size={15} />
                        ) : (
                          <span className="route-dot" />
                        )
                      ) : (
                        <Minus size={14} />
                      )}
                    </button>
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
      {!canRead && (
        <div className="routing-disabled">
          Routing controls are unavailable until FriesXLR has authoritative profile state for this
          device.
        </div>
      )}
    </div>
  );
}

function routeKey(route: RoutingRoute) {
  return `${route.input}:${route.output}`;
}
