import { Search, X } from "lucide-react";
import type { SearchSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
import { Avatar } from "../ui/Avatar";
import { ui } from "../ui/styles";

interface SearchPaletteProps {
  search: SearchSnapshot;
  intents: PulseIntents;
}

export function SearchPalette({ search, intents }: SearchPaletteProps) {
  return (
    <div className="fixed inset-0 z-40 flex items-start justify-center bg-black/40 px-4 pt-[12vh] backdrop-blur-[2px]">
      <section
        aria-label="Search"
        className="w-full max-w-xl overflow-hidden rounded-2xl border border-line bg-surface shadow-panel"
      >
        <div className="flex items-center gap-2 border-b border-line px-4">
          <Search size={18} aria-hidden="true" className="text-subtle" />
          <input
            aria-label="Search messages"
            autoFocus
            className="h-14 flex-1 bg-transparent text-[15px] text-ink outline-none placeholder:text-subtle"
            placeholder="Search messages"
            type="text"
            defaultValue={search.query}
            onChange={(event) => void intents.search(event.target.value)}
          />
          <button
            className={ui.iconButton}
            type="button"
            title="Close search"
            onClick={() => void intents.toggleSearch()}
          >
            <X size={16} aria-hidden="true" />
            <span className="sr-only">Close search</span>
          </button>
        </div>
        <div className="max-h-[50vh] space-y-2 overflow-y-auto p-2">
          {search.searching && <p className="px-2 py-1 text-xs text-subtle">Searching</p>}
          <ErrorNotice error={search.error} />
          <ul className="space-y-0.5">
            {search.hits.map((hit, index) => (
              <li
                key={hit.message_id}
                aria-selected={index === search.selected_index}
                className="rounded-lg aria-selected:bg-accent-soft"
              >
                <button
                  className="flex w-full items-start gap-3 rounded-lg px-3 py-2 text-left hover:bg-muted"
                  type="button"
                  onClick={() => void intents.openSearchHit(hit.message_id)}
                >
                  <Avatar name={hit.author_display_name} size="sm" />
                  <span className="min-w-0 flex-1">
                    <span className="block text-xs text-subtle">
                      {hit.topic_title !== null && <strong className="text-ink">{hit.topic_title} </strong>}
                      {hit.author_display_name}
                    </span>
                    <span className="block text-sm text-ink [&_mark]:rounded [&_mark]:bg-warning-soft [&_mark]:px-0.5 [&_mark]:text-warning">
                      {hit.snippet.map((span, spanIndex) =>
                        span.matched ? (
                          <mark key={spanIndex}>{span.text}</mark>
                        ) : (
                          <span key={spanIndex}>{span.text}</span>
                        ),
                      )}
                    </span>
                  </span>
                </button>
              </li>
            ))}
          </ul>
        </div>
        <footer className="flex items-center gap-3 border-t border-line bg-muted px-4 py-2 text-[11px] text-subtle">
          <span>Up and Down to move</span>
          <span>Enter to open</span>
          <span>Esc to close</span>
        </footer>
      </section>
    </div>
  );
}
