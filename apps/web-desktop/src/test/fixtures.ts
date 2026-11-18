import type {
  AccountId,
  AccountProfile,
  ApiError,
  AttachmentKind,
  AttachmentView,
  DraftAttachmentView,
  HighlightSpan,
  ReminderPreset,
  SearchHit,
  SearchSnapshot,
  ShelfBucket,
  ShelfEntry,
  ShelfSnapshot,
  UploadState,
  VaultEntry,
  VaultSnapshot,
  VaultTab,
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
import { expect, vi } from "vitest";
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
  protocol_version: 4,
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
  drafts: [],
  focus_message_id: null,
  can_pin: false,
  can_upload: false,
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
    attachments: [],
    pending_drafts: [],
    pinned: false,
    shelved: false,
    ...overrides,
  };
  return {
    ...base,
    author_mention_token:
      overrides.author_mention_token ?? mentionToken(base.author_id),
    segments:
      overrides.segments ?? [
        { kind: "text", text: base.body, account_id: null, url: null },
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
    { kind: "text", text: "hi ", account_id: null, url: null },
    {
      kind: "mention",
      text: "@Ada",
      account_id: account.account_id,
      url: null,
    },
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
  drafts: [],
  focus_message_id: null,
  can_pin: false,
  can_upload: false,
  ...overrides,
});

export const ATTACHMENT_A = "0193a5c0-0000-7000-8000-000000000a01";
export const ATTACHMENT_B = "0193a5c0-0000-7000-8000-000000000a02";
export const DRAFT_A = "0193a5c0-0000-7000-8000-000000000c01";
export const DRAFT_B = "0193a5c0-0000-7000-8000-000000000c02";
export const SHELF_ITEM_A = "0193a5c0-0000-7000-8000-000000000b01";

export const emptySearch: SearchSnapshot = {
  open: false,
  query: "",
  hits: [],
  selected_index: null,
  searching: false,
  error: null,
};

export const emptyVault: VaultSnapshot = {
  open: false,
  room_id: null,
  tab: "media",
  entries: [],
  loading: false,
  has_more: false,
  error: null,
};

export const emptyShelf: ShelfSnapshot = {
  open: false,
  due: [],
  upcoming: [],
  done: [],
  due_count: 0,
  latest_reminder: null,
  presets: [],
  error: null,
};

export const allPresets: ReminderPreset[] = [
  "in_twenty_minutes",
  "in_one_hour",
  "in_three_hours",
  "tomorrow",
  "next_week",
];

export const snippetOf = (...spans: [string, boolean][]): HighlightSpan[] =>
  spans.map(([text, matched]) => ({ text, matched }));

export const searchHitOf = (
  message_id: MessageId,
  overrides: Partial<SearchHit> = {},
): SearchHit => ({
  room_id: ROOM_A,
  message_id,
  thread_root_id: null,
  author_display_name: "Grace",
  topic_title: null,
  snippet: snippetOf(["say ", false], ["alpha", true]),
  score: 2,
  ...overrides,
});

export const searchOf = (
  hits: SearchHit[],
  overrides: Partial<SearchSnapshot> = {},
): SearchSnapshot => ({
  open: true,
  query: "alpha",
  hits,
  selected_index: hits.length > 0 ? 0 : null,
  searching: false,
  error: null,
  ...overrides,
});

export const attachmentViewOf = (
  kind: AttachmentKind,
  overrides: Partial<AttachmentView> = {},
): AttachmentView => ({
  attachment_id: ATTACHMENT_A,
  file_name: `sample.${kind}`,
  content_type: `${kind}/sample`,
  kind,
  byte_size: 2048,
  url: `https://files.example.test/${kind}`,
  ...overrides,
});

export const draftOf = (
  upload: UploadState,
  overrides: Partial<DraftAttachmentView> = {},
): DraftAttachmentView => ({
  draft_id: DRAFT_A,
  room_id: ROOM_A,
  file_name: "draft.png",
  kind: "image",
  byte_size: 10,
  upload,
  ...overrides,
});

