use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ProtocolVersion(pub u16);

impl ProtocolVersion {
    pub const CURRENT: ProtocolVersion = ProtocolVersion(4);
}
