import type {
  AccountProfile,
  ApiError,
  CampusId,
  CampusSummary,
  ClientSnapshot,
  ClientNonce,
  ConnectionSnapshot,
  CreateCampusRequest,
  DeliveryState,
  MessageId,
  MessageRejection,
  Room,
  RoomId,
  SessionSnapshot,
  SignInRequest,
  SignUpRequest,
  TimelineEntry,
  TimelineSnapshot,
} from "@pulse/protocol";
import { vi } from "vitest";
import type { PulseIntents } from "../client/usePulseClient";

export const CAMPUS_A: CampusId = "0193a5c0-0000-7000-8000-00000000000a";
export const CAMPUS_B: CampusId = "0193a5c0-0000-7000-8000-00000000000b";

export const ROOM_A: RoomId = "0193a5c0-0000-7000-8000-0000000000d1";
export const ROOM_B: RoomId = "0193a5c0-0000-7000-8000-0000000000d2";
export const MESSAGE_A: MessageId = "0193a5c0-0000-7000-8000-0000000000e1";
export const MESSAGE_B: MessageId = "0193a5c0-0000-7000-8000-0000000000e2";
export const NONCE_A: ClientNonce = "0193a5c0-0000-7000-8000-0000000000f1";
export const NONCE_B: ClientNonce = "0193a5c0-0000-7000-8000-0000000000f2";
export const NONCE_C: ClientNonce = "0193a5c0-0000-7000-8000-0000000000f3";

export const account: AccountProfile = {
  account_id: "0193a5c0-0000-7000-8000-0000000000c1",
  email: "ada@example.test",
  display_name: "Ada",
};

export const campusA: CampusSummary = {
  campus_id: CAMPUS_A,
  slug: "alpha",
  display_name: "Alpha Campus",
  role: "owner",
};

export const campusB: CampusSummary = {
  campus_id: CAMPUS_B,
  slug: "beta",
  display_name: "Beta Campus",
  role: "member",
};

const roomFor = (campus_id: CampusId, room_id: string, slug: string): Room => ({
  room_id,
  campus_id,
  slug,
});

export const rooms: Room[] = [
  roomFor(CAMPUS_A, ROOM_A, "commons"),
  roomFor(CAMPUS_A, ROOM_B, "courtyard"),
];

export const network: ConnectionSnapshot = {
  protocol_version: 2,
  connection: "disconnected",
  retry_in_ms: null,
  campus_id: null,
  session_id: null,
  attempt: 0,
  rejection: null,
  handshake_rejection: null,
};

const signedOutSession: SessionSnapshot = {
  auth: "signed_out",
  account: null,
  campuses: [],
  active_campus_id: null,
  rooms: [],
  active_room_id: null,
  busy: false,
  error: null,
};

export const emptyTimeline: TimelineSnapshot = {
  room_id: null,
  entries: [],
  syncing: false,
  error: null,
};

const entryOf = (
  delivery: DeliveryState,
  overrides: Partial<TimelineEntry>,
): TimelineEntry => ({
  client_nonce: NONCE_A,
  message_id: null,
  author_id: account.account_id,
  author_display_name: account.display_name,
  from_self: true,
  body: "hello",
  delivery,
  rejection: null,
  ...overrides,
});

export const pendingEntry = entryOf("pending", {});

export const confirmedEntry = entryOf("confirmed", {
  client_nonce: NONCE_B,
  message_id: MESSAGE_A,
  author_id: "0193a5c0-0000-7000-8000-0000000000c2",
  author_display_name: "Grace",
  from_self: false,
  body: "welcome",
});

export const confirmedOwnEntry = entryOf("confirmed", {
  client_nonce: NONCE_A,
  message_id: MESSAGE_B,
  body: "sent",
});

export const failedEntryFor = (rejection: MessageRejection) =>
  entryOf("failed", {
    client_nonce: NONCE_C,
    body: "denied",
    rejection,
  });

export const failedEntry = failedEntryFor("forbidden");

export const timelineOf = (
  entries: TimelineEntry[],
  overrides: Partial<TimelineSnapshot> = {},
): TimelineSnapshot => ({
  room_id: ROOM_A,
  entries,
  syncing: false,
  error: null,
  ...overrides,
});

const signedInSession: SessionSnapshot = {
  ...signedOutSession,
  auth: "signed_in",
  account,
};

export const snapshotOf = (
  session: SessionSnapshot,
  timeline: TimelineSnapshot = emptyTimeline,
  sequence = 1,
): ClientSnapshot => ({
  sequence,
  network,
  session,
  timeline,
});

export const invalidEmailError: ApiError = {
  code: "invalid_input",
  field: "email",
};

export const signedOut = snapshotOf(signedOutSession);

export const authenticating = snapshotOf({
  ...signedOutSession,
  auth: "authenticating",
  busy: true,
});

export const signedInNoCampus = snapshotOf(signedInSession);

const roomsSession: SessionSnapshot = {
  ...signedInSession,
  campuses: [campusA, campusB],
  active_campus_id: CAMPUS_A,
  rooms,
  active_room_id: ROOM_A,
};

export const signedInWithRooms = snapshotOf(roomsSession, timelineOf([]));

export const signedInWithMessages = snapshotOf(
  roomsSession,
  timelineOf([confirmedEntry, pendingEntry], { syncing: true }),
  2,
);

export const signedOutWithError = snapshotOf({
  ...signedOutSession,
  error: invalidEmailError,
});

export const networkFixtures: ReadonlyArray<
  [string, ConnectionSnapshot, string]
> = [
  ["disconnected", network, "Disconnected"],
  ["connecting", { ...network, connection: "connecting" }, "Connecting"],
  ["handshaking", { ...network, connection: "handshaking" }, "Handshaking"],
  ["connected", { ...network, connection: "connected" }, "Connected"],
  [
    "backoff",
    { ...network, connection: "backoff", retry_in_ms: 1500, attempt: 2 },
    "Retrying in 1500 ms",
  ],
  [
    "version rejected",
    { ...network, rejection: { supported: 2, received: 99 } },
    "Version rejected: supported 2, received 99",
  ],
  [
    "handshake rejected",
    { ...network, handshake_rejection: "campus_unavailable" },
    "Handshake rejected: campus unavailable",
  ],
];

export const signInRequest: SignInRequest = {
  email: "ada@example.test",
  password: "correct-horse-battery",
};

export const signUpRequest: SignUpRequest = {
  ...signInRequest,
  display_name: "Ada",
};

export const createCampusRequest: CreateCampusRequest = {
  slug: "gamma",
  display_name: "Gamma Campus",
};

export function noopIntents() {
  return {
    signUp: vi.fn(async () => {}),
    signIn: vi.fn(async () => {}),
    signOut: vi.fn(async () => {}),
    createCampus: vi.fn(async () => {}),
    selectCampus: vi.fn(async () => {}),
    refreshCampuses: vi.fn(async () => {}),
    selectRoom: vi.fn(async () => {}),
    sendMessage: vi.fn(async () => {}),
    discardMessage: vi.fn(async () => {}),
  } satisfies PulseIntents;
}
