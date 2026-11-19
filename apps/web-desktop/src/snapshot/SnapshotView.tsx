import type {
  ConnectionPhase,
  ConnectionSnapshot,
  HandshakeRejection,
} from "@pulse/protocol";

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

const phaseDots: Record<ConnectionPhase, string> = {
  disconnected: "bg-side-subtle",
  connecting: "bg-warning animate-pulse",
  handshaking: "bg-warning animate-pulse",
  connected: "bg-success",
  backoff: "bg-danger animate-pulse",
};

const handshakeRejectionLabels: Record<HandshakeRejection, string> = {
  unauthenticated: "Handshake rejected: unauthenticated",
  campus_unavailable: "Handshake rejected: campus unavailable",
};

export function SnapshotView({ network }: { network: ConnectionSnapshot }) {
  return (
    <details className="group text-xs text-side-subtle">
      <summary className="flex cursor-pointer list-none items-center gap-2 rounded-md px-1 py-0.5 hover:text-side-ink">
        <span className={`h-2 w-2 rounded-full ${phaseDots[network.connection]}`} aria-hidden="true" />
        <span>{phaseLabel(network.connection)}</span>
      </summary>
      <div className="mt-1.5 space-y-0.5 rounded-md bg-white/5 px-2 py-1.5">
        <p>Protocol version {network.protocol_version}</p>
        <p>Attempt {network.attempt}</p>
        {network.retry_in_ms !== null && (
          <p>Retrying in {network.retry_in_ms} ms</p>
        )}
        {network.rejection && (
          <p className="text-danger">
            Version rejected: supported {network.rejection.supported}, received{" "}
            {network.rejection.received}
          </p>
        )}
        {network.handshake_rejection && (
          <p className="text-danger">
            {handshakeRejectionLabels[network.handshake_rejection]}
          </p>
        )}
      </div>
    </details>
  );
}
