import { invoke } from "@tauri-apps/api/core";
import type { ClientSnapshot } from "@pulse/protocol";

export function loadSnapshot(): Promise<ClientSnapshot> {
  return invoke<ClientSnapshot>("initial_snapshot");
}
