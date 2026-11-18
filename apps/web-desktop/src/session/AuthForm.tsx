import { useState, type FormEvent } from "react";
import type { ApiError } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "./ErrorNotice";

type Mode = "sign_in" | "sign_up";

interface AuthFormProps {
  busy: boolean;
  error: ApiError | null;
  intents: Pick<PulseIntents, "signIn" | "signUp">;
}

const inputClass = "w-full rounded border border-slate-300 px-2 py-1";

export function AuthForm({ busy, error, intents }: AuthFormProps) {
  const [mode, setMode] = useState<Mode>("sign_in");
  const [email, setEmail] = useState("");
  const [displayName, setDisplayName] = useState("");
  const [password, setPassword] = useState("");
  const signingUp = mode === "sign_up";

  function submit(event: FormEvent) {
    event.preventDefault();
    if (signingUp) {
      void intents.signUp({ email, display_name: displayName, password });
    } else {
      void intents.signIn({ email, password });
    }
  }

  return (
    <main className="mx-auto max-w-md space-y-4 p-6 text-slate-900">
      <h1 className="text-2xl font-semibold">Pulse</h1>
      <h2 className="text-lg">{signingUp ? "Create account" : "Sign in"}</h2>
      <form className="space-y-3" onSubmit={submit}>
        <label className="block text-sm">
          Email
          <input
            className={inputClass}
            type="email"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </label>
        {signingUp && (
          <label className="block text-sm">
            Display name
            <input
              className={inputClass}
              type="text"
              value={displayName}
              onChange={(event) => setDisplayName(event.target.value)}
            />
          </label>
        )}
        <label className="block text-sm">
          Password
          <input
            className={inputClass}
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
          />
        </label>
        <ErrorNotice error={error} />
        <button
          className="rounded bg-slate-900 px-3 py-1 text-white disabled:opacity-50"
          type="submit"
          disabled={busy}
        >
          {signingUp ? "Create account" : "Sign in"}
        </button>
      </form>
      <button
        className="text-sm text-slate-600 underline"
        type="button"
        onClick={() => setMode(signingUp ? "sign_in" : "sign_up")}
      >
        {signingUp ? "Have an account? Sign in" : "New here? Create account"}
      </button>
    </main>
  );
}
