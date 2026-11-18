use crate::mention::{Piece, scan};

pub const MESSAGE_LINKS_MAX: usize = 16;
pub const LINK_MAX_CHARS: usize = 2048;

const SCHEMES: [&str; 2] = ["http://", "https://"];
const TRAILING_PUNCTUATION: [char; 10] = ['.', ',', ';', ':', '!', '?', '\'', '"', ']', '}'];

pub(crate) struct LinkSpan {
    pub raw_len: usize,
    pub url_len: Option<usize>,
}

pub(crate) fn next_scheme(text: &str) -> Option<usize> {
    SCHEMES.iter().filter_map(|scheme| text.find(scheme)).min()
}

fn is_link_end(character: char) -> bool {
    character.is_whitespace() || matches!(character, '<' | '>' | '"')
}

fn trim_trailing(raw: &str) -> &str {
    let mut trimmed = raw;
    while let Some(last) = trimmed.chars().next_back() {
        let unbalanced_paren =
            last == ')' && trimmed.matches(')').count() > trimmed.matches('(').count();
        if !TRAILING_PUNCTUATION.contains(&last) && !unbalanced_paren {
            break;
        }
        trimmed = &trimmed[..trimmed.len() - last.len_utf8()];
    }
    trimmed
}

pub(crate) fn link_span(rest: &str) -> Option<LinkSpan> {
    let scheme = SCHEMES.iter().find(|scheme| rest.starts_with(**scheme))?;
    let raw_len = rest.find(is_link_end).unwrap_or(rest.len());
    let url = trim_trailing(&rest[..raw_len]);
    let valid = url.len() > scheme.len() && url.chars().count() <= LINK_MAX_CHARS;
    Some(LinkSpan {
        raw_len,
        url_len: valid.then_some(url.len()),
    })
}

pub fn extract_links(body: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for piece in scan(body) {
        if let Piece::Link(url) = piece
            && !found.iter().any(|existing| existing == url)
            && found.len() < MESSAGE_LINKS_MAX
        {
            found.push(url.to_owned());
        }
    }
    found
}
