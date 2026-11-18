import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { TimelineEntry, TimelineSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  GRACE_ID,
  MESSAGE_A,
  NONCE_C,
  ROOM_A,
  confirmedEntry,
  confirmedOwnEntry,
  emptyTimeline,
  failedEntryFor,
  mentionEntry,
  mentionToken,
  noopIntents,
  pendingEntry,
  reactedEntry,
  reactionPalette,
  threadedEntry,
  threadPreviewOf,
  timelineOf,
} from "../test/fixtures";
import { RoomView } from "./RoomView";

afterEach(cleanup);

function renderRoom(timeline: TimelineSnapshot = timelineOf([])) {
  const intents = noopIntents();
  const view = render(<RoomView timeline={timeline} intents={intents} />);
  return { ...view, intents };
}

const entryRows: ReadonlyArray<
  [string, TimelineEntry, string, string, string | null, boolean]
> = [
  ["pending", pendingEntry, "pending", "You", "Sending", false],
  ["confirmed from a peer", confirmedEntry, "confirmed", "Grace", null, false],
  ["confirmed from self", confirmedOwnEntry, "confirmed", "You", null, false],
  [
    "failed forbidden",
    failedEntryFor("forbidden"),
    "failed",
    "You",
    "Not allowed to post here",
    true,
  ],
  [
    "failed invalid body",
    failedEntryFor("invalid_body"),
    "failed",
    "You",
    "Message not accepted",
    true,
  ],
  [
    "failed unknown room",
    failedEntryFor("unknown_room"),
    "failed",
    "You",
    "Room not found",
    true,
  ],
  [
    "failed unknown thread",
    failedEntryFor("unknown_thread"),
    "failed",
    "You",
    "Thread not found",
    true,
  ],
];

describe("timeline entries", () => {
  test.each(entryRows)(
    "%s renders delivery, author and status",
    (_name, entry, delivery, author, status, discardable) => {
      const { container, intents } = renderRoom(timelineOf([entry]));
      const row = container.querySelector("li");
      expect(row?.getAttribute("data-delivery")).toBe(delivery);
      expect(screen.getAllByText(author)).toHaveLength(1);
      expect(screen.getAllByText(entry.body)).toHaveLength(1);
      expect(screen.queryByText("Sending") !== null).toBe(status === "Sending");
      if (status !== null && status !== "Sending") {
        expect(row?.textContent).toContain(status);
      }
      const discard = screen.queryByRole("button", { name: "Discard" });
      expect(discard !== null).toBe(discardable);
      if (discard !== null) {
        fireEvent.click(discard);
        expect(intents.discardMessage).toHaveBeenCalledExactlyOnceWith(
          NONCE_C,
        );
      }
    },
  );

  test("entries render in snapshot order", () => {
    const { container } = renderRoom(
      timelineOf([confirmedEntry, confirmedOwnEntry, pendingEntry]),
    );
    const bodies = [...container.querySelectorAll("li")].map((row) =>
      row.getAttribute("data-delivery"),
    );
    expect(bodies).toEqual(["confirmed", "confirmed", "pending"]);
  });
});

describe("body segments", () => {
  test.each<[string, TimelineEntry, string[], string[]]>([
    ["plain text", confirmedEntry, ["welcome"], []],
    ["a resolved mention", mentionEntry, ["hi", "@Ada"], ["@Ada"]],
  ])("%s renders each segment", (_name, entry, texts, strong) => {
    const { container } = renderRoom(timelineOf([entry]));
    for (const text of texts) {
      expect(screen.getAllByText(text).length).toBeGreaterThan(0);
    }
    const emphasised = [...container.querySelectorAll("strong")].map(
      (node) => node.textContent,
    );
    expect(emphasised).toEqual(strong);
  });

  test("the raw mention token is never shown", () => {
    const { container } = renderRoom(timelineOf([mentionEntry]));
    expect(container.textContent).not.toContain("<@");
  });
});

describe("reactions", () => {
  test("chips show emoji and counts with pressed state", () => {
    renderRoom(timelineOf([reactedEntry]));
    const mine = screen.getByRole("button", { name: "👍 2" });
    const theirs = screen.getByRole("button", { name: "🎉 1" });
    expect(mine.getAttribute("aria-pressed")).toBe("true");
    expect(theirs.getAttribute("aria-pressed")).toBe("false");
  });

  test.each<[string, string, "add" | "remove"]>([
    ["chip reacted by self removes", "👍 2", "remove"],
    ["chip reacted by others adds", "🎉 1", "add"],
    ["palette emoji reacted by self removes", "React 👍", "remove"],
    ["palette emoji reacted by others adds", "React 🎉", "add"],
    ["palette emoji not yet used adds", "React 🙏", "add"],
  ])("%s", (_name, label, action) => {
    const { intents } = renderRoom(timelineOf([reactedEntry]));
    fireEvent.click(screen.getByRole("button", { name: label }));
    const emoji = label.replace("React ", "").split(" ")[0];
    expect(intents.react).toHaveBeenCalledExactlyOnceWith(
      ROOM_A,
      MESSAGE_A,
      emoji,
      action,
    );
  });

  test.each<[string, string[]]>([
    ["the protocol palette", reactionPalette],
    ["a custom palette", ["🙏"]],
    ["an empty palette", []],
  ])("palette renders %s from the snapshot", (_name, palette) => {
    renderRoom(timelineOf([confirmedEntry], { reaction_palette: palette }));
    const group = screen.getByRole("group", { name: "Add reaction" });
    const buttons = [...group.querySelectorAll("button")].map(
      (button) => button.textContent,
    );
    expect(buttons).toEqual(palette);
  });

  test("pending entries offer no reactions", () => {
    renderRoom(timelineOf([pendingEntry]));
    expect(screen.queryByRole("group", { name: "Add reaction" })).toBeNull();
  });
});

