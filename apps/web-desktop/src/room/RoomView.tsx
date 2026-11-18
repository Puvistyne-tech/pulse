import type { TimelineSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
import { Composer, useInsertion } from "./Composer";
import { Timeline } from "./Timeline";

interface RoomViewProps {
  timeline: TimelineSnapshot;
  intents: PulseIntents;
}

export function RoomView({ timeline, intents }: RoomViewProps) {
  const [insertion, mention] = useInsertion();
  const roomId = timeline.room_id;

  if (roomId === null) {
    return null;
  }

  return (
    <section aria-label="Room" className="space-y-2">
      <Timeline
        timeline={timeline}
        roomId={roomId}
        actions={{
          react: intents.react,
          discardMessage: intents.discardMessage,
          openThread: intents.openThread,
          mention,
        }}
      />
      <ErrorNotice error={timeline.error} />
      <Composer
        insertion={insertion}
        onSend={(body) => void intents.sendMessage(roomId, body)}
      />
    </section>
  );
}
