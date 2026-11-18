#![allow(dead_code)]

use crate::builders::{message_id, room_message};
use pulse_core::{SearchDocument, SearchIndex, SearchQuery, run_search};
use pulse_protocol::fixtures::{ACCOUNT_A, ACCOUNT_B, CAMPUS_A, CAMPUS_B, NONCE_A, ROOM_A};
use pulse_protocol::{
    AccountId, CampusId, RoomMessage, SEARCH_CANDIDATE_LIMIT, SEARCH_HITS_LIMIT,
    normalize_search_query,
};

#[derive(Debug, Clone, Copy)]
pub struct Seed {
    pub account: AccountId,
    pub campus: CampusId,
    pub index: u128,
    pub author: &'static str,
    pub title: Option<&'static str>,
    pub body: &'static str,
}

#[derive(Debug, Clone)]
pub struct SearchCase {
    pub name: &'static str,
    pub rounds: Vec<Vec<Seed>>,
    pub query: &'static str,
    pub limit: usize,
    pub expected: Vec<u128>,
    pub top_snippet: Option<Vec<(&'static str, bool)>>,
}

fn seed(index: u128, body: &'static str) -> Seed {
    Seed {
        account: ACCOUNT_A,
        campus: CAMPUS_A,
        index,
        author: "Zed",
        title: None,
        body,
    }
}

fn authored(index: u128, author: &'static str, body: &'static str) -> Seed {
    Seed {
        author,
        ..seed(index, body)
    }
}

fn titled(index: u128, title: &'static str, body: &'static str) -> Seed {
    Seed {
        title: Some(title),
        ..seed(index, body)
    }
}

fn case(
    name: &'static str,
    seeds: Vec<Seed>,
    query: &'static str,
    expected: &[u128],
) -> SearchCase {
    SearchCase {
        name,
        rounds: vec![seeds],
        query,
        limit: SEARCH_CANDIDATE_LIMIT,
        expected: expected.to_vec(),
        top_snippet: None,
    }
}

fn message_of(seed: &Seed) -> RoomMessage {
    RoomMessage {
        campus_id: seed.campus,
        author_display_name: seed.author.to_owned(),
        topic_title: seed.title.map(str::to_owned),
        ..room_message(
            ROOM_A,
            message_id(seed.index),
            None,
            seed.account,
            NONCE_A,
            seed.body,
        )
    }
}

pub fn case_named(name: &str) -> SearchCase {
    search_cases()
        .into_iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("unknown case {name}"))
}

pub fn query_of(case: &SearchCase, limit: usize) -> SearchQuery {
    SearchQuery {
        account_id: ACCOUNT_A,
        campus_id: CAMPUS_A,
        terms: normalize_search_query(case.query),
        limit,
    }
}

fn flooded(count: u128) -> Vec<Seed> {
    (1..=count)
        .map(|index| seed(index, "flood marker"))
        .collect()
}

