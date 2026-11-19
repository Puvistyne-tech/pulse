import { useState, type FormEvent } from "react";
import type { ApiError } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { PulseMark } from "../ui/PulseMark";
import { ui } from "../ui/styles";
import { ErrorNotice } from "./ErrorNotice";

type Mode = "sign_in" | "sign_up";

interface AuthFormProps {
  busy: boolean;
  error: ApiError | null;
  intents: Pick<PulseIntents, "signIn" | "signUp">;
}

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
    <main className="flex min-h-full items-center justify-center bg-muted px-4 py-12">
      <div className="w-full max-w-sm space-y-6">
        <div className="flex flex-col items-center gap-3 text-center">
          <PulseMark size={44} />
          <h1 className="text-2xl font-semibold tracking-tight text-ink">Pulse</h1>
          <p className="text-sm text-subtle">
            {signingUp ? "Create your account to join a campus." : "Welcome back. Sign in to your campus."}
          </p>
        </div>
        <div className={`${ui.card} space-y-5 p-6 shadow-panel`}>
          <h2 className="text-lg font-semibold text-ink">{signingUp ? "Create account" : "Sign in"}</h2>
          <form className="space-y-4" onSubmit={submit}>
            <label className={ui.label}>
              Email
              <input
                className={ui.input}
                type="email"
                autoComplete="email"
                placeholder="you@example.org"
                value={email}
                onChange={(event) => setEmail(event.target.value)}
              />
            </label>
            {signingUp && (
              <label className={ui.label}>
                Display name
                <input
                  className={ui.input}
                  type="text"
                  autoComplete="name"
                  value={displayName}
                  onChange={(event) => setDisplayName(event.target.value)}
                />
              </label>
            )}
            <label className={ui.label}>
              Password
              <input
                className={ui.input}
                type="password"
                autoComplete={signingUp ? "new-password" : "current-password"}
                value={password}
                onChange={(event) => setPassword(event.target.value)}
              />
            </label>
            <ErrorNotice error={error} />
            <button className={`${ui.primaryButton} w-full py-2.5`} type="submit" disabled={busy}>
              {signingUp ? "Create account" : "Sign in"}
            </button>
          </form>
        </div>
        <p className="text-center">
          <button
            className={ui.linkButton}
            type="button"
            onClick={() => setMode(signingUp ? "sign_in" : "sign_up")}
          >
            {signingUp ? "Have an account? Sign in" : "New here? Create account"}
          </button>
        </p>
      </div>
    </main>
  );
}
