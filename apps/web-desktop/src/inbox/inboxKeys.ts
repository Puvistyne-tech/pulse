import type { InboxSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";

export type InboxKeyIntent =
  | "toggle_inbox"
  | "next"
  | "previous"
  | "archive"
  | "reply"
  | "cancel_reply";

export interface KeyInput {
  key: string;
  metaKey: boolean;
  ctrlKey: boolean;
  editableTarget: boolean;
}

const openInboxKeys: Record<string, InboxKeyIntent> = {
  j: "next",
  k: "previous",
  e: "archive",
  r: "reply",
};

export function inboxKeyIntent(
  input: KeyInput,
  inbox: InboxSnapshot,
): InboxKeyIntent | null {
  const key = input.key.toLowerCase();
  if (input.metaKey || input.ctrlKey) {
    return key === "i" ? "toggle_inbox" : null;
  }
  if (input.key === "Escape") {
    return inbox.replying ? "cancel_reply" : null;
  }
  if (inbox.open && !input.editableTarget) {
    return openInboxKeys[key] ?? null;
  }
  return null;
}

export function runInboxKeyIntent(
  intent: InboxKeyIntent,
  intents: PulseIntents,
): Promise<void> {
  switch (intent) {
    case "toggle_inbox":
      return intents.toggleInbox();
    case "next":
      return intents.moveInboxSelection("next");
    case "previous":
      return intents.moveInboxSelection("previous");
    case "archive":
      return intents.archiveInboxSelection();
    case "reply":
      return intents.startInboxReply();
    case "cancel_reply":
      return intents.cancelInboxReply();
  }
}
