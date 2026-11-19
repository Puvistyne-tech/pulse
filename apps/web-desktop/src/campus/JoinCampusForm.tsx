import { useState, type FormEvent } from "react";
import type { JoinCampusRequest } from "@pulse/protocol";
import { ui } from "../ui/styles";

interface JoinCampusFormProps {
  busy: boolean;
  joinCampus(request: JoinCampusRequest): Promise<void>;
}

export function JoinCampusForm({ busy, joinCampus }: JoinCampusFormProps) {
  const [code, setCode] = useState("");

  function submit(event: FormEvent) {
    event.preventDefault();
    void joinCampus({ code });
  }

  return (
    <form className="space-y-4" onSubmit={submit}>
      <div className="space-y-1.5">
        <label className={ui.label}>
          Invite code
          <input
            className={`${ui.input} font-mono text-xs`}
            type="text"
            autoComplete="off"
            spellCheck={false}
            aria-describedby="invite-code-hint"
            value={code}
            onChange={(event) => setCode(event.target.value)}
          />
        </label>
        <p id="invite-code-hint" className={ui.hint}>
          Ask an owner or admin of the campus to create one for you.
        </p>
      </div>
      <div className="flex justify-end">
        <button className={ui.primaryButton} type="submit" disabled={busy}>
          Join campus
        </button>
      </div>
    </form>
  );
}
