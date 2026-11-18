import type { ApiError, ApiErrorCode, InputField } from "@pulse/protocol";
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
    "invalid topic title",
    { code: "invalid_input", field: "topic_title" },
    "Enter a shorter topic title without control characters",
  ],
  [
    "conflicting topic title",
    { code: "conflict", field: "topic_title" },
    "That topic title cannot be used",
  ],
  [
    "invalid reaction emoji",
    { code: "invalid_input", field: "reaction_emoji" },
    "Choose a single emoji reaction",
  ],
  [
    "too many reaction kinds",
    { code: "conflict", field: "reaction_emoji" },
    "You have reacted with enough different emoji",
  ],
  [
    "forbidden reaction",
    { code: "forbidden", field: "reaction_emoji" },
    "You are not allowed to do that",
  ],
  ["invalid file name", { code: "invalid_input", field: "file_name" }, "Choose a file with a valid name"],
  ["conflicting file name", { code: "conflict", field: "file_name" }, "That file name cannot be used"],
  ["invalid content type", { code: "invalid_input", field: "content_type" }, "That file type cannot be used"],
  ["conflicting content type", { code: "conflict", field: "content_type" }, "That file type cannot be used"],
  ["invalid byte size", { code: "invalid_input", field: "byte_size" }, "That file is empty or too large"],
  ["conflicting byte size", { code: "conflict", field: "byte_size" }, "That file cannot be used"],
  ["invalid attachments", { code: "invalid_input", field: "attachments" }, "Check the attachments on that message"],
  ["conflicting attachments", { code: "conflict", field: "attachments" }, "Those attachments cannot be used"],
  [
    "invalid reminder time",
    { code: "invalid_input", field: "remind_at" },
    "Choose a reminder time that is not too far in the past or future",
  ],
  ["conflicting reminder time", { code: "conflict", field: "remind_at" }, "That reminder time cannot be used"],
  ["forbidden upload", { code: "forbidden", field: "byte_size" }, "You are not allowed to do that"],
  ["unavailable storage", { code: "unavailable", field: "attachments" }, "Pulse server unavailable"],
  [
    "not found without a field",
    { code: "not_found", field: null },
    "That item could not be found",
  ],
  [
    "fieldless unavailable",
    { code: "unavailable", field: null },
    "Pulse server unavailable",
  ],
])("%s maps to its copy", (_name, error, expected) => {
  expect(errorMessage(error)).toBe(expected);
});

test.each<[string, ApiError]>([
  ["invalid topic title", { code: "invalid_input", field: "topic_title" }],
  ["conflicting topic title", { code: "conflict", field: "topic_title" }],
  ["invalid reaction emoji", { code: "invalid_input", field: "reaction_emoji" }],
  ["conflicting reaction emoji", { code: "conflict", field: "reaction_emoji" }],
  ["invalid message body", { code: "invalid_input", field: "message_body" }],
])("%s copy contains no limit numbers", (_name, error) => {
  expect(errorMessage(error)).not.toMatch(/\d/);
});

const inputFields: InputField[] = [
  "email",
  "display_name",
  "password",
  "slug",
  "message_body",
  "topic_title",
  "reaction_emoji",
  "file_name",
  "content_type",
  "byte_size",
  "attachments",
  "remind_at",
];

const codes: ApiErrorCode[] = [
  "invalid_input",
  "unauthenticated",
  "forbidden",
  "not_found",
  "conflict",
  "unavailable",
  "internal",
];

test.each(
  codes.flatMap((code) =>
    [null, ...inputFields].map((field): [string, ApiError] => [
      `${code} ${field ?? "without a field"}`,
      { code, field },
    ]),
  ),
)("%s copy never carries a limit number", (_name, error) => {
  const text = errorMessage(error);
  expect(text).not.toMatch(/\d/);
  expect(text.length).toBeGreaterThan(0);
});
