import { AppView } from "./AppView";
import { usePulseClient } from "./client/usePulseClient";

export function App() {
  const { snapshot, failed, intents } = usePulseClient();

  if (failed) {
    return <main className="p-6 text-red-700">Pulse core unavailable</main>;
  }

  if (!snapshot) {
    return <main className="p-6 text-slate-500">Loading Pulse</main>;
  }

  return <AppView snapshot={snapshot} intents={intents} />;
}
