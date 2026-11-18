use core::future::Future;
use std::collections::HashMap;

use pulse_protocol::{
    AccountId, CampusId, HighlightSpan, MessageId, RoomId, RoomMessage, SEARCH_HITS_LIMIT,
    SEARCH_SNIPPET_CHARS, SearchHit, body_segments, search_terms,
};
use serde::{Deserialize, Serialize};

use crate::store::StoreError;

const TITLE_WEIGHT: u32 = 4;
const AUTHOR_WEIGHT: u32 = 2;
const BODY_WEIGHT: u32 = 1;
const EXACT_MULTIPLIER: u32 = 2;
const PREFIX_MULTIPLIER: u32 = 1;
const SNIPPET_LEAD_DIVISOR: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchDocument {
    pub account_id: AccountId,
    pub campus_id: CampusId,
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub thread_root_id: Option<MessageId>,
    pub author_display_name: String,
    pub topic_title: Option<String>,
    pub text: String,
    pub terms: Vec<String>,
}

impl SearchDocument {
    pub fn from_message(account_id: AccountId, message: &RoomMessage) -> Self {
        let text = body_segments(&message.body, &message.mentions)
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<String>();
        let combined = format!(
            "{} {} {}",
            message.topic_title.as_deref().unwrap_or_default(),
            message.author_display_name,
            text
        );
        let mut terms = search_terms(&combined);
        terms.sort();
        terms.dedup();
        Self {
            account_id,
            campus_id: message.campus_id,
            room_id: message.room_id,
            message_id: message.message_id,
            thread_root_id: message.thread_root_id,
            author_display_name: message.author_display_name.clone(),
            topic_title: message.topic_title.clone(),
            text,
            terms,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchQuery {
    pub account_id: AccountId,
    pub campus_id: CampusId,
    pub terms: Vec<String>,
    pub limit: usize,
}

pub trait SearchIndex {
    fn upsert(
        &mut self,
        documents: Vec<SearchDocument>,
    ) -> impl Future<Output = Result<(), StoreError>>;
    fn candidates(
        &self,
        query: &SearchQuery,
    ) -> impl Future<Output = Result<Vec<SearchDocument>, StoreError>>;
}

fn term_matches(token: &str, term: &str, prefix_allowed: bool) -> Option<u32> {
    if token == term {
        Some(EXACT_MULTIPLIER)
    } else if prefix_allowed && token.starts_with(term) {
        Some(PREFIX_MULTIPLIER)
    } else {
        None
    }
}

fn field_score(tokens: &[String], term: &str, prefix_allowed: bool, weight: u32) -> Option<u32> {
    tokens
        .iter()
        .filter_map(|token| term_matches(token, term, prefix_allowed))
        .max()
        .map(|multiplier| weight * multiplier)
}

fn score_document(query: &SearchQuery, document: &SearchDocument) -> Option<u32> {
    let title = search_terms(document.topic_title.as_deref().unwrap_or_default());
    let author = search_terms(&document.author_display_name);
    let body = search_terms(&document.text);
    let last = query.terms.len().checked_sub(1)?;
    let mut total = 0u32;
    for (index, term) in query.terms.iter().enumerate() {
        let prefix_allowed = index == last;
        let term_score = [
            field_score(&title, term, prefix_allowed, TITLE_WEIGHT),
            field_score(&author, term, prefix_allowed, AUTHOR_WEIGHT),
            field_score(&body, term, prefix_allowed, BODY_WEIGHT),
        ]
        .into_iter()
        .flatten()
        .sum::<u32>();
        if term_score == 0 {
            return None;
        }
        total = total.saturating_add(term_score);
    }
    Some(total)
}

fn matched_flags(text: &str, query: &SearchQuery) -> Vec<bool> {
    let characters = text.chars().collect::<Vec<_>>();
    let mut flags = vec![false; characters.len()];
    let last = query.terms.len().saturating_sub(1);
    let mut start = 0;
    while start < characters.len() {
        if !characters[start].is_alphanumeric() {
            start += 1;
            continue;
        }
        let mut end = start;
        while end < characters.len() && characters[end].is_alphanumeric() {
            end += 1;
        }
        let run = characters[start..end].iter().collect::<String>();
        let tokens = search_terms(&run);
        let hit = query.terms.iter().enumerate().any(|(index, term)| {
            tokens
                .iter()
                .any(|token| term_matches(token, term, index == last).is_some())
        });
        if hit {
            flags[start..end].iter_mut().for_each(|flag| *flag = true);
        }
        start = end;
    }
    flags
}

fn build_snippet(text: &str, query: &SearchQuery) -> Vec<HighlightSpan> {
    let characters = text.chars().collect::<Vec<_>>();
    let flags = matched_flags(text, query);
    let first_match = flags.iter().position(|flag| *flag).unwrap_or(0);
    let window_start = first_match.saturating_sub(SEARCH_SNIPPET_CHARS / SNIPPET_LEAD_DIVISOR);
    let window_end = (window_start + SEARCH_SNIPPET_CHARS).min(characters.len());
    let mut spans: Vec<HighlightSpan> = Vec::new();
    for index in window_start..window_end {
        let character = characters[index];
        let matched = flags[index];
        match spans.last_mut() {
            Some(span) if span.matched == matched => span.text.push(character),
            _ => spans.push(HighlightSpan {
                text: character.to_string(),
                matched,
            }),
        }
    }
    spans
}

pub fn rank_hits(query: &SearchQuery, candidates: Vec<SearchDocument>) -> Vec<SearchHit> {
    let mut scored = candidates
        .into_iter()
        .filter(|document| {
            document.account_id == query.account_id && document.campus_id == query.campus_id
        })
        .filter_map(|document| score_document(query, &document).map(|score| (score, document)))
        .collect::<Vec<_>>();
    scored.sort_by(|(left_score, left), (right_score, right)| {
        right_score
            .cmp(left_score)
            .then(right.message_id.cmp(&left.message_id))
    });
    scored.truncate(SEARCH_HITS_LIMIT);
    scored
        .into_iter()
        .map(|(score, document)| SearchHit {
            room_id: document.room_id,
            message_id: document.message_id,
            thread_root_id: document.thread_root_id,
            author_display_name: document.author_display_name.clone(),
            topic_title: document.topic_title.clone(),
            snippet: build_snippet(&document.text, query),
            score,
        })
        .collect()
}

pub async fn run_search<I: SearchIndex>(
    index: &I,
    query: &SearchQuery,
) -> Result<Vec<SearchHit>, StoreError> {
    let candidates = index.candidates(query).await?;
    Ok(rank_hits(query, candidates))
}

#[derive(Debug, Default)]
pub struct MemorySearchIndex {
    documents: HashMap<(AccountId, CampusId, MessageId), SearchDocument>,
}

fn terms_match(document: &SearchDocument, terms: &[String]) -> bool {
    let Some(last) = terms.len().checked_sub(1) else {
        return false;
    };
    terms.iter().enumerate().all(|(index, term)| {
        if index == last {
            document.terms.iter().any(|known| known.starts_with(term))
        } else {
            document.terms.iter().any(|known| known == term)
        }
    })
}

impl SearchIndex for MemorySearchIndex {
    async fn upsert(&mut self, documents: Vec<SearchDocument>) -> Result<(), StoreError> {
        for document in documents {
            self.documents.insert(
                (document.account_id, document.campus_id, document.message_id),
                document,
            );
        }
        Ok(())
    }

    async fn candidates(&self, query: &SearchQuery) -> Result<Vec<SearchDocument>, StoreError> {
        let mut found = self
            .documents
            .values()
            .filter(|document| {
                document.account_id == query.account_id
                    && document.campus_id == query.campus_id
                    && terms_match(document, &query.terms)
            })
            .cloned()
            .collect::<Vec<_>>();
        found.sort_by_key(|document| std::cmp::Reverse(document.message_id));
        found.truncate(query.limit);
        Ok(found)
    }
}
