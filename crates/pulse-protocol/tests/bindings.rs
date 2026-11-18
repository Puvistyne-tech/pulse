use pulse_protocol::{
    AccountId, AccountProfile, ApiError, ApiErrorCode, Attachment, AttachmentId, AttachmentKind,
    AttachmentView, AuthPhase, AuthSession, AuthToken, BodySegment, CampusCreated, CampusId,
    CampusList, CampusSummary, ClientFrame, ClientNonce, ClientSnapshot, ConnectionPhase,
    ConnectionSnapshot, CreateCampusRequest, CreateRoomRequest, CreateUploadRequest, DeliveryState,
    DownloadTicket, DownloadTickets, DownloadTicketsRequest, DraftAttachmentView, DraftId,
    HandshakeRejected, HandshakeRejection, Hello, HighlightSpan, InboxItem, InboxItemKind,
    InboxSnapshot, InboxStep, InboxSynced, InputField, MarkThreadRead, MentionRef, MessageAccepted,
    MessageCreated, MessageId, MessageRejected, MessageRejection, MessageUpdated, Permission,
    PinMark, PinRejected, PinRejection, Ping, Pong, ProtocolVersion, React, ReactionAction,
    ReactionRejected, ReactionRejection, ReactionSummary, ReactionTally, ReminderPreset,
    ReplyPreview, Role, Room, RoomCursor, RoomId, RoomList, RoomMessage, RoomSynced,
    RoomUnavailable, SearchHit, SearchSnapshot, SegmentKind, SendMessage, ServerFrame, SessionId,
    SessionSnapshot, SetPin, SetThreadSubscription, SetTopicTitle, ShelfAction, ShelfBucket,
    ShelfChanged, ShelfEntry, ShelfItem, ShelfItemId, ShelfRejected, ShelfRejection, ShelfSnapshot,
    ShelfSynced, SignInRequest, SignUpRequest, SubscriptionOrigin, SubscriptionState, SyncInbox,
    SyncRooms, SyncShelf, SyncThread, SyncVault, ThreadChanged, ThreadPreview, ThreadRejected,
    ThreadRejection, ThreadSnapshot, ThreadSummary, ThreadSynced, TimelineEntry, TimelineSnapshot,
    UpdateShelf, UploadState, UploadTicket, VaultEntry, VaultItem, VaultRejected, VaultRejection,
    VaultSnapshot, VaultSynced, VaultTab, VersionMismatch, Welcome,
};
use std::path::PathBuf;
use ts_rs::{Config, TS};

