import type { ReactNode } from "react";
import { Archive, Hash } from "lucide-react";
import type {
  ReminderPreset,
  TimelineSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
import { StewardMentionPicker } from "../steward/StewardMentionPicker";
import { ui } from "../ui/styles";
import { Composer, useInsertion } from "./Composer";
import { Timeline } from "./Timeline";

interface RoomViewProps {
  timeline: TimelineSnapshot;
  intents: PulseIntents;
  presets?: ReminderPreset[];
  customTime?: boolean;
  roomSlug?: string | null;
  headerExtras?: ReactNode;
}

export function RoomView({
  timeline,
  intents,
  presets = [],
  customTime = true,
  roomSlug = null,
  headerExtras = null,
}: RoomViewProps) {
  const [insertion, mention] = useInsertion();
  const roomId = timeline.room_id;

  if (roomId === null) {
    return null;
  }

  return (
    <section aria-label="Room" className="flex min-h-0 min-w-0 flex-1 flex-col bg-canvas">
      <header className="flex h-14 shrink-0 items-center justify-between gap-3 border-b border-line px-5">
        <div className="flex min-w-0 items-center gap-1.5">
          <Hash size={18} aria-hidden="true" className="shrink-0 text-subtle" />
          <span className="truncate text-[15px] font-semibold text-ink">{roomSlug ?? "room"}</span>
        </div>
        <div className="flex items-center gap-2">
          {headerExtras}
          <button
            className={ui.secondaryButton}
            type="button"
            onClick={() => void intents.openVault(roomId, "media")}
          >
            <Archive size={15} aria-hidden="true" />
            Vault
          </button>
        </div>
      </header>
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
      <div className="shrink-0 space-y-2 px-5 pb-5">
        <ErrorNotice error={timeline.error} />
        <StewardMentionPicker
          stewards={timeline.mentionable_stewards}
          onPick={mention}
        />
        <Composer
          insertion={insertion}
          placeholder={roomSlug === null ? undefined : `Message #${roomSlug}`}
          drafts={timeline.drafts}
          canUpload={timeline.can_upload}
          onAttach={(file) => void intents.attachFile(roomId, file)}
          onDiscardDraft={(draftId) => void intents.discardDraft(draftId)}
          onRetryDraft={(draftId) => void intents.retryDraft(draftId)}
          onSend={(body) => void intents.sendMessage(roomId, body)}
        />
      </div>
    </section>
  );
}
