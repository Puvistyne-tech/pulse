use pulse_protocol::{ApiError, ApiErrorCode, ClientSnapshot};

use crate::api::ApiTransport;
use crate::backoff::BackoffPolicy;
use crate::connection::ConnectionMachine;
use crate::session::{SessionEvent, SessionMachine, drive_session};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    connection: ConnectionMachine,
    session: SessionMachine,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(BackoffPolicy::default())
    }
}

impl Client {
    pub fn new(policy: BackoffPolicy) -> Self {
        Self {
            connection: ConnectionMachine::new(policy),
            session: SessionMachine::new(),
        }
    }

    pub fn snapshot(&self) -> ClientSnapshot {
        ClientSnapshot {
            network: self.connection.snapshot(),
            session: self.session.snapshot(),
        }
    }

    pub async fn apply<A: ApiTransport>(
        &mut self,
        api: &mut A,
        event: SessionEvent,
    ) -> ClientSnapshot {
        let machine = core::mem::take(&mut self.session);
        self.session = drive_session(machine, api, event).await;
        self.snapshot()
    }
}

pub fn initial_snapshot() -> ClientSnapshot {
    Client::default().snapshot()
}

pub fn unavailable_snapshot() -> ClientSnapshot {
    let mut snapshot = initial_snapshot();
    snapshot.session.error = Some(ApiError::new(ApiErrorCode::Unavailable));
    snapshot
}
