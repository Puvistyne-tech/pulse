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
  searchHitOf,
  searchOf,
  shelfEntryOf,
  shelfOf,
  signedInNoCampus,
  signedInWithMessages,
  signedInWithRooms,
  signedOut,
  signedOutWithError,
  threadOf,
  vaultEntryOf,
  vaultOf,
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

const openSearch = withExtras(signedInWithRooms, {
  search: searchOf([searchHitOf(MESSAGE_A)]),
});
const openShelf = withExtras(signedInWithRooms, {
  shelf: shelfOf({ due: [shelfEntryOf("due", MESSAGE_A)] }),
});
const openVault = withExtras(signedInWithRooms, {
  vault: vaultOf([vaultEntryOf("links", MESSAGE_A, { link: "https://example.org/a" })]),
});
const reminded = withExtras(signedInWithRooms, {
  shelf: shelfOf(
    { due: [shelfEntryOf("due", MESSAGE_A)] },
    { open: false, latest_reminder: shelfEntryOf("due", MESSAGE_A, { preview: "call back" }) },
  ),
});

describe("search, vault and shelf wiring", () => {
  test.each<[string, ClientSnapshot, string, boolean]>([
    ["closed search hides the palette", signedInWithRooms, "Search", false],
    ["open search shows the palette", openSearch, "Search", true],
    ["closed shelf hides the shelf", signedInWithRooms, "Shelf", false],
    ["open shelf shows the shelf", openShelf, "Shelf", true],
    ["closed vault hides the vault", signedInWithRooms, "Vault", false],
    ["open vault shows the vault", openVault, "Vault", true],
    ["no reminder hides the notice", signedInWithRooms, "Reminder", false],
    ["a reminder shows the notice", reminded, "Reminder", true],
  ])("%s", (_name, snapshot, label, visible) => {
    render(<AppView snapshot={snapshot} intents={noopIntents()} />);
    expect(screen.queryByLabelText(label) !== null).toBe(visible);
  });

  test.each<[string, string, "toggleSearch" | "toggleShelf"]>([
    ["the search button", "Search", "toggleSearch"],
    ["the shelf button", "Shelf", "toggleShelf"],
  ])("%s toggles its panel", (_name, label, call) => {
    const intents = noopIntents();
    render(<AppView snapshot={signedInWithRooms} intents={intents} />);
    fireEvent.click(screen.getByRole("button", { name: new RegExp(`^${label}`) }));
    expectOnlyCalled(intents, call);
  });

  test("the shelf button shows the due count", () => {
    render(<AppView snapshot={reminded} intents={noopIntents()} />);
    expect(screen.getByLabelText("Due reminders").textContent).toBe("1");
  });

  test("dismissing the notice calls its intent", () => {
    const intents = noopIntents();
    render(<AppView snapshot={reminded} intents={intents} />);
    fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));
    expectOnlyCalled(intents, "dismissReminder");
  });

  test.each<[string, ClientSnapshot, Partial<KeyboardEventInit>, "toggleSearch" | "moveSearchSelection" | null, unknown[]]>([
    ["cmd+k opens the palette", signedInWithRooms, { key: "k", metaKey: true }, "toggleSearch", []],
    ["ctrl+k closes the palette", openSearch, { key: "k", ctrlKey: true }, "toggleSearch", []],
    ["arrows move through hits", openSearch, { key: "ArrowDown" }, "moveSearchSelection", ["next"]],
    ["arrows are ignored while closed", signedInWithRooms, { key: "ArrowDown" }, null, []],
  ])("%s", (_name, snapshot, key, call, args) => {
    const intents = noopIntents();
    render(<AppView snapshot={snapshot} intents={intents} />);
    const event = createEvent.keyDown(window, { cancelable: true, ...key });
    fireEvent(window, event);
    expect(event.defaultPrevented).toBe(call !== null);
    expectOnlyCalled(intents, call, args);
  });
});
