use std::collections::{BTreeMap, BTreeSet};

use idb::{Database, Index, KeyPath, KeyRange, ObjectStore, Query, TransactionMode};
use js_sys::Array;
use pulse_core::{SearchDocument, SearchIndex, SearchQuery, StoreError};
use pulse_protocol::{AccountId, CampusId, MessageId};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

const DATABASE_VERSION: u32 = 1;
const DOCUMENT_STORE: &str = "documents";
const TERM_INDEX: &str = "by_scoped_term";
const KEY_FIELDS: [&str; 3] = ["account_id", "campus_id", "message_id"];
const SCOPED_TERMS_FIELD: &str = "scoped_terms";
const PREFIX_UPPER_BOUND: &str = "\u{ffff}";
const MESSAGE_ID_POSITION: u32 = 2;

#[derive(Serialize, Deserialize)]
struct StoredDocument {
    account_id: AccountId,
    campus_id: CampusId,
    message_id: MessageId,
    scoped_terms: Vec<String>,
    document: SearchDocument,
}

fn unavailable(error: impl std::fmt::Display) -> StoreError {
    StoreError::Unavailable(error.to_string())
}

fn scoped_term(account_id: AccountId, campus_id: CampusId, term: &str) -> String {
    format!("{account_id}:{campus_id}:{term}")
}

fn message_id_of(key: &JsValue) -> Result<String, StoreError> {
    Array::from(key)
        .get(MESSAGE_ID_POSITION)
        .as_string()
        .ok_or_else(|| unavailable("malformed document key"))
}

pub struct IndexedDbSearchIndex {
    database: Database,
}

impl IndexedDbSearchIndex {
    pub async fn open(database_name: &str) -> Result<Self, StoreError> {
        let database = Database::builder(database_name)
            .version(DATABASE_VERSION)
            .add_object_store(
                ObjectStore::builder(DOCUMENT_STORE)
                    .key_path(Some(KeyPath::new_array(KEY_FIELDS)))
                    .add_index(
                        Index::builder(
                            TERM_INDEX.to_owned(),
                            KeyPath::new_single(SCOPED_TERMS_FIELD),
                        )
                        .multi_entry(true),
                    ),
            )
            .build()
            .await
            .map_err(unavailable)?;
        Ok(Self { database })
    }

    async fn matching_keys(
        &self,
        query: &SearchQuery,
    ) -> Result<(Vec<String>, BTreeMap<String, JsValue>), StoreError> {
        let Some(last) = query.terms.len().checked_sub(1) else {
            return Ok((Vec::new(), BTreeMap::new()));
        };
        let transaction = self
            .database
            .transaction(&[DOCUMENT_STORE], TransactionMode::ReadOnly)
            .map_err(unavailable)?;
        let store = transaction
            .object_store(DOCUMENT_STORE)
            .map_err(unavailable)?;
        let index = store.index(TERM_INDEX).map_err(unavailable)?;
        let mut matching: Option<BTreeSet<String>> = None;
        let mut keys: BTreeMap<String, JsValue> = BTreeMap::new();
        for (position, term) in query.terms.iter().enumerate() {
            let lower = scoped_term(query.account_id, query.campus_id, term);
            let upper = if position == last {
                format!("{lower}{PREFIX_UPPER_BOUND}")
            } else {
                lower.clone()
            };
            let range = KeyRange::bound(
                &JsValue::from_str(&lower),
                &JsValue::from_str(&upper),
                Some(false),
                Some(false),
            )
            .map_err(unavailable)?;
            let found_keys = index
                .get_all_keys(Some(Query::from(range)), None)
                .map_err(unavailable)?
                .await
                .map_err(unavailable)?;
            let mut found = BTreeSet::new();
            for key in found_keys {
                let message_id = message_id_of(&key)?;
                keys.entry(message_id.clone()).or_insert(key);
                found.insert(message_id);
            }
            let next = match matching {
                None => found,
                Some(previous) => previous.intersection(&found).cloned().collect(),
            };
            let exhausted = next.is_empty();
            matching = Some(next);
            if exhausted {
                break;
            }
        }
        let mut ordered = matching.unwrap_or_default().into_iter().collect::<Vec<_>>();
        ordered.sort_by(|left, right| right.cmp(left));
        ordered.truncate(query.limit);
        Ok((ordered, keys))
    }
}

impl SearchIndex for IndexedDbSearchIndex {
    async fn upsert(&mut self, documents: Vec<SearchDocument>) -> Result<(), StoreError> {
        if documents.is_empty() {
            return Ok(());
        }
        let transaction = self
            .database
            .transaction(&[DOCUMENT_STORE], TransactionMode::ReadWrite)
            .map_err(unavailable)?;
        let store = transaction
            .object_store(DOCUMENT_STORE)
            .map_err(unavailable)?;
        for document in documents {
            let scoped_terms = document
                .terms
                .iter()
                .map(|term| scoped_term(document.account_id, document.campus_id, term))
                .collect();
            let stored = StoredDocument {
                account_id: document.account_id,
                campus_id: document.campus_id,
                message_id: document.message_id,
                scoped_terms,
                document,
            };
            let value = serde_wasm_bindgen::to_value(&stored).map_err(unavailable)?;
            store
                .put(&value, None)
                .map_err(unavailable)?
                .await
                .map_err(unavailable)?;
        }
        transaction
            .commit()
            .map_err(unavailable)?
            .await
            .map_err(unavailable)?;
        Ok(())
    }

    async fn candidates(&self, query: &SearchQuery) -> Result<Vec<SearchDocument>, StoreError> {
        let (ordered, keys) = self.matching_keys(query).await?;
        let transaction = self
            .database
            .transaction(&[DOCUMENT_STORE], TransactionMode::ReadOnly)
            .map_err(unavailable)?;
        let store = transaction
            .object_store(DOCUMENT_STORE)
            .map_err(unavailable)?;
        let mut documents = Vec::with_capacity(ordered.len());
        for message_id in ordered {
            let Some(key) = keys.get(&message_id) else {
                continue;
            };
            let stored = store
                .get(Query::from(key.clone()))
                .map_err(unavailable)?
                .await
                .map_err(unavailable)?;
            if let Some(value) = stored {
                let stored: StoredDocument =
                    serde_wasm_bindgen::from_value(value).map_err(unavailable)?;
                documents.push(stored.document);
            }
        }
        Ok(documents)
    }
}
