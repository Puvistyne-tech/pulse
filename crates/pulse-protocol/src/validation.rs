use crate::api::{ApiError, InputField};
use crate::auth::{SignInRequest, SignUpRequest};
use crate::campus::{
    CreateCampusRequest, CreateRoomRequest, INVITE_CODE_SEPARATOR, InviteCode, JoinCampusRequest,
};
use crate::ids::{AccountId, AttachmentId, CampusId, RoomId};
use crate::media::{
    ATTACHMENT_MAX_BYTES, CONTENT_TYPE_MAX_CHARS, CreateUploadRequest, FILE_NAME_MAX_CHARS,
    MESSAGE_ATTACHMENTS_MAX,
};
use crate::member::{ASSIGNABLE_ROLES, AddMemberRequest, UpdateMemberRequest};
use crate::messaging::SendMessage;
use crate::note::{NOTE_MEMBERS_MAX, NOTE_SLUG_PREFIX};
use crate::rbac::Role;
use crate::reaction::React;
use crate::search::{SEARCH_QUERY_MAX_CHARS, SEARCH_TERMS_MAX, search_terms};
use crate::shelf::{REMIND_AT_HORIZON_MS, REMIND_AT_PAST_TOLERANCE_MS, ShelfAction, UpdateShelf};
use crate::steward::STEWARD_ROOMS_MAX;
use crate::thread::SetTopicTitle;

pub const EMAIL_MAX_CHARS: usize = 254;
pub const DISPLAY_NAME_MAX_CHARS: usize = 64;
pub const PASSWORD_MIN_CHARS: usize = 12;
pub const PASSWORD_MAX_CHARS: usize = 128;
pub const SLUG_MIN_CHARS: usize = 3;
pub const SLUG_MAX_CHARS: usize = 32;
pub const MESSAGE_BODY_MAX_CHARS: usize = 4000;
pub const TOPIC_TITLE_MAX_CHARS: usize = 120;
pub const REACTION_EMOJI_MAX_CHARS: usize = 16;
pub const INVITE_SECRET_CHARS: usize = 43;

pub fn normalize_email(raw: &str) -> Result<String, InputField> {
    let email = raw.trim().to_lowercase();
    let mut parts = email.split('@');
    let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) else {
        return Err(InputField::Email);
    };
    let domain_is_valid =
        domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.');
    let valid = !local.is_empty()
        && domain_is_valid
        && !email.chars().any(char::is_whitespace)
        && email.chars().count() <= EMAIL_MAX_CHARS;
    if valid {
        Ok(email)
    } else {
        Err(InputField::Email)
    }
}

pub fn normalize_display_name(raw: &str) -> Result<String, InputField> {
    let trimmed = raw.trim();
    let length = trimmed.chars().count();
    let valid =
        (1..=DISPLAY_NAME_MAX_CHARS).contains(&length) && !trimmed.chars().any(char::is_control);
    if valid {
        Ok(trimmed.to_owned())
    } else {
        Err(InputField::DisplayName)
    }
}

pub fn normalize_message_body(raw: &str) -> Result<String, InputField> {
    let trimmed = raw.trim();
    let length = trimmed.chars().count();
    let valid = (1..=MESSAGE_BODY_MAX_CHARS).contains(&length)
        && !trimmed
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'));
    if valid {
        Ok(trimmed.to_owned())
    } else {
        Err(InputField::MessageBody)
    }
}

pub fn normalize_topic_title(raw: &str) -> Result<Option<String>, InputField> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let valid =
        trimmed.chars().count() <= TOPIC_TITLE_MAX_CHARS && !trimmed.chars().any(char::is_control);
    if valid {
        Ok(Some(trimmed.to_owned()))
    } else {
        Err(InputField::TopicTitle)
    }
}

pub fn normalize_reaction_emoji(raw: &str) -> Result<String, InputField> {
    let trimmed = raw.trim();
    let length = trimmed.chars().count();
    let valid = (1..=REACTION_EMOJI_MAX_CHARS).contains(&length)
        && trimmed.chars().all(|character| {
            !character.is_ascii() && !character.is_whitespace() && !character.is_control()
        });
    if valid {
        Ok(trimmed.to_owned())
    } else {
        Err(InputField::ReactionEmoji)
    }
}

