use core::future::Future;

use pulse_protocol::{
    AccountId, AddMemberRequest, ApiError, ApiErrorCode, AuthSession, AuthToken, CampusCreated,
    CampusId, CampusInvite, CampusList, CampusMember, CreateCampusRequest, JoinCampusRequest,
    MemberList, Note, NoteList, RoomList, SignInRequest, SignUpRequest, StartNoteRequest,
    UpdateMemberRequest,
};
use serde::de::DeserializeOwned;

pub trait ApiTransport {
    fn sign_up(
        &mut self,
        request: SignUpRequest,
    ) -> impl Future<Output = Result<AuthSession, ApiError>>;
    fn sign_in(
        &mut self,
        request: SignInRequest,
    ) -> impl Future<Output = Result<AuthSession, ApiError>>;
    fn sign_out(&mut self, token: AuthToken) -> impl Future<Output = Result<(), ApiError>>;
    fn list_campuses(
        &mut self,
        token: AuthToken,
    ) -> impl Future<Output = Result<CampusList, ApiError>>;
    fn create_campus(
        &mut self,
        token: AuthToken,
        request: CreateCampusRequest,
    ) -> impl Future<Output = Result<CampusCreated, ApiError>>;
    fn list_rooms(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> impl Future<Output = Result<RoomList, ApiError>>;
    fn join_campus(
        &mut self,
        token: AuthToken,
        request: JoinCampusRequest,
    ) -> impl Future<Output = Result<CampusCreated, ApiError>>;
    fn create_invite(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> impl Future<Output = Result<CampusInvite, ApiError>>;
    fn list_members(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> impl Future<Output = Result<MemberList, ApiError>>;
    fn add_member(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        request: AddMemberRequest,
    ) -> impl Future<Output = Result<CampusMember, ApiError>>;
    fn update_member(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        account_id: AccountId,
        request: UpdateMemberRequest,
    ) -> impl Future<Output = Result<CampusMember, ApiError>>;
    fn list_notes(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> impl Future<Output = Result<NoteList, ApiError>>;
    fn start_note(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        request: StartNoteRequest,
    ) -> impl Future<Output = Result<Note, ApiError>>;
    fn remove_member(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        account_id: AccountId,
    ) -> impl Future<Output = Result<(), ApiError>>;
}

pub fn decode_api_response<T: DeserializeOwned>(status: u16, body: &[u8]) -> Result<T, ApiError> {
    if (200..300).contains(&status) {
        return serde_json::from_slice(body).map_err(|_| ApiError::new(ApiErrorCode::Internal));
    }
    Err(serde_json::from_slice::<ApiError>(body)
        .unwrap_or_else(|_| ApiError::new(ApiErrorCode::from_http_status(status))))
}
