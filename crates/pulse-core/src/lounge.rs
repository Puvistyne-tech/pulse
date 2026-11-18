use std::collections::BTreeMap;

use pulse_protocol::{
    AccountId, ApiError, ApiErrorCode, ClientFrame, JoinLounge, LOUNGE_HEARTBEAT_MS, LeaveLounge,
    Lounge, LoungeEntry, LoungeHeartbeat, LoungeId, LoungeJoined, LoungeMediaState,
    LoungeParticipantView, LoungeRejected, LoungeRejection, LoungeScope, LoungeSnapshot,
    LoungeTicket,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoungeIntent {
    JoinRequested { lounge_id: LoungeId },
    LeaveRequested,
    MediaReported(LoungeMediaState),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LoungeTracker {
    lounges: BTreeMap<LoungeId, Lounge>,
    joined: Option<LoungeId>,
    joining: bool,
    ticket: Option<LoungeTicket>,
    media: LoungeMediaState,
    last_beat_ms: u64,
    error: Option<ApiError>,
}

fn rejection_error(reason: LoungeRejection) -> ApiError {
    match reason {
        LoungeRejection::Forbidden => ApiError::new(ApiErrorCode::Forbidden),
        LoungeRejection::UnknownLounge => ApiError::new(ApiErrorCode::NotFound),
        LoungeRejection::Full => ApiError::new(ApiErrorCode::Conflict),
        LoungeRejection::Unavailable => ApiError::new(ApiErrorCode::Unavailable),
    }
}

impl LoungeTracker {
    pub(crate) fn apply_lounge(&mut self, lounge: Lounge) {
        let stale = self
            .lounges
            .get(&lounge.lounge_id)
            .is_some_and(|known| known.revision > lounge.revision);
        if !stale {
            self.lounges.insert(lounge.lounge_id, lounge);
        }
    }

    pub(crate) fn apply_synced(&mut self, lounges: Vec<Lounge>) {
        for lounge in lounges {
            self.apply_lounge(lounge);
        }
    }

    pub(crate) fn apply_joined(&mut self, joined: LoungeJoined, clock_ms: u64) {
        let lounge_id = joined.lounge.lounge_id;
        self.apply_lounge(joined.lounge);
        if self.joined == Some(lounge_id) {
            self.ticket = Some(joined.ticket);
            self.joining = false;
            self.error = None;
            self.last_beat_ms = clock_ms;
        }
    }

    pub(crate) fn apply_rejected(&mut self, rejected: LoungeRejected) {
        self.error = Some(rejection_error(rejected.reason));
        if self.joined.is_none() || self.joined == Some(rejected.lounge_id) {
            self.joined = None;
            self.ticket = None;
            self.joining = false;
            self.media = LoungeMediaState::Idle;
        }
    }

    pub(crate) fn apply_intent(
        &mut self,
        intent: LoungeIntent,
        connected: bool,
        clock_ms: u64,
    ) -> Vec<ClientFrame> {
        match intent {
            LoungeIntent::JoinRequested { lounge_id } => {
                if self.joined == Some(lounge_id) {
                    return Vec::new();
                }
                let mut frames = Vec::new();
                if let Some(previous) = self.joined
                    && connected
                {
                    frames.push(ClientFrame::LeaveLounge(LeaveLounge {
                        lounge_id: previous,
                    }));
                }
                self.joined = Some(lounge_id);
                self.joining = true;
                self.ticket = None;
                self.media = LoungeMediaState::Idle;
                self.error = None;
                self.last_beat_ms = clock_ms;
                if connected {
                    frames.push(ClientFrame::JoinLounge(JoinLounge { lounge_id }));
                }
                frames
            }
            LoungeIntent::LeaveRequested => {
                let Some(lounge_id) = self.joined.take() else {
                    return Vec::new();
                };
                self.joining = false;
                self.ticket = None;
                self.media = LoungeMediaState::Idle;
                self.error = None;
                if connected {
                    vec![ClientFrame::LeaveLounge(LeaveLounge { lounge_id })]
                } else {
                    Vec::new()
                }
            }
            LoungeIntent::MediaReported(state) => {
                self.media = state;
                Vec::new()
            }
        }
    }

    pub(crate) fn on_connected(&self) -> Vec<ClientFrame> {
        self.joined
            .map(|lounge_id| ClientFrame::JoinLounge(JoinLounge { lounge_id }))
            .into_iter()
            .collect()
    }

    pub(crate) fn on_disconnected(&mut self) {
        if self.joined.is_some() {
            self.joining = true;
        }
    }

    fn heartbeat_deadline_ms(&self) -> Option<u64> {
        let confirmed = self.ticket.is_some() && !self.joining;
        self.joined
            .filter(|_| confirmed)
            .map(|_| self.last_beat_ms.saturating_add(LOUNGE_HEARTBEAT_MS))
    }

    pub(crate) fn heartbeat(&mut self, clock_ms: u64, connected: bool) -> Vec<ClientFrame> {
        let due = self
            .heartbeat_deadline_ms()
            .is_some_and(|deadline| clock_ms >= deadline);
        let Some(lounge_id) = self.joined.filter(|_| due && connected) else {
            return Vec::new();
        };
        self.last_beat_ms = clock_ms;
        vec![ClientFrame::LoungeHeartbeat(LoungeHeartbeat { lounge_id })]
    }

    pub(crate) fn next_deadline_ms(&self, clock_ms: u64) -> Option<u64> {
        let expiry = self
            .lounges
            .values()
            .flat_map(|lounge| lounge.participants.iter())
            .map(|participant| participant.present_until_ms)
            .filter(|until| *until > clock_ms)
            .min();
        [expiry, self.heartbeat_deadline_ms()]
            .into_iter()
            .flatten()
            .min()
    }

    pub(crate) fn snapshot(&self, self_id: AccountId, clock_ms: u64) -> LoungeSnapshot {
        let mut lounges = self.lounges.values().collect::<Vec<_>>();
        lounges.sort_by_key(|lounge| (lounge.scope == LoungeScope::Room, lounge.lounge_id));
        LoungeSnapshot {
            lounges: lounges
                .into_iter()
                .map(|lounge| LoungeEntry {
                    lounge_id: lounge.lounge_id,
                    scope: lounge.scope,
                    room_id: lounge.room_id,
                    room_slug: None,
                    participants: lounge
                        .participants
                        .iter()
                        .filter(|participant| participant.present_until_ms > clock_ms)
                        .map(|participant| LoungeParticipantView {
                            account_id: participant.account_id,
                            display_name: participant.display_name.clone(),
                            from_self: participant.account_id == self_id,
                            can_speak: participant.can_speak,
                        })
                        .collect(),
                    joined: self.joined == Some(lounge.lounge_id),
                })
                .collect(),
            joined_lounge_id: self.joined,
            joining: self.joining,
            ticket: self.ticket.clone(),
            media: self.media,
            can_join: false,
            can_speak: false,
            error: self.error,
        }
    }
}
