import type { ClientSnapshot } from "@pulse/protocol";

const base: ClientSnapshot = {
  protocol_version: 1,
  connection: { phase: "disconnected" },
  campus_id: null,
  session_id: null,
  attempt: 0,
  rejection: null,
};

export const snapshotFixtures: ReadonlyArray<[string, ClientSnapshot, string]> = [
  ["disconnected", base, "Disconnected"],
  ["connecting", { ...base, connection: { phase: "connecting" } }, "Connecting"],
  ["handshaking", { ...base, connection: { phase: "handshaking" } }, "Handshaking"],
  ["connected", { ...base, connection: { phase: "connected" } }, "Connected"],
  [
    "backoff",
    { ...base, connection: { phase: "backoff", retry_in_ms: 1500 }, attempt: 2 },
    "Retrying in 1500 ms",
  ],
  [
    "rejected",
    { ...base, rejection: { supported: 1, received: 99 } },
    "Version rejected: supported 1, received 99",
  ],
];
