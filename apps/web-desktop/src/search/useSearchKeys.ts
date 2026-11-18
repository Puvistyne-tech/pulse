import { useEffect } from "react";
import type { SearchSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { runSearchKeyIntent, searchKeyIntent } from "./searchKeys";

function isEditable(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) {
    return false;
  }
  return (
    target.isContentEditable ||
    target.tagName === "INPUT" ||
    target.tagName === "TEXTAREA" ||
    target.tagName === "SELECT"
  );
}

export function useSearchKeys(search: SearchSnapshot, intents: PulseIntents) {
  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      const intent = searchKeyIntent(
        {
          key: event.key,
          metaKey: event.metaKey,
          ctrlKey: event.ctrlKey,
          editableTarget: isEditable(event.target),
        },
        search,
      );
      if (intent === null) {
        return;
      }
      event.preventDefault();
      void runSearchKeyIntent(intent, intents);
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [search, intents]);
}
