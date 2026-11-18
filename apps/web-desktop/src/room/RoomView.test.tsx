import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import type {
  AttachmentKind,
  ReminderPreset,
  TimelineEntry,
  TimelineSnapshot,
  UploadState,
} from "@pulse/protocol";
import { afterEach, describe, expect, test, vi } from "vitest";
import {
  DRAFT_A,
  DRAFT_B,
  GRACE_ID,
  MESSAGE_A,
  MESSAGE_B,
  NONCE_C,
  ROOM_A,
  allPresets,
  attachmentViewOf,
  draftOf,
  expectOnlyCalled,
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

function renderRoom(
  timeline: TimelineSnapshot = timelineOf([]),
  presets: ReminderPreset[] = [],
) {
  const intents = noopIntents();
  const view = render(
    <RoomView timeline={timeline} intents={intents} presets={presets} />,
  );
  return { ...view, intents };
}

const confirmedWith = (overrides: Partial<TimelineEntry>): TimelineEntry => ({
  ...confirmedEntry,
  ...overrides,
});

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

describe("vault button", () => {
  test("opens the media tab of the active room", () => {
    const { intents } = renderRoom();
    fireEvent.click(screen.getByRole("button", { name: "Vault" }));
    expectOnlyCalled(intents, "openVault", [ROOM_A, "media"]);
  });
});

describe("attachments", () => {
  test.each<[string, AttachmentKind, string | null, string, string | null]>([
    ["an image with a ticket", "image", "https://files.example.test/i", "IMG", "https://files.example.test/i"],
    ["a video with a ticket", "video", "https://files.example.test/v", "A", "https://files.example.test/v"],
    ["audio with a ticket", "audio", "https://files.example.test/a", "A", "https://files.example.test/a"],
    ["a file with a ticket", "file", "https://files.example.test/f", "A", "https://files.example.test/f"],
    ["an image without a ticket", "image", null, "SPAN", null],
    ["a file without a ticket", "file", null, "SPAN", null],
  ])("%s", (_name, kind, url, tag, target) => {
    renderRoom(
      timelineOf([
        confirmedWith({ attachments: [attachmentViewOf(kind, { url, file_name: "named" })] }),
      ]),
    );
    const list = screen.getByRole("list", { name: "Attachments" });
    const image = list.querySelector("img");
    const anchor = list.querySelector("a");
    const plain = list.querySelector("li > span");
    expect([image !== null, anchor !== null, plain !== null]).toEqual([
      tag === "IMG",
      tag === "A",
      tag === "SPAN",
    ]);
    expect(image?.getAttribute("src") ?? target).toBe(target);
    expect(image?.getAttribute("alt") ?? "named").toBe("named");
    expect(anchor?.getAttribute("href") ?? target).toBe(target);
    expect(anchor?.getAttribute("rel") ?? "noreferrer").toBe("noreferrer");
    expect((anchor ?? plain)?.textContent ?? "named").toBe("named");
  });

  test("attachments render in message order", () => {
    renderRoom(
      timelineOf([
        confirmedWith({
          attachments: [
            attachmentViewOf("file", { attachment_id: "a1", file_name: "first", url: null }),
            attachmentViewOf("file", { attachment_id: "a2", file_name: "second", url: null }),
          ],
        }),
      ]),
    );
    const names = within(screen.getByRole("list", { name: "Attachments" }))
      .getAllByRole("listitem")
      .map((item) => item.textContent);
    expect(names).toEqual(["first", "second"]);
  });

  test("entries without attachments render no list", () => {
    renderRoom(timelineOf([confirmedEntry]));
    expect(screen.queryByRole("list", { name: "Attachments" })).toBeNull();
  });

  test("a sending message lists its uploading files", () => {
    renderRoom(
      timelineOf([
        {
          ...pendingEntry,
          pending_drafts: [
            draftOf("uploading", { file_name: "a.png" }),
            draftOf("uploaded", { draft_id: DRAFT_B, file_name: "b.pdf", kind: "file" }),
          ],
        },
      ]),
    );
    const list = screen.getByRole("list", { name: "Uploading attachments" });
    expect(within(list).getAllByRole("listitem").map((item) => item.textContent)).toEqual([
      "a.png",
      "b.pdf",
    ]);
  });
});

describe("links", () => {
  test("link segments render as anchors without a referrer", () => {
    renderRoom(
      timelineOf([
        confirmedWith({
          segments: [
            { kind: "text", text: "see ", account_id: null, url: null },
            { kind: "link", text: "https://example.org/a", account_id: null, url: "https://example.org/a" },
          ],
        }),
      ]),
    );
    const anchor = screen.getByRole("link", { name: "https://example.org/a" });
    expect(anchor.getAttribute("href")).toBe("https://example.org/a");
    expect(anchor.getAttribute("rel")).toBe("noreferrer");
    expect(anchor.getAttribute("target")).toBe("_blank");
  });

  test("a link segment without a url stays plain text", () => {
    renderRoom(
      timelineOf([
        confirmedWith({
          segments: [{ kind: "link", text: "broken", account_id: null, url: null }],
        }),
      ]),
    );
    expect(screen.queryByRole("link")).toBeNull();
    expect(screen.getByText("broken")).not.toBeNull();
  });
});

describe("drafts", () => {
  test.each<[UploadState, string, boolean]>([
    ["uploading", "Uploading", false],
    ["uploaded", "Ready", false],
    ["failed", "Failed", true],
  ])("a %s draft reads %s and retry is %s", (upload, label, retry) => {
    renderRoom(timelineOf([], { drafts: [draftOf(upload)] }));
    const chip = screen.getByRole("list", { name: "Drafts" }).querySelector("li")!;
    expect(chip.getAttribute("data-upload")).toBe(upload);
    expect(chip.textContent).toContain(`draft.png ${label}`);
    expect(screen.queryByRole("button", { name: "Retry" }) !== null).toBe(retry);
    expect(screen.getByRole("button", { name: "Remove" })).not.toBeNull();
  });

  test("retry and remove call their intents with the draft", () => {
    const { intents } = renderRoom(
      timelineOf([], {
        drafts: [draftOf("failed"), draftOf("uploaded", { draft_id: DRAFT_B })],
      }),
    );
    fireEvent.click(screen.getByRole("button", { name: "Retry" }));
    expectOnlyCalled(intents, "retryDraft", [DRAFT_A]);
    fireEvent.click(screen.getAllByRole("button", { name: "Remove" })[1]!);
    expect(intents.discardDraft).toHaveBeenCalledExactlyOnceWith(DRAFT_B);
  });

  test("no drafts render no chip list", () => {
    renderRoom(timelineOf([]));
    expect(screen.queryByRole("list", { name: "Drafts" })).toBeNull();
  });

  test.each([
    ["without drafts and text", [], "", true],
    ["with a draft and no text", [draftOf("uploaded")], "", false],
    ["with text and no drafts", [], "x", false],
  ])("send %s is disabled %s", (_name, drafts, text, disabled) => {
    renderRoom(timelineOf([], { drafts }));
    fireEvent.change(screen.getByLabelText("Message"), { target: { value: text } });
    expect((screen.getByRole("button", { name: "Send" }) as HTMLButtonElement).disabled).toBe(disabled);
  });

  test.each([
    [true, true],
    [false, false],
  ])("can_upload %s shows the file input %s", (canUpload, visible) => {
    renderRoom(timelineOf([], { can_upload: canUpload }));
    expect(screen.queryByLabelText("Attach file") !== null).toBe(visible);
  });

  test("choosing a file attaches it to the room", () => {
    const { intents } = renderRoom(timelineOf([], { can_upload: true }));
    const input = screen.getByLabelText("Attach file");
    const file = new File(["bytes"], "plan.png", { type: "image/png" });
    fireEvent.change(input, { target: { files: [file] } });
    expectOnlyCalled(intents, "attachFile", [ROOM_A, file]);
  });

  test("an empty file selection attaches nothing", () => {
    const { intents } = renderRoom(timelineOf([], { can_upload: true }));
    fireEvent.change(screen.getByLabelText("Attach file"), { target: { files: [] } });
    expectOnlyCalled(intents, null);
  });
});

describe("pinning", () => {
  test.each<[string, boolean, TimelineEntry, string | null]>([
    ["allowed on a confirmed entry", true, confirmedWith({ pinned: false }), "Pin"],
    ["allowed on a pinned entry", true, confirmedWith({ pinned: true }), "Unpin"],
    ["hidden without permission", false, confirmedWith({ pinned: false }), null],
    ["hidden for a pending entry", true, pendingEntry, null],
  ])("pin button %s", (_name, canPin, entry, label) => {
    renderRoom(timelineOf([entry], { can_pin: canPin }));
    const button = screen.queryByRole("button", { name: /^(Pin|Unpin)$/ });
    expect(button?.textContent ?? null).toBe(label);
  });

  test.each<[boolean, boolean]>([
    [false, true],
    [true, false],
  ])("a pinned=%s entry sends pinned=%s", (pinned, next) => {
    const { intents } = renderRoom(
      timelineOf([confirmedWith({ pinned })], { can_pin: true }),
    );
    fireEvent.click(screen.getByRole("button", { name: pinned ? "Unpin" : "Pin" }));
    expectOnlyCalled(intents, "setPin", [ROOM_A, MESSAGE_A, next]);
  });

  test.each([
    [true, true],
    [false, false],
  ])("pinned=%s shows the pinned marker %s", (pinned, visible) => {
    renderRoom(timelineOf([confirmedWith({ pinned })]));
    expect(screen.queryByText("Pinned") !== null).toBe(visible);
  });
});

describe("shelf menu on entries", () => {
  test.each<[string, boolean, string]>([
    ["unshelved offers saving", false, "Save to shelf"],
    ["shelved offers removal", true, "Remove from shelf"],
  ])("a %s entry", (_name, shelved, label) => {
    renderRoom(timelineOf([confirmedWith({ shelved })]), allPresets);
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    expect(screen.getByRole("button", { name: label })).not.toBeNull();
    expect(screen.queryByRole("button", { name: shelved ? "Save to shelf" : "Remove from shelf" })).toBeNull();
  });

  test.each<[string, string, unknown[]]>([
    ["save", "Save to shelf", ["save", null, null]],
    ["a preset", "Tomorrow", ["save", "tomorrow", null]],
  ])("choosing %s updates the shelf for that message", (_name, label, tail) => {
    const { intents } = renderRoom(timelineOf([confirmedEntry]), allPresets);
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    fireEvent.click(screen.getByRole("button", { name: label }));
    expectOnlyCalled(intents, "updateShelf", [ROOM_A, MESSAGE_A, ...tail]);
  });

  test("pending entries have no shelf menu", () => {
    renderRoom(timelineOf([pendingEntry]), allPresets);
    expect(screen.queryByRole("button", { name: "Shelf" })).toBeNull();
  });
});

describe("focus", () => {
  test("only the focused entry is marked and scrolled into view", () => {
    const scroll = vi.fn();
    Object.defineProperty(Element.prototype, "scrollIntoView", {
      configurable: true,
      value: scroll,
    });
    const { container } = renderRoom(
      timelineOf(
        [
          confirmedWith({ message_id: MESSAGE_A }),
          confirmedWith({ message_id: MESSAGE_B, client_nonce: NONCE_C }),
        ],
        { focus_message_id: MESSAGE_B },
      ),
    );
    const flags = [...container.querySelectorAll("li[data-focused]")].map((row) =>
      row.getAttribute("data-focused"),
    );
    expect(flags).toEqual(["false", "true"]);
    expect(scroll).toHaveBeenCalledExactlyOnceWith({ block: "center" });
    Reflect.deleteProperty(Element.prototype, "scrollIntoView");
  });

  test("without a focus nothing is marked", () => {
    const { container } = renderRoom(timelineOf([confirmedEntry, pendingEntry]));
    const flags = [...container.querySelectorAll("li[data-focused]")].map((row) =>
      row.getAttribute("data-focused"),
    );
    expect(flags).toEqual(["false", "false"]);
  });
});
