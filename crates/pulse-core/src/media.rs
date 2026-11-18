use core::future::Future;
use std::collections::{BTreeMap, BTreeSet};

use pulse_protocol::{
    ApiError, ApiErrorCode, Attachment, AttachmentId, AttachmentKind, AttachmentView, AuthToken,
    CampusId, ClientNonce, CreateUploadRequest, DOWNLOAD_TICKETS_MAX, DownloadTicket,
    DownloadTickets, DownloadTicketsRequest, DraftAttachmentView, DraftId, InputField, MessageId,
    RoomId, RoomMessage, SyncVault, UploadState, UploadTicket, VaultEntry, VaultItem,
    VaultRejected, VaultRejection, VaultSnapshot, VaultSynced, VaultTab, guess_content_type,
    normalize_file_name, validate_byte_size,
};

use crate::effects::UploadJob;
use crate::inbox::preview_text;

pub const TICKET_REFRESH_MARGIN_MS: u64 = 60_000;
const TICKET_RETRY_DELAY_MS: u64 = 30_000;

pub trait MediaTransport {
    fn create_upload(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        room_id: RoomId,
        request: CreateUploadRequest,
    ) -> impl Future<Output = Result<UploadTicket, ApiError>>;
    fn put_object(
        &mut self,
        ticket: UploadTicket,
        bytes: Vec<u8>,
    ) -> impl Future<Output = Result<(), ApiError>>;
    fn download_tickets(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
        request: DownloadTicketsRequest,
    ) -> impl Future<Output = Result<DownloadTickets, ApiError>>;
}

