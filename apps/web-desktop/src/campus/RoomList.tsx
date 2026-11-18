import type { Room } from "@pulse/protocol";

export function RoomList({ rooms }: { rooms: Room[] }) {
  return (
    <ul aria-label="Rooms" className="space-y-1">
      {rooms.map((room) => (
        <li key={room.room_id}>{`#${room.slug}`}</li>
      ))}
    </ul>
  );
}
