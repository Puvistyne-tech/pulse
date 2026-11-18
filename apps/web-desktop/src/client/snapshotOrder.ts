import type { ClientSnapshot } from "@pulse/protocol";

export function newerSnapshot(
  current: ClientSnapshot | null,
  next: ClientSnapshot,
): ClientSnapshot {
  return current !== null && current.sequence > next.sequence ? current : next;
}
