import { SmilePlus } from "lucide-react";
import type { MessageId, ReactionSummary, RoomId } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";

interface ReactionsProps {
  roomId: RoomId;
  messageId: MessageId;
  reactions: ReactionSummary[];
  palette: string[];
  react: PulseIntents["react"];
}

export function Reactions({
  roomId,
  messageId,
  reactions,
  palette,
  react,
}: ReactionsProps) {
  function toggle(emoji: string) {
    const current = reactions.find((reaction) => reaction.emoji === emoji);
    const action = current?.reacted_by_self ? "remove" : "add";
    void react(roomId, messageId, emoji, action);
  }

  return (
    <div className="flex flex-wrap items-center gap-1.5">
      {reactions.map((reaction) => (
        <button
          key={reaction.emoji}
          type="button"
          aria-pressed={reaction.reacted_by_self}
          className="inline-flex h-6 items-center gap-1 rounded-full border border-line bg-surface px-2 text-xs text-ink transition-colors hover:border-subtle aria-pressed:border-accent aria-pressed:bg-accent-soft aria-pressed:text-accent"
          onClick={() => toggle(reaction.emoji)}
        >
          {reaction.emoji} {reaction.count}
        </button>
      ))}
      <span
        role="group"
        aria-label="Add reaction"
        className="hidden items-center gap-0.5 focus-within:flex group-focus-within:flex group-hover:flex"
      >
        <SmilePlus size={14} aria-hidden="true" className="mr-1 text-subtle" />
        {palette.map((emoji) => (
          <button
            key={emoji}
            type="button"
            aria-label={`React ${emoji}`}
            className="flex h-6 w-6 items-center justify-center rounded-md text-sm transition hover:scale-110 hover:bg-muted"
            onClick={() => toggle(emoji)}
          >
            {emoji}
          </button>
        ))}
      </span>
    </div>
  );
}
