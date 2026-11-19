import type {
  ReminderPreset,
  TimelineSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
import { StewardMentionPicker } from "../steward/StewardMentionPicker";
import { Composer, useInsertion } from "./Composer";
import { Timeline } from "./Timeline";

interface RoomViewProps {
  timeline: TimelineSnapshot;
  intents: PulseIntents;
  presets?: ReminderPreset[];
  customTime?: boolean;
}

export function RoomView({
  timeline,
  intents,
  presets = [],
  customTime = true,
}: RoomViewProps) {
  const [insertion, mention] = useInsertion();
  const roomId = timeline.room_id;

  if (roomId === null) {
    return null;
  }

  return (
    <section aria-label="Room" className="space-y-2">
      <button
        className="text-sm underline"
        type="button"
        onClick={() => void intents.openVault(roomId, "media")}
      >
        Vault
      </button>
      <Timeline
        timeline={timeline}
        roomId={roomId}
        actions={{
          react: intents.react,
          discardMessage: intents.discardMessage,
          openThread: intents.openThread,
          setPin: intents.setPin,
          updateShelf: intents.updateShelf,
          canPin: timeline.can_pin,
          presets,
          customTime,
          mention,
        }}
      />
      <ErrorNotice error={timeline.error} />
      <StewardMentionPicker
        stewards={timeline.mentionable_stewards}
        onPick={mention}
      />
      <Composer
        insertion={insertion}
        drafts={timeline.drafts}
        canUpload={timeline.can_upload}
        onAttach={(file) => void intents.attachFile(roomId, file)}
        onDiscardDraft={(draftId) => void intents.discardDraft(draftId)}
        onRetryDraft={(draftId) => void intents.retryDraft(draftId)}
        onSend={(body) => void intents.sendMessage(roomId, body)}
      />
    </section>
  );
}
