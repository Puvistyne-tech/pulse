use pulse_protocol::ProtocolVersion;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initial_snapshot() -> Result<JsValue, JsError> {
    let serializer = Serializer::new()
        .serialize_missing_as_null(true)
        .serialize_maps_as_objects(true);
    pulse_core::initial_snapshot()
        .serialize(&serializer)
        .map_err(JsError::from)
}

#[wasm_bindgen]
pub fn protocol_version() -> u16 {
    ProtocolVersion::CURRENT.0
}
