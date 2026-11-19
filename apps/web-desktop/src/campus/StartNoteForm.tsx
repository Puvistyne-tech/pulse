import { useState, type FormEvent } from "react";
import type { AccountId, MemberEntry, StartNoteRequest } from "@pulse/protocol";
import { Avatar } from "../ui/Avatar";
import { ui } from "../ui/styles";

interface StartNoteFormProps {
  people: MemberEntry[];
  busy: boolean;
  startNote(request: StartNoteRequest): Promise<void>;
}

export function StartNoteForm({ people, busy, startNote }: StartNoteFormProps) {
  const [picked, setPicked] = useState<AccountId[]>([]);

  function toggle(accountId: AccountId) {
    setPicked((current) =>
      current.includes(accountId) ? current.filter((id) => id !== accountId) : [...current, accountId],
    );
  }

  function submit(event: FormEvent) {
    event.preventDefault();
    void startNote({ account_ids: picked });
  }

  return (
    <form className="space-y-4" onSubmit={submit}>
      <ul aria-label="People" className="max-h-72 divide-y divide-line overflow-y-auto rounded-xl border border-line">
        {people.map((person) => (
          <li key={person.account_id}>
            <label className="flex cursor-pointer items-center gap-3 px-3 py-2 hover:bg-muted">
              <input
                type="checkbox"
                className="accent-accent"
                checked={picked.includes(person.account_id)}
                onChange={() => toggle(person.account_id)}
              />
              <Avatar name={person.display_name} />
              <span className="min-w-0 flex-1">
                <span className="block truncate text-sm font-medium text-ink">{person.display_name}</span>
                <span className="block truncate text-xs text-subtle">{person.email}</span>
              </span>
            </label>
          </li>
        ))}
      </ul>
      <div className="flex justify-end">
        <button className={ui.primaryButton} type="submit" disabled={busy || picked.length === 0}>
          Start note
        </button>
      </div>
    </form>
  );
}
