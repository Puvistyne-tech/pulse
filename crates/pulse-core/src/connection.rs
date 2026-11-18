use pulse_protocol::{
    AuthToken, CampusId, ClientFrame, ConnectionPhase, ConnectionSnapshot, HandshakeRejected,
    HandshakeRejection, Hello, ProtocolVersion, Role, ServerFrame, SessionId, VersionMismatch,
    Welcome,
};

use crate::backoff::BackoffPolicy;
use crate::transport::NetworkTransport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionEvent {
    ConnectRequested {
        campus_id: CampusId,
        auth_token: AuthToken,
    },
    TransportOpened,
    WelcomeReceived(Welcome),
    VersionRejected(VersionMismatch),
    HandshakeRejected(HandshakeRejected),
    TransportFailed,
    BackoffElapsed,
    DisconnectRequested,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionCommand {
    OpenTransport,
    SendHello(Hello),
    ScheduleRetry { delay_ms: u32 },
    CloseTransport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionMachine {
    phase: ConnectionPhase,
    attempt: u32,
    retry_in_ms: Option<u32>,
    campus_id: Option<CampusId>,
    auth_token: Option<AuthToken>,
    session_id: Option<SessionId>,
    role: Option<Role>,
    rejection: Option<VersionMismatch>,
    handshake_rejection: Option<HandshakeRejection>,
    policy: BackoffPolicy,
}

impl ConnectionMachine {
    pub fn new(policy: BackoffPolicy) -> Self {
        Self {
            phase: ConnectionPhase::Disconnected,
            attempt: 0,
            retry_in_ms: None,
            campus_id: None,
            auth_token: None,
            session_id: None,
            role: None,
            rejection: None,
            handshake_rejection: None,
            policy,
        }
    }

    pub fn role(&self) -> Option<Role> {
        self.role
            .filter(|_| matches!(self.phase, ConnectionPhase::Connected))
    }

    pub fn phase(&self) -> &ConnectionPhase {
        &self.phase
    }

    pub fn snapshot(&self) -> ConnectionSnapshot {
        ConnectionSnapshot {
            protocol_version: ProtocolVersion::CURRENT,
            connection: self.phase,
            campus_id: self.campus_id,
            session_id: self.session_id,
            retry_in_ms: self.retry_in_ms,
            attempt: self.attempt,
            rejection: self.rejection.clone(),
            handshake_rejection: self.handshake_rejection,
        }
    }

    fn enter(&mut self, phase: ConnectionPhase) {
        self.phase = phase;
        self.retry_in_ms = None;
    }

    fn enter_backoff(&mut self, delay_ms: u32) {
        self.phase = ConnectionPhase::Backoff;
        self.retry_in_ms = Some(delay_ms);
    }

    pub fn step(mut self, event: ConnectionEvent) -> (Self, Option<ConnectionCommand>) {
        let command = match (self.phase, event) {
            (
                ConnectionPhase::Disconnected,
                ConnectionEvent::ConnectRequested {
                    campus_id,
                    auth_token,
                },
            ) => {
                self.enter(ConnectionPhase::Connecting);
                self.campus_id = Some(campus_id);
                self.auth_token = Some(auth_token);
                self.rejection = None;
                self.handshake_rejection = None;
                Some(ConnectionCommand::OpenTransport)
            }
            (ConnectionPhase::Connecting, ConnectionEvent::TransportOpened) => {
                let credentials = self.campus_id.zip(self.auth_token.clone());
                credentials.map(|(campus_id, auth_token)| {
                    self.enter(ConnectionPhase::Handshaking);
                    ConnectionCommand::SendHello(Hello {
                        protocol_version: ProtocolVersion::CURRENT,
                        campus_id,
                        auth_token,
                    })
                })
            }
            (
                ConnectionPhase::Connecting | ConnectionPhase::Handshaking,
                ConnectionEvent::TransportFailed,
            ) => {
                let delay_ms = self.policy.delay_ms(self.attempt);
                self.enter_backoff(delay_ms);
                self.attempt = self.attempt.saturating_add(1);
                Some(ConnectionCommand::ScheduleRetry { delay_ms })
            }
            (ConnectionPhase::Handshaking, ConnectionEvent::WelcomeReceived(welcome)) => {
                if Some(welcome.campus_id) == self.campus_id {
                    self.enter(ConnectionPhase::Connected);
                    self.session_id = Some(welcome.session_id);
                    self.role = Some(welcome.role);
                    self.attempt = 0;
                    None
                } else {
                    self.enter(ConnectionPhase::Disconnected);
                    self.session_id = None;
                    Some(ConnectionCommand::CloseTransport)
                }
            }
            (ConnectionPhase::Handshaking, ConnectionEvent::VersionRejected(mismatch)) => {
                self.enter(ConnectionPhase::Disconnected);
                self.rejection = Some(mismatch);
                Some(ConnectionCommand::CloseTransport)
            }
            (ConnectionPhase::Handshaking, ConnectionEvent::HandshakeRejected(rejected)) => {
                self.enter(ConnectionPhase::Disconnected);
                self.handshake_rejection = Some(rejected.reason);
                Some(ConnectionCommand::CloseTransport)
            }
            (ConnectionPhase::Connected, ConnectionEvent::TransportFailed) => {
                let delay_ms = self.policy.delay_ms(0);
                self.enter_backoff(delay_ms);
                self.attempt = 1;
                self.session_id = None;
                Some(ConnectionCommand::ScheduleRetry { delay_ms })
            }
            (ConnectionPhase::Backoff, ConnectionEvent::BackoffElapsed) => {
                self.enter(ConnectionPhase::Connecting);
                Some(ConnectionCommand::OpenTransport)
            }
            (ConnectionPhase::Disconnected, ConnectionEvent::DisconnectRequested) => None,
            (_, ConnectionEvent::DisconnectRequested) => {
                self.enter(ConnectionPhase::Disconnected);
                self.attempt = 0;
                self.session_id = None;
                Some(ConnectionCommand::CloseTransport)
            }
            _ => None,
        };
        (self, command)
    }
}

async fn run_command<T: NetworkTransport>(
    command: ConnectionCommand,
    transport: &mut T,
) -> Option<ConnectionEvent> {
    match command {
        ConnectionCommand::OpenTransport => Some(match transport.open().await {
            Ok(()) => ConnectionEvent::TransportOpened,
            Err(_) => ConnectionEvent::TransportFailed,
        }),
        ConnectionCommand::SendHello(hello) => {
            if transport.send(ClientFrame::Hello(hello)).await.is_err() {
                return Some(ConnectionEvent::TransportFailed);
            }
            Some(match transport.receive().await {
                Ok(ServerFrame::Welcome(welcome)) => ConnectionEvent::WelcomeReceived(welcome),
                Ok(ServerFrame::VersionMismatch(mismatch)) => {
                    ConnectionEvent::VersionRejected(mismatch)
                }
                Ok(ServerFrame::HandshakeRejected(rejected)) => {
                    ConnectionEvent::HandshakeRejected(rejected)
                }
                Ok(_) | Err(_) => ConnectionEvent::TransportFailed,
            })
        }
        ConnectionCommand::CloseTransport => {
            transport.close().await;
            None
        }
        ConnectionCommand::ScheduleRetry { .. } => None,
    }
}

pub async fn handshake<T: NetworkTransport>(
    machine: ConnectionMachine,
    transport: &mut T,
    campus_id: CampusId,
    auth_token: AuthToken,
) -> ConnectionMachine {
    let machine = match machine.phase() {
        ConnectionPhase::Disconnected => machine,
        _ if machine.campus_id != Some(campus_id)
            || machine.auth_token.as_ref() != Some(&auth_token) =>
        {
            let (reset, command) = machine.step(ConnectionEvent::DisconnectRequested);
            if let Some(command) = command {
                run_command(command, transport).await;
            }
            reset
        }
        _ => machine,
    };
    let starting_event = match machine.phase() {
        ConnectionPhase::Disconnected => ConnectionEvent::ConnectRequested {
            campus_id,
            auth_token,
        },
        ConnectionPhase::Backoff => ConnectionEvent::BackoffElapsed,
        _ => return machine,
    };
    let (mut machine, mut command) = machine.step(starting_event);
    while let Some(current) = command.take() {
        let Some(event) = run_command(current, transport).await else {
            break;
        };
        let (next, next_command) = machine.step(event);
        machine = next;
        command = next_command;
    }
    machine
}
