import { useEffect, useRef } from "react";
import { MessagesSquare } from "lucide-react";
import type { RoomId, TimelineSnapshot } from "@pulse/protocol";
import { MessageEntry, type EntryActions } from "./MessageEntry";

interface TimelineProps {
  timeline: TimelineSnapshot;
  roomId: RoomId;
  actions: EntryActions;
}

export function Timeline({ timeline, roomId, actions }: TimelineProps) {
  const endRef = useRef<HTMLDivElement>(null);
  const count = timeline.entries.length;

  useEffect(() => {
    if (timeline.focus_message_id === null) {
      endRef.current?.scrollIntoView?.({ block: "end" });
    }
  }, [count, timeline.focus_message_id]);

  return (
    <section aria-label="Timeline" className="min-h-0 flex-1 overflow-y-auto py-4">
      {timeline.syncing && (
        <p className="mx-5 mb-2 inline-flex items-center gap-2 rounded-full bg-muted px-3 py-1 text-xs text-subtle">
          <span className="h-1.5 w-1.5 animate-pulse rounded-full bg-accent" aria-hidden="true" />
          Syncing
        </p>
      )}
      {count === 0 && !timeline.syncing && (
        <div className="flex flex-col items-center gap-2 px-6 py-16 text-center text-subtle">
          <MessagesSquare size={32} aria-hidden="true" className="text-accent" />
          <p className="text-sm font-medium text-ink">No messages yet</p>
          <p className="text-xs">Start the conversation below.</p>
        </div>
      )}
      <ul className="space-y-1">
        {timeline.entries.map((entry) => (
          <MessageEntry
            key={entry.message_id ?? entry.client_nonce}
            entry={entry}
            roomId={roomId}
            palette={timeline.reaction_palette}
            actions={actions}
            focused={
              entry.message_id !== null &&
              entry.message_id === timeline.focus_message_id
            }
          />
        ))}
      </ul>
      <div ref={endRef} />
    </section>
  );
}
