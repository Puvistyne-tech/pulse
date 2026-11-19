import { CircleAlert } from "lucide-react";
import type { ApiError } from "@pulse/protocol";
import { errorMessage } from "./errorMessage";

export function ErrorNotice({ error }: { error: ApiError | null }) {
  if (error === null) {
    return null;
  }
  return (
    <p
      role="alert"
      className="flex items-start gap-2 rounded-lg border border-danger/30 bg-danger-soft px-3 py-2 text-sm text-danger"
    >
      <CircleAlert size={16} aria-hidden="true" className="mt-0.5 shrink-0" />
      <span>{errorMessage(error)}</span>
    </p>
  );
}
