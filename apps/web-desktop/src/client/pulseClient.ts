import { Channel, invoke } from "@tauri-apps/api/core";
import type {
  CampusId,
  ClientNonce,
  ClientSnapshot,
  CreateCampusRequest,
  JoinCampusRequest,
  StartNoteRequest,
  AddMemberRequest,
  UpdateMemberRequest,
  DraftId,
  AccountId,
  InboxStep,
  LoungeId,
  LoungeMediaState,
  MessageId,
  ReactionAction,
  ReminderPreset,
  RoomId,
  ShelfAction,
  SignInRequest,
  SignUpRequest,
  SubscriptionState,
  VaultTab,
} from "@pulse/protocol";

export const pulseClient = {
  snapshot: () => invoke<ClientSnapshot>("client_snapshot"),
  signUp: (request: SignUpRequest) =>
    invoke<ClientSnapshot>("sign_up", { request }),
  signIn: (request: SignInRequest) =>
    invoke<ClientSnapshot>("sign_in", { request }),
  signOut: () => invoke<ClientSnapshot>("sign_out"),
  quickSignIn: (email: string) =>
    invoke<ClientSnapshot>("quick_sign_in", { email }),
  refreshCampuses: () => invoke<ClientSnapshot>("refresh_campuses"),
  createCampus: (request: CreateCampusRequest) =>
    invoke<ClientSnapshot>("create_campus", { request }),
  joinCampus: (request: JoinCampusRequest) =>
    invoke<ClientSnapshot>("join_campus", { request }),
  createInvite: () => invoke<ClientSnapshot>("create_invite"),
  loadMembers: () => invoke<ClientSnapshot>("load_members"),
  startNote: (request: StartNoteRequest) =>
    invoke<ClientSnapshot>("start_note", { request }),
  addMember: (request: AddMemberRequest) =>
    invoke<ClientSnapshot>("add_member", { request }),
  updateMember: (accountId: AccountId, request: UpdateMemberRequest) =>
    invoke<ClientSnapshot>("update_member", { accountId, request }),
  removeMember: (accountId: AccountId) =>
    invoke<ClientSnapshot>("remove_member", { accountId }),
  selectCampus: (campusId: CampusId) =>
    invoke<ClientSnapshot>("select_campus", { campusId }),
  selectRoom: (roomId: RoomId) =>
    invoke<ClientSnapshot>("select_room", { roomId }),
  sendMessage: (roomId: RoomId, body: string) =>
    invoke<ClientSnapshot>("send_message", { roomId, body }),
  discardMessage: (clientNonce: ClientNonce) =>
    invoke<ClientSnapshot>("discard_message", { clientNonce }),
  sendReply: (roomId: RoomId, rootMessageId: MessageId, body: string) =>
    invoke<ClientSnapshot>("send_reply", { roomId, rootMessageId, body }),
  react: (
    roomId: RoomId,
    messageId: MessageId,
    emoji: string,
    action: ReactionAction,
  ) => invoke<ClientSnapshot>("react", { roomId, messageId, emoji, action }),
  setTopicTitle: (
    roomId: RoomId,
    rootMessageId: MessageId,
    topicTitle: string,
  ) =>
    invoke<ClientSnapshot>("set_topic_title", {
      roomId,
      rootMessageId,
      topicTitle,
    }),
  setThreadSubscription: (
    roomId: RoomId,
    rootMessageId: MessageId,
    state: SubscriptionState,
  ) =>
    invoke<ClientSnapshot>("set_thread_subscription", {
      roomId,
      rootMessageId,
      state,
    }),
  openThread: (roomId: RoomId, rootMessageId: MessageId) =>
    invoke<ClientSnapshot>("open_thread", { roomId, rootMessageId }),
  closeThread: () => invoke<ClientSnapshot>("close_thread"),
  toggleInbox: () => invoke<ClientSnapshot>("toggle_inbox"),
  moveInboxSelection: (step: InboxStep) =>
    invoke<ClientSnapshot>("move_inbox_selection", { step }),
  archiveInboxSelection: () =>
    invoke<ClientSnapshot>("archive_inbox_selection"),
  startInboxReply: () => invoke<ClientSnapshot>("start_inbox_reply"),
  cancelInboxReply: () => invoke<ClientSnapshot>("cancel_inbox_reply"),
  toggleSearch: () => invoke<ClientSnapshot>("toggle_search"),
  search: (query: string) => invoke<ClientSnapshot>("search", { query }),
  moveSearchSelection: (step: InboxStep) =>
    invoke<ClientSnapshot>("move_search_selection", { step }),
  openSearchSelection: () => invoke<ClientSnapshot>("open_search_selection"),
  openSearchHit: (messageId: MessageId) =>
    invoke<ClientSnapshot>("open_search_hit", { messageId }),
  attachFile: async (roomId: RoomId, file: File) => {
    const bytes = new Uint8Array(await file.arrayBuffer());
    return invoke<ClientSnapshot>("attach_file", bytes, {
      headers: {
        "pulse-room-id": roomId,
        "pulse-file-name": encodeURIComponent(file.name),
      },
    });
  },
  discardDraft: (draftId: DraftId) =>
    invoke<ClientSnapshot>("discard_draft", { draftId }),
  retryDraft: (draftId: DraftId) =>
    invoke<ClientSnapshot>("retry_draft", { draftId }),
  setPin: (roomId: RoomId, messageId: MessageId, pinned: boolean) =>
    invoke<ClientSnapshot>("set_pin", { roomId, messageId, pinned }),
  openVault: (roomId: RoomId, tab: VaultTab) =>
    invoke<ClientSnapshot>("open_vault", { roomId, tab }),
  loadMoreVault: () => invoke<ClientSnapshot>("load_more_vault"),
  closeVault: () => invoke<ClientSnapshot>("close_vault"),
  toggleShelf: () => invoke<ClientSnapshot>("toggle_shelf"),
  updateShelf: (
    roomId: RoomId,
    messageId: MessageId,
    action: ShelfAction,
    preset: ReminderPreset | null,
    remindAtMs: number | null,
  ) =>
    invoke<ClientSnapshot>("update_shelf", {
      roomId,
      messageId,
      action,
      preset,
      remindAtMs,
    }),
  dismissReminder: () => invoke<ClientSnapshot>("dismiss_reminder"),
  joinLounge: (loungeId: LoungeId) =>
    invoke<ClientSnapshot>("join_lounge", { loungeId }),
  leaveLounge: () => invoke<ClientSnapshot>("leave_lounge"),
  reportLoungeMedia: (state: LoungeMediaState) =>
    invoke<ClientSnapshot>("report_lounge_media", { state }),
  toggleStewards: () => invoke<ClientSnapshot>("toggle_stewards"),
  createSteward: (displayName: string, roomIds: RoomId[]) =>
    invoke<ClientSnapshot>("create_steward", { displayName, roomIds }),
  rotateStewardToken: (stewardId: AccountId) =>
    invoke<ClientSnapshot>("rotate_steward_token", { stewardId }),
  revokeSteward: (stewardId: AccountId) =>
    invoke<ClientSnapshot>("revoke_steward", { stewardId }),
  assignStewardRooms: (stewardId: AccountId, roomIds: RoomId[]) =>
    invoke<ClientSnapshot>("assign_steward_rooms", { stewardId, roomIds }),
  dismissStewardToken: () => invoke<ClientSnapshot>("dismiss_steward_token"),
  watchSnapshots: (onSnapshot: (snapshot: ClientSnapshot) => void) => {
    const channel = new Channel<ClientSnapshot>();
    channel.onmessage = onSnapshot;
    return invoke<void>("watch_snapshots", { onSnapshot: channel });
  },
};
