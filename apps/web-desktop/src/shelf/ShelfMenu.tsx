import { useState } from "react";
import type { ReminderPreset, ShelfAction } from "@pulse/protocol";

export const presetLabels: Record<ReminderPreset, string> = {
  in_twenty_minutes: "In twenty minutes",
  in_one_hour: "In one hour",
  in_three_hours: "In three hours",
  tomorrow: "Tomorrow",
  next_week: "Next week",
};

export type ShelfUpdate = (
  action: ShelfAction,
  preset: ReminderPreset | null,
  remindAtMs: number | null,
) => void;

interface ShelfMenuProps {
  presets: ReminderPreset[];
  shelved: boolean;
  customTime?: boolean;
  onUpdate: ShelfUpdate;
}

export function ShelfMenu({
  presets,
  shelved,
  customTime = true,
  onUpdate,
}: ShelfMenuProps) {
  const [open, setOpen] = useState(false);
  const [custom, setCustom] = useState("");
  const scheduleAction: ShelfAction = shelved ? "reschedule" : "save";

  function choose(update: () => void) {
    update();
    setOpen(false);
  }

  return (
    <span className="relative">
      <button
        className="underline"
        type="button"
        aria-expanded={open}
        onClick={() => setOpen(!open)}
      >
        Shelf
      </button>
      {open && (
        <div
          role="menu"
          className="space-y-1 rounded border border-slate-300 p-2"
        >
          {!shelved && (
            <button
              className="block underline"
              type="button"
              onClick={() => choose(() => onUpdate("save", null, null))}
            >
              Save to shelf
            </button>
          )}
          {presets.map((preset) => (
            <button
              key={preset}
              className="block underline"
              type="button"
              onClick={() =>
                choose(() => onUpdate(scheduleAction, preset, null))
              }
            >
              {presetLabels[preset]}
            </button>
          ))}
          {customTime && (
            <>
              <input
                aria-label="Remind at"
                type="datetime-local"
                value={custom}
                onChange={(event) => setCustom(event.target.value)}
              />
              <button
                className="block underline"
                type="button"
                disabled={custom === ""}
                onClick={() =>
                  choose(() =>
                    onUpdate(scheduleAction, null, Date.parse(custom)),
                  )
                }
              >
                Remind at chosen time
              </button>
            </>
          )}
          {shelved && (
            <button
              className="block underline"
              type="button"
              onClick={() => choose(() => onUpdate("remove", null, null))}
            >
              Remove from shelf
            </button>
          )}
        </div>
      )}
    </span>
  );
}
