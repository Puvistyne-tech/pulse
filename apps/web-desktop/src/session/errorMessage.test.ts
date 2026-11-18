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
