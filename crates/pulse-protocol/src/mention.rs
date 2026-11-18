use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::ids::AccountId;
use crate::messaging::MentionRef;

pub const MESSAGE_MENTIONS_MAX: usize = 16;

const TOKEN_OPEN: &str = "<@";
const TOKEN_CLOSE: char = '>';
const UUID_TEXT_LEN: usize = 36;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum SegmentKind {
    Text,
    Mention,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct BodySegment {
    pub kind: SegmentKind,
    pub text: String,
    pub account_id: Option<AccountId>,
}

enum Piece<'a> {
    Text(&'a str),
    Token { raw: &'a str, account_id: AccountId },
}

pub fn mention_token(account_id: AccountId) -> String {
    format!("{TOKEN_OPEN}{account_id}{TOKEN_CLOSE}")
}

fn token_at(rest: &str) -> Option<(usize, AccountId)> {
    let inner = rest.strip_prefix(TOKEN_OPEN)?;
    let candidate = inner.get(..UUID_TEXT_LEN)?;
    inner[UUID_TEXT_LEN..].strip_prefix(TOKEN_CLOSE)?;
    let uuid = Uuid::parse_str(candidate).ok()?;
    Some((
        TOKEN_OPEN.len() + UUID_TEXT_LEN + TOKEN_CLOSE.len_utf8(),
        AccountId(uuid),
    ))
}

fn scan(body: &str) -> Vec<Piece<'_>> {
    let mut pieces = Vec::new();
    let mut text_start = 0;
    let mut cursor = 0;
    while let Some(offset) = body[cursor..].find(TOKEN_OPEN) {
        let start = cursor + offset;
        match token_at(&body[start..]) {
            Some((length, account_id)) => {
                if start > text_start {
                    pieces.push(Piece::Text(&body[text_start..start]));
                }
                pieces.push(Piece::Token {
                    raw: &body[start..start + length],
                    account_id,
                });
                cursor = start + length;
                text_start = cursor;
            }
            None => cursor = start + TOKEN_OPEN.len(),
        }
    }
    if text_start < body.len() {
        pieces.push(Piece::Text(&body[text_start..]));
    }
    pieces
}

pub fn extract_mentions(body: &str) -> Vec<AccountId> {
    let mut found: Vec<AccountId> = Vec::new();
    for piece in scan(body) {
        if let Piece::Token { account_id, .. } = piece
            && !found.contains(&account_id)
            && found.len() < MESSAGE_MENTIONS_MAX
        {
            found.push(account_id);
        }
    }
    found
}

fn push_text(segments: &mut Vec<BodySegment>, text: &str) {
    if let Some(last) = segments
        .last_mut()
        .filter(|last| last.kind == SegmentKind::Text)
    {
        last.text.push_str(text);
    } else {
        segments.push(BodySegment {
            kind: SegmentKind::Text,
            text: text.to_owned(),
            account_id: None,
        });
    }
}

pub fn body_segments(body: &str, mentions: &[MentionRef]) -> Vec<BodySegment> {
    let mut segments = Vec::new();
    for piece in scan(body) {
        match piece {
            Piece::Text(text) => push_text(&mut segments, text),
            Piece::Token { raw, account_id } => {
                match mentions
                    .iter()
                    .find(|mention| mention.account_id == account_id)
                {
                    Some(mention) => segments.push(BodySegment {
                        kind: SegmentKind::Mention,
                        text: format!("@{}", mention.display_name),
                        account_id: Some(account_id),
                    }),
                    None => push_text(&mut segments, raw),
                }
            }
        }
    }
    segments
}
