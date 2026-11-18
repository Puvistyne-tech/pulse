import type { RoomId, StewardEntry } from "@pulse/protocol";

interface StewardMentionPickerProps {
  stewards: StewardEntry[];
  roomId: RoomId;
  onPick(mentionToken: string): void;
}

export function StewardMentionPicker({
  stewards,
  roomId,
  onPick,
}: StewardMentionPickerProps) {
  const available = stewards.filter(
    (steward) => !steward.revoked && steward.room_ids.includes(roomId),
  );
  if (available.length === 0) {
    return null;
  }
  return (
    <div role="group" aria-label="Mention a steward" className="flex gap-2 text-xs">
      {available.map((steward) => (
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
