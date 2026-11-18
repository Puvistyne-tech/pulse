import { Channel } from "@tauri-apps/api/core";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import type { ClientSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  CAMPUS_B,
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
});