fn render_bindings() -> String {
    let config = Config::default();
    let declarations = [
        CampusId::decl(&config),
        SessionId::decl(&config),
        AccountId::decl(&config),
        RoomId::decl(&config),
        MessageId::decl(&config),
        ClientNonce::decl(&config),
        AttachmentId::decl(&config),
        ShelfItemId::decl(&config),
        DraftId::decl(&config),
        ProtocolVersion::decl(&config),
        AuthToken::decl(&config),
        AccountProfile::decl(&config),
        SignUpRequest::decl(&config),
        SignInRequest::decl(&config),
        AuthSession::decl(&config),
        Role::decl(&config),
        Permission::decl(&config),
        Room::decl(&config),
        CampusSummary::decl(&config),
        CreateCampusRequest::decl(&config),
        CampusCreated::decl(&config),
        CampusList::decl(&config),
        RoomList::decl(&config),
        CreateRoomRequest::decl(&config),
        ApiErrorCode::decl(&config),
        InputField::decl(&config),
        ApiError::decl(&config),
        HandshakeRejection::decl(&config),
        HandshakeRejected::decl(&config),
        Hello::decl(&config),
        Welcome::decl(&config),
        Ping::decl(&config),
        Pong::decl(&config),
        VersionMismatch::decl(&config),
        MentionRef::decl(&config),
        ReactionTally::decl(&config),
        SegmentKind::decl(&config),
        BodySegment::decl(&config),
        AttachmentKind::decl(&config),
        Attachment::decl(&config),
        PinMark::decl(&config),
        RoomMessage::decl(&config),
        SendMessage::decl(&config),
        RoomCursor::decl(&config),
        SyncRooms::decl(&config),
        MessageAccepted::decl(&config),
        MessageCreated::decl(&config),
        MessageRejection::decl(&config),
        MessageRejected::decl(&config),
        RoomSynced::decl(&config),
        RoomUnavailable::decl(&config),
        MessageUpdated::decl(&config),
        ReactionAction::decl(&config),
        React::decl(&config),
        ReactionRejection::decl(&config),
        ReactionRejected::decl(&config),
        SubscriptionState::decl(&config),
        SubscriptionOrigin::decl(&config),
        SetThreadSubscription::decl(&config),
        MarkThreadRead::decl(&config),
        SetTopicTitle::decl(&config),
        SyncThread::decl(&config),
        ThreadSynced::decl(&config),
        SyncInbox::decl(&config),
        ThreadSummary::decl(&config),
        InboxSynced::decl(&config),
        ThreadChanged::decl(&config),
        ThreadRejection::decl(&config),
        ThreadRejected::decl(&config),
        CreateUploadRequest::decl(&config),
        UploadTicket::decl(&config),
        DownloadTicketsRequest::decl(&config),
        DownloadTicket::decl(&config),
        DownloadTickets::decl(&config),
        SetPin::decl(&config),
        PinRejection::decl(&config),
        PinRejected::decl(&config),
        VaultTab::decl(&config),
        SyncVault::decl(&config),
        VaultItem::decl(&config),
        VaultSynced::decl(&config),
        VaultRejection::decl(&config),
        VaultRejected::decl(&config),
        ShelfAction::decl(&config),
        ReminderPreset::decl(&config),
        ShelfBucket::decl(&config),
        UpdateShelf::decl(&config),
        SyncShelf::decl(&config),
        ShelfItem::decl(&config),
        ShelfSynced::decl(&config),
        ShelfChanged::decl(&config),
        ShelfRejection::decl(&config),
        ShelfRejected::decl(&config),
        ClientFrame::decl(&config),
        ServerFrame::decl(&config),
        ConnectionPhase::decl(&config),
        AuthPhase::decl(&config),
        ConnectionSnapshot::decl(&config),
        SessionSnapshot::decl(&config),
        DeliveryState::decl(&config),
        UploadState::decl(&config),
        AttachmentView::decl(&config),
        DraftAttachmentView::decl(&config),
        ReactionSummary::decl(&config),
        ReplyPreview::decl(&config),
        ThreadPreview::decl(&config),
        TimelineEntry::decl(&config),
        TimelineSnapshot::decl(&config),
        ThreadSnapshot::decl(&config),
        InboxItemKind::decl(&config),
        InboxStep::decl(&config),
        InboxItem::decl(&config),
        InboxSnapshot::decl(&config),
        HighlightSpan::decl(&config),
        SearchHit::decl(&config),
        SearchSnapshot::decl(&config),
        VaultEntry::decl(&config),
        VaultSnapshot::decl(&config),
        ShelfEntry::decl(&config),
        ShelfSnapshot::decl(&config),
        ClientSnapshot::decl(&config),
    ];
    let mut rendered = declarations
        .iter()
        .map(|declaration| format!("export {declaration}"))
        .collect::<Vec<_>>()
        .join("\n");
    rendered.push('\n');
    rendered
}

fn bindings_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bindings/protocol.ts")
}

#[test]
fn bindings_match_committed_file() {
    let rendered = render_bindings();
    if std::env::var("PULSE_BLESS_BINDINGS").as_deref() == Ok("1") {
        std::fs::write(bindings_path(), &rendered).expect("write bindings");
        return;
    }
    let committed = std::fs::read_to_string(bindings_path()).expect("read bindings");
    assert_eq!(committed, rendered);
}

#[test]
fn bindings_contain_no_comments() {
    let rendered = render_bindings();
    assert!(!rendered.contains("//"));
    assert!(!rendered.contains("/*"));
}
