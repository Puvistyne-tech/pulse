use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pulse_protocol::{AttachmentId, AttachmentKind, CampusId, RoomId};
use reqwest::header::CONTENT_LENGTH;
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use url::Url;

const UPLOAD_TTL: Duration = Duration::from_secs(15 * 60);
const DOWNLOAD_TTL: Duration = Duration::from_secs(10 * 60);
const MAINTENANCE_TTL: Duration = Duration::from_secs(60);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const CONTENT_DISPOSITION_PARAMETER: &str = "response-content-disposition";
const CONTENT_TYPE_HEADER: &str = "content-type";

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("invalid storage endpoint: {0}")]
    Endpoint(#[from] url::ParseError),
    #[error("invalid storage bucket: {0}")]
    Bucket(#[from] rusty_s3::BucketError),
    #[error("storage request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("storage answered with status {0}")]
    Status(u16),
    #[error("storage is not configured")]
    Unconfigured,
}

#[derive(Clone)]
pub struct StorageConfig {
    pub endpoint: String,
    pub public_endpoint: String,
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
}

const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:9000";

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            endpoint: DEFAULT_ENDPOINT.to_owned(),
            public_endpoint: DEFAULT_ENDPOINT.to_owned(),
            bucket: "pulse-attachments".to_owned(),
            region: "us-east-1".to_owned(),
            access_key: "pulse".to_owned(),
            secret_key: "pulse-dev-secret".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectLocation {
    pub campus_id: CampusId,
    pub room_id: RoomId,
    pub attachment_id: AttachmentId,
}

impl ObjectLocation {
    pub fn key(&self) -> String {
        format!(
            "campuses/{}/rooms/{}/attachments/{}",
            self.campus_id, self.room_id, self.attachment_id
        )
    }
}

struct Inner {
    internal: Bucket,
    public: Bucket,
    credentials: Credentials,
    http: reqwest::Client,
}

#[derive(Clone)]
pub struct Storage {
    inner: Option<Arc<Inner>>,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
        })
}

fn expires_at_ms(ttl: Duration) -> u64 {
    now_ms().saturating_add(u64::try_from(ttl.as_millis()).unwrap_or(u64::MAX))
}

fn percent_encode(text: &str) -> String {
    text.bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-' => {
                char::from(byte).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

fn disposition(file_name: &str, kind: AttachmentKind) -> String {
    let mode = match kind {
        AttachmentKind::Image | AttachmentKind::Video | AttachmentKind::Audio => "inline",
        AttachmentKind::File => "attachment",
    };
    format!("{mode}; filename*=UTF-8''{}", percent_encode(file_name))
}

fn bucket(endpoint: &str, config: &StorageConfig) -> Result<Bucket, StorageError> {
    Ok(Bucket::new(
        Url::parse(endpoint)?,
        UrlStyle::Path,
        config.bucket.clone(),
        config.region.clone(),
    )?)
}

impl Storage {
    pub fn new(config: StorageConfig) -> Result<Self, StorageError> {
        let inner = Inner {
            internal: bucket(&config.endpoint, &config)?,
            public: bucket(&config.public_endpoint, &config)?,
            credentials: Credentials::new(config.access_key.clone(), config.secret_key.clone()),
            http: reqwest::Client::builder()
                .timeout(REQUEST_TIMEOUT)
                .build()?,
        };
        Ok(Self {
            inner: Some(Arc::new(inner)),
        })
    }

    pub fn unconfigured() -> Self {
        Self { inner: None }
    }

    fn inner(&self) -> Result<&Inner, StorageError> {
        self.inner.as_deref().ok_or(StorageError::Unconfigured)
    }

    pub async fn ensure_bucket(&self) -> Result<(), StorageError> {
        let inner = self.inner()?;
        let url = inner
            .internal
            .create_bucket(&inner.credentials)
            .sign(MAINTENANCE_TTL);
        let status = inner.http.put(url).send().await?.status();
        if status.is_success() || status.as_u16() == 409 {
            Ok(())
        } else {
            Err(StorageError::Status(status.as_u16()))
        }
    }

    pub fn presign_put(&self, location: &ObjectLocation, content_type: &str) -> (String, u64) {
        let Ok(inner) = self.inner() else {
            return (String::new(), 0);
        };
        let key = location.key();
        let mut action = inner.public.put_object(Some(&inner.credentials), &key);
        action
            .headers_mut()
            .insert(CONTENT_TYPE_HEADER, content_type.to_owned());
        (
            action.sign(UPLOAD_TTL).to_string(),
            expires_at_ms(UPLOAD_TTL),
        )
    }

    pub fn presign_get(
        &self,
        location: &ObjectLocation,
        file_name: &str,
        kind: AttachmentKind,
    ) -> (String, u64) {
        let Ok(inner) = self.inner() else {
            return (String::new(), 0);
        };
        let key = location.key();
        let mut action = inner.public.get_object(Some(&inner.credentials), &key);
        action
            .query_mut()
            .insert(CONTENT_DISPOSITION_PARAMETER, disposition(file_name, kind));
        (
            action.sign(DOWNLOAD_TTL).to_string(),
            expires_at_ms(DOWNLOAD_TTL),
        )
    }

    pub async fn head_size(&self, location: &ObjectLocation) -> Result<Option<u64>, StorageError> {
        let inner = self.inner()?;
        let key = location.key();
        let url = inner
            .internal
            .head_object(Some(&inner.credentials), &key)
            .sign(MAINTENANCE_TTL);
        let response = inner.http.head(url).send().await?;
        let status = response.status();
        if status.as_u16() == 404 {
            return Ok(None);
        }
        if !status.is_success() {
            return Err(StorageError::Status(status.as_u16()));
        }
        Ok(response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|value| value.to_str().ok())
            .and_then(|text| text.parse().ok()))
    }

    pub async fn delete(&self, location: &ObjectLocation) -> Result<(), StorageError> {
        let inner = self.inner()?;
        let key = location.key();
        let url = inner
            .internal
            .delete_object(Some(&inner.credentials), &key)
            .sign(MAINTENANCE_TTL);
        let status = inner.http.delete(url).send().await?.status();
        if status.is_success() || status.as_u16() == 404 {
            Ok(())
        } else {
            Err(StorageError::Status(status.as_u16()))
        }
    }
}
