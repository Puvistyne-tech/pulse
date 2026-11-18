import { Channel, invoke } from "@tauri-apps/api/core";
import type {
  CampusId,
  ClientNonce,
  ClientSnapshot,
  CreateCampusRequest,
  RoomId,
  SignInRequest,
  SignUpRequest,
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
  watchSnapshots: (onSnapshot: (snapshot: ClientSnapshot) => void) => {
    const channel = new Channel<ClientSnapshot>();
    channel.onmessage = onSnapshot;
    return invoke<void>("watch_snapshots", { onSnapshot: channel });
  },
};
