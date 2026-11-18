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

const handshakeRejectionLabels: Record<HandshakeRejection, string> = {
  unauthenticated: "Handshake rejected: unauthenticated",
  campus_unavailable: "Handshake rejected: campus unavailable",
};

export function SnapshotView({ network }: { network: ConnectionSnapshot }) {
  return (
    <section className="space-y-1 text-slate-900">
      <p className="text-lg">{phaseLabel(network.connection)}</p>
      <p className="text-sm text-slate-600">
        Protocol version {network.protocol_version}
      </p>
      <p className="text-sm text-slate-600">Attempt {network.attempt}</p>
      {network.retry_in_ms !== null && (
        <p className="text-sm text-slate-600">
          Retrying in {network.retry_in_ms} ms
        </p>
      )}
      {network.rejection && (
        <p className="text-sm text-red-700">
          Version rejected: supported {network.rejection.supported}, received{" "}
          {network.rejection.received}
        </p>
      )}
      {network.handshake_rejection && (
        <p className="text-sm text-red-700">
          {handshakeRejectionLabels[network.handshake_rejection]}
        </p>
      )}
    </section>
  );
}
