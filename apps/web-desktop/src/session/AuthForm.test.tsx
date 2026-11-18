import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, test } from "vitest";
import { noopIntents } from "../test/fixtures";
import { AuthForm } from "./AuthForm";

afterEach(cleanup);

interface Case {
  toggle: string | null;
  fields: Record<string, string>;
  submit: string;
  intent: "signIn" | "signUp";
  payload: Record<string, string>;
}

const cases: ReadonlyArray<[string, Case]> = [
  [
    "sign in exactly as typed",
    {
      toggle: null,
      fields: { Email: "Ada@Example.test", Password: " p4ss word " },
      submit: "Sign in",
      intent: "signIn",
      payload: { email: "Ada@Example.test", password: " p4ss word " },
    },
  ],
  [
    "sign up exactly as typed",
    {
      toggle: "New here? Create account",
      fields: {
        Email: "ada@example.test",
        "Display name": "  Ada ",
        Password: "correct-horse-battery",
      },
      submit: "Create account",
      intent: "signUp",
      payload: {
        email: "ada@example.test",
        display_name: "  Ada ",
        password: "correct-horse-battery",
      },
    },
  ],
];

test.each(cases)("%s", (_name, { toggle, fields, submit, intent, payload }) => {
  const intents = noopIntents();
  render(<AuthForm busy={false} error={null} intents={intents} />);
  if (toggle) {
    fireEvent.click(screen.getByText(toggle));
  }
  for (const [label, value] of Object.entries(fields)) {
    fireEvent.change(screen.getByLabelText(label), { target: { value } });
  }
  fireEvent.click(screen.getByRole("button", { name: submit }));
  expect(intents[intent]).toHaveBeenCalledExactlyOnceWith(payload);
});

test.each([
  [true, true],
  [false, false],
])("busy %s disables submit %s", (busy, disabled) => {
  render(<AuthForm busy={busy} error={null} intents={noopIntents()} />);
  const button = screen.getByRole("button", { name: "Sign in" });
  expect((button as HTMLButtonElement).disabled).toBe(disabled);
});

test.each([
  ["Email", "email"],
  ["Password", "password"],
])("%s field has input type %s", (label, type) => {
  render(<AuthForm busy={false} error={null} intents={noopIntents()} />);
  expect(screen.getByLabelText(label).getAttribute("type")).toBe(type);
});
