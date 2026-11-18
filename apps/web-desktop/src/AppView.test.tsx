import { cleanup, render, screen } from "@testing-library/react";
import type { ClientSnapshot } from "@pulse/protocol";
import { afterEach, expect, test } from "vitest";
import { AppView } from "./AppView";
import {
  authenticating,
  noopIntents,
  signedInNoCampus,
  signedInWithRooms,
  signedOut,
  signedOutWithError,
} from "./test/fixtures";

afterEach(cleanup);

interface Case {
  snapshot: ClientSnapshot;
  present: string[];
  absent: string[];
}

const cases: ReadonlyArray<[string, Case]> = [
  [
    "signed out",
    {
      snapshot: signedOut,
      present: ["Sign in"],
      absent: ["Sign out", "Create campus"],
    },
  ],
  [
    "authenticating",
    {
      snapshot: authenticating,
      present: ["Sign in"],
      absent: ["Sign out", "Create campus"],
    },
  ],
  [
    "signed out with error",
    {
      snapshot: signedOutWithError,
      present: ["Enter a valid email address"],
      absent: ["Sign out"],
    },
  ],
  [
    "signed in without campus",
    {
      snapshot: signedInNoCampus,
      present: ["Ada", "Sign out", "Create campus"],
      absent: ["#commons", "#courtyard"],
    },
  ],
  [
    "signed in with rooms",
    {
      snapshot: signedInWithRooms,
      present: [
        "Ada",
        "Sign out",
        "Alpha Campus",
        "Beta Campus",
        "#commons",
        "#courtyard",
      ],
      absent: [],
    },
  ],
];

test.each(cases)("%s", (_name, { snapshot, present, absent }) => {
  render(<AppView snapshot={snapshot} intents={noopIntents()} />);
  for (const text of present) {
    expect(screen.getAllByText(text).length).toBeGreaterThan(0);
  }
  for (const text of absent) {
    expect(screen.queryByText(text)).toBeNull();
  }
});

test("rooms render in snapshot order", () => {
  render(<AppView snapshot={signedInWithRooms} intents={noopIntents()} />);
  const labels = screen
    .getAllByRole("listitem")
    .map((item) => item.textContent)
    .filter((text) => text?.startsWith("#"));
  expect(labels).toEqual(["#commons", "#courtyard"]);
});
