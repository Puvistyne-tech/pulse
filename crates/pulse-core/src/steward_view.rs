use std::collections::{BTreeMap, BTreeSet};

use pulse_protocol::{
    AccountId, ApiError, ApiErrorCode, ClientFrame, ClientNonce, InputField, IssuedStewardToken,
    ManageSteward, RoomId, StewardAction, StewardChanged, StewardEntry, StewardIssued,
    StewardProfile, StewardRejected, StewardRejection, StewardSnapshot, StewardsSynced,
    mention_token, normalize_display_name, normalize_room_ids,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StewardIntent {
    Toggled,
    CreateRequested {
        request_nonce: ClientNonce,
        display_name: String,
        room_ids: Vec<RoomId>,
    },
    RotateRequested {
        request_nonce: ClientNonce,
        steward_id: AccountId,
    },
    RevokeRequested {
        request_nonce: ClientNonce,
        steward_id: AccountId,
    },
    RoomsAssigned {
        request_nonce: ClientNonce,
        steward_id: AccountId,
        room_ids: Vec<RoomId>,
    },
    IssuedTokenDismissed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PendingRequest {
    request_nonce: ClientNonce,
    action: StewardAction,
    steward_id: Option<AccountId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct StewardTracker {
    open: bool,
    directory: BTreeMap<AccountId, StewardProfile>,
    observed: BTreeSet<AccountId>,
    issued: Option<IssuedStewardToken>,
    pending: Option<PendingRequest>,
    error: Option<ApiError>,
}

fn rejection_error(reason: StewardRejection) -> ApiError {
    match reason {
        StewardRejection::Forbidden => ApiError::new(ApiErrorCode::Forbidden),
        StewardRejection::UnknownSteward => ApiError::new(ApiErrorCode::NotFound),
        StewardRejection::InvalidDisplayName => ApiError::invalid(InputField::DisplayName),
        StewardRejection::InvalidRooms => ApiError::invalid(InputField::Rooms),
        StewardRejection::Revoked | StewardRejection::TooManyStewards => {
            ApiError::new(ApiErrorCode::Conflict)
        }
    }
}

impl StewardTracker {
    pub(crate) fn is_steward(&self, account_id: AccountId) -> bool {
        self.directory.contains_key(&account_id) || self.observed.contains(&account_id)
    }

    pub(crate) fn observe(&mut self, account_id: AccountId) {
        self.observed.insert(account_id);
    }

    pub(crate) fn apply_synced(&mut self, synced: StewardsSynced) {
        self.directory = synced
            .stewards
            .into_iter()
            .map(|profile| (profile.steward_id, profile))
            .collect();
    }

    pub(crate) fn apply_changed(&mut self, changed: StewardChanged) {
        let steward_id = changed.steward.steward_id;
        self.directory.insert(steward_id, changed.steward);
        let settles = self.pending.as_ref().is_some_and(|pending| {
            pending.steward_id == Some(steward_id)
                && matches!(
                    pending.action,
                    StewardAction::Revoke | StewardAction::AssignRooms
                )
        });
        if settles {
            self.pending = None;
        }
    }

    pub(crate) fn apply_issued(&mut self, issued: StewardIssued) {
        let matches_pending = self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.request_nonce == issued.request_nonce);
        if !matches_pending {
            return;
        }
        self.pending = None;
        self.error = None;
        self.issued = Some(IssuedStewardToken {
            steward_id: issued.steward.steward_id,
            display_name: issued.steward.display_name.clone(),
            token: issued.token,
        });
        self.directory
            .insert(issued.steward.steward_id, issued.steward);
    }

    pub(crate) fn apply_rejected(&mut self, rejected: &StewardRejected) {
        let matches_pending = self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.request_nonce == rejected.request_nonce);
        if matches_pending {
            self.pending = None;
            self.error = Some(rejection_error(rejected.reason));
        }
    }

    pub(crate) fn clear_pending(&mut self) {
        self.pending = None;
    }

    pub(crate) fn apply_intent(
        &mut self,
        intent: StewardIntent,
        connected: bool,
    ) -> Vec<ClientFrame> {
        match intent {
            StewardIntent::Toggled => {
                self.open = !self.open;
                self.error = None;
                Vec::new()
            }
            StewardIntent::IssuedTokenDismissed => {
                self.issued = None;
                Vec::new()
            }
            StewardIntent::CreateRequested {
                request_nonce,
                display_name,
                room_ids,
            } => {
                let validated = normalize_display_name(&display_name)
                    .and_then(|name| normalize_room_ids(&room_ids).map(|rooms| (name, rooms)));
                match validated {
                    Err(field) => self.fail(ApiError::invalid(field)),
                    Ok((name, rooms)) => self.request(
                        connected,
                        ManageSteward {
                            request_nonce,
                            action: StewardAction::Create,
                            steward_id: None,
                            display_name: Some(name),
                            room_ids: rooms,
                        },
                    ),
                }
            }
            StewardIntent::RotateRequested {
                request_nonce,
                steward_id,
            } => self.request(
                connected,
                ManageSteward {
                    request_nonce,
                    action: StewardAction::RotateToken,
                    steward_id: Some(steward_id),
                    display_name: None,
                    room_ids: Vec::new(),
                },
            ),
            StewardIntent::RevokeRequested {
                request_nonce,
                steward_id,
            } => self.request(
                connected,
                ManageSteward {
                    request_nonce,
                    action: StewardAction::Revoke,
                    steward_id: Some(steward_id),
                    display_name: None,
                    room_ids: Vec::new(),
                },
            ),
            StewardIntent::RoomsAssigned {
                request_nonce,
                steward_id,
                room_ids,
            } => match normalize_room_ids(&room_ids) {
                Err(field) => self.fail(ApiError::invalid(field)),
                Ok(rooms) => self.request(
                    connected,
                    ManageSteward {
                        request_nonce,
                        action: StewardAction::AssignRooms,
                        steward_id: Some(steward_id),
                        display_name: None,
                        room_ids: rooms,
                    },
                ),
            },
        }
    }

    fn fail(&mut self, error: ApiError) -> Vec<ClientFrame> {
        self.error = Some(error);
        Vec::new()
    }

    fn request(&mut self, connected: bool, request: ManageSteward) -> Vec<ClientFrame> {
        if !connected {
            return self.fail(ApiError::new(ApiErrorCode::Unavailable));
        }
        self.error = None;
        self.pending = Some(PendingRequest {
            request_nonce: request.request_nonce,
            action: request.action,
            steward_id: request.steward_id,
        });
        vec![ClientFrame::ManageSteward(request)]
    }

    pub(crate) fn snapshot(&self) -> StewardSnapshot {
        StewardSnapshot {
            open: self.open,
            stewards: self
                .directory
                .values()
                .map(|profile| StewardEntry {
                    steward_id: profile.steward_id,
                    display_name: profile.display_name.clone(),
                    mention_token: mention_token(profile.steward_id),
                    room_ids: profile.room_ids.clone(),
                    revoked: profile.revoked,
                })
                .collect(),
            can_manage: false,
            issued: self.issued.clone(),
            pending: self.pending.is_some(),
            error: self.error,
        }
    }
}
