import { BellRing } from "lucide-react";
import type { ShelfEntry } from "@pulse/protocol";
import { ui } from "../ui/styles";

interface ReminderNoticeProps {
  entry: ShelfEntry;
  dismiss(): Promise<void>;
}

export function ReminderNotice({ entry, dismiss }: ReminderNoticeProps) {
  return (
    <div
      role="status"
      aria-label="Reminder"
      className="fixed right-5 top-5 z-50 flex w-80 items-start gap-3 rounded-xl border border-warning/30 bg-surface p-3 text-sm shadow-panel"
    >
      <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-warning-soft text-warning">
        <BellRing size={16} aria-hidden="true" />
      </span>
      <span className="min-w-0 flex-1 text-ink">
        {entry.author_display_name}: {entry.preview}
      </span>
      <button className={ui.ghostButton} type="button" onClick={() => void dismiss()}>
        Dismiss
      </button>
    </div>
  );
}
