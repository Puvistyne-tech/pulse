use pulse_protocol::{Permission, ProtocolVersion, Role};
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

#[wasm_bindgen]
pub fn role_permits(role: JsValue, permission: JsValue) -> Result<bool, JsError> {
    let role: Role = serde_wasm_bindgen::from_value(role)?;
    let permission: Permission = serde_wasm_bindgen::from_value(permission)?;
    Ok(pulse_protocol::role_permits(role, permission))
}
