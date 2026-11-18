import { useCallback, useEffect, useMemo, useState } from "react";
import type {
  CampusId,
  ClientSnapshot,
  CreateCampusRequest,
  SignInRequest,
  SignUpRequest,
} from "@pulse/protocol";
import { pulseClient } from "./pulseClient";

export interface PulseIntents {
  signUp(request: SignUpRequest): Promise<void>;
  signIn(request: SignInRequest): Promise<void>;
  signOut(): Promise<void>;
  createCampus(request: CreateCampusRequest): Promise<void>;
  selectCampus(campusId: CampusId): Promise<void>;
  refreshCampuses(): Promise<void>;
}

export function usePulseClient(): {
  snapshot: ClientSnapshot | null;
  failed: boolean;
  intents: PulseIntents;
} {
  const [snapshot, setSnapshot] = useState<ClientSnapshot | null>(null);
  const [failed, setFailed] = useState(false);

  const publish = useCallback(async (pending: Promise<ClientSnapshot>) => {
    try {
      setSnapshot(await pending);
    } catch {
      setFailed(true);
    }
  }, []);

  useEffect(() => {
    void publish(pulseClient.snapshot());
  }, [publish]);

  const intents = useMemo<PulseIntents>(
    () => ({
      signUp: (request) => publish(pulseClient.signUp(request)),
      signIn: (request) => publish(pulseClient.signIn(request)),
      signOut: () => publish(pulseClient.signOut()),
      createCampus: (request) => publish(pulseClient.createCampus(request)),
      selectCampus: (campusId) => publish(pulseClient.selectCampus(campusId)),
      refreshCampuses: () => publish(pulseClient.refreshCampuses()),
    }),
    [publish],
  );

  return { snapshot, failed, intents };
}
