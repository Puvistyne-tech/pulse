use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::{Connection, OptionalExtension, params};

use crate::search::{MemorySearchIndex, SearchDocument, SearchIndex, SearchQuery};
use crate::store::StoreError;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS search_documents (
    doc_id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT NOT NULL,
    campus_id TEXT NOT NULL,
    message_id TEXT NOT NULL,
    document TEXT NOT NULL,
    UNIQUE (account_id, campus_id, message_id)
);
CREATE VIRTUAL TABLE IF NOT EXISTS search_terms USING fts5(
    terms,
    tokenize = 'unicode61 remove_diacritics 0'
);
";

fn unavailable(error: impl std::fmt::Display) -> StoreError {
    StoreError::Unavailable(error.to_string())
}

fn fts_expression(terms: &[String]) -> Option<String> {
    let last = terms.len().checked_sub(1)?;
    Some(
        terms
            .iter()
            .enumerate()
            .map(|(index, term)| {
                let quoted = format!("\"{}\"", term.replace('"', "\"\""));
                if index == last {
                    format!("{quoted}*")
                } else {
                    quoted
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    )
}

#[derive(Clone)]
pub struct SqliteSearchIndex {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteSearchIndex {
    fn from_connection(connection: Connection) -> Result<Self, StoreError> {
        connection.execute_batch(SCHEMA).map_err(unavailable)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn open(path: &Path) -> Result<Self, StoreError> {
        let connection = Connection::open(path).map_err(unavailable)?;
        Self::from_connection(connection)
    }

    pub fn in_memory() -> Result<Self, StoreError> {
        let connection = Connection::open_in_memory().map_err(unavailable)?;
        Self::from_connection(connection)
    }

    async fn blocking<T, F>(&self, work: F) -> Result<T, StoreError>
    where
        T: Send + 'static,
        F: FnOnce(&mut Connection) -> Result<T, StoreError> + Send + 'static,
    {
        let connection = Arc::clone(&self.connection);
        tokio::task::spawn_blocking(move || {
            let mut guard = connection
                .lock()
                .map_err(|_| StoreError::Unavailable("search index poisoned".to_owned()))?;
            work(&mut guard)
        })
        .await
        .map_err(unavailable)?
    }
}

fn upsert_documents(
    connection: &mut Connection,
    documents: &[SearchDocument],
) -> Result<(), StoreError> {
    let transaction = connection.transaction().map_err(unavailable)?;
    for document in documents {
        let account = document.account_id.to_string();
        let campus = document.campus_id.to_string();
        let message = document.message_id.to_string();
        let payload = serde_json::to_string(document).map_err(unavailable)?;
        let existing: Option<i64> = transaction
            .query_row(
                "SELECT doc_id FROM search_documents \
                 WHERE account_id = ?1 AND campus_id = ?2 AND message_id = ?3",
                params![account, campus, message],
                |row| row.get(0),
            )
            .optional()
            .map_err(unavailable)?;
        let doc_id = match existing {
            Some(doc_id) => {
                transaction
                    .execute(
                        "UPDATE search_documents SET document = ?1 WHERE doc_id = ?2",
                        params![payload, doc_id],
                    )
                    .map_err(unavailable)?;
                transaction
                    .execute("DELETE FROM search_terms WHERE rowid = ?1", params![doc_id])
                    .map_err(unavailable)?;
                doc_id
            }
            None => {
                transaction
                    .execute(
                        "INSERT INTO search_documents (account_id, campus_id, message_id, document) \
                         VALUES (?1, ?2, ?3, ?4)",
                        params![account, campus, message, payload],
                    )
                    .map_err(unavailable)?;
                transaction.last_insert_rowid()
            }
        };
        transaction
            .execute(
                "INSERT INTO search_terms (rowid, terms) VALUES (?1, ?2)",
                params![doc_id, document.terms.join(" ")],
            )
            .map_err(unavailable)?;
    }
    transaction.commit().map_err(unavailable)
}

fn select_candidates(
    connection: &mut Connection,
    query: &SearchQuery,
) -> Result<Vec<SearchDocument>, StoreError> {
    let Some(expression) = fts_expression(&query.terms) else {
        return Ok(Vec::new());
    };
    let limit = i64::try_from(query.limit).unwrap_or(i64::MAX);
    let mut statement = connection
        .prepare(
            "SELECT d.document FROM search_terms \
             JOIN search_documents d ON d.doc_id = search_terms.rowid \
             WHERE search_terms MATCH ?1 AND d.account_id = ?2 AND d.campus_id = ?3 \
             ORDER BY d.message_id DESC LIMIT ?4",
        )
        .map_err(unavailable)?;
    let rows = statement
        .query_map(
            params![
                expression,
                query.account_id.to_string(),
                query.campus_id.to_string(),
                limit
            ],
            |row| row.get::<_, String>(0),
        )
        .map_err(unavailable)?;
    let mut documents = Vec::new();
    for row in rows {
        let payload = row.map_err(unavailable)?;
        documents.push(serde_json::from_str(&payload).map_err(unavailable)?);
    }
    Ok(documents)
}

impl SearchIndex for SqliteSearchIndex {
    async fn upsert(&mut self, documents: Vec<SearchDocument>) -> Result<(), StoreError> {
        self.blocking(move |connection| upsert_documents(connection, &documents))
            .await
    }

    async fn candidates(&self, query: &SearchQuery) -> Result<Vec<SearchDocument>, StoreError> {
        let query = query.clone();
        self.blocking(move |connection| select_candidates(connection, &query))
            .await
    }
}

pub enum NativeSearchIndex {
    Memory(MemorySearchIndex),
    Sqlite(SqliteSearchIndex),
}

impl SearchIndex for NativeSearchIndex {
    async fn upsert(&mut self, documents: Vec<SearchDocument>) -> Result<(), StoreError> {
        match self {
            NativeSearchIndex::Memory(index) => index.upsert(documents).await,
            NativeSearchIndex::Sqlite(index) => index.upsert(documents).await,
        }
    }

    async fn candidates(&self, query: &SearchQuery) -> Result<Vec<SearchDocument>, StoreError> {
        match self {
            NativeSearchIndex::Memory(index) => index.candidates(query).await,
            NativeSearchIndex::Sqlite(index) => index.candidates(query).await,
        }
    }
}
