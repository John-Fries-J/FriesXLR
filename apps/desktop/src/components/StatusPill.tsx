interface StatusPillProps {
  tone: "good" | "warn" | "neutral";
  children: string;
}

export function StatusPill({ tone, children }: StatusPillProps) {
  return <span className={`status-pill ${tone}`}>{children}</span>;
}

