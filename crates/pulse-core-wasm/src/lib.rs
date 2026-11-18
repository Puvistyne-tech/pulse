#[cfg(target_arch = "wasm32")]
mod indexed_db;

#[cfg(target_arch = "wasm32")]
pub use indexed_db::IndexedDbSearchIndex;

use pulse_protocol::{Permission, ProtocolVersion, Role};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

fn js_serializer() -> Serializer {
    Serializer::new()
        .serialize_missing_as_null(true)
        .serialize_maps_as_objects(true)
}

#[wasm_bindgen]
pub fn initial_snapshot() -> Result<JsValue, JsError> {
    pulse_core::initial_snapshot()
        .serialize(&js_serializer())
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct SearchIndexHandle {
    index: IndexedDbSearchIndex,
}

#[cfg(target_arch = "wasm32")]
fn store_failure(error: pulse_core::StoreError) -> JsError {
    JsError::new(&error.to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl SearchIndexHandle {
    pub async fn open(database_name: String) -> Result<SearchIndexHandle, JsError> {
        let index = IndexedDbSearchIndex::open(&database_name)
            .await
            .map_err(store_failure)?;
        Ok(SearchIndexHandle { index })
    }

    pub async fn upsert(&mut self, documents: JsValue) -> Result<(), JsError> {
        let documents: Vec<pulse_core::SearchDocument> = serde_wasm_bindgen::from_value(documents)?;
        pulse_core::SearchIndex::upsert(&mut self.index, documents)
            .await
            .map_err(store_failure)
    }

    pub async fn search(
        &self,
        account_id: JsValue,
        campus_id: JsValue,
        query: String,
    ) -> Result<JsValue, JsError> {
        let query = pulse_core::SearchQuery {
            account_id: serde_wasm_bindgen::from_value(account_id)?,
            campus_id: serde_wasm_bindgen::from_value(campus_id)?,
            terms: pulse_protocol::normalize_search_query(&query),
            limit: pulse_protocol::SEARCH_CANDIDATE_LIMIT,
        };
        let hits = pulse_core::run_search(&self.index, &query)
            .await
            .map_err(store_failure)?;
        hits.serialize(&js_serializer()).map_err(JsError::from)
    }
}
