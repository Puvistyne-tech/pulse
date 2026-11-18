import { useState, type FormEvent } from "react";
import type { CreateCampusRequest } from "@pulse/protocol";

interface CreateCampusFormProps {
  busy: boolean;
  createCampus(request: CreateCampusRequest): Promise<void>;
}

const inputClass = "w-full rounded border border-slate-300 px-2 py-1";

export function CreateCampusForm({ busy, createCampus }: CreateCampusFormProps) {
  const [slug, setSlug] = useState("");
  const [displayName, setDisplayName] = useState("");

  function submit(event: FormEvent) {
    event.preventDefault();
    void createCampus({ slug, display_name: displayName });
  }

  return (
    <form className="space-y-2" onSubmit={submit}>
      <h2 className="text-lg">Create campus</h2>
      <label className="block text-sm">
        Campus slug
        <input
          className={inputClass}
          type="text"
          value={slug}
          onChange={(event) => setSlug(event.target.value)}
        />
      </label>
      <label className="block text-sm">
        Campus name
        <input
          className={inputClass}
          type="text"
          value={displayName}
          onChange={(event) => setDisplayName(event.target.value)}
        />
      </label>
      <button
        className="rounded bg-slate-900 px-3 py-1 text-white disabled:opacity-50"
        type="submit"
        disabled={busy}
      >
        Create campus
      </button>
    </form>
  );
}
