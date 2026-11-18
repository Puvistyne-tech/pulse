import { useState } from "react";
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
      className="flex items-center gap-2 rounded bg-slate-100 px-2 py-1 text-xs"
    >
      <button
        type="button"
        aria-expanded={!collapsed}
        onClick={() => setCollapsed(!collapsed)}
      >
        Latest reply
      </button>
      {!collapsed && (
        <>
          <span className="font-semibold">{latest.author_display_name}</span>
          <span className="flex-1 truncate">{latest.preview}</span>
          <button className="underline" type="button" onClick={openThread}>
            Open thread
          </button>
        </>
      )}
    </aside>
  );
}
