import type { ApiError } from "@pulse/protocol";
import { expect, test } from "vitest";
import { errorMessage } from "./errorMessage";

test.each<[string, ApiError, string]>([
  [
    "invalid message body",
    { code: "invalid_input", field: "message_body" },
    "Message is empty or too long",
  ],
  [
    "conflicting message body",
    { code: "conflict", field: "message_body" },
    "That message cannot be used",
  ],
  [
    "forbidden message body",
    { code: "forbidden", field: "message_body" },
    "You are not allowed to do that",
  ],
  [
    "fieldless unavailable",
    { code: "unavailable", field: null },
    "Pulse server unavailable",
  ],
])("%s maps to its copy", (_name, error, expected) => {
  expect(errorMessage(error)).toBe(expected);
});
