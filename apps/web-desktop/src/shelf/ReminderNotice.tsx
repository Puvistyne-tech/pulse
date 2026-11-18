import type { ShelfEntry } from "@pulse/protocol";

interface ReminderNoticeProps {
  entry: ShelfEntry;
  dismiss(): Promise<void>;
}

export function ReminderNotice({ entry, dismiss }: ReminderNoticeProps) {
  return (
    <div
      role="status"
      aria-label="Reminder"
      className="flex items-center justify-between rounded bg-amber-50 p-2 text-sm"
    >
      <span>
        {entry.author_display_name}: {entry.preview}
      </span>
      <button className="underline" type="button" onClick={() => void dismiss()}>
        Dismiss
      </button>
    </div>
  );
}
