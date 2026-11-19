import { useState } from "react";
import { Bookmark } from "lucide-react";
import type { ReminderPreset, ShelfAction } from "@pulse/protocol";
import { ui } from "../ui/styles";

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

const menuItem =
  "block w-full rounded-md px-2.5 py-1.5 text-left text-sm text-ink hover:bg-muted disabled:cursor-not-allowed disabled:opacity-50";

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
    <span className="relative inline-flex">
      <button
        className={`${ui.ghostButton} ${shelved ? "text-accent" : ""}`}
        type="button"
        aria-expanded={open}
        onClick={() => setOpen(!open)}
      >
        <Bookmark size={14} aria-hidden="true" fill={shelved ? "currentColor" : "none"} />
        Shelf
      </button>
      {open && (
        <div
          role="menu"
          className="absolute right-0 top-full z-30 mt-1 w-56 space-y-0.5 rounded-xl border border-line bg-surface p-1.5 shadow-panel"
        >
          {!shelved && (
            <button
              className={`${menuItem} font-medium text-accent`}
              type="button"
              onClick={() => choose(() => onUpdate("save", null, null))}
            >
              Save to shelf
            </button>
          )}
          {presets.map((preset) => (
            <button
              key={preset}
              className={menuItem}
              type="button"
              onClick={() =>
                choose(() => onUpdate(scheduleAction, preset, null))
              }
            >
              {presetLabels[preset]}
            </button>
          ))}
          {customTime && (
            <div className="space-y-1.5 border-t border-line px-1 pt-2">
              <input
                aria-label="Remind at"
                className={`${ui.input} py-1.5 text-xs`}
                type="datetime-local"
                value={custom}
                onChange={(event) => setCustom(event.target.value)}
              />
              <button
                className={menuItem}
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
            </div>
          )}
          {shelved && (
            <button
              className={`${menuItem} text-danger`}
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
