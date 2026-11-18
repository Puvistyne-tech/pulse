import { Channel } from "@tauri-apps/api/core";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import type { ClientSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  CAMPUS_B,
  DRAFT_A,
  MESSAGE_A,
  NONCE_B,
  ROOM_B,
  createCampusRequest,
  signInRequest,
  signUpRequest,
  signedInWithMessages,
  signedInWithRooms,
  snapshotOf,
  timelineOf,
} from "../test/fixtures";
import { pulseClient } from "./pulseClient";

interface Invocation {
  command: string;
  args: unknown;
}

type Call = () => Promise<ClientSnapshot>;

const recordInvocations = (reply: ClientSnapshot) => {
  const invocations: Invocation[] = [];
  mockIPC((command, args) => {
    invocations.push({ command, args });
    return reply;
  });
  return invocations;
};

interface RawInvocation {
  command: string;
  payload: unknown;
  options: { headers?: Record<string, string> } | undefined;
}

const recordRawInvocations = (reply: ClientSnapshot) => {
  const invocations: RawInvocation[] = [];
  Object.defineProperty(window, "__TAURI_INTERNALS__", {
    configurable: true,
    value: {
      invoke: async (
        command: string,
        payload: unknown,
        options: RawInvocation["options"],
      ) => {
        invocations.push({ command, payload, options });
        return reply;
      },
    },
  });
  return invocations;
};

