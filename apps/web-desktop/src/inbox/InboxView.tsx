import { Archive, AtSign, Inbox, Reply } from "lucide-react";
import type { InboxSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { Composer } from "../room/Composer";
import { Avatar } from "../ui/Avatar";
import { Panel } from "../ui/Panel";
import { ui } from "../ui/styles";

interface InboxViewProps {
  inbox: InboxSnapshot;
  intents: PulseIntents;
}

export function InboxView({ inbox, intents }: InboxViewProps) {
  return (
    <Panel
      label="Inbox"
      title={
        <>
          <Inbox size={16} aria-hidden="true" className="text-accent" />
          Inbox
          <span aria-label="Unread threads" className={`${ui.pill} bg-accent-soft text-accent`}>
            {inbox.unread_threads}
          </span>
        </>
      }
      closeLabel="Close inbox"
      onClose={() => void intents.toggleInbox()}
    >
      {inbox.items.length === 0 && (
        <p className="py-10 text-center text-sm text-subtle">You are all caught up.</p>
      )}
      <ul className="space-y-2">
        {inbox.items.map((item) => {
          const selected = item.root_message_id === inbox.selected_root_message_id;
          return (
            <li
              key={item.root_message_id}
              aria-selected={selected}
              className="space-y-2 rounded-xl border border-line p-3 transition-colors aria-selected:border-accent aria-selected:bg-accent-soft/60"
            >
              <div className="flex items-center gap-2">
                <strong className="flex-1 truncate text-sm font-semibold text-ink">
                  {item.topic_title ?? item.root_preview}
                </strong>
                {item.unread_mentions > 0 && (
                  <span aria-label="Unread mentions" className={`${ui.pill} bg-danger text-white`}>@</span>
                )}
                {item.unread_replies > 0 && (
                  <span aria-label="Unread replies" className={`${ui.pill} bg-accent text-on-accent`}>
                    {item.unread_replies}
                  </span>
                )}
                <span className="text-[11px] uppercase tracking-wide text-subtle">
                  {item.subscription}
                </span>
              </div>
              <p className="flex items-center gap-2 text-xs text-subtle">
                <Avatar name={item.root_author_display_name} size="sm" />
                <span className="truncate">
                  {item.root_author_display_name}: {item.root_preview}
                </span>
              </p>
              {item.latest_reply !== null && (
                <p className="truncate border-l-2 border-line pl-2 text-xs text-subtle">
                  {item.latest_reply.author_display_name}:{" "}
                  {item.latest_reply.preview}
                </p>
              )}
              {selected && (
                <div className="flex gap-2">
                  <button
                    className={ui.secondaryButton}
                    type="button"
                    onClick={() => void intents.archiveInboxSelection()}
                  >
                    <Archive size={14} aria-hidden="true" />
                    Archive
                  </button>
                  <button
                    className={ui.secondaryButton}
                    type="button"
                    onClick={() => void intents.startInboxReply()}
                  >
                    <Reply size={14} aria-hidden="true" />
                    Reply
                  </button>
                </div>
              )}
              {selected && inbox.replying && (
                <Composer
                  label="Inbox reply"
                  placeholder="Reply to this thread"
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
      {inbox.items.length > 0 && (
        <p className="flex items-center gap-1 text-[11px] text-subtle">
          <AtSign size={11} aria-hidden="true" />
          J and K move, E archives, R replies.
        </p>
      )}
    </Panel>
  );
}
