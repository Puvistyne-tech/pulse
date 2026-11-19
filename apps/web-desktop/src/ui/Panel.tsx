import type { ReactNode } from "react";
import { X } from "lucide-react";
import { ui } from "./styles";

interface PanelProps {
  label: string;
  title: ReactNode;
  closeLabel: string;
  onClose(): void;
  children: ReactNode;
  as?: "section" | "aside";
}

export function Panel({
  label,
  title,
  closeLabel,
  onClose,
  children,
  as: Tag = "section",
}: PanelProps) {
  return (
    <Tag aria-label={label} className="flex min-h-0 flex-1 flex-col bg-surface">
      <header className="flex h-14 shrink-0 items-center justify-between border-b border-line px-4">
        <h2 className="flex items-center gap-2 text-sm font-semibold text-ink">{title}</h2>
        <button className={ui.iconButton} type="button" title={closeLabel} onClick={onClose}>
          <X size={16} aria-hidden="true" />
          <span className="sr-only">{closeLabel}</span>
        </button>
      </header>
      <div className="min-h-0 flex-1 space-y-4 overflow-y-auto p-4">{children}</div>
    </Tag>
  );
}
