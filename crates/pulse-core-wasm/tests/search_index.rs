#![cfg(target_arch = "wasm32")]

#[path = "../../pulse-core/tests/fixtures/builders.rs"]
mod builders;
#[path = "../../pulse-core/tests/fixtures/search_cases.rs"]
mod search_cases;

use pulse_core::{SearchIndex, run_search};
use pulse_core_wasm::IndexedDbSearchIndex;
use search_cases::{assert_search_case, case_named, documents_of, query_of, search_cases};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

fn database_name(label: &str) -> String {
    format!("pulse-search-test-{label}-{}", js_sys::Math::random())
}

#[wasm_bindgen_test]
async fn every_case_passes_on_the_indexed_db_backend() {
    for (position, case) in search_cases().into_iter().enumerate() {
        let name = database_name(&position.to_string());
        let mut index = IndexedDbSearchIndex::open(&name).await.expect("open");
        assert_search_case(&mut index, &case).await;
    }
}

#[wasm_bindgen_test]
async fn documents_survive_reopening_the_database() {
    let case = case_named("and_semantics_require_every_term");
    let name = database_name("reopen");
    {
        let mut index = IndexedDbSearchIndex::open(&name).await.expect("open");
        for round in &case.rounds {
            index.upsert(documents_of(round)).await.expect("upsert");
        }
    }
    let reopened = IndexedDbSearchIndex::open(&name).await.expect("reopen");
    let query = query_of(&case, 10);
    let hits = run_search(&reopened, &query).await.expect("search");
    assert_eq!(hits.len(), case.expected.len());
}
