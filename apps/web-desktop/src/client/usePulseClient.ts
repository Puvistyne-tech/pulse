import { useCallback, useEffect, useMemo, useState } from "react";
import type {
  CampusId,
  ClientNonce,
  ClientSnapshot,
  CreateCampusRequest,
  RoomId,
  SignInRequest,
  SignUpRequest,
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
    }),
    [publish],
  );

  return { snapshot, failed, intents };
}
