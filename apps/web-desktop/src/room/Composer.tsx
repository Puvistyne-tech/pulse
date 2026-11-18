import { useEffect, useRef, useState, type FormEvent } from "react";

export interface Insertion {
  text: string;
  sequence: number;
}

export function useInsertion(): [Insertion | null, (text: string) => void] {
  const [insertion, setInsertion] = useState<Insertion | null>(null);
  const counter = useRef(0);

  function insert(text: string) {
    counter.current += 1;
    setInsertion({ text, sequence: counter.current });
  }

  return [insertion, insert];
}

interface ComposerProps {
  label?: string;
  insertion?: Insertion | null;
  onSend(body: string): void;
}

export function Composer({
  label = "Message",
  insertion = null,
  onSend,
}: ComposerProps) {
  const [draft, setDraft] = useState("");

  useEffect(() => {
    if (insertion !== null) {
      setDraft((current) =>
        current === "" ? `${insertion.text} ` : `${current} ${insertion.text} `,
      );
    }
  }, [insertion]);

  function submit(event: FormEvent) {
    event.preventDefault();
    onSend(draft);
    setDraft("");
  }

  return (
    <form className="flex gap-2" onSubmit={submit}>
      <input
        aria-label={label}
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
