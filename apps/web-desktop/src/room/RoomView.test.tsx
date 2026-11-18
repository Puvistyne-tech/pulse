import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { TimelineEntry } from "@pulse/protocol";
import { afterEach, describe, expect, test, vi } from "vitest";
import {
  NONCE_C,
  ROOM_A,
  confirmedEntry,
  confirmedOwnEntry,
  emptyTimeline,
  failedEntryFor,
  pendingEntry,
  timelineOf,
} from "../test/fixtures";
import { RoomView } from "./RoomView";

afterEach(cleanup);

function renderRoom(timeline = timelineOf([])) {
  const sendMessage = vi.fn(async () => {});
  const discardMessage = vi.fn(async () => {});
  const view = render(
    <RoomView
      timeline={timeline}
      sendMessage={sendMessage}
      discardMessage={discardMessage}
    />,
  );
  return { ...view, sendMessage, discardMessage };
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
];

describe("timeline entries", () => {
  test.each(entryRows)(
    "%s renders delivery, author and status",
    (_name, entry, delivery, author, status, discardable) => {
      const { container, discardMessage } = renderRoom(timelineOf([entry]));
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
        expect(discardMessage).toHaveBeenCalledExactlyOnceWith(NONCE_C);
      }
    },
  );

  test("entries render in snapshot order", () => {
    const { container } = renderRoom(
      timelineOf([confirmedEntry, confirmedOwnEntry, pendingEntry]),
    );
    const bodies = [...container.querySelectorAll("li")].map(
      (row) => row.getAttribute("data-delivery"),
    );
    expect(bodies).toEqual(["confirmed", "confirmed", "pending"]);
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
    const { sendMessage } = renderRoom();
    const input = screen.getByLabelText("Message") as HTMLInputElement;
    fireEvent.change(input, { target: { value: draft } });
    fireEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(sendMessage).toHaveBeenCalledExactlyOnceWith(ROOM_A, draft);
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
    const { sendMessage } = renderRoom();
    const input = screen.getByLabelText("Message");
    fireEvent.change(input, { target: { value: "x" } });
    fireEvent.click(screen.getByRole("button", { name: "Send" }));
    const button = screen.getByRole("button", { name: "Send" });
    expect((button as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(button);
    expect(sendMessage).toHaveBeenCalledTimes(1);
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
  ])("timeline %s", (_name, error, notice) => {
    renderRoom(timelineOf([], { error }));
    if (notice === null) {
      expect(screen.queryByRole("alert")).toBeNull();
    } else {
      expect(screen.getByText(notice)).not.toBeNull();
    }
  });
});
