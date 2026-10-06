import { Activity, Cable, Cog, SlidersHorizontal } from "lucide-react";
import { useEffect, useState } from "react";
import { AppShell, NavItem } from "./components/AppShell";
import { DashboardView } from "./features/device/DashboardView";
import { DeviceView } from "./features/device/DeviceView";
import { MixerView } from "./features/mixer/MixerView";
import { SettingsView } from "./features/settings/SettingsView";
import { useBackendEvents } from "./hooks/useBackendEvents";
import { useDeviceStore } from "./stores/deviceStore";

type ViewKey = "dashboard" | "mixer" | "device" | "settings";

const navItems: NavItem<ViewKey>[] = [
  { key: "dashboard", label: "Dashboard", icon: Activity },
  { key: "mixer", label: "Mixer", icon: SlidersHorizontal },
  { key: "device", label: "Device", icon: Cable },
  { key: "settings", label: "Settings", icon: Cog }
];

export function App() {
  const [activeView, setActiveView] = useState<ViewKey>("dashboard");
  const hydrate = useDeviceStore((state) => state.hydrate);
  const loading = useDeviceStore((state) => state.loading);
  const error = useDeviceStore((state) => state.error);

  useBackendEvents();

  useEffect(() => {
    void hydrate();
  }, [hydrate]);

  return (
    <AppShell
      title="FriesXLR"
      subtitle="GoXLR Control"
      icon={SlidersHorizontal}
      items={navItems}
      activeKey={activeView}
      onSelect={setActiveView}
      loading={loading}
      error={error}
    >
      {activeView === "dashboard" && <DashboardView />}
      {activeView === "mixer" && <MixerView />}
      {activeView === "device" && <DeviceView />}
      {activeView === "settings" && <SettingsView />}
    </AppShell>
  );
}
