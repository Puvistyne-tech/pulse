const buttonBase =
  "inline-flex items-center justify-center gap-1.5 rounded-lg font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50";

export const ui = {
  primaryButton: `${buttonBase} bg-accent px-3.5 py-2 text-sm text-on-accent hover:bg-accent-strong`,
  secondaryButton: `${buttonBase} border border-line bg-surface px-3 py-1.5 text-sm text-ink hover:bg-muted`,
  ghostButton: `${buttonBase} px-2 py-1 text-xs text-subtle hover:bg-muted hover:text-ink`,
  linkButton: "text-sm font-medium text-accent hover:underline",
  iconButton: `${buttonBase} h-8 w-8 text-subtle hover:bg-muted hover:text-ink`,
  toggleButton: `${buttonBase} border border-line px-2.5 py-1 text-xs text-subtle hover:bg-muted aria-pressed:border-accent aria-pressed:bg-accent-soft aria-pressed:text-accent aria-selected:border-accent aria-selected:bg-accent-soft aria-selected:text-accent`,
  input:
    "w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm text-ink placeholder:text-subtle outline-none transition focus:border-accent focus:ring-2 focus:ring-accent/25",
  label: "block space-y-1.5 text-sm font-medium text-ink",
  hint: "text-xs font-normal text-subtle",
  card: "rounded-xl border border-line bg-surface",
  sectionTitle: "text-[11px] font-semibold uppercase tracking-wider text-subtle",
  muted: "text-xs text-subtle",
  pill: "inline-flex items-center rounded-full px-2 py-0.5 text-[11px] font-semibold",
} as const;
