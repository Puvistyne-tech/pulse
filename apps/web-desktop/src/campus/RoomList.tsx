import type { Room, RoomId } from "@pulse/protocol";

interface RoomListProps {
  rooms: Room[];
  activeRoomId: RoomId | null;
  selectRoom(roomId: RoomId): Promise<void>;
}

export function RoomList({ rooms, activeRoomId, selectRoom }: RoomListProps) {
  return (
    <ul aria-label="Rooms" className="space-y-0.5">
      {rooms.map((room) => {
        const active = room.room_id === activeRoomId;
        return (
          <li key={room.room_id}>
            <button
              className={`flex w-full items-center rounded-md px-2.5 py-1.5 text-left text-sm transition-colors ${
                active
                  ? "bg-accent font-semibold text-on-accent"
                  : "text-side-ink hover:bg-side-hover hover:text-white"
              }`}
              type="button"
              aria-current={active ? "true" : undefined}
              onClick={() => void selectRoom(room.room_id)}
            >
              {`#${room.slug}`}
            </button>
          </li>
        );
      })}
    </ul>
  );
}
