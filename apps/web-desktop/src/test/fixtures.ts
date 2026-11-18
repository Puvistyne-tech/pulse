import type {
  AccountProfile,
  ApiError,
  CampusId,
  CampusSummary,
  ClientSnapshot,
  ConnectionSnapshot,
  Room,
  SessionSnapshot,
} from "@pulse/protocol";
import { vi } from "vitest";
import type { PulseIntents } from "../client/usePulseClient";

export const CAMPUS_A: CampusId = "0193a5c0-0000-7000-8000-00000000000a";
export const CAMPUS_B: CampusId = "0193a5c0-0000-7000-8000-00000000000b";

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
  roomFor(CAMPUS_A, "0193a5c0-0000-7000-8000-0000000000d1", "commons"),
  roomFor(CAMPUS_A, "0193a5c0-0000-7000-8000-0000000000d2", "courtyard"),
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
  busy: false,
  error: null,
};

const signedInSession: SessionSnapshot = {
  ...signedOutSession,
  auth: "signed_in",
  account,
};

const snapshotOf = (session: SessionSnapshot): ClientSnapshot => ({
  network,
  session,
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

export const signedInWithRooms = snapshotOf({
  ...signedInSession,
  campuses: [campusA, campusB],
  active_campus_id: CAMPUS_A,
  rooms,
});

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

export function noopIntents() {
  return {
    signUp: vi.fn(async () => {}),
    signIn: vi.fn(async () => {}),
    signOut: vi.fn(async () => {}),
    createCampus: vi.fn(async () => {}),
    selectCampus: vi.fn(async () => {}),
    refreshCampuses: vi.fn(async () => {}),
  } satisfies PulseIntents;
}