describe("thread preview", () => {
  test.each<[string, Parameters<typeof threadPreviewOf>[0], boolean]>([
    ["shown when buried", { show_latest_reply: true }, true],
    ["hidden when not buried", { show_latest_reply: false }, false],
    [
      "hidden without a latest reply",
      { show_latest_reply: true, latest_reply: null },
      false,
    ],
  ])("latest reply bar is %s", (_name, preview, visible) => {
    renderRoom(timelineOf([threadedEntry(preview)]));
    expect(screen.queryByLabelText("Latest reply") !== null).toBe(visible);
    expect(screen.queryByText("latest words") !== null).toBe(
      visible && preview?.latest_reply !== null,
    );
  });

  test("collapsing hides the reply and expanding restores it", () => {
    renderRoom(timelineOf([threadedEntry()]));
    const toggle = screen.getByRole("button", { name: "Latest reply" });
    expect(toggle.getAttribute("aria-expanded")).toBe("true");
    fireEvent.click(toggle);
    expect(screen.queryByText("latest words")).toBeNull();
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
    fireEvent.click(toggle);
    expect(screen.getByText("latest words")).not.toBeNull();
  });

  test.each([
    ["Open thread"],
    ["Reply in thread"],
  ])("%s opens the thread of the entry", (label) => {
    const { intents } = renderRoom(timelineOf([threadedEntry()]));
    fireEvent.click(screen.getByRole("button", { name: label }));
    expect(intents.openThread).toHaveBeenCalledExactlyOnceWith(
      ROOM_A,
      MESSAGE_A,
    );
  });

  test.each<[string, Parameters<typeof threadPreviewOf>[0], string[]]>([
    ["counts and unread", {}, ["Replies 3", "following", "2"]],
    [
      "topic title",
      { topic_title: "Plans", unread_replies: 0 },
      ["Plans", "Replies 3"],
    ],
  ])("summary shows %s", (_name, preview, texts) => {
    renderRoom(timelineOf([threadedEntry(preview)]));
    for (const text of texts) {
      expect(screen.getAllByText(text).length).toBeGreaterThan(0);
    }
  });

  test("entries without a thread show no thread summary", () => {
    renderRoom(timelineOf([confirmedEntry]));
    expect(screen.queryByText(/^Replies/)).toBeNull();
  });
});

describe("mentions", () => {
  test("mention appends the author token to the draft", () => {
    renderRoom(timelineOf([confirmedEntry, mentionEntry]));
    const buttons = screen.getAllByRole("button", { name: "Mention" });
    fireEvent.click(buttons[0]!);
    const input = screen.getByLabelText("Message") as HTMLInputElement;
    expect(input.value).toBe(`${mentionToken(GRACE_ID)} `);
    fireEvent.change(input, { target: { value: "hey" } });
    fireEvent.click(buttons[1]!);
    expect(input.value).toBe(`hey ${mentionToken(GRACE_ID)} `);
  });

  test("own entries have no mention button", () => {
    renderRoom(timelineOf([confirmedOwnEntry, pendingEntry]));
    expect(screen.queryByRole("button", { name: "Mention" })).toBeNull();
  });
});

describe("syncing marker", () => {
  test.each([
    [true, true],
    [false, false],
  ])("syncing %s shows marker %s", (syncing, visible) => {
    renderRoom(timelineOf([], { syncing }));
    expect(screen.queryByText("Syncing") !== null).toBe(visible);
  });
});

describe("composer", () => {
  test.each([
    ["plain", "hello"],
    ["padded", "  padded  "],
    ["whitespace only", "   "],
  ])("sends %s drafts exactly as typed and clears", (_name, draft) => {
    const { intents } = renderRoom();
    const input = screen.getByLabelText("Message") as HTMLInputElement;
    fireEvent.change(input, { target: { value: draft } });
    fireEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(intents.sendMessage).toHaveBeenCalledExactlyOnceWith(ROOM_A, draft);
    expect(input.value).toBe("");
  });

  test.each([
    ["empty draft", "", true],
    ["typed draft", "x", false],
    ["whitespace draft", " ", false],
  ])("%s disables send %s", (_name, draft, disabled) => {
    renderRoom();
    fireEvent.change(screen.getByLabelText("Message"), {
      target: { value: draft },
    });
    const button = screen.getByRole("button", { name: "Send" });
    expect((button as HTMLButtonElement).disabled).toBe(disabled);
  });

  test("send is disabled again after the draft is cleared", () => {
    const { intents } = renderRoom();
    const input = screen.getByLabelText("Message");
    fireEvent.change(input, { target: { value: "x" } });
    fireEvent.click(screen.getByRole("button", { name: "Send" }));
    const button = screen.getByRole("button", { name: "Send" });
    expect((button as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(button);
    expect(intents.sendMessage).toHaveBeenCalledTimes(1);
  });
});

describe("room view shell", () => {
  test("renders nothing without an active room", () => {
    const { container } = renderRoom(emptyTimeline);
    expect(container.innerHTML).toBe("");
  });

  test.each([
    ["no error", null, null],
    [
      "message body error",
      { code: "invalid_input" as const, field: "message_body" as const },
      "Message is empty or too long",
    ],
    [
      "reaction emoji error",
      { code: "invalid_input" as const, field: "reaction_emoji" as const },
      "Choose a single emoji reaction",
    ],
  ])("timeline %s", (_name, error, notice) => {
    renderRoom(timelineOf([], { error }));
    if (notice === null) {
      expect(screen.queryByRole("alert")).toBeNull();
    } else {
      expect(screen.getByText(notice)).not.toBeNull();
    }
  });
});
