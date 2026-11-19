import { useCallback, useEffect, useMemo, useState } from "react";
import type {
  CampusId,
  ClientNonce,
  ClientSnapshot,
  CreateCampusRequest,
  JoinCampusRequest,
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
import { pulseClient } from "./pulseClient";
import { newerSnapshot } from "./snapshotOrder";

export interface PulseIntents {
  signUp(request: SignUpRequest): Promise<void>;
  signIn(request: SignInRequest): Promise<void>;
  signOut(): Promise<void>;
  quickSignIn(email: string): Promise<void>;
  createCampus(request: CreateCampusRequest): Promise<void>;
  joinCampus(request: JoinCampusRequest): Promise<void>;
  createInvite(): Promise<void>;
  loadMembers(): Promise<void>;
  addMember(request: AddMemberRequest): Promise<void>;
  updateMember(accountId: AccountId, request: UpdateMemberRequest): Promise<void>;
  removeMember(accountId: AccountId): Promise<void>;
  selectCampus(campusId: CampusId): Promise<void>;
  refreshCampuses(): Promise<void>;
  selectRoom(roomId: RoomId): Promise<void>;
  sendMessage(roomId: RoomId, body: string): Promise<void>;
  discardMessage(clientNonce: ClientNonce): Promise<void>;
  sendReply(
    roomId: RoomId,
    rootMessageId: MessageId,
    body: string,
  ): Promise<void>;
  react(
    roomId: RoomId,
    messageId: MessageId,
    emoji: string,
    action: ReactionAction,
  ): Promise<void>;
  setTopicTitle(
    roomId: RoomId,
    rootMessageId: MessageId,
    topicTitle: string,
  ): Promise<void>;
  setThreadSubscription(
    roomId: RoomId,
    rootMessageId: MessageId,
    state: SubscriptionState,
  ): Promise<void>;
  openThread(roomId: RoomId, rootMessageId: MessageId): Promise<void>;
  closeThread(): Promise<void>;
  toggleInbox(): Promise<void>;
  moveInboxSelection(step: InboxStep): Promise<void>;
  archiveInboxSelection(): Promise<void>;
  startInboxReply(): Promise<void>;
  cancelInboxReply(): Promise<void>;
  toggleSearch(): Promise<void>;
  search(query: string): Promise<void>;
  moveSearchSelection(step: InboxStep): Promise<void>;
  openSearchSelection(): Promise<void>;
  openSearchHit(messageId: MessageId): Promise<void>;
  attachFile(roomId: RoomId, file: File): Promise<void>;
  discardDraft(draftId: DraftId): Promise<void>;
  retryDraft(draftId: DraftId): Promise<void>;
  setPin(roomId: RoomId, messageId: MessageId, pinned: boolean): Promise<void>;
  openVault(roomId: RoomId, tab: VaultTab): Promise<void>;
  loadMoreVault(): Promise<void>;
  closeVault(): Promise<void>;
  toggleShelf(): Promise<void>;
  updateShelf(
    roomId: RoomId,
    messageId: MessageId,
    action: ShelfAction,
    preset: ReminderPreset | null,
    remindAtMs: number | null,
  ): Promise<void>;
  dismissReminder(): Promise<void>;
  joinLounge(loungeId: LoungeId): Promise<void>;
  leaveLounge(): Promise<void>;
  reportLoungeMedia(state: LoungeMediaState): Promise<void>;
  toggleStewards(): Promise<void>;
  createSteward(displayName: string, roomIds: RoomId[]): Promise<void>;
  rotateStewardToken(stewardId: AccountId): Promise<void>;
  revokeSteward(stewardId: AccountId): Promise<void>;
  assignStewardRooms(stewardId: AccountId, roomIds: RoomId[]): Promise<void>;
  dismissStewardToken(): Promise<void>;
}

export function usePulseClient(): {
  snapshot: ClientSnapshot | null;
  failed: boolean;
  intents: PulseIntents;
} {
  const [snapshot, setSnapshot] = useState<ClientSnapshot | null>(null);
  const [failed, setFailed] = useState(false);

  const apply = useCallback((next: ClientSnapshot) => {
    setSnapshot((current) => newerSnapshot(current, next));
  }, []);

  const publish = useCallback(
    async (pending: Promise<ClientSnapshot>) => {
      try {
        apply(await pending);
      } catch {
        setFailed(true);
      }
    },
    [apply],
  );

  useEffect(() => {
    let active = true;
    void publish(pulseClient.snapshot());
    pulseClient
      .watchSnapshots((next) => {
        if (active) {
          apply(next);
        }
      })
      .catch(() => setFailed(true));
    return () => {
      active = false;
    };
  }, [publish, apply]);

  const intents = useMemo<PulseIntents>(
    () => ({
      signUp: (request) => publish(pulseClient.signUp(request)),
      signIn: (request) => publish(pulseClient.signIn(request)),
      signOut: () => publish(pulseClient.signOut()),
      quickSignIn: (email) => publish(pulseClient.quickSignIn(email)),
      createCampus: (request) => publish(pulseClient.createCampus(request)),
      joinCampus: (request) => publish(pulseClient.joinCampus(request)),
      createInvite: () => publish(pulseClient.createInvite()),
      loadMembers: () => publish(pulseClient.loadMembers()),
      addMember: (request) => publish(pulseClient.addMember(request)),
      updateMember: (accountId, request) =>
        publish(pulseClient.updateMember(accountId, request)),
      removeMember: (accountId) => publish(pulseClient.removeMember(accountId)),
      selectCampus: (campusId) => publish(pulseClient.selectCampus(campusId)),
      refreshCampuses: () => publish(pulseClient.refreshCampuses()),
      selectRoom: (roomId) => publish(pulseClient.selectRoom(roomId)),
      sendMessage: (roomId, body) =>
        publish(pulseClient.sendMessage(roomId, body)),
      discardMessage: (clientNonce) =>
        publish(pulseClient.discardMessage(clientNonce)),
      sendReply: (roomId, rootMessageId, body) =>
        publish(pulseClient.sendReply(roomId, rootMessageId, body)),
      react: (roomId, messageId, emoji, action) =>
        publish(pulseClient.react(roomId, messageId, emoji, action)),
      setTopicTitle: (roomId, rootMessageId, topicTitle) =>
        publish(pulseClient.setTopicTitle(roomId, rootMessageId, topicTitle)),
      setThreadSubscription: (roomId, rootMessageId, state) =>
        publish(pulseClient.setThreadSubscription(roomId, rootMessageId, state)),
      openThread: (roomId, rootMessageId) =>
        publish(pulseClient.openThread(roomId, rootMessageId)),
      closeThread: () => publish(pulseClient.closeThread()),
      toggleInbox: () => publish(pulseClient.toggleInbox()),
      moveInboxSelection: (step) => publish(pulseClient.moveInboxSelection(step)),
      archiveInboxSelection: () => publish(pulseClient.archiveInboxSelection()),
      startInboxReply: () => publish(pulseClient.startInboxReply()),
      cancelInboxReply: () => publish(pulseClient.cancelInboxReply()),
      toggleSearch: () => publish(pulseClient.toggleSearch()),
      search: (query) => publish(pulseClient.search(query)),
      moveSearchSelection: (step) =>
        publish(pulseClient.moveSearchSelection(step)),
      openSearchSelection: () => publish(pulseClient.openSearchSelection()),
      openSearchHit: (messageId) =>
        publish(pulseClient.openSearchHit(messageId)),
      attachFile: (roomId, file) =>
        publish(pulseClient.attachFile(roomId, file)),
      discardDraft: (draftId) => publish(pulseClient.discardDraft(draftId)),
      retryDraft: (draftId) => publish(pulseClient.retryDraft(draftId)),
      setPin: (roomId, messageId, pinned) =>
        publish(pulseClient.setPin(roomId, messageId, pinned)),
      openVault: (roomId, tab) => publish(pulseClient.openVault(roomId, tab)),
      loadMoreVault: () => publish(pulseClient.loadMoreVault()),
      closeVault: () => publish(pulseClient.closeVault()),
      toggleShelf: () => publish(pulseClient.toggleShelf()),
      updateShelf: (roomId, messageId, action, preset, remindAtMs) =>
        publish(
          pulseClient.updateShelf(roomId, messageId, action, preset, remindAtMs),
        ),
      dismissReminder: () => publish(pulseClient.dismissReminder()),
      joinLounge: (loungeId) => publish(pulseClient.joinLounge(loungeId)),
      leaveLounge: () => publish(pulseClient.leaveLounge()),
      reportLoungeMedia: (state) =>
        publish(pulseClient.reportLoungeMedia(state)),
      toggleStewards: () => publish(pulseClient.toggleStewards()),
      createSteward: (displayName, roomIds) =>
        publish(pulseClient.createSteward(displayName, roomIds)),
      rotateStewardToken: (stewardId) =>
        publish(pulseClient.rotateStewardToken(stewardId)),
      revokeSteward: (stewardId) =>
        publish(pulseClient.revokeSteward(stewardId)),
      assignStewardRooms: (stewardId, roomIds) =>
        publish(pulseClient.assignStewardRooms(stewardId, roomIds)),
      dismissStewardToken: () => publish(pulseClient.dismissStewardToken()),
    }),
    [publish],
  );

  return { snapshot, failed, intents };
}
