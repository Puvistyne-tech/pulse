#![allow(dead_code)]

use std::collections::VecDeque;

use pulse_core::{NetworkTransport, TransportError};
use pulse_protocol::{
    CampusId, ClientFrame, Hello, ProtocolVersion, ServerFrame, SessionId, Welcome,
};

pub const CAMPUS_A: &str = "0193a5c0-0000-7000-8000-00000000000a";
pub const CAMPUS_B: &str = "0193a5c0-0000-7000-8000-00000000000b";
pub const SESSION_A: &str = "0193a5c0-0000-7000-8000-0000000000a1";

pub fn campus(text: &str) -> CampusId {
    text.parse().expect("valid campus fixture")
}

pub fn session(text: &str) -> SessionId {
    text.parse().expect("valid session fixture")
}

pub fn welcome_for(campus_text: &str) -> Welcome {
    Welcome {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: campus(campus_text),
        session_id: session(SESSION_A),
    }
}

pub fn hello_frame(campus_text: &str) -> ClientFrame {
    ClientFrame::Hello(Hello {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: campus(campus_text),
    })
}

pub fn welcome_frame(campus_text: &str) -> Result<ServerFrame, TransportError> {
    Ok(ServerFrame::Welcome(welcome_for(campus_text)))
}

pub fn refused() -> Result<(), TransportError> {
    Err(TransportError::Failure("refused".into()))
}

#[derive(Debug, Default)]
pub struct ScriptedTransport {
    pub open_results: VecDeque<Result<(), TransportError>>,
    pub incoming: VecDeque<Result<ServerFrame, TransportError>>,
    pub sent: Vec<ClientFrame>,
    pub closes: u32,
}

impl ScriptedTransport {
    pub fn with_incoming(incoming: Vec<Result<ServerFrame, TransportError>>) -> Self {
        Self {
            incoming: incoming.into(),
            ..Self::default()
        }
    }

    pub fn scripted(
        open_results: Vec<Result<(), TransportError>>,
        incoming: Vec<Result<ServerFrame, TransportError>>,
    ) -> Self {
        Self {
            open_results: open_results.into(),
            incoming: incoming.into(),
            ..Self::default()
        }
    }

    pub fn with_open_results(open_results: Vec<Result<(), TransportError>>) -> Self {
        Self {
            open_results: open_results.into(),
            ..Self::default()
        }
    }
}

impl NetworkTransport for ScriptedTransport {
    async fn open(&mut self) -> Result<(), TransportError> {
        self.open_results.pop_front().unwrap_or(Ok(()))
    }

    async fn send(&mut self, frame: ClientFrame) -> Result<(), TransportError> {
        self.sent.push(frame);
        Ok(())
    }

    async fn receive(&mut self) -> Result<ServerFrame, TransportError> {
        self.incoming
            .pop_front()
            .unwrap_or(Err(TransportError::Closed))
    }

    async fn close(&mut self) {
        self.closes += 1;
    }
}
