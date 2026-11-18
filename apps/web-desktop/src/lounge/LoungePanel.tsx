import type {
  LoungeEntry,
  LoungeMediaState,
  LoungeSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
import { useLoungeMedia } from "./useLoungeMedia";

const mediaLabels: Record<LoungeMediaState, string | null> = {
  idle: null,
  connecting: "Connecting audio",
  connected: "Audio connected",
  failed: "Audio unavailable",
};

const loungeLabel = (entry: LoungeEntry) =>
  entry.scope === "campus" ? "Campus lounge" : `#${entry.room_slug ?? ""} lounge`;

interface LoungePanelProps {
  lounges: LoungeSnapshot;
  intents: PulseIntents;
}

export function LoungePanel({ lounges, intents }: LoungePanelProps) {
  useLoungeMedia(lounges.ticket, lounges.can_speak);

  if (lounges.lounges.length === 0) {
    return null;
  }
  const mediaLabel = mediaLabels[lounges.media];

  return (
    <section aria-label="Lounges" className="space-y-2">
      <h2 className="font-semibold">Lounges</h2>
      {lounges.joining && <p className="text-xs text-slate-500">Joining</p>}
      {lounges.joined_lounge_id !== null && mediaLabel !== null && (
        <p role="status" className="text-xs text-slate-600">
          {mediaLabel}
        </p>
      )}
      <ErrorNotice error={lounges.error} />
      <ul className="space-y-2">
        {lounges.lounges.map((entry) => (
          <li
            key={entry.lounge_id}
            data-joined={entry.joined}
            className="space-y-1 text-sm"
          >
            <div className="flex items-center gap-3">
              <span className="font-medium">{loungeLabel(entry)}</span>
              {entry.joined ? (
                <button
                  className="text-xs underline"
                  type="button"
                  onClick={() => void intents.leaveLounge()}
                >
                  Leave
                </button>
              ) : (
                lounges.can_join && (
                  <button
                    className="text-xs underline disabled:opacity-50"
                    type="button"
                    disabled={lounges.joining}
                    aria-label={`Join ${loungeLabel(entry)}`}
                    onClick={() => void intents.joinLounge(entry.lounge_id)}
                  >
                    Join
                  </button>
                )
              )}
            </div>
            <ul aria-label="Participants" className="flex flex-wrap gap-2 text-xs">
              {entry.participants.map((participant) => (
                <li key={participant.account_id}>
                  {participant.from_self ? "You" : participant.display_name}
                  {!participant.can_speak && " listening"}
                </li>
              ))}
            </ul>
          </li>
        ))}
      </ul>
    </section>
  );
}
