import { Channel, invoke } from "@tauri-apps/api/core";
import type {
  CampusId,
  ClientNonce,
  ClientSnapshot,
  CreateCampusRequest,
  InboxStep,
  MessageId,
  ReactionAction,
  RoomId,
  SignInRequest,
  SignUpRequest,
  SubscriptionState,
} from "@pulse/protocol";

export const pulseClient = {
  snapshot: () => invoke<ClientSnapshot>("client_snapshot"),
  signUp: (request: SignUpRequest) =>
    invoke<ClientSnapshot>("sign_up", { request }),
  signIn: (request: SignInRequest) =>
    invoke<ClientSnapshot>("sign_in", { request }),
  signOut: () => invoke<ClientSnapshot>("sign_out"),
  refreshCampuses: () => invoke<ClientSnapshot>("refresh_campuses"),
  createCampus: (request: CreateCampusRequest) =>
    invoke<ClientSnapshot>("create_campus", { request }),
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
  watchSnapshots: (onSnapshot: (snapshot: ClientSnapshot) => void) => {
    const channel = new Channel<ClientSnapshot>();
    channel.onmessage = onSnapshot;
    return invoke<void>("watch_snapshots", { onSnapshot: channel });
  },
};
