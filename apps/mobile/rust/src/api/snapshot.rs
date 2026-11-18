use pulse_protocol::snapshot::ClientSnapshot;

pub async fn initial_snapshot() -> ClientSnapshot {
    pulse_core::initial_snapshot()
}
