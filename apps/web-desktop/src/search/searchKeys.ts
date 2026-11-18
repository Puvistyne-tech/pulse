import type { SearchSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import type { KeyInput } from "../inbox/inboxKeys";

export type SearchKeyIntent =
  | "toggle_search"
  | "next"
  | "previous"
  | "open"
  | "close";

const openSearchKeys: Record<string, SearchKeyIntent> = {
  ArrowDown: "next",
  ArrowUp: "previous",
  Enter: "open",
  Escape: "close",
};

export function searchKeyIntent(
  input: KeyInput,
  search: SearchSnapshot,
): SearchKeyIntent | null {
  if (input.metaKey || input.ctrlKey) {
    return input.key.toLowerCase() === "k" ? "toggle_search" : null;
  }
  if (!search.open) {
    return null;
  }
  return openSearchKeys[input.key] ?? null;
}

export function runSearchKeyIntent(
  intent: SearchKeyIntent,
  intents: PulseIntents,
): Promise<void> {
  switch (intent) {
    case "toggle_search":
    case "close":
      return intents.toggleSearch();
    case "next":
      return intents.moveSearchSelection("next");
    case "previous":
      return intents.moveSearchSelection("previous");
    case "open":
      return intents.openSearchSelection();
  }
}
