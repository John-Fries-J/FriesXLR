import { FolderOpen, Power } from "lucide-react";
import { Toggle } from "../../components/Toggle";
import { useDeviceStore } from "../../stores/deviceStore";

export function SettingsView() {
  const snapshot = useDeviceStore((state) => state.snapshot);
  const setMockDeviceEnabled = useDeviceStore((state) => state.setMockDeviceEnabled);
  const setStartMinimized = useDeviceStore((state) => state.setStartMinimized);
  const setLaunchAtStartupEnabled = useDeviceStore(
    (state) => state.setLaunchAtStartupEnabled
  );
  const quitApplication = useDeviceStore((state) => state.quitApplication);
  const settings = snapshot?.settings;

  return (
    <section className="settings-view">
      <div className="section-heading">
        <div>
          <span className="eyebrow">Application</span>
          <h2>Settings</h2>
        </div>
      </div>

      <div className="settings-list">
        <Toggle
          checked={settings?.mockDeviceEnabled ?? false}
          disabled={!settings}
          label="Mock device"
          onChange={(enabled) => void setMockDeviceEnabled(enabled)}
        />

        <Toggle
          checked={settings?.startMinimized ?? false}
          disabled={!settings}
          label="Start minimized"
          onChange={(enabled) => void setStartMinimized(enabled)}
        />

        <Toggle
          checked={settings?.launchAtStartup ?? false}
          disabled={!settings}
          label="Launch with Windows"
          onChange={(enabled) => void setLaunchAtStartupEnabled(enabled)}
        />

        <div className="settings-path">
          <FolderOpen size={17} />
          <span>Config</span>
          <code>{settings?.configPath ?? "Not available"}</code>
        </div>

        <button className="quit-button" type="button" onClick={() => void quitApplication()}>
          <Power size={17} />
          <span>Quit FriesXLR</span>
        </button>
      </div>
    </section>
  );
}