pub fn normalize_file_name(raw: &str) -> Result<String, InputField> {
    let name = raw.rsplit(['/', '\\']).next().unwrap_or_default().trim();
    let valid = !name.is_empty()
        && name != "."
        && name != ".."
        && !name.chars().any(char::is_control)
        && name.chars().count() <= FILE_NAME_MAX_CHARS;
    if valid {
        Ok(name.to_owned())
    } else {
        Err(InputField::FileName)
    }
}

fn is_token_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || "!#$&^_.+-".contains(character)
}

pub fn normalize_content_type(raw: &str) -> Result<String, InputField> {
    let essence = raw
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_lowercase();
    let valid = essence.chars().count() <= CONTENT_TYPE_MAX_CHARS
        && essence.split_once('/').is_some_and(|(major, minor)| {
            !major.is_empty()
                && !minor.is_empty()
                && major.chars().all(is_token_char)
                && minor.chars().all(is_token_char)
        });
    if valid {
        Ok(essence)
    } else {
        Err(InputField::ContentType)
    }
}

pub fn validate_byte_size(byte_size: u64) -> Result<u64, InputField> {
    if (1..=ATTACHMENT_MAX_BYTES).contains(&byte_size) {
        Ok(byte_size)
    } else {
        Err(InputField::ByteSize)
    }
}

pub fn normalize_attachment_ids(ids: &[AttachmentId]) -> Result<Vec<AttachmentId>, InputField> {
    let mut unique: Vec<AttachmentId> = Vec::with_capacity(ids.len());
    for id in ids {
        if !unique.contains(id) {
            unique.push(*id);
        }
    }
    if unique.len() <= MESSAGE_ATTACHMENTS_MAX {
        Ok(unique)
    } else {
        Err(InputField::Attachments)
    }
}

pub fn normalize_room_ids(room_ids: &[RoomId]) -> Result<Vec<RoomId>, InputField> {
    let mut sorted = room_ids.to_vec();
    sorted.sort();
    sorted.dedup();
    if sorted.len() <= STEWARD_ROOMS_MAX {
        Ok(sorted)
    } else {
        Err(InputField::Rooms)
    }
}

pub fn normalize_message_content(
    body: &str,
    attachment_ids: &[AttachmentId],
) -> Result<(String, Vec<AttachmentId>), InputField> {
    let attachments = normalize_attachment_ids(attachment_ids)?;
    if body.trim().is_empty() && !attachments.is_empty() {
        return Ok((String::new(), attachments));
    }
    normalize_message_body(body).map(|normalized| (normalized, attachments))
}

pub fn validate_remind_at(remind_at_ms: u64, now_ms: u64) -> Result<u64, InputField> {
    let earliest = now_ms.saturating_sub(REMIND_AT_PAST_TOLERANCE_MS);
    let latest = now_ms.saturating_add(REMIND_AT_HORIZON_MS);
    if (earliest..=latest).contains(&remind_at_ms) {
        Ok(remind_at_ms)
    } else {
        Err(InputField::RemindAt)
    }
}

pub fn normalize_search_query(raw: &str) -> Vec<String> {
    let clipped: String = raw.chars().take(SEARCH_QUERY_MAX_CHARS).collect();
    let mut terms: Vec<String> = Vec::new();
    for term in search_terms(&clipped) {
        if !terms.contains(&term) {
            terms.push(term);
        }
    }
    terms.truncate(SEARCH_TERMS_MAX);
    terms
}

pub fn validate_display_name(raw: &str) -> Result<(), InputField> {
    normalize_display_name(raw).map(|_| ())
}

pub fn validate_password(raw: &str) -> Result<(), InputField> {
    if (PASSWORD_MIN_CHARS..=PASSWORD_MAX_CHARS).contains(&raw.chars().count()) {
        Ok(())
    } else {
        Err(InputField::Password)
    }
}

pub fn validate_slug(raw: &str) -> Result<(), InputField> {
    let length = raw.chars().count();
    let is_alphanumeric =
        |character: char| character.is_ascii_lowercase() || character.is_ascii_digit();
    let boundaries_valid = raw.chars().next().is_some_and(is_alphanumeric)
        && raw.chars().next_back().is_some_and(is_alphanumeric);
    let valid = (SLUG_MIN_CHARS..=SLUG_MAX_CHARS).contains(&length)
        && boundaries_valid
        && raw
            .chars()
            .all(|character| is_alphanumeric(character) || character == '-');
    if valid && !raw.starts_with(NOTE_SLUG_PREFIX) {
        Ok(())
    } else {
        Err(InputField::Slug)
    }
}

