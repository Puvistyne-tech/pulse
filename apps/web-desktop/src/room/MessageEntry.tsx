import type {
  ClientNonce,
  MessageRejection,
  MessageId,
  RoomId,
  TimelineEntry,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { LatestReplyBar } from "./LatestReplyBar";
import { MessageBody } from "./MessageBody";
import { Reactions } from "./Reactions";

const rejectionLabels: Record<MessageRejection, string> = {
  forbidden: "Not allowed to post here",
  invalid_body: "Message not accepted",
  unknown_room: "Room not found",
  unknown_thread: "Thread not found",
};

export interface EntryActions {
  react: PulseIntents["react"];
  discardMessage(clientNonce: ClientNonce): Promise<void>;
  mention(token: string): void;
  openThread?(roomId: RoomId, rootMessageId: MessageId): Promise<void>;
}

interface MessageEntryProps {
  entry: TimelineEntry;
  roomId: RoomId;
  palette: string[];
  actions: EntryActions;
}

function EntryStatus({
  entry,
  discardMessage,
}: {
  entry: TimelineEntry;
  discardMessage(clientNonce: ClientNonce): Promise<void>;
}) {
  if (entry.delivery === "pending") {
    return <span className="text-xs text-slate-500">Sending</span>;
  }
  if (entry.delivery === "failed") {
    return (
      <span className="text-xs text-red-700">
        {entry.rejection === null ? "Failed" : rejectionLabels[entry.rejection]}
        <button
          className="ml-2 underline"
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
}: MessageEntryProps) {
  const messageId = entry.message_id;
  const thread = entry.thread;
  const openThread = actions.openThread;

  return (
    <li data-delivery={entry.delivery} className="space-y-1">
      <div>
        <span className="font-semibold">
          {entry.from_self ? "You" : entry.author_display_name}
        </span>{" "}
        <MessageBody segments={entry.segments} />{" "}
        <EntryStatus entry={entry} discardMessage={actions.discardMessage} />
      </div>
      {thread !== null && (
        <div className="flex items-center gap-2 text-xs text-slate-600">
          {thread.topic_title !== null && (
            <strong>{thread.topic_title}</strong>
          )}
          <span>Replies {thread.reply_count}</span>
          {thread.subscription !== null && <span>{thread.subscription}</span>}
          {thread.unread_replies > 0 && (
            <span aria-label="Unread replies">{thread.unread_replies}</span>
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
      <div className="flex gap-3 text-xs">
        {messageId !== null && openThread && entry.thread_root_id === null && (
          <button
            className="underline"
            type="button"
            onClick={() => void openThread(roomId, messageId)}
          >
            Reply in thread
          </button>
        )}
        {!entry.from_self && (
          <button
            className="underline"
            type="button"
            onClick={() => actions.mention(entry.author_mention_token)}
          >
            Mention
          </button>
        )}
      </div>
    </li>
  );
}
