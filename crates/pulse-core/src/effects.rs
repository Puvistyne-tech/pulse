use pulse_protocol::{
    ApiError, AttachmentId, CampusId, CreateUploadRequest, DownloadTickets, DraftId, RoomId,
    SearchHit,
};

use crate::search::{SearchDocument, SearchQuery};
use crate::store::StoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadJob {
    pub draft_id: DraftId,
    pub campus_id: CampusId,
    pub room_id: RoomId,
    pub request: CreateUploadRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    IndexDocuments(Vec<SearchDocument>),
    RunSearch {
        generation: u64,
        query: SearchQuery,
    },
    Upload(UploadJob),
    ReleaseDraft(DraftId),
    FetchDownloadTickets {
        generation: u64,
        campus_id: CampusId,
        attachment_ids: Vec<AttachmentId>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectOutcome {
    SearchCompleted {
        generation: u64,
        result: Result<Vec<SearchHit>, StoreError>,
    },
    UploadFinished {
        draft_id: DraftId,
        result: Result<AttachmentId, ApiError>,
    },
    TicketsReceived {
        generation: u64,
        result: Result<DownloadTickets, ApiError>,
    },
}
