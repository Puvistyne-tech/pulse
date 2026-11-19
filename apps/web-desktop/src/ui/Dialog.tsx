import { useEffect, type ReactNode } from "react";
import { X } from "lucide-react";
import { ui } from "./styles";

interface DialogProps {
  title: string;
  description?: string;
  closeLabel: string;
  onClose(): void;
  children: ReactNode;
}

export function Dialog({ title, description, closeLabel, onClose, children }: DialogProps) {
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        onClose();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  return (
    <div className="fixed inset-0 z-40 flex items-center justify-center bg-black/40 px-4 backdrop-blur-[2px]">
      <div
        role="dialog"
        aria-modal="true"
        aria-label={title}
        className="w-full max-w-md space-y-5 rounded-2xl border border-line bg-surface p-6 shadow-panel"
      >
        <header className="flex items-start justify-between gap-4">
          <div className="space-y-1">
            <h2 className="text-lg font-semibold text-ink">{title}</h2>
            {description && <p className="text-sm text-subtle">{description}</p>}
          </div>
          <button className={ui.iconButton} type="button" title={closeLabel} onClick={onClose}>
            <X size={16} aria-hidden="true" />
            <span className="sr-only">{closeLabel}</span>
          </button>
        </header>
        {children}
      </div>
    </div>
  );
}
