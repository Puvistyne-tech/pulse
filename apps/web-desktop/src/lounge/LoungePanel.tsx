import { Headphones, MicOff } from "lucide-react";
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
  useLoungeMedia(lounges.ticket, lounges.can_speak, intents.reportLoungeMedia);
  if (lounges.lounges.length === 0) {
    return null;
  }
  const mediaLabel = mediaLabels[lounges.media];
  return (
    <section aria-label="Lounges" className="space-y-2">
      <h2 className="px-2.5 text-[11px] font-semibold uppercase tracking-wider text-side-subtle">Lounges</h2>
      {lounges.joining && <p className="px-2.5 text-xs text-side-subtle">Joining</p>}
      {lounges.joined_lounge_id !== null && mediaLabel !== null && (
        <p role="status" className="mx-2.5 rounded-md bg-accent/15 px-2 py-1 text-xs text-accent">
          {mediaLabel}
        </p>
      )}
      <ErrorNotice error={lounges.error} />
      <ul className="space-y-1">
        {lounges.lounges.map((entry) => (
          <li
            key={entry.lounge_id}
            data-joined={entry.joined}
            className={`space-y-1.5 rounded-md px-2.5 py-1.5 text-sm ${entry.joined ? "bg-accent/15" : ""}`}
          >
            <div className="flex items-center gap-2">
              <Headphones size={14} aria-hidden="true" className={entry.joined ? "text-accent" : "text-side-subtle"} />
              <span className="flex-1 truncate font-medium text-side-ink">{loungeLabel(entry)}</span>
              {entry.joined ? (
                <button
                  className="rounded-md bg-danger px-2 py-0.5 text-[11px] font-semibold text-white hover:opacity-90"
                  type="button"
                  onClick={() => void intents.leaveLounge()}
                >
                  Leave
                </button>
              ) : (
                lounges.can_join && (
                  <button
                    className="rounded-md bg-white/10 px-2 py-0.5 text-[11px] font-semibold text-side-ink hover:bg-white/20 disabled:opacity-50"
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
            <ul aria-label="Participants" className="flex flex-wrap gap-1.5 pl-5 text-xs text-side-subtle empty:hidden">
              {entry.participants.map((participant) => (
                <li key={participant.account_id} className="inline-flex items-center gap-1 rounded-full bg-white/5 px-2 py-0.5">
                  {!participant.can_speak && <MicOff size={10} aria-hidden="true" />}
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
