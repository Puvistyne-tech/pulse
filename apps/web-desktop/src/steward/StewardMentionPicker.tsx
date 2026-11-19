import { Bot } from "lucide-react";
import type { StewardEntry } from "@pulse/protocol";

interface StewardMentionPickerProps {
  stewards: StewardEntry[];
  onPick(mentionToken: string): void;
}

export function StewardMentionPicker({
  stewards,
  onPick,
}: StewardMentionPickerProps) {
  if (stewards.length === 0) {
    return null;
  }
  return (
    <div role="group" aria-label="Mention a steward" className="flex flex-wrap items-center gap-1.5 text-xs">
      {stewards.map((steward) => (
        <button
          key={steward.steward_id}
          className="inline-flex items-center gap-1 rounded-full border border-line bg-surface px-2.5 py-1 font-medium text-subtle hover:border-accent hover:text-accent"
          type="button"
          onClick={() => onPick(steward.mention_token)}
        >
          <Bot size={12} aria-hidden="true" />
          {steward.display_name}
        </button>
      ))}
    </div>
  );
}
