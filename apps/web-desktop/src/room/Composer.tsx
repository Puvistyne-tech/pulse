import { useState, type FormEvent } from "react";
import type { RoomId } from "@pulse/protocol";

interface ComposerProps {
  roomId: RoomId;
  sendMessage(roomId: RoomId, body: string): Promise<void>;
}

export function Composer({ roomId, sendMessage }: ComposerProps) {
  const [draft, setDraft] = useState("");

  function submit(event: FormEvent) {
    event.preventDefault();
    void sendMessage(roomId, draft);
    setDraft("");
  }

  return (
    <form className="flex gap-2" onSubmit={submit}>
      <input
        aria-label="Message"
        className="flex-1 rounded border border-slate-300 px-2 py-1"
        type="text"
        value={draft}
        onChange={(event) => setDraft(event.target.value)}
      />
      <button
        className="rounded bg-slate-900 px-3 py-1 text-white disabled:opacity-50"
        type="submit"
        disabled={draft === ""}
      >
        Send
      </button>
    </form>
  );
}
