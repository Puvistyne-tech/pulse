import type { ApiError, ApiErrorCode, InputField } from "@pulse/protocol";

const invalidFieldMessages: Record<InputField, string> = {
  email: "Enter a valid email address",
  display_name: "Enter a valid display name",
  password: "Enter a valid password",
  slug: "Use lowercase letters, digits and hyphens for the slug",
  message_body: "Message is empty or too long",
};

const conflictFieldMessages: Record<InputField, string> = {
  email: "That email is already registered",
  display_name: "That display name is already taken",
  password: "That password cannot be used",
  slug: "That slug is already taken",
  message_body: "That message cannot be used",
};

const codeMessages: Record<ApiErrorCode, string> = {
  invalid_input: "Check the details you entered",
  unauthenticated: "Sign in again to continue",
  forbidden: "You are not allowed to do that",
  not_found: "That item could not be found",
  conflict: "That already exists",
  unavailable: "Pulse server unavailable",
  internal: "Something went wrong",
};

export function errorMessage(error: ApiError): string {
  if (error.field === null) {
    return codeMessages[error.code];
  }
  if (error.code === "invalid_input") {
    return invalidFieldMessages[error.field];
  }
  if (error.code === "conflict") {
    return conflictFieldMessages[error.field];
  }
  return codeMessages[error.code];
}