pub async fn drive_upload<M: MediaTransport>(
    media: &mut M,
    token: AuthToken,
    job: UploadJob,
    bytes: Vec<u8>,
) -> Result<AttachmentId, ApiError> {
    if u64::try_from(bytes.len()).ok() != Some(job.request.byte_size) {
        return Err(ApiError::invalid(InputField::ByteSize));
    }
    let ticket = media
        .create_upload(token, job.campus_id, job.room_id, job.request)
        .await?;
    let attachment_id = ticket.attachment_id;
    media.put_object(ticket, bytes).await?;
    Ok(attachment_id)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaIntent {
    DraftAttached {
        draft_id: DraftId,
        room_id: RoomId,
        file_name: String,
        byte_size: u64,
    },
    DraftDiscarded(DraftId),
    DraftRetried(DraftId),
    PinRequested {
        room_id: RoomId,
        message_id: MessageId,
        pinned: bool,
    },
    VaultOpened {
        room_id: RoomId,
        tab: VaultTab,
    },
    VaultMoreRequested,
    VaultClosed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Draft {
    draft_id: DraftId,
    room_id: RoomId,
    file_name: String,
    content_type: String,
    kind: AttachmentKind,
    byte_size: u64,
    state: UploadState,
    attachment_id: Option<AttachmentId>,
    claimed_by: Option<ClientNonce>,
}

impl Draft {
    fn view(&self) -> DraftAttachmentView {
        DraftAttachmentView {
            draft_id: self.draft_id,
            room_id: self.room_id,
            file_name: self.file_name.clone(),
            kind: self.kind,
            byte_size: self.byte_size,
            upload: self.state,
        }
    }

    fn request(&self) -> CreateUploadRequest {
        CreateUploadRequest {
            file_name: self.file_name.clone(),
            content_type: self.content_type.clone(),
            byte_size: self.byte_size,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Readiness {
    NoDrafts,
    Failed,
    Waiting,
    Ready(Vec<AttachmentId>),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct VaultState {
    open: bool,
    room_id: Option<RoomId>,
    tab: VaultTab,
    items: Vec<VaultItem>,
    loading: bool,
    next_before: Option<MessageId>,
    requested_before: Option<MessageId>,
    error: Option<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MediaTracker {
    drafts: Vec<Draft>,
    tickets: BTreeMap<AttachmentId, DownloadTicket>,
    inflight: BTreeMap<AttachmentId, u64>,
    wanted: BTreeSet<AttachmentId>,
    blocked_until_ms: u64,
    vault: VaultState,
}

fn attachment_view(attachment: &Attachment, url: Option<String>) -> AttachmentView {
    AttachmentView {
        attachment_id: attachment.attachment_id,
        file_name: attachment.file_name.clone(),
        content_type: attachment.content_type.clone(),
        kind: attachment.kind,
        byte_size: attachment.byte_size,
        url,
    }
}

impl MediaTracker {
    pub(crate) fn attach(
        &mut self,
        draft_id: DraftId,
        room_id: RoomId,
        file_name: &str,
        byte_size: u64,
    ) -> Result<Option<CreateUploadRequest>, InputField> {
        let file_name = normalize_file_name(file_name)?;
        let byte_size = validate_byte_size(byte_size)?;
        if self.drafts.iter().any(|draft| draft.draft_id == draft_id) {
            return Ok(None);
        }
        let content_type = guess_content_type(&file_name).to_owned();
        let draft = Draft {
            draft_id,
            room_id,
            kind: AttachmentKind::from_content_type(&content_type),
            file_name,
            content_type,
            byte_size,
            state: UploadState::Uploading,
            attachment_id: None,
            claimed_by: None,
        };
        let request = draft.request();
        self.drafts.push(draft);
        Ok(Some(request))
    }

    pub(crate) fn discard(&mut self, draft_id: DraftId) -> bool {
        let before = self.drafts.len();
        self.drafts
            .retain(|draft| !(draft.draft_id == draft_id && draft.claimed_by.is_none()));
        self.drafts.len() != before
    }

    pub(crate) fn retry(
        &mut self,
        draft_id: DraftId,
    ) -> Option<(RoomId, CreateUploadRequest, Option<ClientNonce>)> {
        let draft = self.drafts.iter_mut().find(|draft| {
            draft.draft_id == draft_id && matches!(draft.state, UploadState::Failed)
        })?;
        draft.state = UploadState::Uploading;
        Some((draft.room_id, draft.request(), draft.claimed_by))
    }

    pub(crate) fn finish(
        &mut self,
        draft_id: DraftId,
        result: Result<AttachmentId, ApiError>,
    ) -> Option<Option<ClientNonce>> {
        let draft = self
            .drafts
            .iter_mut()
            .find(|draft| draft.draft_id == draft_id)?;
        match result {
            Ok(attachment_id) => {
                draft.state = UploadState::Uploaded;
                draft.attachment_id = Some(attachment_id);
            }
            Err(_) => draft.state = UploadState::Failed,
        }
        Some(draft.claimed_by)
    }

    pub(crate) fn room_draft_ids(&self, room_id: RoomId) -> Vec<DraftId> {
        self.drafts
            .iter()
            .filter(|draft| draft.room_id == room_id && draft.claimed_by.is_none())
            .map(|draft| draft.draft_id)
            .collect()
    }

    pub(crate) fn claim(&mut self, room_id: RoomId, client_nonce: ClientNonce) {
        self.drafts
            .iter_mut()
            .filter(|draft| draft.room_id == room_id && draft.claimed_by.is_none())
            .for_each(|draft| draft.claimed_by = Some(client_nonce));
    }

    pub(crate) fn readiness(&self, client_nonce: ClientNonce) -> Readiness {
        let claimed = self
            .drafts
            .iter()
            .filter(|draft| draft.claimed_by == Some(client_nonce))
            .collect::<Vec<_>>();
        if claimed.is_empty() {
            return Readiness::NoDrafts;
        }
        if claimed
            .iter()
            .any(|draft| matches!(draft.state, UploadState::Failed))
        {
            return Readiness::Failed;
        }
        let ids = claimed
            .iter()
            .filter_map(|draft| draft.attachment_id)
            .collect::<Vec<_>>();
        if ids.len() == claimed.len() {
            Readiness::Ready(ids)
        } else {
            Readiness::Waiting
        }
    }

    pub(crate) fn has_failed_claim(&self, client_nonce: ClientNonce) -> bool {
        self.drafts.iter().any(|draft| {
            draft.claimed_by == Some(client_nonce) && matches!(draft.state, UploadState::Failed)
        })
    }

    pub(crate) fn release_claimed(&mut self, client_nonce: ClientNonce) -> Vec<DraftId> {
        let released = self
            .drafts
            .iter()
            .filter(|draft| draft.claimed_by == Some(client_nonce))
            .map(|draft| draft.draft_id)
            .collect::<Vec<_>>();
        self.drafts
            .retain(|draft| draft.claimed_by != Some(client_nonce));
        released
    }

    pub(crate) fn composer_drafts(&self, room_id: RoomId) -> Vec<DraftAttachmentView> {
        self.drafts
            .iter()
            .filter(|draft| draft.room_id == room_id && draft.claimed_by.is_none())
            .map(Draft::view)
            .collect()
    }

    pub(crate) fn claimed_drafts(&self, client_nonce: ClientNonce) -> Vec<DraftAttachmentView> {
        self.drafts
            .iter()
            .filter(|draft| draft.claimed_by == Some(client_nonce))
            .map(Draft::view)
            .collect()
    }

    pub(crate) fn plan_fetch(
        &mut self,
        ids: impl IntoIterator<Item = AttachmentId>,
        now_ms: u64,
        generation: u64,
    ) -> Vec<AttachmentId> {
        self.wanted.extend(ids);
        if now_ms < self.blocked_until_ms {
            return Vec::new();
        }
        let needed = self
            .wanted
            .iter()
            .copied()
            .filter(|id| !self.inflight.contains_key(id) && !self.is_fresh(*id, now_ms))
            .collect::<Vec<_>>();
        needed.iter().for_each(|id| {
            self.inflight.insert(*id, generation);
        });
        needed
    }

    fn is_fresh(&self, id: AttachmentId, now_ms: u64) -> bool {
        self.tickets.get(&id).is_some_and(|ticket| {
            ticket.expires_at_ms > now_ms.saturating_add(TICKET_REFRESH_MARGIN_MS)
        })
    }

    pub(crate) fn apply_tickets(
        &mut self,
        generation: u64,
        result: Result<DownloadTickets, ApiError>,
        now_ms: u64,
    ) {
        let owned = self
            .inflight
            .iter()
            .filter(|(_, owner)| **owner == generation)
            .map(|(id, _)| *id)
            .collect::<BTreeSet<_>>();
        if owned.is_empty() {
            return;
        }
        owned.iter().for_each(|id| {
            self.inflight.remove(id);
        });
        match result {
            Ok(tickets) => {
                for ticket in tickets.tickets {
                    if owned.contains(&ticket.attachment_id) {
                        self.tickets.insert(ticket.attachment_id, ticket);
                    }
                }
            }
            Err(_) => self.blocked_until_ms = now_ms.saturating_add(TICKET_RETRY_DELAY_MS),
        }
    }

    pub(crate) fn url_for(&self, id: AttachmentId, now_ms: u64) -> Option<String> {
        self.tickets
            .get(&id)
            .filter(|ticket| ticket.expires_at_ms > now_ms)
            .map(|ticket| ticket.url.clone())
    }

    pub(crate) fn view_of(&self, attachment: &Attachment, now_ms: u64) -> AttachmentView {
        attachment_view(attachment, self.url_for(attachment.attachment_id, now_ms))
    }

    pub(crate) fn chunked(ids: Vec<AttachmentId>) -> Vec<Vec<AttachmentId>> {
        ids.chunks(DOWNLOAD_TICKETS_MAX)
            .map(<[AttachmentId]>::to_vec)
            .collect()
    }

    pub(crate) fn live_draft_ids(&self) -> Vec<DraftId> {
        self.drafts.iter().map(|draft| draft.draft_id).collect()
    }

    pub(crate) fn open_vault(&mut self, room_id: RoomId, tab: VaultTab) -> SyncVault {
        self.vault = VaultState {
            open: true,
            room_id: Some(room_id),
            tab,
            loading: true,
            ..VaultState::default()
        };
        SyncVault {
            room_id,
            tab,
            before: None,
        }
    }

    pub(crate) fn vault_more(&mut self) -> Option<SyncVault> {
        let room_id = self.vault.room_id.filter(|_| self.vault.open)?;
        let before = self.vault.next_before.filter(|_| !self.vault.loading)?;
        self.vault.loading = true;
        self.vault.requested_before = Some(before);
        Some(SyncVault {
            room_id,
            tab: self.vault.tab,
            before: Some(before),
        })
    }

    pub(crate) fn close_vault(&mut self) {
        self.vault = VaultState::default();
    }

    pub(crate) fn vault_resume(&self) -> Option<SyncVault> {
        let room_id = self.vault.room_id.filter(|_| self.vault.open)?;
        self.vault.loading.then_some(SyncVault {
            room_id,
            tab: self.vault.tab,
            before: self.vault.requested_before,
        })
    }

    fn vault_entry(&self, tab: VaultTab, item: &VaultItem, now_ms: u64) -> VaultEntry {
        VaultEntry {
            tab,
            room_id: item.message.room_id,
            message_id: item.message.message_id,
            author_display_name: item.message.author_display_name.clone(),
            preview: preview_text(&item.message),
            attachment: item
                .attachment
                .as_ref()
                .map(|attachment| self.view_of(attachment, now_ms)),
            link: item.link.clone(),
            pinned_at_ms: item.message.pin.map(|pin| pin.pinned_at_ms),
        }
    }

    pub(crate) fn receive_vault(&mut self, synced: VaultSynced) {
        let matches = self.vault.open
            && self.vault.loading
            && self.vault.room_id == Some(synced.room_id)
            && self.vault.tab == synced.tab;
        if !matches {
            return;
        }
        if self.vault.requested_before.is_none() {
            self.vault.items.clear();
        }
        for item in synced.items {
            let known = self.vault.items.iter().any(|existing| {
                existing.message.message_id == item.message.message_id
                    && existing.attachment.as_ref().map(|view| view.attachment_id)
                        == item.attachment.as_ref().map(|view| view.attachment_id)
                    && existing.link == item.link
            });
            if !known {
                self.vault.items.push(item);
            }
        }
        self.vault.loading = false;
        self.vault.requested_before = None;
        self.vault.next_before = synced.next_before;
        self.vault.error = None;
    }

    pub(crate) fn reject_vault(&mut self, rejected: &VaultRejected) {
        let matches = self.vault.open
            && self.vault.room_id == Some(rejected.room_id)
            && self.vault.tab == rejected.tab;
        if !matches {
            return;
        }
        self.vault.loading = false;
        self.vault.requested_before = None;
        self.vault.error = Some(match rejected.reason {
            VaultRejection::Forbidden => ApiError::new(ApiErrorCode::Forbidden),
            VaultRejection::UnknownRoom => ApiError::new(ApiErrorCode::NotFound),
        });
    }

    pub(crate) fn observe_pin(&mut self, message: &RoomMessage) {
        let pinned_view = self.vault.open
            && self.vault.tab == VaultTab::Pinned
            && self.vault.room_id == Some(message.room_id);
        if !pinned_view {
            return;
        }
        self.vault
            .items
            .retain(|item| item.message.message_id != message.message_id);
        if message.pin.is_some() {
            self.vault.items.push(VaultItem {
                message: message.clone(),
                attachment: None,
                link: None,
            });
            self.vault.items.sort_by(|left, right| {
                let pinned_at = |item: &VaultItem| item.message.pin.map(|pin| pin.pinned_at_ms);
                pinned_at(right).cmp(&pinned_at(left))
            });
        }
    }

    pub(crate) fn vault_snapshot(&self, now_ms: u64) -> VaultSnapshot {
        VaultSnapshot {
            open: self.vault.open,
            room_id: self.vault.room_id,
            tab: self.vault.tab,
            entries: self
                .vault
                .items
                .iter()
                .map(|item| self.vault_entry(self.vault.tab, item, now_ms))
                .collect(),
            loading: self.vault.loading,
            has_more: self.vault.next_before.is_some(),
            error: self.vault.error,
        }
    }
}
