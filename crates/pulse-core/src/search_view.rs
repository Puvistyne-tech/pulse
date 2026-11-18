use pulse_protocol::{
    ApiError, ApiErrorCode, InboxStep, MessageId, SearchHit, SearchSnapshot, normalize_search_query,
};

use crate::store::StoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchIntent {
    Toggled,
    QueryChanged(String),
    SelectionMoved(InboxStep),
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct SearchTracker {
    open: bool,
    query: String,
    generation: u64,
    hits: Vec<SearchHit>,
    selected: Option<usize>,
    searching: bool,
    error: Option<ApiError>,
}

impl SearchTracker {
    pub(crate) fn toggle(&mut self) {
        let next = !self.open;
        self.reset();
        self.open = next;
    }

    pub(crate) fn close(&mut self) {
        self.reset();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn change_query(&mut self, query: String, generation: u64) -> Option<Vec<String>> {
        if !self.open {
            return None;
        }
        let terms = normalize_search_query(&query);
        self.query = query;
        self.generation = generation;
        self.hits.clear();
        self.selected = None;
        self.error = None;
        self.searching = !terms.is_empty();
        (!terms.is_empty()).then_some(terms)
    }

    pub(crate) fn complete(&mut self, generation: u64, result: Result<Vec<SearchHit>, StoreError>) {
        if !self.open || !self.searching || generation != self.generation {
            return;
        }
        self.searching = false;
        match result {
            Ok(hits) => {
                self.selected = (!hits.is_empty()).then_some(0);
                self.hits = hits;
            }
            Err(_) => {
                self.hits.clear();
                self.selected = None;
                self.error = Some(ApiError::new(ApiErrorCode::Unavailable));
            }
        }
    }

    pub(crate) fn step_selection(&mut self, step: InboxStep) {
        if !self.open {
            return;
        }
        let Some(last) = self.hits.len().checked_sub(1) else {
            return;
        };
        self.selected = Some(match (self.selected, step) {
            (Some(index), InboxStep::Next) => (index + 1).min(last),
            (Some(index), InboxStep::Previous) => index.saturating_sub(1),
            (None, _) => 0,
        });
    }

    pub(crate) fn hit(&self, message_id: MessageId) -> Option<&SearchHit> {
        self.hits.iter().find(|hit| hit.message_id == message_id)
    }

    pub(crate) fn selected_message_id(&self) -> Option<MessageId> {
        self.selected
            .and_then(|index| self.hits.get(index))
            .map(|hit| hit.message_id)
    }

    pub(crate) fn snapshot(&self) -> SearchSnapshot {
        SearchSnapshot {
            open: self.open,
            query: self.query.clone(),
            hits: self.hits.clone(),
            selected_index: self
                .selected
                .map(|index| u32::try_from(index).unwrap_or(u32::MAX)),
            searching: self.searching,
            error: self.error,
        }
    }
}
