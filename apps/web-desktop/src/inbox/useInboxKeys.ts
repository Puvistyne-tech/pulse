import { useEffect } from "react";
import type { InboxSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { inboxKeyIntent, runInboxKeyIntent } from "./inboxKeys";

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

export function useInboxKeys(inbox: InboxSnapshot, intents: PulseIntents) {
  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      const intent = inboxKeyIntent(
        {
          key: event.key,
          metaKey: event.metaKey,
          ctrlKey: event.ctrlKey,
          editableTarget: isEditable(event.target),
        },
        inbox,
      );
      if (intent === null) {
        return;
      }
      event.preventDefault();
      void runInboxKeyIntent(intent, intents);
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [inbox, intents]);
}
