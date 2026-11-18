import type { ApiError } from "@pulse/protocol";
import { errorMessage } from "./errorMessage";

export function ErrorNotice({ error }: { error: ApiError | null }) {
  if (error === null) {
    return null;
  }
  return (
    <p role="alert" className="text-sm text-red-700">
      {errorMessage(error)}
    </p>
  );
}