describe("pulseClient", () => {
  afterEach(() => clearMocks());

  test.each<[string, Call, string, unknown]>([
    ["snapshot", () => pulseClient.snapshot(), "client_snapshot", undefined],
    [
      "signUp",
      () => pulseClient.signUp(signUpRequest),
      "sign_up",
      { request: signUpRequest },
    ],
    [
      "signIn",
      () => pulseClient.signIn(signInRequest),
      "sign_in",
      { request: signInRequest },
    ],
    ["signOut", () => pulseClient.signOut(), "sign_out", undefined],
    [
      "refreshCampuses",
      () => pulseClient.refreshCampuses(),
      "refresh_campuses",
      undefined,
    ],
    [
      "createCampus",
      () => pulseClient.createCampus(createCampusRequest),
      "create_campus",
      { request: createCampusRequest },
    ],
    [
      "selectCampus",
      () => pulseClient.selectCampus(CAMPUS_B),
      "select_campus",
      { campusId: CAMPUS_B },
    ],
    [
      "selectRoom",
      () => pulseClient.selectRoom(ROOM_B),
      "select_room",
      { roomId: ROOM_B },
    ],
    [
      "sendMessage",
      () => pulseClient.sendMessage(ROOM_B, "  hello  "),
      "send_message",
      { roomId: ROOM_B, body: "  hello  " },
    ],
    [
      "discardMessage",
      () => pulseClient.discardMessage(NONCE_B),
      "discard_message",
      { clientNonce: NONCE_B },
    ],
    [
      "sendReply",
      () => pulseClient.sendReply(ROOM_B, MESSAGE_A, "reply"),
      "send_reply",
      { roomId: ROOM_B, rootMessageId: MESSAGE_A, body: "reply" },
    ],
    [
      "react add",
      () => pulseClient.react(ROOM_B, MESSAGE_A, "👍", "add"),
      "react",
      { roomId: ROOM_B, messageId: MESSAGE_A, emoji: "👍", action: "add" },
    ],
    [
      "react remove",
      () => pulseClient.react(ROOM_B, MESSAGE_A, "🎉", "remove"),
      "react",
      { roomId: ROOM_B, messageId: MESSAGE_A, emoji: "🎉", action: "remove" },
    ],
    [
      "setTopicTitle",
      () => pulseClient.setTopicTitle(ROOM_B, MESSAGE_A, "Plans"),
      "set_topic_title",
      { roomId: ROOM_B, rootMessageId: MESSAGE_A, topicTitle: "Plans" },
    ],
    [
      "setThreadSubscription",
      () => pulseClient.setThreadSubscription(ROOM_B, MESSAGE_A, "muted"),
      "set_thread_subscription",
      { roomId: ROOM_B, rootMessageId: MESSAGE_A, state: "muted" },
    ],
    [
      "openThread",
      () => pulseClient.openThread(ROOM_B, MESSAGE_A),
      "open_thread",
      { roomId: ROOM_B, rootMessageId: MESSAGE_A },
    ],
    ["closeThread", () => pulseClient.closeThread(), "close_thread", undefined],
    ["toggleInbox", () => pulseClient.toggleInbox(), "toggle_inbox", undefined],
    [
      "moveInboxSelection next",
      () => pulseClient.moveInboxSelection("next"),
      "move_inbox_selection",
      { step: "next" },
    ],
    [
      "moveInboxSelection previous",
      () => pulseClient.moveInboxSelection("previous"),
      "move_inbox_selection",
      { step: "previous" },
    ],
    [
      "archiveInboxSelection",
      () => pulseClient.archiveInboxSelection(),
      "archive_inbox_selection",
      undefined,
    ],
    [
      "startInboxReply",
      () => pulseClient.startInboxReply(),
      "start_inbox_reply",
      undefined,
    ],
    [
      "cancelInboxReply",
      () => pulseClient.cancelInboxReply(),
      "cancel_inbox_reply",
      undefined,
    ],
    ["toggleSearch", () => pulseClient.toggleSearch(), "toggle_search", undefined],
    [
      "search",
      () => pulseClient.search("alpha beta"),
      "search",
      { query: "alpha beta" },
    ],
    [
      "moveSearchSelection next",
      () => pulseClient.moveSearchSelection("next"),
      "move_search_selection",
      { step: "next" },
    ],
    [
      "moveSearchSelection previous",
      () => pulseClient.moveSearchSelection("previous"),
      "move_search_selection",
      { step: "previous" },
    ],
    [
      "openSearchSelection",
      () => pulseClient.openSearchSelection(),
      "open_search_selection",
      undefined,
    ],
    [
      "openSearchHit",
      () => pulseClient.openSearchHit(MESSAGE_A),
      "open_search_hit",
      { messageId: MESSAGE_A },
    ],
    [
      "discardDraft",
      () => pulseClient.discardDraft(DRAFT_A),
      "discard_draft",
      { draftId: DRAFT_A },
    ],
    [
      "retryDraft",
      () => pulseClient.retryDraft(DRAFT_A),
      "retry_draft",
      { draftId: DRAFT_A },
    ],
    [
      "setPin pins",
      () => pulseClient.setPin(ROOM_B, MESSAGE_A, true),
      "set_pin",
      { roomId: ROOM_B, messageId: MESSAGE_A, pinned: true },
    ],
    [
      "setPin unpins",
      () => pulseClient.setPin(ROOM_B, MESSAGE_A, false),
      "set_pin",
      { roomId: ROOM_B, messageId: MESSAGE_A, pinned: false },
    ],
    ...(["media", "files", "links", "pinned"] as const).map(
      (tab): [string, Call, string, unknown] => [
        `openVault ${tab}`,
        () => pulseClient.openVault(ROOM_B, tab),
        "open_vault",
        { roomId: ROOM_B, tab },
      ],
    ),
    ["loadMoreVault", () => pulseClient.loadMoreVault(), "load_more_vault", undefined],
    ["closeVault", () => pulseClient.closeVault(), "close_vault", undefined],
    ["toggleShelf", () => pulseClient.toggleShelf(), "toggle_shelf", undefined],
    [
      "updateShelf with a preset",
      () =>
        pulseClient.updateShelf(ROOM_B, MESSAGE_A, "reschedule", "in_one_hour", null),
      "update_shelf",
      {
        roomId: ROOM_B,
        messageId: MESSAGE_A,
        action: "reschedule",
        preset: "in_one_hour",
        remindAtMs: null,
      },
    ],
    [
      "updateShelf with a custom time",
      () => pulseClient.updateShelf(ROOM_B, MESSAGE_A, "save", null, 1_234),
      "update_shelf",
      {
        roomId: ROOM_B,
        messageId: MESSAGE_A,
        action: "save",
        preset: null,
        remindAtMs: 1_234,
      },
    ],
    [
      "updateShelf without a reminder",
      () => pulseClient.updateShelf(ROOM_B, MESSAGE_A, "complete", null, null),
      "update_shelf",
      {
        roomId: ROOM_B,
        messageId: MESSAGE_A,
        action: "complete",
        preset: null,
        remindAtMs: null,
      },
    ],
    [
      "dismissReminder",
      () => pulseClient.dismissReminder(),
      "dismiss_reminder",
      undefined,
    ],
  ])("%s invokes the matching command", async (_name, call, command, args) => {
    const invocations = recordInvocations(signedInWithRooms);
    const result = await call();
    expect(invocations).toHaveLength(1);
    const [invocation] = invocations;
    expect(invocation?.command).toBe(command);
    if (args === undefined) {
      expect(Object.keys((invocation?.args as object) ?? {})).toEqual([]);
    } else {
      expect(invocation?.args).toEqual(args);
    }
    expect(result).toEqual(signedInWithRooms);
  });

  test.each([
    ["one push", 1],
    ["three pushes", 3],
  ])("watchSnapshots delivers %s in order", async (_name, pushes) => {
    const received: ClientSnapshot[] = [];
    const invocations: Invocation[] = [];
    mockIPC((command, args) => {
      invocations.push({ command, args });
    });
    await pulseClient.watchSnapshots((snapshot) => received.push(snapshot));
    expect(invocations).toHaveLength(1);
    const [invocation] = invocations;
    expect(invocation?.command).toBe("watch_snapshots");
    const channel = (invocation?.args as { onSnapshot: Channel<ClientSnapshot> })
      .onSnapshot;
    expect(channel).toBeInstanceOf(Channel);
    const pushed = Array.from({ length: pushes }, (_, index) =>
      snapshotOf(signedInWithMessages.session, timelineOf([]), index + 1),
    );
    pushed.forEach((snapshot) => channel.onmessage(snapshot));
    expect(received).toEqual(pushed);
  });

  test.each<[string, string, string]>([
    ["a plain name", "plan.png", "plan.png"],
    ["spaces and parentheses", "my plan (1).png", "my%20plan%20(1).png"],
    ["non-ascii letters", "résumé.pdf", "r%C3%A9sum%C3%A9.pdf"],
    ["a slash", "a/b.png", "a%2Fb.png"],
  ])("attachFile sends the raw bytes with %s encoded in the header", async (_name, fileName, encoded) => {
    const invocations = recordRawInvocations(signedInWithRooms);
    const bytes = new Uint8Array([1, 2, 3, 250]);
    const file = Object.assign(new File([bytes], fileName), {
      arrayBuffer: async () => bytes.buffer,
    });
    const result = await pulseClient.attachFile(ROOM_B, file);
    expect(invocations).toHaveLength(1);
    const [invocation] = invocations;
    expect(invocation?.command).toBe("attach_file");
    expect(invocation?.payload).toBeInstanceOf(Uint8Array);
    expect([...(invocation?.payload as Uint8Array)]).toEqual([...bytes]);
    expect(invocation?.options?.headers).toEqual({
      "pulse-room-id": ROOM_B,
      "pulse-file-name": encoded,
    });
    expect(decodeURIComponent(invocation?.options?.headers?.["pulse-file-name"] ?? "")).toBe(fileName);
    expect(result).toEqual(signedInWithRooms);
  });
});
