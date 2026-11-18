import type { InboxSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { Composer } from "../room/Composer";

interface InboxViewProps {
  inbox: InboxSnapshot;
  intents: PulseIntents;
}

export function InboxView({ inbox, intents }: InboxViewProps) {
  return (
    <section aria-label="Inbox" className="space-y-3">
      <header className="flex items-center justify-between">
        <h2 className="font-semibold">
          Inbox <span aria-label="Unread threads">{inbox.unread_threads}</span>
        </h2>
        <button
          className="text-sm underline"
          type="button"
          onClick={() => void intents.toggleInbox()}
        >
          Close inbox
        </button>
      </header>
      <ul className="space-y-1">
        {inbox.items.map((item) => {
          const selected = item.root_message_id === inbox.selected_root_message_id;
          return (
            <li
              key={item.root_message_id}
              aria-selected={selected}
              className="space-y-1 rounded border border-slate-200 p-2 aria-selected:bg-slate-100"
            >
              <div className="flex items-center gap-2">
                <strong className="flex-1">
                  {item.topic_title ?? item.root_preview}
                </strong>
                {item.unread_mentions > 0 && (
                  <span aria-label="Unread mentions">@</span>
                )}
                {item.unread_replies > 0 && (
                  <span aria-label="Unread replies">{item.unread_replies}</span>
                )}
                <span className="text-xs text-slate-500">
                  {item.subscription}
                </span>
              </div>
              <p className="text-xs text-slate-600">
                {item.root_author_display_name}: {item.root_preview}
              </p>
              {item.latest_reply !== null && (
                <p className="text-xs text-slate-600">
                  {item.latest_reply.author_display_name}:{" "}
                  {item.latest_reply.preview}
                </p>
              )}
              {selected && (
                <div className="flex gap-2 text-xs">
                  <button
                    className="underline"
                    type="button"
                    onClick={() => void intents.archiveInboxSelection()}
                  >
                    Archive
                  </button>
                  <button
                    className="underline"
                    type="button"
                    onClick={() => void intents.startInboxReply()}
                  >
                    Reply
                  </button>
                </div>
              )}
              {selected && inbox.replying && (
                <Composer
                  label="Inbox reply"
                  onSend={(body) =>
                    void intents.sendReply(
                      item.room_id,
                      item.root_message_id,
                      body,
                    )
                  }
                />
              )}
            </li>
          );
        })}
      </ul>
    </section>
  );
}
