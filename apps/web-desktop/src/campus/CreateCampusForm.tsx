import { useState, type FormEvent } from "react";
import type { CreateCampusRequest } from "@pulse/protocol";
import { ui } from "../ui/styles";

interface CreateCampusFormProps {
  busy: boolean;
  createCampus(request: CreateCampusRequest): Promise<void>;
}

export function CreateCampusForm({ busy, createCampus }: CreateCampusFormProps) {
  const [slug, setSlug] = useState("");
  const [displayName, setDisplayName] = useState("");

  function submit(event: FormEvent) {
    event.preventDefault();
    void createCampus({ slug, display_name: displayName });
  }

  return (
    <form className="space-y-4" onSubmit={submit}>
      <label className={ui.label}>
        Campus name
        <input
          className={ui.input}
          type="text"
          placeholder="Design Guild"
          value={displayName}
          onChange={(event) => setDisplayName(event.target.value)}
        />
      </label>
      <div className="space-y-1.5">
        <label className={ui.label}>
          Campus slug
          <input
            className={ui.input}
            type="text"
            placeholder="design-guild"
            aria-describedby="campus-slug-hint"
            value={slug}
            onChange={(event) => setSlug(event.target.value)}
          />
        </label>
        <p id="campus-slug-hint" className={ui.hint}>
          Lowercase letters, digits and hyphens. Every campus on this server needs its own slug.
        </p>
      </div>
      <div className="flex justify-end">
        <button className={ui.primaryButton} type="submit" disabled={busy}>
          Create campus
        </button>
      </div>
    </form>
  );
}
