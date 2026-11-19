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
    <div role="group" aria-label="Mention a steward" className="flex gap-2 text-xs">
      {stewards.map((steward) => (
        <button
          key={steward.steward_id}
          className="underline"
          type="button"
          onClick={() => onPick(steward.mention_token)}
        >
          {steward.display_name}
        </button>
      ))}
    </div>
  );
}
