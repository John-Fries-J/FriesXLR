import type { LucideIcon } from "lucide-react";
import type { ReactNode } from "react";

export interface NavItem<TKey extends string> {
  key: TKey;
  label: string;
  icon: LucideIcon;
}

interface AppShellProps<TKey extends string> {
  title: string;
  subtitle: string;
  icon: LucideIcon;
  items: NavItem<TKey>[];
  activeKey: TKey;
  onSelect: (key: TKey) => void;
  loading: boolean;
  error: string | null;
  children: ReactNode;
}

export function AppShell<TKey extends string>({
  title,
  subtitle,
  icon: Icon,
  items,
  activeKey,
  onSelect,
  loading,
  error,
  children
}: AppShellProps<TKey>) {
  return (
    <div className="app-frame">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true">
            <Icon size={20} />
          </div>
          <div>
            <h1>{title}</h1>
            <span>{subtitle}</span>
          </div>
        </div>

        <nav className="nav-list" aria-label="Primary">
          {items.map((item) => {
            const ItemIcon = item.icon;
            const isActive = item.key === activeKey;

            return (
              <button
                key={item.key}
                className={isActive ? "nav-item active" : "nav-item"}
                type="button"
                onClick={() => onSelect(item.key)}
              >
                <ItemIcon size={17} />
                <span>{item.label}</span>
              </button>
            );
          })}
        </nav>
      </aside>

      <main className="main-surface">
        <header className="topbar">
          <div className={loading ? "sync-dot active" : "sync-dot"} />
          <span>{loading ? "Syncing" : "Ready"}</span>
          {error && <strong>{error}</strong>}
        </header>
        {children}
      </main>
    </div>
  );
}

