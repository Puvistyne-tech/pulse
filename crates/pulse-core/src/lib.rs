pub mod api;
pub mod backoff;
pub mod client;
pub mod connection;
mod inbox;
#[cfg(feature = "native")]
pub mod native;
pub mod session;
pub mod store;
pub mod thread_view;
pub mod timeline;
pub mod transport;

pub use api::{ApiTransport, decode_api_response};
pub use backoff::BackoffPolicy;
pub use client::{Client, LinkTarget, initial_snapshot, unavailable_snapshot};
pub use connection::{ConnectionCommand, ConnectionEvent, ConnectionMachine, handshake};
pub use session::{SessionCommand, SessionEvent, SessionMachine, drive_session};
pub use store::{LocalStore, StoreError};
pub use thread_view::{THREAD_BURIED_DEPTH, THREAD_VIEW_FOLLOW_MS};
pub use timeline::{ThreadIntent, TimelineEvent, TimelineMachine};
pub use transport::{NetworkTransport, TransportError};
