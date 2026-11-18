import { useEffect, useRef, useState, type FormEvent } from "react";
import type { DraftAttachmentView, DraftId } from "@pulse/protocol";

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
  drafts?: DraftAttachmentView[];
  canUpload?: boolean;
  onAttach?(file: File): void;
  onDiscardDraft?(draftId: DraftId): void;
  onRetryDraft?(draftId: DraftId): void;
}

const uploadLabels: Record<DraftAttachmentView["upload"], string> = {
  uploading: "Uploading",
  uploaded: "Ready",
  failed: "Failed",
};

export function Composer({
  label = "Message",
  insertion = null,
  onSend,
  drafts = [],
  canUpload = false,
  onAttach,
  onDiscardDraft,
  onRetryDraft,
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
    <form className="space-y-1" onSubmit={submit}>
      {drafts.length > 0 && (
        <ul aria-label="Drafts" className="flex flex-wrap gap-2 text-xs">
          {drafts.map((attachment) => (
            <li
              key={attachment.draft_id}
              data-upload={attachment.upload}
              className="rounded border border-slate-300 px-2 py-1"
            >
              {attachment.file_name} {uploadLabels[attachment.upload]}
              {attachment.upload === "failed" && onRetryDraft && (
                <button
                  className="ml-2 underline"
                  type="button"
                  onClick={() => onRetryDraft(attachment.draft_id)}
                >
                  Retry
                </button>
              )}
              {onDiscardDraft && (
                <button
                  className="ml-2 underline"
                  type="button"
                  onClick={() => onDiscardDraft(attachment.draft_id)}
                >
                  Remove
                </button>
              )}
            </li>
          ))}
        </ul>
      )}
      <div className="flex gap-2">
      <input
        aria-label={label}
        className="flex-1 rounded border border-slate-300 px-2 py-1"
        type="text"
        value={draft}
        onChange={(event) => setDraft(event.target.value)}
      />
      {canUpload && onAttach && (
        <input
          aria-label="Attach file"
          type="file"
          onChange={(event) => {
            const file = event.target.files?.[0];
            if (file) {
              onAttach(file);
            }
            event.target.value = "";
          }}
        />
      )}
      <button
        className="rounded bg-slate-900 px-3 py-1 text-white disabled:opacity-50"
        type="submit"
        disabled={draft === "" && drafts.length === 0}
      >
        Send
      </button>
      </div>
    </form>
  );
}
