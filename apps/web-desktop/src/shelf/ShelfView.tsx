import type {
  ReminderPreset,
  ShelfEntry,
  ShelfSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
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
    <section aria-label={title} className="space-y-1">
      <h3 className="text-sm font-semibold">{title}</h3>
      <ul className="space-y-1">
        {entries.map((entry) => (
          <li key={entry.message_id} className="text-sm">
            <span className="block text-xs text-slate-600">
              {entry.author_display_name}
              {entry.remind_at_ms !== null &&
                ` ${remindFormat.format(entry.remind_at_ms)}`}
              {entry.pending && " Saving"}
            </span>
            <span className="block">{entry.preview}</span>
            <span className="flex gap-3 text-xs">
              {entry.bucket === "done" ? (
                <button
                  className="underline"
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
                  Reopen
                </button>
              ) : (
                <button
                  className="underline"
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
  return (
    <section aria-label="Shelf" className="space-y-2">
      <header className="flex items-center justify-between">
        <h2 className="font-semibold">Shelf</h2>
        <button
          className="text-sm underline"
          type="button"
          onClick={() => void intents.toggleShelf()}
        >
          Close shelf
        </button>
      </header>
      <ErrorNotice error={shelf.error} />
      <ShelfSection
        title="Due"
        entries={shelf.due}
        presets={shelf.presets}
        customTime={shelf.picker_anchor_ms !== null}
        intents={intents}
      />
      <ShelfSection
        title="Upcoming"
        entries={shelf.upcoming}
        presets={shelf.presets}
        customTime={shelf.picker_anchor_ms !== null}
        intents={intents}
      />
      <ShelfSection
        title="Done"
        entries={shelf.done}
        presets={shelf.presets}
        customTime={shelf.picker_anchor_ms !== null}
        intents={intents}
      />
    </section>
  );
}
