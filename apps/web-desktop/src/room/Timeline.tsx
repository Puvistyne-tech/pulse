import type {
  ClientNonce,
  MessageRejection,
  TimelineEntry,
  TimelineSnapshot,
} from "@pulse/protocol";

const rejectionLabels: Record<MessageRejection, string> = {
  forbidden: "Not allowed to post here",
  invalid_body: "Message not accepted",
  unknown_room: "Room not found",
};

interface TimelineProps {
  timeline: TimelineSnapshot;
  discardMessage(clientNonce: ClientNonce): Promise<void>;
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

export function Timeline({ timeline, discardMessage }: TimelineProps) {
  return (
    <section aria-label="Timeline" className="space-y-2">
      {timeline.syncing && (
        <p className="text-xs text-slate-500">Syncing</p>
      )}
      <ul className="space-y-1">
        {timeline.entries.map((entry) => (
          <li
            key={entry.message_id ?? entry.client_nonce}
            data-delivery={entry.delivery}
          >
            <span className="font-semibold">
              {entry.from_self ? "You" : entry.author_display_name}
            </span>{" "}
            <span>{entry.body}</span>{" "}
            <EntryStatus entry={entry} discardMessage={discardMessage} />
          </li>
        ))}
      </ul>
    </section>
  );
}
