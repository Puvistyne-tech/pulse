pub const SEARCH_QUERY_MAX_CHARS: usize = 200;
pub const SEARCH_TERMS_MAX: usize = 8;
pub const SEARCH_TERM_MAX_CHARS: usize = 64;
pub const SEARCH_HITS_LIMIT: usize = 50;
pub const SEARCH_CANDIDATE_LIMIT: usize = 500;
pub const SEARCH_SNIPPET_CHARS: usize = 120;

pub fn search_terms(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|character: char| !character.is_alphanumeric())
        .filter(|term| !term.is_empty())
        .map(|term| term.chars().take(SEARCH_TERM_MAX_CHARS).collect())
        .collect()
}
