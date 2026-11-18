import { useEffect, useState } from "react";
import type { ClientSnapshot } from "@pulse/protocol";
import { loadSnapshot } from "./snapshot/loadSnapshot";
import { SnapshotView } from "./snapshot/SnapshotView";

export function App() {
  const [snapshot, setSnapshot] = useState<ClientSnapshot | null>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    loadSnapshot().then(setSnapshot, () => setFailed(true));
  }, []);

  if (failed) {
    return <main className="p-6 text-red-700">Pulse core unavailable</main>;
  }

  if (!snapshot) {
    return <main className="p-6 text-slate-500">Loading Pulse</main>;
  }

  return <SnapshotView snapshot={snapshot} />;
}
