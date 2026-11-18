use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hmac::{Hmac, Mac};
use pulse_protocol::{CampusId, LoungeId};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const HEADER: &str = r#"{"alg":"HS256","typ":"JWT"}"#;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoGrant {
    pub room: String,
    pub room_join: bool,
    pub can_publish: bool,
    pub can_subscribe: bool,
    pub can_publish_data: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveKitClaims {
    pub iss: String,
    pub sub: String,
    pub name: String,
    pub nbf: u64,
    pub exp: u64,
    pub video: VideoGrant,
}

pub fn livekit_room_name(campus_id: CampusId, lounge_id: LoungeId) -> String {
    format!("pulse-{campus_id}-{lounge_id}")
}

pub fn sign_livekit_token(claims: &LiveKitClaims, api_secret: &str) -> String {
    let payload = serde_json::to_string(claims).unwrap_or_default();
    let signing_input = format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(HEADER),
        URL_SAFE_NO_PAD.encode(payload)
    );
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()) else {
        return String::new();
    };
    mac.update(signing_input.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    format!("{signing_input}.{signature}")
}
