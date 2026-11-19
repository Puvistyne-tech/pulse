import { AppView } from "./AppView";
import { usePulseClient } from "./client/usePulseClient";
import { PulseMark } from "./ui/PulseMark";

function Splash({ message, tone }: { message: string; tone: string }) {
  return (
    <main className="flex h-full flex-col items-center justify-center gap-3 bg-canvas">
      <PulseMark size={44} />
      <p className={`text-sm ${tone}`}>{message}</p>
    </main>
  );
}

export function App() {
  const { snapshot, failed, intents } = usePulseClient();

  if (failed) {
    return <Splash message="Pulse core unavailable" tone="text-danger" />;
  }

  if (!snapshot) {
    return <Splash message="Loading Pulse" tone="animate-pulse text-subtle" />;
  }

  return <AppView snapshot={snapshot} intents={intents} />;
}
