import { Archive } from "lucide-react";
import type { VaultSnapshot, VaultTab } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { AttachmentLink } from "../room/AttachmentList";
import { ErrorNotice } from "../session/ErrorNotice";
import { Panel } from "../ui/Panel";
import { ui } from "../ui/styles";

const tabs: { tab: VaultTab; label: string }[] = [
  { tab: "media", label: "Media" },
  { tab: "files", label: "Files" },
  { tab: "links", label: "Links" },
  { tab: "pinned", label: "Pinned" },
];

interface VaultPanelProps {
  vault: VaultSnapshot;
  intents: PulseIntents;
}

export function VaultPanel({ vault, intents }: VaultPanelProps) {
  const roomId = vault.room_id;
  return (
    <Panel
      label="Vault"
      title={
        <>
          <Archive size={16} aria-hidden="true" className="text-accent" />
          Vault
        </>
      }
      closeLabel="Close vault"
      onClose={() => void intents.closeVault()}
    >
      <div role="tablist" className="flex gap-1 rounded-lg bg-muted p-1">
        {tabs.map(({ tab, label }) => (
          <button
            key={tab}
            role="tab"
            aria-selected={tab === vault.tab}
            className="flex-1 rounded-md px-2 py-1 text-xs font-medium text-subtle transition hover:text-ink aria-selected:bg-surface aria-selected:text-ink aria-selected:shadow-sm"
            type="button"
            disabled={roomId === null}
            onClick={() =>
              roomId !== null && void intents.openVault(roomId, tab)
            }
          >
            {label}
          </button>
        ))}
      </div>
      <ErrorNotice error={vault.error} />
      {vault.loading && <p className={ui.muted}>Loading</p>}
      {!vault.loading && vault.entries.length === 0 && (
        <p className="py-8 text-center text-sm text-subtle">Nothing saved in this room yet.</p>
      )}
      <ul className="space-y-2">
        {vault.entries.map((entry, index) => (
          <li key={`${entry.message_id}-${index}`} className="space-y-1.5 rounded-xl border border-line p-3 text-sm">
            <span className="block text-xs font-medium text-subtle">
              {entry.author_display_name}
            </span>
            {entry.attachment !== null && (
              <AttachmentLink attachment={entry.attachment} />
            )}
            {entry.link !== null && (
              <a
                className="block truncate text-accent hover:underline"
                href={entry.link}
                rel="noreferrer"
                target="_blank"
              >
                {entry.link}
              </a>
            )}
            <span className="block text-xs text-subtle">{entry.preview}</span>
          </li>
        ))}
      </ul>
      {vault.has_more && (
        <button
          className={`${ui.secondaryButton} w-full`}
          type="button"
          onClick={() => void intents.loadMoreVault()}
        >
          Load more
        </button>
      )}
    </Panel>
  );
}
