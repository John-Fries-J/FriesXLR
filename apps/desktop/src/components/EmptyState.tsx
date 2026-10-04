import { Cable } from "lucide-react";

export function EmptyState() {
  return (
    <section className="empty-state">
      <Cable size={28} />
      <h2>No GoXLR detected</h2>
      <p>Connect a GoXLR or GoXLR Mini, or enable the mock device in Settings.</p>
    </section>
  );
}

