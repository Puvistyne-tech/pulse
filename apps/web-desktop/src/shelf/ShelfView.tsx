import { Bookmark, Check, RotateCcw } from "lucide-react";
import type {
  ReminderPreset,
  ShelfEntry,
  ShelfSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
import { Panel } from "../ui/Panel";
import { ui } from "../ui/styles";
import { ShelfMenu } from "./ShelfMenu";

const remindFormat = new Intl.DateTimeFormat(undefined, {
  dateStyle: "medium",
  timeStyle: "short",
});

interface ShelfSectionProps {
  title: string;
  entries: ShelfEntry[];
  presets: ReminderPreset[];
  customTime: boolean;
  intents: PulseIntents;
}

function ShelfSection({
  title,
  entries,
  presets,
  customTime,
  intents,
}: ShelfSectionProps) {
  return (
    <section aria-label={title} className="space-y-2">
      <h3 className={ui.sectionTitle}>{title}</h3>
      {entries.length === 0 && <p className="text-xs text-subtle">Nothing here.</p>}
      <ul className="space-y-2">
        {entries.map((entry) => (
          <li key={entry.message_id} className="space-y-1.5 rounded-xl border border-line p-3 text-sm">
            <span className="block text-xs text-subtle">
              {entry.author_display_name}
              {entry.remind_at_ms !== null &&
                ` ${remindFormat.format(entry.remind_at_ms)}`}
              {entry.pending && " Saving"}
            </span>
            <span className="block text-ink">{entry.preview}</span>
            <span className="flex flex-wrap items-center gap-1">
              {entry.bucket === "done" ? (
                <button
                  className={ui.ghostButton}
                  type="button"
                  onClick={() =>
                    void intents.updateShelf(
                      entry.room_id,
                      entry.message_id,
                      "reopen",
                      null,
                      null,
                    )
                  }
                >
                  <RotateCcw size={13} aria-hidden="true" />
                  Reopen
                </button>
              ) : (
                <button
                  className={ui.ghostButton}
                  type="button"
                  onClick={() =>
                    void intents.updateShelf(
                      entry.room_id,
                      entry.message_id,
                      "complete",
                      null,
                      null,
                    )
                  }
                >
                  <Check size={13} aria-hidden="true" />
                  Complete
                </button>
              )}
              <ShelfMenu
                presets={presets}
                customTime={customTime}
                shelved
                onUpdate={(action, preset, remindAtMs) =>
                  void intents.updateShelf(
                    entry.room_id,
                    entry.message_id,
                    action,
                    preset,
                    remindAtMs,
                  )
                }
              />
            </span>
          </li>
        ))}
      </ul>
    </section>
  );
}

interface ShelfViewProps {
  shelf: ShelfSnapshot;
  intents: PulseIntents;
}

export function ShelfView({ shelf, intents }: ShelfViewProps) {
  const customTime = shelf.picker_anchor_ms !== null;
  return (
    <Panel
      label="Shelf"
      title={
        <>
          <Bookmark size={16} aria-hidden="true" className="text-accent" />
          Shelf
        </>
      }
      closeLabel="Close shelf"
      onClose={() => void intents.toggleShelf()}
    >
      <ErrorNotice error={shelf.error} />
      <ShelfSection title="Due" entries={shelf.due} presets={shelf.presets} customTime={customTime} intents={intents} />
      <ShelfSection title="Upcoming" entries={shelf.upcoming} presets={shelf.presets} customTime={customTime} intents={intents} />
      <ShelfSection title="Done" entries={shelf.done} presets={shelf.presets} customTime={customTime} intents={intents} />
    </Panel>
  );
}
