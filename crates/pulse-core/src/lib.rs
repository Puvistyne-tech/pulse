pub mod api;
pub mod backoff;
pub mod client;
pub mod connection;
pub mod effects;
mod inbox;
pub mod media;
#[cfg(feature = "native")]
pub mod native;
pub mod search;
mod search_view;
pub mod session;
mod shelf;
pub mod store;
pub mod thread_view;
pub mod timeline;
pub mod transport;

pub use api::{ApiTransport, decode_api_response};
pub use backoff::BackoffPolicy;
pub use client::{Client, LinkTarget, initial_snapshot, unavailable_snapshot};
pub use connection::{ConnectionCommand, ConnectionEvent, ConnectionMachine, handshake};
pub use effects::{Effect, EffectOutcome, UploadJob};
pub use media::{MediaIntent, MediaTransport, TICKET_REFRESH_MARGIN_MS, drive_upload};
pub use search::{
    MemorySearchIndex, SearchDocument, SearchIndex, SearchQuery, rank_hits, run_search,
};
pub use search_view::SearchIntent;
pub use session::{SessionCommand, SessionEvent, SessionMachine, drive_session};
pub use shelf::ShelfIntent;
pub use store::{LocalStore, StoreError};
pub use thread_view::{THREAD_BURIED_DEPTH, THREAD_VIEW_FOLLOW_MS};
pub use timeline::{ThreadIntent, TimelineEvent, TimelineMachine};
pub use transport::{NetworkTransport, TransportError};
