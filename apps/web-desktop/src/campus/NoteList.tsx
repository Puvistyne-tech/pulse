import type { NoteEntry, RoomId } from "@pulse/protocol";

interface NoteListProps {
  notes: NoteEntry[];
  activeRoomId: RoomId | null;
  selectRoom(roomId: RoomId): Promise<void>;
}

export function NoteList({ notes, activeRoomId, selectRoom }: NoteListProps) {
  return (
    <ul aria-label="Notes" className="space-y-0.5">
      {notes.map((note) => {
        const active = note.room_id === activeRoomId;
        return (
          <li key={note.room_id}>
            <button
              className={`flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-sm transition-colors ${
                active
                  ? "bg-accent font-semibold text-on-accent"
                  : "text-side-ink hover:bg-side-hover hover:text-white"
              }`}
              type="button"
              aria-current={active ? "true" : undefined}
              onClick={() => void selectRoom(note.room_id)}
            >
              <span className="min-w-0 flex-1 truncate">{note.title}</span>
              {note.member_count > 2 && (
                <span className="shrink-0 text-[11px] opacity-70">{note.member_count}</span>
              )}
            </button>
          </li>
        );
      })}
    </ul>
  );
}
