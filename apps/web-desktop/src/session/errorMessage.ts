import type { ApiError, ApiErrorCode, InputField } from "@pulse/protocol";

const invalidFieldMessages: Record<InputField, string> = {
  email: "Enter a valid email address",
  display_name: "Enter a valid display name",
  password: "Enter a valid password",
  slug: "Use lowercase letters, digits and hyphens for the slug",
  message_body: "Message is empty or too long",
  topic_title: "Enter a shorter topic title without control characters",
  reaction_emoji: "Choose a single emoji reaction",
  file_name: "Choose a file with a valid name",
  content_type: "That file type cannot be used",
  byte_size: "That file is empty or too large",
  attachments: "Check the attachments on that message",
  remind_at: "Choose a reminder time that is not too far in the past or future",
  rooms: "Choose a valid set of rooms for that steward",
  invite_code: "That invite code is not valid or has expired",
  role: "Choose a role you can assign",
  note_members: "Pick people from this campus to start a note",
};

const conflictFieldMessages: Record<InputField, string> = {
  email: "That email is already registered",
  display_name: "That display name is already taken",
  password: "That password cannot be used",
  slug: "That slug is already taken",
  message_body: "That message cannot be used",
  topic_title: "That topic title cannot be used",
  reaction_emoji: "You have reacted with enough different emoji",
  file_name: "That file name cannot be used",
  content_type: "That file type cannot be used",
  byte_size: "That file cannot be used",
  attachments: "Those attachments cannot be used",
  remind_at: "That reminder time cannot be used",
  rooms: "Those rooms cannot be used",
  invite_code: "That invite code cannot be used",
  role: "That role cannot be used",
  note_members: "Those people cannot be in a note together",
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
  if (error.code === "not_found" && error.field === "email") {
    return "No Pulse account uses that email yet";
  }
  if (error.code === "conflict") {
    return conflictFieldMessages[error.field];
  }
  return codeMessages[error.code];
}
