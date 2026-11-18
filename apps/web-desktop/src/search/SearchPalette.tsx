import type { SearchSnapshot } from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";

interface SearchPaletteProps {
  search: SearchSnapshot;
  intents: PulseIntents;
}

export function SearchPalette({ search, intents }: SearchPaletteProps) {
  return (
    <section
      aria-label="Search"
      className="space-y-2 rounded border border-slate-300 p-3 shadow"
    >
      <input
        aria-label="Search messages"
        autoFocus
        className="w-full rounded border border-slate-300 px-2 py-1"
        type="text"
        defaultValue={search.query}
        onChange={(event) => void intents.search(event.target.value)}
      />
      {search.searching && <p className="text-xs text-slate-500">Searching</p>}
      <ErrorNotice error={search.error} />
      <ul className="space-y-1">
        {search.hits.map((hit, index) => (
          <li
            key={hit.message_id}
            aria-selected={index === search.selected_index}
            className="rounded p-2 aria-selected:bg-slate-100"
          >
            <button
              className="w-full text-left"
              type="button"
              onClick={() => void intents.openSearchHit(hit.message_id)}
            >
              <span className="block text-xs text-slate-600">
                {hit.topic_title !== null && <strong>{hit.topic_title} </strong>}
                {hit.author_display_name}
              </span>
              <span className="block text-sm">
                {hit.snippet.map((span, spanIndex) =>
                  span.matched ? (
                    <mark key={spanIndex}>{span.text}</mark>
                  ) : (
                    <span key={spanIndex}>{span.text}</span>
                  ),
                )}
              </span>
            </button>
          </li>
        ))}
      </ul>
      <button
        className="text-sm underline"
        type="button"
        onClick={() => void intents.toggleSearch()}
      >
        Close search
      </button>
    </section>
  );
}
