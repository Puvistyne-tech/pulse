import { useEffect, useState } from "react";
import { MessagesSquare } from "lucide-react";
import type {
  SubscriptionState,
  ThreadSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { StewardMentionPicker } from "../steward/StewardMentionPicker";
import { Panel } from "../ui/Panel";
import { ui } from "../ui/styles";
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
    <Panel
      as="aside"
      label="Thread"
      title={
        <>
          <MessagesSquare size={16} aria-hidden="true" className="text-accent" />
          Thread
        </>
      }
      closeLabel="Close thread"
      onClose={() => void intents.closeThread()}
    >
      <form
        className="flex gap-2"
        onSubmit={(event) => {
          event.preventDefault();
          void intents.setTopicTitle(roomId, rootId, titleDraft);
        }}
      >
        <input
          aria-label="Topic title"
          className={ui.input}
          placeholder="Add a topic title"
          type="text"
          value={titleDraft}
          onChange={(event) => setTitleDraft(event.target.value)}
        />
        <button className={`${ui.secondaryButton} shrink-0`} type="submit">
          Save title
        </button>
      </form>
      <div role="group" aria-label="Subscription" className="flex gap-1.5">
        {subscriptionStates.map(({ state, label }) => (
          <button
            key={state}
            type="button"
            aria-pressed={thread.subscription === state}
            className={ui.toggleButton}
            onClick={() =>
              void intents.setThreadSubscription(roomId, rootId, state)
            }
          >
            {label}
          </button>
        ))}
      </div>
      {thread.syncing && <p className={ui.muted}>Syncing</p>}
      <ul className="-mx-4 space-y-1 border-y border-line py-3">
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
    </Panel>
  );
}
