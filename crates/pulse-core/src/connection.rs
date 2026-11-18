use pulse_protocol::{
    CampusId, ClientFrame, ClientSnapshot, ConnectionPhase, Hello, ProtocolVersion, ServerFrame,
    SessionId, VersionMismatch, Welcome,
};

use crate::backoff::BackoffPolicy;
use crate::transport::NetworkTransport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionEvent {
    ConnectRequested { campus_id: CampusId },
    TransportOpened,
    WelcomeReceived(Welcome),
    VersionRejected(VersionMismatch),
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
    campus_id: Option<CampusId>,
    session_id: Option<SessionId>,
    rejection: Option<VersionMismatch>,
    policy: BackoffPolicy,
}

impl ConnectionMachine {
    pub fn new(policy: BackoffPolicy) -> Self {
        Self {
            phase: ConnectionPhase::Disconnected,
            attempt: 0,
            campus_id: None,
            session_id: None,
            rejection: None,
            policy,
        }
    }

    pub fn phase(&self) -> &ConnectionPhase {
        &self.phase
    }

    pub fn snapshot(&self) -> ClientSnapshot {
        ClientSnapshot {
            protocol_version: ProtocolVersion::CURRENT,
            connection: self.phase,
            campus_id: self.campus_id,
            session_id: self.session_id,
            attempt: self.attempt,
            rejection: self.rejection.clone(),
        }
    }

    pub fn step(mut self, event: ConnectionEvent) -> (Self, Option<ConnectionCommand>) {
        let command = match (self.phase, event) {
            (ConnectionPhase::Disconnected, ConnectionEvent::ConnectRequested { campus_id }) => {
                self.phase = ConnectionPhase::Connecting;
                self.campus_id = Some(campus_id);
                self.rejection = None;
                Some(ConnectionCommand::OpenTransport)
            }
            (ConnectionPhase::Connecting, ConnectionEvent::TransportOpened) => {
                self.campus_id.map(|campus_id| {
                    self.phase = ConnectionPhase::Handshaking;
                    ConnectionCommand::SendHello(Hello {
                        protocol_version: ProtocolVersion::CURRENT,
                        campus_id,
                    })
                })
            }
            (
                ConnectionPhase::Connecting | ConnectionPhase::Handshaking,
                ConnectionEvent::TransportFailed,
            ) => {
                let delay_ms = self.policy.delay_ms(self.attempt);
                self.phase = ConnectionPhase::Backoff {
                    retry_in_ms: delay_ms,
                };
                self.attempt = self.attempt.saturating_add(1);
                Some(ConnectionCommand::ScheduleRetry { delay_ms })
            }
            (ConnectionPhase::Handshaking, ConnectionEvent::WelcomeReceived(welcome)) => {
                if Some(welcome.campus_id) == self.campus_id {
                    self.phase = ConnectionPhase::Connected;
                    self.session_id = Some(welcome.session_id);
                    self.attempt = 0;
                    None
                } else {
                    self.phase = ConnectionPhase::Disconnected;
                    self.session_id = None;
                    Some(ConnectionCommand::CloseTransport)
                }
            }
            (ConnectionPhase::Handshaking, ConnectionEvent::VersionRejected(mismatch)) => {
                self.phase = ConnectionPhase::Disconnected;
                self.rejection = Some(mismatch);
                Some(ConnectionCommand::CloseTransport)
            }
            (ConnectionPhase::Connected, ConnectionEvent::TransportFailed) => {
                let delay_ms = self.policy.delay_ms(0);
                self.phase = ConnectionPhase::Backoff {
                    retry_in_ms: delay_ms,
                };
                self.attempt = 1;
                self.session_id = None;
                Some(ConnectionCommand::ScheduleRetry { delay_ms })
            }
            (ConnectionPhase::Backoff { .. }, ConnectionEvent::BackoffElapsed) => {
                self.phase = ConnectionPhase::Connecting;
                Some(ConnectionCommand::OpenTransport)
            }
            (ConnectionPhase::Disconnected, ConnectionEvent::DisconnectRequested) => None,
            (_, ConnectionEvent::DisconnectRequested) => {
                self.phase = ConnectionPhase::Disconnected;
                self.attempt = 0;
                self.session_id = None;
                Some(ConnectionCommand::CloseTransport)
            }
            _ => None,
        };
        (self, command)
    }
}

pub fn initial_snapshot() -> ClientSnapshot {
    ConnectionMachine::new(BackoffPolicy::default()).snapshot()
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
                Ok(ServerFrame::Pong(_)) | Err(_) => ConnectionEvent::TransportFailed,
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
) -> ConnectionMachine {
    let machine = match machine.phase() {
        ConnectionPhase::Backoff { .. } if machine.campus_id != Some(campus_id) => {
            let (reset, command) = machine.step(ConnectionEvent::DisconnectRequested);
            if let Some(command) = command {
                run_command(command, transport).await;
            }
            reset
        }
        _ => machine,
    };
    let starting_event = match machine.phase() {
        ConnectionPhase::Disconnected => ConnectionEvent::ConnectRequested { campus_id },
        ConnectionPhase::Backoff { .. } => ConnectionEvent::BackoffElapsed,
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
