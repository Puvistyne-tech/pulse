import { useEffect, useState } from "react";
import type {
  SubscriptionState,
  ThreadSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { StewardMentionPicker } from "../steward/StewardMentionPicker";
import { Composer, useInsertion } from "./Composer";
import { MessageEntry } from "./MessageEntry";

const subscriptionStates: { state: SubscriptionState; label: string }[] = [
  { state: "following", label: "Following" },
  { state: "muted", label: "Muted" },
  { state: "archived", label: "Archived" },
];

interface ThreadPanelProps {
  thread: ThreadSnapshot;
  palette: string[];
  intents: PulseIntents;
}

export function ThreadPanel({
  thread,
  palette,
  intents,
}: ThreadPanelProps) {
  const [insertion, mention] = useInsertion();
  const [titleDraft, setTitleDraft] = useState(thread.topic_title ?? "");
  const roomId = thread.room_id;
  const rootId = thread.root_message_id;

  useEffect(() => {
    setTitleDraft(thread.topic_title ?? "");
  }, [thread.topic_title, rootId]);

  if (roomId === null || rootId === null) {
    return null;
  }

  const actions = {
    react: intents.react,
    discardMessage: intents.discardMessage,
    mention,
  };
  const entries = thread.root === null ? thread.replies : [thread.root, ...thread.replies];

  return (
    <aside aria-label="Thread" className="space-y-3 border-l border-slate-200 p-4">
      <header className="flex items-center justify-between">
        <h2 className="font-semibold">Thread</h2>
        <button
          className="text-sm underline"
          type="button"
          onClick={() => void intents.closeThread()}
        >
          Close thread
        </button>
      </header>
      <form
        className="flex gap-2"
        onSubmit={(event) => {
          event.preventDefault();
          void intents.setTopicTitle(roomId, rootId, titleDraft);
        }}
      >
        <input
          aria-label="Topic title"
          className="flex-1 rounded border border-slate-300 px-2 py-1"
          type="text"
          value={titleDraft}
          onChange={(event) => setTitleDraft(event.target.value)}
        />
        <button className="rounded border px-2 py-1" type="submit">
          Save title
        </button>
      </form>
      <div role="group" aria-label="Subscription" className="flex gap-2">
        {subscriptionStates.map(({ state, label }) => (
          <button
            key={state}
            type="button"
            aria-pressed={thread.subscription === state}
            className="rounded border px-2 py-1 text-sm aria-pressed:bg-slate-200"
            onClick={() =>
              void intents.setThreadSubscription(roomId, rootId, state)
            }
          >
            {label}
          </button>
        ))}
      </div>
      {thread.syncing && <p className="text-xs text-slate-500">Syncing</p>}
      <ul className="space-y-2">
        {entries.map((entry) => (
          <MessageEntry
            key={entry.message_id ?? entry.client_nonce}
            entry={entry}
            roomId={roomId}
            palette={palette}
            actions={actions}
          />
        ))}
      </ul>
      <StewardMentionPicker
        stewards={thread.mentionable_stewards}
        onPick={mention}
      />
      <Composer
        label="Reply"
        insertion={insertion}
        onSend={(body) => void intents.sendReply(roomId, rootId, body)}
      />
    </aside>
  );
}
