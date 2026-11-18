import { useCallback, useEffect, useMemo, useState } from "react";
import type {
  CampusId,
  ClientNonce,
  ClientSnapshot,
  CreateCampusRequest,
  DraftId,
  InboxStep,
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
  createCampus(request: CreateCampusRequest): Promise<void>;
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
      createCampus: (request) => publish(pulseClient.createCampus(request)),
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
    }),
    [publish],
  );

  return { snapshot, failed, intents };
}
