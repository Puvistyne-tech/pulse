import { useEffect, useRef } from "react";
import { AtSign, MessageSquareReply, Pin } from "lucide-react";
import type {
  ClientNonce,
  MessageRejection,
  MessageId,
  ReminderPreset,
  RoomId,
  StreamState,
  TimelineEntry,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ShelfMenu } from "../shelf/ShelfMenu";
import { Avatar } from "../ui/Avatar";
import { ui } from "../ui/styles";
import { AttachmentList } from "./AttachmentList";
import { LatestReplyBar } from "./LatestReplyBar";
import { MessageBody } from "./MessageBody";
import { Reactions } from "./Reactions";

const rejectionLabels: Record<MessageRejection, string> = {
  forbidden: "Not allowed to post here",
  invalid_body: "Message not accepted",
  unknown_room: "Room not found",
  unknown_thread: "Thread not found",
  invalid_attachment: "Attachment not accepted",
};

const streamLabels: Record<StreamState, string> = {
  streaming: "Streaming",
  finalizing: "Finalizing",
};

export interface EntryActions {
  react: PulseIntents["react"];
  discardMessage(clientNonce: ClientNonce): Promise<void>;
  mention(token: string): void;
  openThread?(roomId: RoomId, rootMessageId: MessageId): Promise<void>;
  setPin?: PulseIntents["setPin"];
  updateShelf?: PulseIntents["updateShelf"];
  canPin?: boolean;
  presets?: ReminderPreset[];
  customTime?: boolean;
}

interface MessageEntryProps {
  entry: TimelineEntry;
  roomId: RoomId;
  palette: string[];
  actions: EntryActions;
  focused?: boolean;
}

function EntryStatus({
  entry,
  discardMessage,
}: {
  entry: TimelineEntry;
  discardMessage(clientNonce: ClientNonce): Promise<void>;
}) {
  if (entry.delivery === "pending") {
    return <span className="text-xs text-subtle">Sending</span>;
  }
  if (entry.delivery === "failed") {
    return (
      <span className="inline-flex items-center gap-2 text-xs text-danger">
        {entry.rejection === null ? "Failed" : rejectionLabels[entry.rejection]}
        <button
          className="font-medium underline-offset-2 hover:underline"
          type="button"
          onClick={() => void discardMessage(entry.client_nonce)}
        >
          Discard
        </button>
      </span>
    );
  }
  return null;
}

export function MessageEntry({
  entry,
  roomId,
  palette,
  actions,
  focused = false,
}: MessageEntryProps) {
  const itemRef = useRef<HTMLLIElement>(null);

  useEffect(() => {
    if (focused) {
      itemRef.current?.scrollIntoView?.({ block: "center" });
    }
  }, [focused]);

  const messageId = entry.message_id;
  const thread = entry.thread;
  const openThread = actions.openThread;
  const author = entry.from_self ? "You" : entry.author_display_name;

  return (
    <li
      ref={itemRef}
      data-delivery={entry.delivery}
      data-focused={focused}
      data-stream={entry.stream ?? undefined}
      className={`group relative flex gap-3 px-5 py-2 transition-colors hover:bg-muted/70 ${
        focused ? "bg-warning-soft" : ""
      } ${entry.delivery === "pending" ? "opacity-70" : ""} ${
        entry.pinned ? "border-l-2 border-warning" : ""
      }`}
    >
      <Avatar name={entry.author_display_name} />
      <div className="min-w-0 flex-1 space-y-1.5">
        <div className="flex flex-wrap items-baseline gap-x-2 gap-y-0.5">
          <span className="text-sm font-semibold text-ink">{author}</span>
          {entry.author_is_steward && (
            <span className={`${ui.pill} bg-accent-soft text-accent`}>Steward</span>
          )}
          {entry.pinned && (
            <span className={`${ui.pill} gap-1 bg-warning-soft text-warning`}>
              <Pin size={10} aria-hidden="true" />
              Pinned
            </span>
          )}
          <EntryStatus entry={entry} discardMessage={actions.discardMessage} />
          {entry.stream !== null && (
            <span className="animate-pulse text-xs text-accent">{streamLabels[entry.stream]}</span>
          )}
        </div>
        <div className="whitespace-pre-wrap break-words text-sm leading-relaxed text-ink">
          <MessageBody segments={entry.segments} />
        </div>
        <AttachmentList attachments={entry.attachments} />
        {entry.pending_drafts.length > 0 && (
          <ul aria-label="Uploading attachments" className="flex flex-wrap gap-2 text-xs text-subtle">
            {entry.pending_drafts.map((attachment) => (
              <li key={attachment.draft_id} className="rounded-md border border-dashed border-line px-2 py-1">
                {attachment.file_name}
              </li>
            ))}
          </ul>
        )}
        {thread !== null && (
          <div className="flex flex-wrap items-center gap-2 text-xs text-subtle">
            {thread.topic_title !== null && (
              <strong className="font-semibold text-ink">{thread.topic_title}</strong>
            )}
            <span className="font-medium text-accent">Replies {thread.reply_count}</span>
            {thread.subscription !== null && <span>{thread.subscription}</span>}
            {thread.unread_replies > 0 && (
              <span aria-label="Unread replies" className={`${ui.pill} bg-accent text-on-accent`}>
                {thread.unread_replies}
              </span>
            )}
          </div>
        )}
        {thread !== null && messageId !== null && openThread && (
          <LatestReplyBar
            thread={thread}
            openThread={() => void openThread(roomId, messageId)}
          />
        )}
        {messageId !== null && (
          <Reactions
            roomId={roomId}
            messageId={messageId}
            reactions={entry.reactions}
            palette={palette}
            react={actions.react}
          />
        )}
      </div>
      <div className="absolute -top-3 right-4 z-10 flex items-center gap-0.5 rounded-lg border border-line bg-surface p-0.5 opacity-0 shadow-panel transition-opacity empty:hidden focus-within:opacity-100 group-hover:opacity-100">
        {messageId !== null && openThread && entry.thread_root_id === null && (
          <button
            className={ui.ghostButton}
            type="button"
            onClick={() => void openThread(roomId, messageId)}
          >
            <MessageSquareReply size={14} aria-hidden="true" />
            Reply in thread
          </button>
        )}
        {messageId !== null && actions.canPin && actions.setPin && (
          <button
            className={ui.ghostButton}
            type="button"
            onClick={() =>
              void actions.setPin?.(roomId, messageId, !entry.pinned)
            }
          >
            <Pin size={14} aria-hidden="true" />
            {entry.pinned ? "Unpin" : "Pin"}
          </button>
        )}
        {messageId !== null && actions.updateShelf && (
          <ShelfMenu
            presets={actions.presets ?? []}
            customTime={actions.customTime}
            shelved={entry.shelved}
            onUpdate={(action, preset, remindAtMs) =>
              void actions.updateShelf?.(
                roomId,
                messageId,
                action,
                preset,
                remindAtMs,
              )
            }
          />
        )}
        {!entry.from_self && (
          <button
            className={ui.ghostButton}
            type="button"
            onClick={() => actions.mention(entry.author_mention_token)}
          >
            <AtSign size={14} aria-hidden="true" />
            Mention
          </button>
        )}
      </div>
    </li>
  );
}