export const vaultEntryOf = (
  tab: VaultTab,
  message_id: MessageId,
  overrides: Partial<VaultEntry> = {},
): VaultEntry => ({
  tab,
  room_id: ROOM_A,
  message_id,
  author_display_name: "Grace",
  preview: `preview ${message_id.slice(-2)}`,
  attachment: null,
  link: null,
  pinned_at_ms: null,
  ...overrides,
});

export const vaultOf = (
  entries: VaultEntry[],
  overrides: Partial<VaultSnapshot> = {},
): VaultSnapshot => ({
  open: true,
  room_id: ROOM_A,
  tab: entries[0]?.tab ?? "media",
  entries,
  loading: false,
  has_more: false,
  error: null,
  ...overrides,
});

export const shelfEntryOf = (
  bucket: ShelfBucket,
  message_id: MessageId,
  overrides: Partial<ShelfEntry> = {},
): ShelfEntry => ({
  shelf_item_id: SHELF_ITEM_A,
  room_id: ROOM_A,
  message_id,
  author_display_name: "Grace",
  preview: `shelved ${message_id.slice(-2)}`,
  remind_at_ms: bucket === "upcoming" ? 4_000_000 : null,
  done_at_ms: bucket === "done" ? 3_000_000 : null,
  bucket,
  pending: false,
  ...overrides,
});

export const shelfOf = (
  sections: Partial<Pick<ShelfSnapshot, "due" | "upcoming" | "done">>,
  overrides: Partial<ShelfSnapshot> = {},
): ShelfSnapshot => {
  const due = sections.due ?? [];
  return {
    open: true,
    due,
    upcoming: sections.upcoming ?? [],
    done: sections.done ?? [],
    due_count: due.length,
    latest_reminder: null,
    presets: allPresets,
    error: null,
    ...overrides,
  };
};

const signedInSession: SessionSnapshot = {
  ...signedOutSession,
  auth: "signed_in",
  account,
};

export interface SnapshotExtras {
  thread?: ThreadSnapshot;
  inbox?: InboxSnapshot;
  search?: SearchSnapshot;
  vault?: VaultSnapshot;
  shelf?: ShelfSnapshot;
}

export const snapshotOf = (
  session: SessionSnapshot,
  timeline: TimelineSnapshot = emptyTimeline,
  sequence = 1,
  extras: SnapshotExtras = {},
): ClientSnapshot => ({
  sequence,
  network,
  session,
  timeline,
  thread: extras.thread ?? emptyThread,
  inbox: extras.inbox ?? emptyInbox,
  search: extras.search ?? emptySearch,
  vault: extras.vault ?? emptyVault,
  shelf: extras.shelf ?? emptyShelf,
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
  extras: SnapshotExtras,
): ClientSnapshot => ({
  ...base,
  thread: extras.thread ?? base.thread,
  inbox: extras.inbox ?? base.inbox,
  search: extras.search ?? base.search,
  vault: extras.vault ?? base.vault,
  shelf: extras.shelf ?? base.shelf,
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
    { ...network, rejection: { supported: 4, received: 99 } },
    "Version rejected: supported 4, received 99",
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
    toggleSearch: vi.fn(async () => {}),
    search: vi.fn(async () => {}),
    moveSearchSelection: vi.fn(async () => {}),
    openSearchSelection: vi.fn(async () => {}),
    openSearchHit: vi.fn(async () => {}),
    attachFile: vi.fn(async () => {}),
    discardDraft: vi.fn(async () => {}),
    retryDraft: vi.fn(async () => {}),
    setPin: vi.fn(async () => {}),
    openVault: vi.fn(async () => {}),
    loadMoreVault: vi.fn(async () => {}),
    closeVault: vi.fn(async () => {}),
    toggleShelf: vi.fn(async () => {}),
    updateShelf: vi.fn(async () => {}),
    dismissReminder: vi.fn(async () => {}),
  } satisfies PulseIntents;
}

export type IntentName = keyof ReturnType<typeof noopIntents>;

export function expectOnlyCalled(
  intents: ReturnType<typeof noopIntents>,
  call: IntentName | null,
  args: unknown[] = [],
) {
  for (const name of Object.keys(intents) as IntentName[]) {
    if (name === call) {
      expect(intents[name]).toHaveBeenCalledExactlyOnceWith(...args);
    } else {
      expect(intents[name]).not.toHaveBeenCalled();
    }
  }
}
