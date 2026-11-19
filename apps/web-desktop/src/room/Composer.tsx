import { useEffect, useRef, useState, type FormEvent } from "react";
import { Paperclip, RotateCw, SendHorizontal, X } from "lucide-react";
import type { DraftAttachmentView, DraftId } from "@pulse/protocol";
import { ui } from "../ui/styles";

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
  placeholder?: string;
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

const uploadTones: Record<DraftAttachmentView["upload"], string> = {
  uploading: "border-line text-subtle",
  uploaded: "border-accent/40 text-ink",
  failed: "border-danger/40 text-danger",
};

export function Composer({
  label = "Message",
  placeholder,
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
    <form
      className="rounded-xl border border-line bg-surface shadow-sm transition focus-within:border-accent focus-within:ring-2 focus-within:ring-accent/20"
      onSubmit={submit}
    >
      {drafts.length > 0 && (
        <ul aria-label="Drafts" className="flex flex-wrap gap-2 border-b border-line px-3 py-2 text-xs">
          {drafts.map((attachment) => (
            <li
              key={attachment.draft_id}
              data-upload={attachment.upload}
              className={`inline-flex items-center gap-1.5 rounded-lg border bg-muted px-2 py-1 ${uploadTones[attachment.upload]}`}
            >
              <Paperclip size={12} aria-hidden="true" />
              {attachment.file_name} {uploadLabels[attachment.upload]}
              {attachment.upload === "failed" && onRetryDraft && (
                <button
                  className="inline-flex items-center gap-0.5 font-medium hover:underline"
                  type="button"
                  onClick={() => onRetryDraft(attachment.draft_id)}
                >
                  <RotateCw size={11} aria-hidden="true" />
                  Retry
                </button>
              )}
              {onDiscardDraft && (
                <button
                  className="inline-flex items-center gap-0.5 font-medium text-subtle hover:text-ink"
                  type="button"
                  onClick={() => onDiscardDraft(attachment.draft_id)}
                >
                  <X size={11} aria-hidden="true" />
                  Remove
                </button>
              )}
            </li>
          ))}
        </ul>
      )}
      <div className="flex items-center gap-1 p-1.5">
        {canUpload && onAttach && (
          <label className={`${ui.iconButton} cursor-pointer`} title="Attach file">
            <Paperclip size={17} aria-hidden="true" />
            <input
              aria-label="Attach file"
              className="sr-only"
              type="file"
              onChange={(event) => {
                const file = event.target.files?.[0];
                if (file) {
                  onAttach(file);
                }
                event.target.value = "";
              }}
            />
          </label>
        )}
        <input
          aria-label={label}
          className="min-w-0 flex-1 bg-transparent px-2 py-1.5 text-sm text-ink outline-none placeholder:text-subtle"
          type="text"
          placeholder={placeholder ?? `Write a ${label.toLowerCase()}`}
          value={draft}
          onChange={(event) => setDraft(event.target.value)}
        />
        <button
          className={`${ui.primaryButton} px-3 py-1.5`}
          type="submit"
          disabled={draft === "" && drafts.length === 0}
        >
          <SendHorizontal size={15} aria-hidden="true" />
          Send
        </button>
      </div>
    </form>
  );
}
