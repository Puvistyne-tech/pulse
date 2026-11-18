import type { VaultSnapshot, VaultTab } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";

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
    <section aria-label="Vault" className="space-y-2">
      <header className="flex items-center justify-between">
        <h2 className="font-semibold">Vault</h2>
        <button
          className="text-sm underline"
          type="button"
          onClick={() => void intents.closeVault()}
        >
          Close vault
        </button>
      </header>
      <div role="tablist" className="flex gap-3 text-sm">
        {tabs.map(({ tab, label }) => (
          <button
            key={tab}
            role="tab"
            aria-selected={tab === vault.tab}
            className="underline aria-selected:font-semibold"
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
      {vault.loading && <p className="text-xs text-slate-500">Loading</p>}
      <ul className="space-y-2">
        {vault.entries.map((entry, index) => (
          <li key={`${entry.message_id}-${index}`} className="text-sm">
            <span className="block text-xs text-slate-600">
              {entry.author_display_name}
            </span>
            {entry.attachment !== null && (
              <AttachmentLink attachment={entry.attachment} />
            )}
            {entry.link !== null && (
              <a
                className="underline"
                href={entry.link}
                rel="noreferrer"
                target="_blank"
              >
                {entry.link}
              </a>
            )}
            <span className="block text-xs text-slate-600">{entry.preview}</span>
          </li>
        ))}
      </ul>
      {vault.has_more && (
        <button
          className="text-sm underline"
          type="button"
          onClick={() => void intents.loadMoreVault()}
        >
          Load more
        </button>
      )}
    </section>
  );
}

function AttachmentLink({
  attachment,
}: {
  attachment: NonNullable<VaultSnapshot["entries"][number]["attachment"]>;
}) {
  if (attachment.url === null) {
    return <span>{attachment.file_name}</span>;
  }
  if (attachment.kind === "image") {
    return (
      <img
        alt={attachment.file_name}
        className="max-h-32 rounded"
        src={attachment.url}
      />
    );
  }
  return (
    <a
      className="underline"
      href={attachment.url}
      rel="noreferrer"
      target="_blank"
    >
      {attachment.file_name}
    </a>
  );
}
