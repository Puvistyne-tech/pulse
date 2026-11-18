import type {
  AccountId,
  AccountProfile,
  ApiError,
  CampusId,
  CampusSummary,
  ClientSnapshot,
  ClientNonce,
  ConnectionSnapshot,
  CreateCampusRequest,
  DeliveryState,
  InboxItem,
  InboxSnapshot,
  MessageId,
  MessageRejection,
  ReactionSummary,
  Room,
  RoomId,
  SessionSnapshot,
  SignInRequest,
  SignUpRequest,
  SubscriptionState,
  ThreadPreview,
  ThreadSnapshot,
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
export const MESSAGE_C: MessageId = "0193a5c0-0000-7000-8000-0000000000e3";
export const MESSAGE_D: MessageId = "0193a5c0-0000-7000-8000-0000000000e4";
export const GRACE_ID: AccountId = "0193a5c0-0000-7000-8000-0000000000c2";
export const mentionToken = (accountId: AccountId) => `<@${accountId}>`;
export const reactionPalette = ["👍", "❤️", "🎉", "👀", "✅", "🙏"];
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
  protocol_version: 3,
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
  reaction_palette: reactionPalette,
};

const entryOf = (
  delivery: DeliveryState,
  overrides: Partial<TimelineEntry>,
): TimelineEntry => {
  const base: TimelineEntry = {
    client_nonce: NONCE_A,
    message_id: null,
    author_id: account.account_id,
    author_display_name: account.display_name,
    author_mention_token: "",
    from_self: true,
    body: "hello",
    segments: [],
    thread_root_id: null,
    reactions: [],
    thread: null,
    delivery,
    rejection: null,
    ...overrides,
  };
  return {
    ...base,
    author_mention_token:
      overrides.author_mention_token ?? mentionToken(base.author_id),
    segments:
      overrides.segments ?? [
        { kind: "text", text: base.body, account_id: null },
      ],
  };
};

export const pendingEntry = entryOf("pending", {});

export const confirmedEntry = entryOf("confirmed", {
  client_nonce: NONCE_B,
  message_id: MESSAGE_A,
  author_id: GRACE_ID,
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

export const reactionOf = (
  emoji: string,
  count: number,
  reacted_by_self: boolean,
): ReactionSummary => ({ emoji, count, reacted_by_self });

export const mentionEntry = entryOf("confirmed", {
  client_nonce: NONCE_C,
  message_id: MESSAGE_C,
  author_id: GRACE_ID,
  author_display_name: "Grace",
  from_self: false,
  body: `hi ${mentionToken(account.account_id)}`,
  segments: [
    { kind: "text", text: "hi ", account_id: null },
    { kind: "mention", text: "@Ada", account_id: account.account_id },
  ],
});

export const reactedEntry = entryOf("confirmed", {
  client_nonce: NONCE_B,
  message_id: MESSAGE_A,
  author_id: GRACE_ID,
  author_display_name: "Grace",
  from_self: false,
  body: "welcome",
  reactions: [reactionOf("👍", 2, true), reactionOf("🎉", 1, false)],
});

export const threadPreviewOf = (
  overrides: Partial<ThreadPreview> = {},
): ThreadPreview => ({
  topic_title: null,
  reply_count: 3,
  latest_reply: {
    message_id: MESSAGE_D,
    author_id: GRACE_ID,
    author_display_name: "Grace",
    preview: "latest words",
  },
  show_latest_reply: true,
  subscription: "following",
  unread_replies: 2,
  ...overrides,
});

export const threadedEntry = (overrides: Partial<ThreadPreview> = {}) =>
  entryOf("confirmed", {
    client_nonce: NONCE_B,
    message_id: MESSAGE_A,
    author_id: GRACE_ID,
    author_display_name: "Grace",
    from_self: false,
    body: "welcome",
    thread: threadPreviewOf(overrides),
  });

export const replyEntry = entryOf("confirmed", {
  client_nonce: NONCE_C,
  message_id: MESSAGE_D,
  author_id: GRACE_ID,
  author_display_name: "Grace",
  from_self: false,
  body: "a reply",
  thread_root_id: MESSAGE_A,
});

export const emptyThread: ThreadSnapshot = {
  room_id: null,
  root_message_id: null,
  root: null,
  replies: [],
  topic_title: null,
  subscription: null,
  syncing: false,
};

export const threadOf = (
  overrides: Partial<ThreadSnapshot> = {},
): ThreadSnapshot => ({
  room_id: ROOM_A,
  root_message_id: MESSAGE_A,
  root: confirmedEntry,
  replies: [replyEntry],
  topic_title: null,
  subscription: "following" as SubscriptionState,
  syncing: false,
  ...overrides,
});

export const emptyInbox: InboxSnapshot = {
  open: false,
  items: [],
  unread_threads: 0,
  selected_root_message_id: null,
  replying: false,
};

export const inboxItemOf = (
  root: MessageId,
  overrides: Partial<InboxItem> = {},
): InboxItem => ({
  kind: "thread",
  room_id: ROOM_A,
  root_message_id: root,
  topic_title: null,
  root_author_display_name: "Grace",
  root_preview: `root ${root.slice(-2)}`,
  latest_reply: null,
  reply_count: 0,
  unread_replies: 0,
  unread_mentions: 0,
  subscription: "following",
  activity_message_id: root,
  ...overrides,
});

export const inboxOf = (
  items: InboxItem[],
  overrides: Partial<InboxSnapshot> = {},
): InboxSnapshot => ({
  open: true,
  items,
  unread_threads: items.filter(
    (item) => item.unread_replies > 0 || item.unread_mentions > 0,
  ).length,
  selected_root_message_id: items[0]?.root_message_id ?? null,
  replying: false,
  ...overrides,
});

export const timelineOf = (
  entries: TimelineEntry[],
  overrides: Partial<TimelineSnapshot> = {},
): TimelineSnapshot => ({
  room_id: ROOM_A,
  entries,
  syncing: false,
  error: null,
  reaction_palette: reactionPalette,
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
  extras: { thread?: ThreadSnapshot; inbox?: InboxSnapshot } = {},
): ClientSnapshot => ({
  sequence,
  network,
  session,
  timeline,
  thread: extras.thread ?? emptyThread,
  inbox: extras.inbox ?? emptyInbox,
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

export const withExtras = (
  base: ClientSnapshot,
  extras: { thread?: ThreadSnapshot; inbox?: InboxSnapshot },
): ClientSnapshot => ({
  ...base,
  thread: extras.thread ?? base.thread,
  inbox: extras.inbox ?? base.inbox,
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
    { ...network, rejection: { supported: 3, received: 99 } },
    "Version rejected: supported 3, received 99",
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
    sendReply: vi.fn(async () => {}),
    react: vi.fn(async () => {}),
    setTopicTitle: vi.fn(async () => {}),
    setThreadSubscription: vi.fn(async () => {}),
    openThread: vi.fn(async () => {}),
    closeThread: vi.fn(async () => {}),
    toggleInbox: vi.fn(async () => {}),
    moveInboxSelection: vi.fn(async () => {}),
    archiveInboxSelection: vi.fn(async () => {}),
    startInboxReply: vi.fn(async () => {}),
    cancelInboxReply: vi.fn(async () => {}),
  } satisfies PulseIntents;
}
