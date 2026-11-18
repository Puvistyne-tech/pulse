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
    <div className="flex flex-wrap items-center gap-1">
      {reactions.map((reaction) => (
        <button
          key={reaction.emoji}
          type="button"
          aria-pressed={reaction.reacted_by_self}
          className="rounded-full border border-slate-300 px-2 text-xs aria-pressed:bg-slate-200"
          onClick={() => toggle(reaction.emoji)}
        >
          {reaction.emoji} {reaction.count}
        </button>
      ))}
      <span role="group" aria-label="Add reaction" className="flex gap-1">
        {palette.map((emoji) => (
          <button
            key={emoji}
            type="button"
            aria-label={`React ${emoji}`}
            className="rounded px-1 text-xs opacity-60 hover:opacity-100"
            onClick={() => toggle(emoji)}
          >
            {emoji}
          </button>
        ))}
      </span>
    </div>
  );
}
