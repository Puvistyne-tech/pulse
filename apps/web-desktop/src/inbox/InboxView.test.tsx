import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { InboxItem, InboxSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  GRACE_ID,
  MESSAGE_A,
  MESSAGE_B,
  MESSAGE_C,
  ROOM_A,
  inboxItemOf,
  inboxOf,
  noopIntents,
} from "../test/fixtures";
import { InboxView } from "./InboxView";

afterEach(cleanup);

function renderInbox(inbox: InboxSnapshot) {
  const intents = noopIntents();
  const view = render(<InboxView inbox={inbox} intents={intents} />);
  return { ...view, intents };
}

const rowTexts = (container: HTMLElement) =>
  [...container.querySelectorAll("li")].map(
    (row) => row.querySelector("strong")?.textContent,
  );

describe("rows", () => {
  test("render in snapshot order", () => {
    const items = [
      inboxItemOf(MESSAGE_C, { topic_title: "third" }),
      inboxItemOf(MESSAGE_A, { topic_title: "first" }),
      inboxItemOf(MESSAGE_B, { topic_title: "second" }),
    ];
    const { container } = renderInbox(inboxOf(items));
    expect(rowTexts(container)).toEqual(["third", "first", "second"]);
  });

  test.each<[string, Partial<InboxItem>, string]>([
    ["topic title", { topic_title: "Plans" }, "Plans"],
    ["root preview when untitled", { topic_title: null }, "root e1"],
  ])("heading uses the %s", (_name, overrides, heading) => {
    const { container } = renderInbox(
      inboxOf([inboxItemOf(MESSAGE_A, overrides)]),
    );
    expect(rowTexts(container)).toEqual([heading]);
  });

  test("shows the root author and latest reply", () => {
    renderInbox(
      inboxOf([
        inboxItemOf(MESSAGE_A, {
          latest_reply: {
            message_id: MESSAGE_B,
            author_id: GRACE_ID,
            author_display_name: "Linus",
            preview: "see you there",
          },
        }),
      ]),
    );
    expect(screen.getByText(/Grace: root e1/)).not.toBeNull();
    expect(screen.getByText(/Linus:\s+see you there/)).not.toBeNull();
  });

  test.each<[string, number, number, boolean, boolean]>([
    ["nothing unread", 0, 0, false, false],
    ["replies only", 4, 0, true, false],
    ["mentions only", 0, 2, false, true],
    ["replies and mentions", 3, 1, true, true],
  ])(
    "%s shows badges accordingly",
    (_name, replies, mentions, repliesBadge, mentionMarker) => {
      renderInbox(
        inboxOf([
          inboxItemOf(MESSAGE_A, {
            unread_replies: replies,
            unread_mentions: mentions,
          }),
        ]),
      );
      const badge = screen.queryByLabelText("Unread replies");
      expect(badge !== null).toBe(repliesBadge);
      if (badge !== null) {
        expect(badge.textContent).toBe(String(replies));
      }
      const marker = screen.queryByLabelText("Unread mentions");
      expect(marker !== null).toBe(mentionMarker);
      if (marker !== null) {
        expect(marker.textContent).toBe("@");
      }
    },
  );

  test.each(["following", "muted", "archived"] as const)(
    "shows the %s state",
    (subscription) => {
      renderInbox(inboxOf([inboxItemOf(MESSAGE_A, { subscription })]));
      expect(screen.getByText(subscription)).not.toBeNull();
    },
  );
});

describe("header", () => {
  test.each([0, 1, 7])("shows %i unread threads from the snapshot", (count) => {
    renderInbox(inboxOf([inboxItemOf(MESSAGE_A)], { unread_threads: count }));
    expect(screen.getByLabelText("Unread threads").textContent).toBe(
      String(count),
    );
  });

  test("close button toggles the inbox", () => {
    const { intents } = renderInbox(inboxOf([]));
    fireEvent.click(screen.getByRole("button", { name: "Close inbox" }));
    expect(intents.toggleInbox).toHaveBeenCalledOnce();
  });

  test("an empty inbox renders no rows", () => {
    const { container } = renderInbox(inboxOf([]));
    expect(container.querySelectorAll("li")).toHaveLength(0);
  });
});

describe("selection", () => {
  const items = [inboxItemOf(MESSAGE_B), inboxItemOf(MESSAGE_A)];

  test.each<[string, string | null, boolean[]]>([
    ["first", MESSAGE_B, [true, false]],
    ["second", MESSAGE_A, [false, true]],
    ["none", null, [false, false]],
  ])("%s selected marks only that row", (_name, selected, expected) => {
    const { container } = renderInbox(
      inboxOf(items, { selected_root_message_id: selected }),
    );
    const flags = [...container.querySelectorAll("li")].map(
      (row) => row.getAttribute("aria-selected") === "true",
    );
    expect(flags).toEqual(expected);
  });

  test("only the selected row offers archive and reply", () => {
    renderInbox(inboxOf(items, { selected_root_message_id: MESSAGE_A }));
    expect(screen.getAllByRole("button", { name: "Archive" })).toHaveLength(1);
    expect(screen.getAllByRole("button", { name: "Reply" })).toHaveLength(1);
  });

  test("archive and reply buttons call their intents", () => {
    const { intents } = renderInbox(
      inboxOf(items, { selected_root_message_id: MESSAGE_A }),
    );
    fireEvent.click(screen.getByRole("button", { name: "Archive" }));
    fireEvent.click(screen.getByRole("button", { name: "Reply" }));
    expect(intents.archiveInboxSelection).toHaveBeenCalledOnce();
    expect(intents.startInboxReply).toHaveBeenCalledOnce();
  });
});

describe("inline reply", () => {
  const items = [inboxItemOf(MESSAGE_B), inboxItemOf(MESSAGE_A)];

  test.each<[string, boolean, string | null, number]>([
    ["not replying", false, MESSAGE_A, 0],
    ["replying with a selection", true, MESSAGE_A, 1],
    ["replying without a selection", true, null, 0],
  ])("%s shows %i composers", (_name, replying, selected, composers) => {
    renderInbox(
      inboxOf(items, { replying, selected_root_message_id: selected }),
    );
    expect(screen.queryAllByLabelText("Inbox reply")).toHaveLength(composers);
  });

  test("sends the reply to the selected thread", () => {
    const { intents } = renderInbox(
      inboxOf(items, {
        replying: true,
        selected_root_message_id: MESSAGE_A,
      }),
    );
    fireEvent.change(screen.getByLabelText("Inbox reply"), {
      target: { value: "on it" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(intents.sendReply).toHaveBeenCalledExactlyOnceWith(
      ROOM_A,
      MESSAGE_A,
      "on it",
    );
  });
});
