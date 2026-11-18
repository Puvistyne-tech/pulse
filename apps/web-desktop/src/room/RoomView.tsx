import type { ClientNonce, RoomId, TimelineSnapshot } from "@pulse/protocol";
import { ErrorNotice } from "../session/ErrorNotice";
import { Composer } from "./Composer";
import { Timeline } from "./Timeline";

interface RoomViewProps {
  timeline: TimelineSnapshot;
  sendMessage(roomId: RoomId, body: string): Promise<void>;
  discardMessage(clientNonce: ClientNonce): Promise<void>;
}

export function RoomView({
  timeline,
  sendMessage,
  discardMessage,
}: RoomViewProps) {
  if (timeline.room_id === null) {
    return null;
  }

  return (
    <section aria-label="Room" className="space-y-2">
      <Timeline timeline={timeline} discardMessage={discardMessage} />
      <ErrorNotice error={timeline.error} />
      <Composer roomId={timeline.room_id} sendMessage={sendMessage} />
    </section>
  );
}
