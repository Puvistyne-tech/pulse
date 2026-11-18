use crate::api::{ApiError, InputField};
use crate::auth::{SignInRequest, SignUpRequest};
use crate::campus::{CreateCampusRequest, CreateRoomRequest};

pub const EMAIL_MAX_CHARS: usize = 254;
pub const DISPLAY_NAME_MAX_CHARS: usize = 64;
pub const PASSWORD_MIN_CHARS: usize = 12;
pub const PASSWORD_MAX_CHARS: usize = 128;
pub const SLUG_MIN_CHARS: usize = 3;
pub const SLUG_MAX_CHARS: usize = 32;

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
    if valid { Ok(()) } else { Err(InputField::Slug) }
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

impl Validate for CreateRoomRequest {
    fn validate(&self) -> Result<(), ApiError> {
        validate_slug(&self.slug).map_err(invalid)
    }
}
