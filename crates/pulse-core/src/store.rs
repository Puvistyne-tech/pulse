use core::future::Future;

use pulse_protocol::CampusId;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StoreError {
    #[error("store unavailable: {0}")]
    Unavailable(String),
}

pub trait LocalStore {
    fn load_campus(&self) -> impl Future<Output = Result<Option<CampusId>, StoreError>>;
    fn save_campus(&mut self, campus_id: CampusId) -> impl Future<Output = Result<(), StoreError>>;
}
