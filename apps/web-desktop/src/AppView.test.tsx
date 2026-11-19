import { cleanup, createEvent, fireEvent, render, screen } from "@testing-library/react";
import type { ApiError, ClientSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import { AppView } from "./AppView";
import {
  LOUNGE_A,
  MESSAGE_A,
  MESSAGE_B,
  ROOM_A,
  STEWARD_A,
  authenticating,
  expectOnlyCalled,
  inboxItemOf,
  inboxOf,
  loungeEntryOf,
  loungesOf,
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
  stewardEntryOf,
  stewardsOf,
  streamingEntryOf,
  timelineOf,
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

const withLounges = withExtras(signedInWithRooms, {
  lounges: loungesOf([loungeEntryOf("campus")]),
});
const withStewards = withExtras(signedInWithRooms, {
  stewards: stewardsOf([stewardEntryOf(STEWARD_A, "Scribe")]),
});
const withMentionable = {
  ...withStewards,
  timeline: timelineOf([], { mentionable_stewards: [stewardEntryOf(STEWARD_A, "Scribe")] }),
};
const closedStewards = withExtras(signedInWithRooms, {
  stewards: stewardsOf([stewardEntryOf(STEWARD_A, "Scribe")], { open: false }),
});

describe("steward and lounge wiring", () => {
  test.each<[string, ClientSnapshot, string, boolean]>([
    ["no lounges hides the lounge panel", signedInWithRooms, "Lounges", false],
    ["lounges show the lounge panel", withLounges, "Lounges", true],
    ["a closed stewards panel is hidden", closedStewards, "Stewards", false],
    ["an open stewards panel is shown", withStewards, "Stewards", true],
  ])("%s", (_name, snapshot, label, visible) => {
    render(<AppView snapshot={snapshot} intents={noopIntents()} />);
    expect(screen.queryByLabelText(label) !== null).toBe(visible);
  });

  test("the stewards button toggles the panel", () => {
    const intents = noopIntents();
    render(<AppView snapshot={signedInWithRooms} intents={intents} />);
    fireEvent.click(screen.getByRole("button", { name: "Stewards" }));
    expectOnlyCalled(intents, "toggleStewards");
  });

  test("joining a lounge reaches the client", () => {
    const intents = noopIntents();
    render(<AppView snapshot={withLounges} intents={intents} />);
    fireEvent.click(screen.getByRole("button", { name: "Join Campus lounge" }));
    expectOnlyCalled(intents, "joinLounge", [LOUNGE_A]);
  });

  test("assigned stewards offer a mention in the room composer", () => {
    render(<AppView snapshot={withMentionable} intents={noopIntents()} />);
    const group = screen.getAllByRole("group", { name: "Mention a steward" });
    expect(group.length).toBeGreaterThan(0);
    fireEvent.click(group[0]!.querySelector("button")!);
    expect((screen.getByLabelText("Message") as HTMLInputElement).value).toBe(
      `<@${STEWARD_A}> `,
    );
  });

  test("a streaming steward entry renders in the timeline", () => {
    const snapshot = withExtras(
      { ...signedInWithRooms, timeline: timelineOf([streamingEntryOf("streaming")]) },
      {},
    );
    render(<AppView snapshot={snapshot} intents={noopIntents()} />);
    expect(screen.getByText("Streaming")).not.toBeNull();
    expect(screen.getByText("Steward")).not.toBeNull();
  });

  test.each<[string, boolean]>([
    ["with a picker anchor the custom time is offered", true],
    ["without a picker anchor it is withheld", false],
  ])("shelf %s", (_name, offered) => {
    const snapshot = withExtras(signedInWithRooms, {
      shelf: shelfOf(
        { due: [shelfEntryOf("due", MESSAGE_A, { room_id: ROOM_A })] },
        offered ? {} : { picker_anchor_ms: null, picker_latest_ms: null },
      ),
    });
    render(<AppView snapshot={snapshot} intents={noopIntents()} />);
    fireEvent.click(screen.getAllByRole("button", { name: "Shelf" })[0]!);
    expect(screen.queryByLabelText("Remind at") !== null).toBe(offered);
  });
});

describe("campus dialogs", () => {
  const failing = (error: ApiError) =>
    ({ ...signedInNoCampus, session: { ...signedInNoCampus.session, error } }) as ClientSnapshot;

  test.each<[string, string, ApiError, string]>([
    ["Create campus", "Create campus", { code: "conflict", field: "slug" }, "That slug is already taken"],
    [
      "Join with invite code",
      "Join campus",
      { code: "invalid_input", field: "invite_code" },
      "That invite code is not valid or has expired",
    ],
  ])("%s shows its error inside the %s dialog", (opener, title, error, message) => {
    render(<AppView snapshot={failing(error)} intents={noopIntents()} />);
    fireEvent.click(screen.getByRole("button", { name: opener }));
    const dialog = screen.getByRole("dialog", { name: title });
    expect(dialog.querySelector('[role="alert"]')?.textContent).toBe(message);
    expect(screen.getAllByRole("alert")).toHaveLength(1);
  });

  test.each(["Create campus", "Join with invite code"])("%s closes once the campus arrives", (opener) => {
    const intents = noopIntents();
    const { rerender } = render(<AppView snapshot={signedInNoCampus} intents={intents} />);
    fireEvent.click(screen.getByRole("button", { name: opener }));
    expect(screen.queryByRole("dialog")).not.toBeNull();
    rerender(<AppView snapshot={signedInWithRooms} intents={intents} />);
    expect(screen.queryByRole("dialog")).toBeNull();
  });

  test.each<[string, boolean, boolean]>([
    ["an owner", true, true],
    ["a member", false, false],
  ])("people dialog for %s offers invites %s", (_who, canInvite, offersInvite) => {
    const intents = noopIntents();
    const snapshot = {
      ...signedInWithRooms,
      session: { ...signedInWithRooms.session, can_invite: canInvite },
    } as ClientSnapshot;
    render(<AppView snapshot={snapshot} intents={intents} />);
    fireEvent.click(screen.getByRole("button", { name: "People" }));
    expect(screen.getByRole("dialog", { name: "People" })).not.toBeNull();
    expect(intents.loadMembers).toHaveBeenCalledOnce();
    expect(screen.queryByRole("button", { name: "Create invite code" }) !== null).toBe(offersInvite);
  });
});

test("an active note shows its title instead of a room slug", () => {
  const noteRoom = "0193a5c0-0000-7000-8000-0000000000d9";
  const snapshot = {
    ...signedInWithRooms,
    session: {
      ...signedInWithRooms.session,
      notes: [{ room_id: noteRoom, title: "Grace Hopper", member_count: 2, group: false }],
      active_room_id: noteRoom,
    },
    timeline: { ...signedInWithRooms.timeline, room_id: noteRoom },
  } as ClientSnapshot;
  render(<AppView snapshot={snapshot} intents={noopIntents()} />);
  expect(screen.getAllByText("Grace Hopper").length).toBe(2);
  expect(screen.getByPlaceholderText("Message Grace Hopper")).not.toBeNull();
});
