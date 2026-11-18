import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { SubscriptionState, ThreadSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  GRACE_ID,
  MESSAGE_A,
  ROOM_A,
  confirmedEntry,
  emptyThread,
  mentionToken,
  noopIntents,
  pendingEntry,
  reactedEntry,
  reactionPalette,
  replyEntry,
  threadOf,
} from "../test/fixtures";
import { ThreadPanel } from "./ThreadPanel";

afterEach(cleanup);

function renderThread(thread: ThreadSnapshot, palette = reactionPalette) {
  const intents = noopIntents();
  const view = render(
    <ThreadPanel thread={thread} palette={palette} intents={intents} />,
  );
  return { ...view, intents };
}

describe("visibility", () => {
  test.each<[string, ThreadSnapshot]>([
    ["an empty snapshot", emptyThread],
    ["a missing root id", threadOf({ root_message_id: null })],
    ["a missing room", threadOf({ room_id: null })],
  ])("renders nothing for %s", (_name, thread) => {
    const { container } = renderThread(thread);
    expect(container.innerHTML).toBe("");
  });
});

describe("entries", () => {
  test("lists the root then replies in snapshot order", () => {
    const { container } = renderThread(
      threadOf({ replies: [replyEntry, pendingEntry] }),
    );
    const delivery = [...container.querySelectorAll("li")].map((row) =>
      row.getAttribute("data-delivery"),
    );
    expect(delivery).toEqual(["confirmed", "confirmed", "pending"]);
    const bodies = [...container.querySelectorAll("li")].map((row) =>
      row.textContent?.includes("a reply"),
    );
    expect(bodies).toEqual([false, true, false]);
  });

  test("renders replies alone while the root is not loaded", () => {
    const { container } = renderThread(threadOf({ root: null }));
    expect(container.querySelectorAll("li")).toHaveLength(1);
    expect(screen.getByText("a reply")).not.toBeNull();
  });

  test("shows reactions with the supplied palette", () => {
    renderThread(threadOf({ root: reactedEntry, replies: [] }), ["🙏"]);
    expect(screen.getByRole("button", { name: "👍 2" })).not.toBeNull();
    const group = screen.getByRole("group", { name: "Add reaction" });
    expect(group.querySelectorAll("button")).toHaveLength(1);
  });

  test.each([
    [true, true],
    [false, false],
  ])("syncing %s shows marker %s", (syncing, visible) => {
    renderThread(threadOf({ syncing }));
    expect(screen.queryByText("Syncing") !== null).toBe(visible);
  });
});

describe("topic title", () => {
  test.each<[string, string | null, string]>([
    ["a stored title", "Plans", "Plans"],
    ["no title", null, ""],
  ])("input starts with %s", (_name, topic_title, value) => {
    renderThread(threadOf({ topic_title }));
    expect((screen.getByLabelText("Topic title") as HTMLInputElement).value).toBe(
      value,
    );
  });

  test.each([
    ["new title", "Agenda"],
    ["padded title", "  Agenda  "],
    ["cleared title", ""],
  ])("saving a %s sends the draft exactly as typed", (_name, draft) => {
    const { intents } = renderThread(threadOf({ topic_title: "Plans" }));
    fireEvent.change(screen.getByLabelText("Topic title"), {
      target: { value: draft },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save title" }));
    expect(intents.setTopicTitle).toHaveBeenCalledExactlyOnceWith(
      ROOM_A,
      MESSAGE_A,
      draft,
    );
  });

  test("a snapshot title replaces the draft", () => {
    const { rerender, intents } = renderThread(threadOf({ topic_title: "Plans" }));
    fireEvent.change(screen.getByLabelText("Topic title"), {
      target: { value: "typing" },
    });
    rerender(
      <ThreadPanel
        thread={threadOf({ topic_title: "Agenda" })}
        palette={reactionPalette}
        intents={intents}
      />,
    );
    expect((screen.getByLabelText("Topic title") as HTMLInputElement).value).toBe(
      "Agenda",
    );
  });
});

describe("subscription", () => {
  const states: SubscriptionState[] = ["following", "muted", "archived"];
  const labels = ["Following", "Muted", "Archived"];

  test.each(states.map((state, index) => [state, labels[index]!] as const))(
    "%s is the only pressed state when current",
    (current, label) => {
      renderThread(threadOf({ subscription: current }));
      const pressed = screen
        .getAllByRole("button", { pressed: true })
        .map((button) => button.textContent);
      expect(pressed).toEqual([label]);
    },
  );

  test("no state is pressed without a subscription", () => {
    renderThread(threadOf({ subscription: null }));
    expect(screen.queryAllByRole("button", { pressed: true })).toHaveLength(0);
  });

  test.each(states.map((state, index) => [state, labels[index]!] as const))(
    "clicking %s requests that state",
    (state, label) => {
      const { intents } = renderThread(threadOf({ subscription: null }));
      fireEvent.click(screen.getByRole("button", { name: label }));
      expect(intents.setThreadSubscription).toHaveBeenCalledExactlyOnceWith(
        ROOM_A,
        MESSAGE_A,
        state,
      );
    },
  );
});

describe("actions", () => {
  test("close button closes the thread", () => {
    const { intents } = renderThread(threadOf());
    fireEvent.click(screen.getByRole("button", { name: "Close thread" }));
    expect(intents.closeThread).toHaveBeenCalledOnce();
  });

  test.each([
    ["plain", "thanks"],
    ["padded", "  thanks  "],
  ])("reply composer sends a %s draft to the root", (_name, draft) => {
    const { intents } = renderThread(threadOf());
    const input = screen.getByLabelText("Reply") as HTMLInputElement;
    fireEvent.change(input, { target: { value: draft } });
    fireEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(intents.sendReply).toHaveBeenCalledExactlyOnceWith(
      ROOM_A,
      MESSAGE_A,
      draft,
    );
    expect(input.value).toBe("");
  });

  test("mention inserts the author token into the reply draft", () => {
    renderThread(threadOf({ root: confirmedEntry }));
    fireEvent.click(screen.getAllByRole("button", { name: "Mention" })[0]!);
    expect((screen.getByLabelText("Reply") as HTMLInputElement).value).toBe(
      `${mentionToken(GRACE_ID)} `,
    );
  });

  test("replies cannot open nested threads", () => {
    renderThread(threadOf({ root: null }));
    expect(screen.queryByRole("button", { name: "Reply in thread" })).toBeNull();
  });
});