pub fn normalize_invite_code(raw: &str) -> Result<InviteCode, InputField> {
    let (campus, secret) = raw
        .trim()
        .split_once(INVITE_CODE_SEPARATOR)
        .ok_or(InputField::InviteCode)?;
    let campus_id: CampusId = campus.parse().map_err(|_| InputField::InviteCode)?;
    let secret_valid = secret.len() == INVITE_SECRET_CHARS
        && secret
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'));
    if secret_valid {
        Ok(InviteCode {
            campus_id,
            secret: secret.to_owned(),
        })
    } else {
        Err(InputField::InviteCode)
    }
}

pub fn normalize_note_members(
    requested: &[AccountId],
    starter: AccountId,
) -> Result<Vec<AccountId>, InputField> {
    let mut others: Vec<AccountId> = requested
        .iter()
        .copied()
        .filter(|account_id| *account_id != starter)
        .collect();
    others.sort_by_key(|account_id| account_id.to_string());
    others.dedup();
    if others.is_empty() || others.len() > NOTE_MEMBERS_MAX {
        Err(InputField::NoteMembers)
    } else {
        Ok(others)
    }
}

pub fn validate_assignable_role(role: Role) -> Result<(), InputField> {
    if ASSIGNABLE_ROLES.contains(&role) {
        Ok(())
    } else {
        Err(InputField::Role)
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), ApiError>;
}

fn invalid(field: InputField) -> ApiError {
    ApiError::invalid(field)
}

impl Validate for SignUpRequest {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_email(&self.email).map_err(invalid)?;
        validate_display_name(&self.display_name).map_err(invalid)?;
        validate_password(&self.password).map_err(invalid)
    }
}

impl Validate for SignInRequest {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_email(&self.email).map_err(invalid)?;
        if (1..=PASSWORD_MAX_CHARS).contains(&self.password.chars().count()) {
            Ok(())
        } else {
            Err(ApiError::invalid(InputField::Password))
        }
    }
}

impl Validate for CreateCampusRequest {
    fn validate(&self) -> Result<(), ApiError> {
        validate_slug(&self.slug).map_err(invalid)?;
        validate_display_name(&self.display_name).map_err(invalid)
    }
}

impl Validate for JoinCampusRequest {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_invite_code(&self.code)
            .map(|_| ())
            .map_err(invalid)
    }
}

impl Validate for AddMemberRequest {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_email(&self.email).map_err(invalid)?;
        validate_assignable_role(self.role).map_err(invalid)
    }
}

impl Validate for UpdateMemberRequest {
    fn validate(&self) -> Result<(), ApiError> {
        validate_assignable_role(self.role).map_err(invalid)
    }
}

impl Validate for CreateRoomRequest {
    fn validate(&self) -> Result<(), ApiError> {
        validate_slug(&self.slug).map_err(invalid)
    }
}

impl Validate for SendMessage {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_message_content(&self.body, &self.attachment_ids)
            .map(|_| ())
            .map_err(invalid)
    }
}

impl Validate for React {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_reaction_emoji(&self.emoji)
            .map(|_| ())
            .map_err(invalid)
    }
}

impl Validate for SetTopicTitle {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_topic_title(&self.topic_title)
            .map(|_| ())
            .map_err(invalid)
    }
}

impl Validate for CreateUploadRequest {
    fn validate(&self) -> Result<(), ApiError> {
        normalize_file_name(&self.file_name).map_err(invalid)?;
        normalize_content_type(&self.content_type).map_err(invalid)?;
        validate_byte_size(self.byte_size)
            .map(|_| ())
            .map_err(invalid)
    }
}

impl Validate for UpdateShelf {
    fn validate(&self) -> Result<(), ApiError> {
        let valid = match self.action {
            ShelfAction::Reschedule => self.remind_at_ms.is_some(),
            ShelfAction::Complete | ShelfAction::Reopen | ShelfAction::Remove => {
                self.remind_at_ms.is_none()
            }
            ShelfAction::Save => true,
        };
        if valid {
            Ok(())
        } else {
            Err(ApiError::invalid(InputField::RemindAt))
        }
    }
}
