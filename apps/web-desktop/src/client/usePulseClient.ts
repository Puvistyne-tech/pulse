import { useCallback, useEffect, useMemo, useState } from "react";
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
    }),
    [publish],
  );

  return { snapshot, failed, intents };
}
