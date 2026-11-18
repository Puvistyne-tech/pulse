import { cleanup, createEvent, fireEvent, render, screen } from "@testing-library/react";
import type { ClientSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import { AppView } from "./AppView";
import {
  MESSAGE_A,
  MESSAGE_B,
  authenticating,
  expectOnlyCalled,
  inboxItemOf,
  inboxOf,
  noopIntents,
  signedInNoCampus,
  signedInWithMessages,
  signedInWithRooms,
  signedOut,
  signedOutWithError,
  threadOf,
  withExtras,
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
  [
    "signed in with messages",
    {
      snapshot: signedInWithMessages,
      present: ["Grace", "welcome", "You", "hello", "Sending", "Syncing", "Send"],
      absent: ["Discard"],
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

const inboxItems = [inboxItemOf(MESSAGE_B), inboxItemOf(MESSAGE_A)];
const openInbox = withExtras(signedInWithRooms, { inbox: inboxOf(inboxItems) });

describe("panels", () => {
  test.each<[string, ClientSnapshot, string, boolean]>([
    ["closed inbox hides the inbox", signedInWithRooms, "Inbox", false],
    ["open inbox shows the inbox", openInbox, "Inbox", true],
    ["no open thread hides the thread panel", signedInWithRooms, "Thread", false],
    [
      "open thread shows the thread panel",
      withExtras(signedInWithRooms, { thread: threadOf() }),
      "Thread",
      true,
    ],
  ])("%s", (_name, snapshot, label, visible) => {
    render(<AppView snapshot={snapshot} intents={noopIntents()} />);
    expect(screen.queryByLabelText(label) !== null).toBe(visible);
  });
});

describe("inbox keys", () => {
  test.each<[string, ClientSnapshot, Partial<KeyboardEventInit>, boolean]>([
    ["a mapped key is dispatched and prevented", openInbox, { key: "j" }, true],
    ["an unmapped key is ignored and not prevented", openInbox, { key: "x" }, false],
  ])("%s", (_name, snapshot, key, dispatched) => {
    const intents = noopIntents();
    render(<AppView snapshot={snapshot} intents={intents} />);
    const event = createEvent.keyDown(window, { cancelable: true, ...key });
    fireEvent(window, event);
    expect(event.defaultPrevented).toBe(dispatched);
    expectOnlyCalled(
      intents,
      dispatched ? "moveInboxSelection" : null,
      dispatched ? ["next"] : [],
    );
  });

  test("typing in the composer never triggers inbox shortcuts", () => {
    const intents = noopIntents();
    render(<AppView snapshot={openInbox} intents={intents} />);
    const input = screen.getByLabelText("Message");
    for (const key of ["j", "k", "e", "r"]) {
      fireEvent.keyDown(input, { key });
    }
    expect(intents.moveInboxSelection).not.toHaveBeenCalled();
    expect(intents.archiveInboxSelection).not.toHaveBeenCalled();
    expect(intents.startInboxReply).not.toHaveBeenCalled();
  });

  test("cmd+i still toggles from the composer", () => {
    const intents = noopIntents();
    render(<AppView snapshot={signedInWithRooms} intents={intents} />);
    fireEvent.keyDown(screen.getByLabelText("Message"), {
      key: "i",
      metaKey: true,
    });
    expect(intents.toggleInbox).toHaveBeenCalledOnce();
  });

  test("shortcuts stop after the view unmounts", () => {
    const intents = noopIntents();
    const { unmount } = render(
      <AppView snapshot={signedInWithRooms} intents={intents} />,
    );
    unmount();
    fireEvent.keyDown(window, { key: "i", metaKey: true });
    expect(intents.toggleInbox).not.toHaveBeenCalled();
  });
});
