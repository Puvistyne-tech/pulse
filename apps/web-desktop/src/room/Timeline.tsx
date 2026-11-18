import type { RoomId, TimelineSnapshot } from "@pulse/protocol";
import { MessageEntry, type EntryActions } from "./MessageEntry";

interface TimelineProps {
  timeline: TimelineSnapshot;
  roomId: RoomId;
  actions: EntryActions;
}

export function Timeline({ timeline, roomId, actions }: TimelineProps) {
  return (
    <section aria-label="Timeline" className="space-y-2">
      {timeline.syncing && <p className="text-xs text-slate-500">Syncing</p>}
      <ul className="space-y-2">
        {timeline.entries.map((entry) => (
          <MessageEntry
            key={entry.message_id ?? entry.client_nonce}
            entry={entry}
            roomId={roomId}
            palette={timeline.reaction_palette}
            actions={actions}
          />
        ))}
      </ul>
    </section>
  );
}
