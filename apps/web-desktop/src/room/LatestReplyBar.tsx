import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import type { ThreadPreview } from "@pulse/protocol";

interface LatestReplyBarProps {
  thread: ThreadPreview;
  openThread(): void;
}

export function LatestReplyBar({ thread, openThread }: LatestReplyBarProps) {
  const [collapsed, setCollapsed] = useState(false);
  const latest = thread.latest_reply;
  if (!thread.show_latest_reply || latest === null) {
    return null;
  }
  return (
    <aside
      aria-label="Latest reply"
      className="flex max-w-xl items-center gap-2 rounded-lg border border-line bg-muted px-2.5 py-1.5 text-xs"
    >
      <button
        className="inline-flex items-center gap-1 font-medium text-subtle hover:text-ink"
        type="button"
        aria-expanded={!collapsed}
        onClick={() => setCollapsed(!collapsed)}
      >
        {collapsed ? <ChevronRight size={12} aria-hidden="true" /> : <ChevronDown size={12} aria-hidden="true" />}
        Latest reply
      </button>
      {!collapsed && (
        <>
          <span className="font-semibold text-ink">{latest.author_display_name}</span>
          <span className="flex-1 truncate text-subtle">{latest.preview}</span>
          <button className="font-medium text-accent hover:underline" type="button" onClick={openThread}>
            Open thread
          </button>
        </>
      )}
    </aside>
  );
}
