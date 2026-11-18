import { Channel } from "@tauri-apps/api/core";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import type { ClientSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  CAMPUS_B,
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
