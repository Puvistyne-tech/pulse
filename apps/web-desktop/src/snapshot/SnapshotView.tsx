import type { ClientSnapshot, ConnectionPhase } from "@pulse/protocol";

function phaseLabel(connection: ConnectionPhase): string {
  switch (connection) {
    case "disconnected":
      return "Disconnected";
    case "connecting":
      return "Connecting";
    case "handshaking":
      return "Handshaking";
    case "connected":
      return "Connected";
    case "backoff":
      return "Backoff";
  }
}

export function SnapshotView({ snapshot }: { snapshot: ClientSnapshot }) {
  return (
    <main className="mx-auto max-w-md space-y-2 p-6 text-slate-900">
      <h1 className="text-2xl font-semibold">Pulse</h1>
      <p className="text-lg">{phaseLabel(snapshot.connection)}</p>
      <p className="text-sm text-slate-600">
        Protocol version {snapshot.protocol_version}
      </p>
      <p className="text-sm text-slate-600">Attempt {snapshot.attempt}</p>
      {snapshot.retry_in_ms !== null && (
        <p className="text-sm text-slate-600">
          Retrying in {snapshot.retry_in_ms} ms
        </p>
      )}
      {snapshot.rejection && (
        <p className="text-sm text-red-700">
          Version rejected: supported {snapshot.rejection.supported}, received{" "}
          {snapshot.rejection.received}
        </p>
      )}
    </main>
  );
}
