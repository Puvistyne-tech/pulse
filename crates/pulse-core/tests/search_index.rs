#[path = "fixtures/search_cases.rs"]
mod search_cases;

use pulse_core::MemorySearchIndex;
use search_cases::{assert_search_case, search_cases};

#[tokio::test]
async fn every_case_passes_on_the_memory_backend() {
    for case in search_cases() {
        let mut index = MemorySearchIndex::default();
        assert_search_case(&mut index, &case).await;
    }
}

#[cfg(feature = "native")]
mod native {
    use std::path::PathBuf;

    use super::search_cases::{Seed, assert_search_case, documents_of, search_cases};
    use pulse_core::native::SqliteSearchIndex;
    use pulse_core::{SearchIndex, SearchQuery, run_search};
    use pulse_protocol::fixtures::{ACCOUNT_A, CAMPUS_A};
    use pulse_protocol::normalize_search_query;

    fn scratch_path(label: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "pulse-search-{label}-{}-{nanos}.sqlite3",
            std::process::id()
        ))
    }

    #[tokio::test]
    async fn every_case_passes_on_the_sqlite_backend() {
        for case in search_cases() {
            let mut index = SqliteSearchIndex::in_memory().expect("index");
            assert_search_case(&mut index, &case).await;
        }
    }

    #[tokio::test]
    async fn on_disk_documents_survive_a_reopen_and_stay_replaceable() {
        let path = scratch_path("reopen");
        let case = search_cases()
            .into_iter()
            .find(|case| case.name == "and_semantics_require_every_term")
            .expect("case");
        {
            let mut index = SqliteSearchIndex::open(&path).expect("open");
            for round in &case.rounds {
                index.upsert(documents_of(round)).await.expect("upsert");
            }
        }
        let mut reopened = SqliteSearchIndex::open(&path).expect("reopen");
        let query = SearchQuery {
            account_id: ACCOUNT_A,
            campus_id: CAMPUS_A,
            terms: normalize_search_query(case.query),
            limit: 10,
        };
        let hits = run_search(&reopened, &query).await.expect("search");
        assert_eq!(hits.len(), case.expected.len());
        let replacement: Vec<Seed> = case.rounds[0][..1]
            .iter()
            .map(|seed| Seed {
                body: "gamma only",
                ..*seed
            })
            .collect();
        reopened
            .upsert(documents_of(&replacement))
            .await
            .expect("replace");
        let after = run_search(&reopened, &query).await.expect("search");
        assert_eq!(after.len(), case.expected.len() - 1);
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn concurrent_clones_share_one_index() {
        let index = SqliteSearchIndex::in_memory().expect("index");
        let case = search_cases()
            .into_iter()
            .find(|case| case.name == "newest_message_id_breaks_ties")
            .expect("case");
        let mut writers = tokio::task::JoinSet::new();
        for seed in case.rounds[0].clone() {
            let mut handle = index.clone();
            writers.spawn(async move { handle.upsert(documents_of(&[seed])).await });
        }
        while let Some(done) = writers.join_next().await {
            done.expect("writer task").expect("upsert");
        }
        let query = SearchQuery {
            account_id: ACCOUNT_A,
            campus_id: CAMPUS_A,
            terms: normalize_search_query(case.query),
            limit: 10,
        };
        let hits = run_search(&index, &query).await.expect("search");
        assert_eq!(hits.len(), case.expected.len());
    }
}
