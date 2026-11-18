import type { Room, RoomId } from "@pulse/protocol";

interface RoomListProps {
  rooms: Room[];
  activeRoomId: RoomId | null;
  selectRoom(roomId: RoomId): Promise<void>;
}

export function RoomList({ rooms, activeRoomId, selectRoom }: RoomListProps) {
  return (
    <ul aria-label="Rooms" className="space-y-1">
      {rooms.map((room) => (
        <li key={room.room_id}>
          <button
            className={
              room.room_id === activeRoomId ? "font-semibold underline" : ""
            }
            type="button"
            aria-current={room.room_id === activeRoomId ? "true" : undefined}
            onClick={() => void selectRoom(room.room_id)}
          >
            {`#${room.slug}`}
          </button>
        </li>
      ))}
    </ul>
  );
}
