import { useState } from "react";
import { Check, Copy } from "lucide-react";
import type { CampusInvite } from "@pulse/protocol";
import { ui } from "../ui/styles";

interface InvitePanelProps {
  invite: CampusInvite | null;
  busy: boolean;
  createInvite(): Promise<void>;
}

export function InvitePanel({ invite, busy, createInvite }: InvitePanelProps) {
  const [copiedCode, setCopiedCode] = useState<string | null>(null);
  const copied = invite !== null && copiedCode === invite.code;

  function copy(code: string) {
    void navigator.clipboard
      ?.writeText(code)
      .then(() => setCopiedCode(code))
      .catch(() => setCopiedCode(null));
  }

  return (
    <div className="space-y-4">
      {invite !== null && (
        <div className="space-y-1.5">
          <label className={ui.label}>
            Invite code
            <div className="flex gap-2">
              <input
                className={`${ui.input} font-mono text-xs`}
                type="text"
                readOnly
                value={invite.code}
                onFocus={(event) => event.currentTarget.select()}
              />
              <button
                className={ui.secondaryButton}
                type="button"
                aria-label="Copy invite code"
                onClick={() => copy(invite.code)}
              >
                {copied ? <Check size={14} aria-hidden="true" /> : <Copy size={14} aria-hidden="true" />}
                {copied ? "Copied" : "Copy"}
              </button>
            </div>
          </label>
          <p className={ui.hint}>
            Valid until {new Date(invite.expires_at_ms).toLocaleDateString()}. Anyone with this code can join this campus.
          </p>
        </div>
      )}
      <div className="flex justify-end">
        <button className={ui.primaryButton} type="button" disabled={busy} onClick={() => void createInvite()}>
          {invite === null ? "Create invite code" : "Create another code"}
        </button>
      </div>
    </div>
  );
}
