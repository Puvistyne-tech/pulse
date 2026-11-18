use core::future::Future;

use pulse_protocol::{ClientFrame, ServerFrame};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TransportError {
    #[error("transport closed")]
    Closed,
    #[error("transport failure: {0}")]
    Failure(String),
    #[error("malformed frame")]
    Malformed,
}

pub trait NetworkTransport {
    fn open(&mut self) -> impl Future<Output = Result<(), TransportError>>;
    fn send(&mut self, frame: ClientFrame) -> impl Future<Output = Result<(), TransportError>>;
    fn receive(&mut self) -> impl Future<Output = Result<ServerFrame, TransportError>>;
    fn close(&mut self) -> impl Future<Output = ()>;
}