pub fn search_cases() -> Vec<SearchCase> {
    let cap = u128::try_from(SEARCH_CANDIDATE_LIMIT).expect("cap");
    let hits = u128::try_from(SEARCH_HITS_LIMIT).expect("hits");
    let overflow = cap + 5;
    let mut flood = flooded(overflow);
    flood[0] = titled(1, "flood", "elsewhere");
    let mut snippet_case = case(
        "snippet_marks_matched_words",
        vec![seed(1, "say alpha beta now")],
        "alpha bet",
        &[1],
    );
    snippet_case.top_snippet = Some(vec![
        ("say ", false),
        ("alpha", true),
        (" ", false),
        ("beta", true),
        (" now", false),
    ]);
    let mut limited = case(
        "limit_keeps_newest_candidates",
        flooded(5),
        "flood",
        &[5, 4],
    );
    limited.limit = 2;
    let replacement_rounds = |query: &'static str, expected: &[u128], name: &'static str| {
        let mut replaced = case(name, vec![seed(1, "old words")], query, expected);
        replaced.rounds.push(vec![seed(1, "new words")]);
        replaced
    };
    vec![
        case(
            "and_semantics_require_every_term",
            vec![
                seed(1, "alpha beta"),
                seed(2, "alpha gamma"),
                seed(3, "beta alpha delta"),
            ],
            "alpha beta",
            &[3, 1],
        ),
        case(
            "last_term_matches_as_prefix",
            vec![seed(1, "release notes"), seed(2, "relevant")],
            "rel",
            &[2, 1],
        ),
        case(
            "earlier_terms_must_match_exactly",
            vec![seed(1, "release notes")],
            "rel notes",
            &[],
        ),
        case(
            "exact_outranks_prefix_despite_newer_id",
            vec![seed(5, "plans"), seed(3, "plan")],
            "plan",
            &[3, 5],
        ),
        case(
            "field_weights_title_author_body",
            vec![
                seed(30, "ops"),
                authored(20, "Ops", "quiet"),
                titled(10, "Ops", "quiet"),
            ],
            "ops",
            &[10, 20, 30],
        ),
        case(
            "newest_message_id_breaks_ties",
            vec![seed(1, "same"), seed(3, "same"), seed(2, "same")],
            "same",
            &[3, 2, 1],
        ),
        case(
            "terms_may_span_fields",
            vec![titled(1, "roadmap", "draft ready"), seed(2, "draft ready")],
            "roadmap draft",
            &[1],
        ),
        case(
            "term_order_does_not_matter",
            vec![seed(1, "beta alpha")],
            "alpha beta",
            &[1],
        ),
        case("query_case_is_folded", vec![seed(1, "Rust")], "RUST", &[1]),
        case(
            "blank_query_finds_nothing",
            vec![seed(1, "anything")],
            "  ",
            &[],
        ),
        case(
            "punctuation_only_query_finds_nothing",
            vec![seed(1, "anything")],
            "?!",
            &[],
        ),
        case(
            "foreign_account_is_excluded",
            vec![
                seed(1, "shared words"),
                Seed {
                    account: ACCOUNT_B,
                    ..seed(2, "shared words")
                },
            ],
            "shared",
            &[1],
        ),
        case(
            "foreign_campus_is_excluded",
            vec![
                seed(1, "shared words"),
                Seed {
                    campus: CAMPUS_B,
                    ..seed(2, "shared words")
                },
            ],
            "shared",
            &[1],
        ),
        replacement_rounds("old", &[], "upsert_replaces_old_terms"),
        replacement_rounds("new", &[1], "upsert_indexes_new_terms"),
        case(
            "diacritics_stay_distinct_for_plain_query",
            vec![seed(1, "cafe"), seed(2, "café")],
            "cafe",
            &[1],
        ),
        case(
            "diacritics_stay_distinct_for_accented_query",
            vec![seed(1, "cafe"), seed(2, "café")],
            "café",
            &[2],
        ),
        case(
            "combining_mark_splits_like_the_core_tokenizer",
            vec![seed(1, "cafe\u{301}")],
            "cafe",
            &[1],
        ),
        case(
            "cjk_run_matches_by_prefix_and_exact",
            vec![seed(1, "你好世界"), seed(2, "你好")],
            "你好",
            &[2, 1],
        ),
        case(
            "cjk_terms_split_on_spaces",
            vec![seed(1, "你好 世界"), seed(2, "你好")],
            "你好 世界",
            &[1],
        ),
        case(
            "boolean_looking_words_are_plain_terms",
            vec![seed(1, "this or that"), seed(2, "salt and pepper")],
            "or",
            &[1],
        ),
        case(
            "near_and_not_are_plain_terms",
            vec![seed(1, "not near here")],
            "not near",
            &[1],
        ),
        case(
            "numbers_are_terms",
            vec![seed(1, "build 2024 done"), seed(2, "build 2025 done")],
            "2024",
            &[1],
        ),
        snippet_case,
        limited,
        case(
            "result_list_is_capped_at_the_hit_limit",
            flooded(hits + 7),
            "flood marker",
            &(8..=hits + 7).rev().collect::<Vec<_>>(),
        ),
        case(
            "candidate_window_drops_older_stronger_matches",
            flood,
            "flood",
            &((overflow - hits + 1)..=overflow).rev().collect::<Vec<_>>(),
        ),
    ]
}

pub fn documents_of(seeds: &[Seed]) -> Vec<SearchDocument> {
    seeds
        .iter()
        .map(|seed| SearchDocument::from_message(seed.account, &message_of(seed)))
        .collect()
}

pub async fn assert_search_case<I: SearchIndex>(index: &mut I, case: &SearchCase) {
    for round in &case.rounds {
        index
            .upsert(documents_of(round))
            .await
            .unwrap_or_else(|error| panic!("{}: upsert failed: {error:?}", case.name));
    }
    let query = query_of(case, case.limit);
    let hits = run_search(index, &query)
        .await
        .unwrap_or_else(|error| panic!("{}: search failed: {error:?}", case.name));
    let found: Vec<u128> = hits
        .iter()
        .map(|hit| {
            (0..=u128::try_from(SEARCH_CANDIDATE_LIMIT).expect("cap") + 10)
                .find(|index| message_id(*index) == hit.message_id)
                .unwrap_or_else(|| panic!("{}: unknown hit {:?}", case.name, hit.message_id))
        })
        .collect();
    assert_eq!(found, case.expected, "{}", case.name);
    if let Some(expected) = &case.top_snippet {
        let spans: Vec<(String, bool)> = hits[0]
            .snippet
            .iter()
            .map(|span| (span.text.clone(), span.matched))
            .collect();
        let expected: Vec<(String, bool)> = expected
            .iter()
            .map(|(text, matched)| ((*text).to_owned(), *matched))
            .collect();
        assert_eq!(spans, expected, "{}", case.name);
    }
}
