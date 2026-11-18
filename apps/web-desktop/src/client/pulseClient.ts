import { invoke } from "@tauri-apps/api/core";
import type {
  CampusId,
  ClientSnapshot,
  CreateCampusRequest,
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
};
