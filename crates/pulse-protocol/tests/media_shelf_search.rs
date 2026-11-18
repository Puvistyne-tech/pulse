mod common;

use common::{assert_wire, room_message, room_message_json};
use pulse_protocol::fixtures::{
    ACCOUNT_A, ATTACHMENT_A, ATTACHMENT_B, LINK_A, MESSAGE_A, MESSAGE_B, NONCE_A, ROOM_A,
    SHELF_ITEM_A,
};
use pulse_protocol::{
    ATTACHMENT_MAX_BYTES, Attachment, AttachmentId, AttachmentKind, BodySegment,
    CONTENT_TYPE_MAX_CHARS, ClientFrame, CreateUploadRequest, FILE_NAME_MAX_CHARS, InputField,
    LINK_MAX_CHARS, MESSAGE_LINKS_MAX, MentionRef, PinMark, PinRejected, PinRejection,
    REMIND_AT_HORIZON_MS, REMIND_AT_PAST_TOLERANCE_MS, ReminderPreset, SEARCH_QUERY_MAX_CHARS,
    SEARCH_TERM_MAX_CHARS, SEARCH_TERMS_MAX, SegmentKind, SendMessage, ServerFrame, SetPin,
    ShelfAction, ShelfChanged, ShelfItem, ShelfRejected, ShelfRejection, ShelfSynced, SyncShelf,
    SyncVault, UpdateShelf, Validate, VaultItem, VaultRejected, VaultRejection, VaultSynced,
    VaultTab, body_segments, extract_links, guess_content_type, mention_token,
    normalize_attachment_ids, normalize_content_type, normalize_file_name,
    normalize_message_content, normalize_search_query, search_terms, validate_byte_size,
    validate_remind_at,
};
use rstest::rstest;
use serde_json::{Value, json};
use uuid::Uuid;

fn attachment_id(index: u128) -> AttachmentId {
    AttachmentId(Uuid::from_u128(
        0x0193a5c0_0000_7000_8000_000000000a00 + index,
    ))
}

fn ids(indexes: &[u128]) -> Vec<AttachmentId> {
    indexes.iter().copied().map(attachment_id).collect()
}

fn attachment() -> Attachment {
    Attachment {
        attachment_id: ATTACHMENT_A,
        file_name: "cover.png".to_owned(),
        content_type: "image/png".to_owned(),
        kind: AttachmentKind::Image,
        byte_size: 2048,
    }
}

fn attachment_json() -> Value {
    json!({
        "attachment_id": ATTACHMENT_A.to_string(),
        "file_name": "cover.png",
        "content_type": "image/png",
        "kind": "image",
        "byte_size": 2048,
    })
}

fn shelf_item() -> ShelfItem {
    ShelfItem {
        shelf_item_id: SHELF_ITEM_A,
        message: room_message(MESSAGE_A, None),
        remind_at_ms: Some(1_000),
        reminded_at_ms: None,
        done_at_ms: None,
        saved_at_ms: 500,
        revision: 3,
    }
}

fn shelf_item_json() -> Value {
    json!({
        "shelf_item_id": SHELF_ITEM_A.to_string(),
        "message": room_message_json(MESSAGE_A, None),
        "remind_at_ms": 1000,
        "reminded_at_ms": null,
        "done_at_ms": null,
        "saved_at_ms": 500,
        "revision": 3,
    })
}

#[rstest]
#[case::set_pin(
    ClientFrame::SetPin(SetPin { room_id: ROOM_A, message_id: MESSAGE_A, pinned: true }),
    json!({
        "type": "set_pin",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "pinned": true,
    }),
)]
#[case::sync_vault_first_page(
    ClientFrame::SyncVault(SyncVault { room_id: ROOM_A, tab: VaultTab::Links, before: None }),
    json!({
        "type": "sync_vault",
        "room_id": ROOM_A.to_string(),
        "tab": "links",
        "before": null,
    }),
)]
#[case::sync_vault_older_page(
    ClientFrame::SyncVault(SyncVault {
        room_id: ROOM_A,
        tab: VaultTab::Pinned,
        before: Some(MESSAGE_B),
    }),
    json!({
        "type": "sync_vault",
        "room_id": ROOM_A.to_string(),
        "tab": "pinned",
        "before": MESSAGE_B.to_string(),
    }),
)]
#[case::update_shelf_reschedule(
    ClientFrame::UpdateShelf(UpdateShelf {
        room_id: ROOM_A,
        message_id: MESSAGE_A,
        action: ShelfAction::Reschedule,
        remind_at_ms: Some(123),
    }),
    json!({
        "type": "update_shelf",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "action": "reschedule",
        "remind_at_ms": 123,
    }),
)]
#[case::update_shelf_remove(
    ClientFrame::UpdateShelf(UpdateShelf {
        room_id: ROOM_A,
        message_id: MESSAGE_A,
        action: ShelfAction::Remove,
        remind_at_ms: None,
    }),
    json!({
        "type": "update_shelf",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "action": "remove",
        "remind_at_ms": null,
    }),
)]
#[case::sync_shelf(ClientFrame::SyncShelf(SyncShelf {}), json!({"type": "sync_shelf"}))]
#[case::send_with_attachments(
    ClientFrame::SendMessage(SendMessage {
        room_id: ROOM_A,
        client_nonce: NONCE_A,
        body: String::new(),
        thread_root_id: None,
        attachment_ids: vec![ATTACHMENT_A, ATTACHMENT_B],
    }),
    json!({
        "type": "send_message",
        "room_id": ROOM_A.to_string(),
        "client_nonce": NONCE_A.to_string(),
        "body": "",
        "thread_root_id": null,
        "attachment_ids": [ATTACHMENT_A.to_string(), ATTACHMENT_B.to_string()],
    }),
)]
fn new_client_frames_match_wire_format(#[case] frame: ClientFrame, #[case] expected: Value) {
    assert_wire(frame, expected);
}

#[test]
fn send_message_without_attachment_ids_defaults_to_empty() {
    let frame: ClientFrame = serde_json::from_value(json!({
        "type": "send_message",
        "room_id": ROOM_A.to_string(),
        "client_nonce": NONCE_A.to_string(),
        "body": "hello",
        "thread_root_id": null,
    }))
    .expect("legacy frame");
    match frame {
        ClientFrame::SendMessage(request) => assert!(request.attachment_ids.is_empty()),
        other => panic!("unexpected frame {other:?}"),
    }
}

#[rstest]
#[case::pin_rejected(
    ServerFrame::PinRejected(PinRejected {
        room_id: ROOM_A,
        message_id: MESSAGE_A,
        reason: PinRejection::LimitReached,
    }),
    json!({
        "type": "pin_rejected",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "reason": "limit_reached",
    }),
)]
#[case::vault_rejected(
    ServerFrame::VaultRejected(VaultRejected {
        room_id: ROOM_A,
        tab: VaultTab::Files,
        reason: VaultRejection::UnknownRoom,
    }),
    json!({
        "type": "vault_rejected",
        "room_id": ROOM_A.to_string(),
        "tab": "files",
        "reason": "unknown_room",
    }),
)]
#[case::shelf_rejected(
    ServerFrame::ShelfRejected(ShelfRejected {
        room_id: ROOM_A,
        message_id: MESSAGE_A,
        action: ShelfAction::Complete,
        reason: ShelfRejection::ShelfFull,
    }),
    json!({
        "type": "shelf_rejected",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "action": "complete",
        "reason": "shelf_full",
    }),
)]
#[case::vault_synced(
    ServerFrame::VaultSynced(VaultSynced {
        room_id: ROOM_A,
        tab: VaultTab::Media,
        items: vec![
            VaultItem {
                message: room_message(MESSAGE_A, None),
                attachment: Some(attachment()),
                link: None,
            },
            VaultItem {
                message: room_message(MESSAGE_B, Some(MESSAGE_A)),
                attachment: None,
                link: Some(LINK_A.to_owned()),
            },
        ],
        next_before: Some(MESSAGE_A),
    }),
    json!({
        "type": "vault_synced",
        "room_id": ROOM_A.to_string(),
        "tab": "media",
        "items": [
            {
                "message": room_message_json(MESSAGE_A, None),
                "attachment": attachment_json(),
                "link": null,
            },
            {
                "message": room_message_json(MESSAGE_B, Some(MESSAGE_A)),
                "attachment": null,
                "link": LINK_A,
            },
        ],
        "next_before": MESSAGE_A.to_string(),
    }),
)]
#[case::shelf_synced(
    ServerFrame::ShelfSynced(ShelfSynced { items: vec![shelf_item()] }),
    json!({"type": "shelf_synced", "items": [shelf_item_json()]}),
)]
#[case::shelf_changed(
    ServerFrame::ShelfChanged(ShelfChanged { item: shelf_item(), removed: true }),
    json!({"type": "shelf_changed", "item": shelf_item_json(), "removed": true}),
)]
fn new_server_frames_match_wire_format(#[case] frame: ServerFrame, #[case] expected: Value) {
    assert_wire(frame, expected);
}

#[test]
fn messages_carry_attachments_and_pin_marks() {
    let mut message = room_message(MESSAGE_A, None);
    message.attachments = vec![attachment()];
    message.pin = Some(PinMark {
        pinned_by: ACCOUNT_A,
        pinned_at_ms: 77,
    });
    let mut expected = room_message_json(MESSAGE_A, None);
    expected["attachments"] = json!([attachment_json()]);
    expected["pin"] = json!({"pinned_by": ACCOUNT_A.to_string(), "pinned_at_ms": 77});
    assert_wire(message, expected);
}

fn long_link(total_chars: usize) -> String {
    let prefix = "https://e.org/";
    format!("{prefix}{}", "a".repeat(total_chars - prefix.len()))
}

fn distinct_links(count: usize) -> Vec<String> {
    (0..count)
        .map(|index| format!("https://example.org/{index}"))
        .collect()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[rstest]
#[case::plain("see https://example.org/a", strings(&[LINK_A]))]
#[case::insecure("go http://example.org/a now", strings(&["http://example.org/a"]))]
#[case::trailing_period("read https://example.org/a.", strings(&[LINK_A]))]
#[case::trailing_mixed_punctuation("really? https://example.org/a?!,;:", strings(&[LINK_A]))]
#[case::trailing_quote_and_bracket("[https://example.org/a]'", strings(&[LINK_A]))]
#[case::unbalanced_closing_paren("(https://example.org/a)", strings(&[LINK_A]))]
#[case::balanced_parens_kept(
    "https://en.example.org/wiki/Rust_(language)",
    strings(&["https://en.example.org/wiki/Rust_(language)"])
)]
#[case::balanced_parens_then_period(
    "https://en.example.org/wiki/Rust_(language).",
    strings(&["https://en.example.org/wiki/Rust_(language)"])
)]
#[case::double_quote_ends_link("\"https://example.org/a\"", strings(&[LINK_A]))]
#[case::angle_brackets_end_link("<https://example.org/a>", strings(&[LINK_A]))]
#[case::query_and_fragment_kept(
    "https://example.org/a?b=1&c=2#top",
    strings(&["https://example.org/a?b=1&c=2#top"])
)]
#[case::duplicates_removed_in_order(
    "https://example.org/b https://example.org/a https://example.org/b",
    strings(&["https://example.org/b", LINK_A])
)]
#[case::other_schemes_ignored("ftp://example.org/a mailto:a@example.org", Vec::new())]
#[case::uppercase_scheme_ignored("HTTPS://example.org/a", Vec::new())]
#[case::bare_scheme_ignored("https:// and http://.", Vec::new())]
#[case::no_links("nothing to see", Vec::new())]
#[case::empty("", Vec::new())]
#[case::exact_length_kept(&long_link(LINK_MAX_CHARS), vec![long_link(LINK_MAX_CHARS)])]
#[case::overlong_skipped(&long_link(LINK_MAX_CHARS + 1), Vec::new())]
#[case::overlong_skipped_beside_valid(
    &format!("{} {LINK_A}", long_link(LINK_MAX_CHARS + 1)),
    strings(&[LINK_A])
)]
#[case::capped_at_maximum(&distinct_links(MESSAGE_LINKS_MAX + 4).join(" "), distinct_links(MESSAGE_LINKS_MAX))]
#[case::mention_token_is_not_a_link(&mention_token(ACCOUNT_A), Vec::new())]
#[case::link_after_mention(&format!("{}{LINK_A}", mention_token(ACCOUNT_A)), strings(&[LINK_A]))]
#[case::link_before_mention(&format!("{LINK_A}{}", mention_token(ACCOUNT_A)), strings(&[LINK_A]))]
#[case::link_between_mentions(
    &format!("{} {LINK_A} {}", mention_token(ACCOUNT_A), mention_token(ACCOUNT_A)),
    strings(&[LINK_A])
)]
fn links_are_extracted_once_and_in_order(#[case] body: &str, #[case] expected: Vec<String>) {
    assert_eq!(extract_links(body), expected);
}

fn text(value: &str) -> BodySegment {
    BodySegment {
        kind: SegmentKind::Text,
        text: value.to_owned(),
        account_id: None,
        url: None,
    }
}

fn link(url: &str) -> BodySegment {
    BodySegment {
        kind: SegmentKind::Link,
        text: url.to_owned(),
        account_id: None,
        url: Some(url.to_owned()),
    }
}

fn mention(name: &str) -> BodySegment {
    BodySegment {
        kind: SegmentKind::Mention,
        text: format!("@{name}"),
        account_id: Some(ACCOUNT_A),
        url: None,
    }
}

fn ada() -> Vec<MentionRef> {
    vec![MentionRef {
        account_id: ACCOUNT_A,
        display_name: "Ada".to_owned(),
    }]
}

#[rstest]
#[case::link_only(LINK_A.to_owned(), Vec::new(), vec![link(LINK_A)])]
#[case::text_around_link(format!("visit {LINK_A} now"), Vec::new(), vec![text("visit "), link(LINK_A), text(" now")])]
#[case::punctuation_stays_text(format!("see {LINK_A}."), Vec::new(), vec![text("see "), link(LINK_A), text(".")])]
#[case::parens_stay_text(format!("({LINK_A})"), Vec::new(), vec![text("("), link(LINK_A), text(")")])]
#[case::two_links(
    format!("{LINK_A} and https://example.org/b"),
    Vec::new(),
    vec![link(LINK_A), text(" and "), link("https://example.org/b")]
)]
#[case::repeated_link_is_still_a_segment(
    format!("{LINK_A} {LINK_A}"),
    Vec::new(),
    vec![link(LINK_A), text(" "), link(LINK_A)]
)]
#[case::mention_then_link(
    format!("{} {LINK_A}", mention_token(ACCOUNT_A)),
    ada(),
    vec![mention("Ada"), text(" "), link(LINK_A)]
)]
#[case::link_then_mention(
    format!("{LINK_A} {}", mention_token(ACCOUNT_A)),
    ada(),
    vec![link(LINK_A), text(" "), mention("Ada")]
)]
#[case::adjacent_mention_and_link(
    format!("{}{LINK_A}", mention_token(ACCOUNT_A)),
    ada(),
    vec![mention("Ada"), link(LINK_A)]
)]
#[case::unresolved_mention_merges_into_text(
    format!("{} {LINK_A}", mention_token(ACCOUNT_A)),
    Vec::new(),
    vec![text(&format!("{} ", mention_token(ACCOUNT_A))), link(LINK_A)]
)]
#[case::non_http_scheme_is_text(
    "ftp://example.org/a".to_owned(),
    Vec::new(),
    vec![text("ftp://example.org/a")]
)]
fn body_segments_mix_text_mentions_and_links(
    #[case] body: String,
    #[case] mentions: Vec<MentionRef>,
    #[case] expected: Vec<BodySegment>,
) {
    assert_eq!(body_segments(&body, &mentions), expected);
}

fn repeated(character: char, count: usize) -> String {
    character.to_string().repeat(count)
}

#[rstest]
#[case::plain("plan.pdf", Ok("plan.pdf".to_owned()))]
#[case::trimmed("  plan.pdf  ", Ok("plan.pdf".to_owned()))]
#[case::unix_path_stripped("dir/sub/plan.pdf", Ok("plan.pdf".to_owned()))]
#[case::windows_path_stripped("C:\\dir\\plan.pdf", Ok("plan.pdf".to_owned()))]
#[case::non_ascii("résumé.pdf", Ok("résumé.pdf".to_owned()))]
#[case::empty("", Err(InputField::FileName))]
#[case::blank("   ", Err(InputField::FileName))]
#[case::dot(".", Err(InputField::FileName))]
#[case::dot_dot("..", Err(InputField::FileName))]
#[case::path_ending_in_dot_dot("a/..", Err(InputField::FileName))]
#[case::path_ending_in_separator("dir/", Err(InputField::FileName))]
#[case::bell("a\u{7}b", Err(InputField::FileName))]
#[case::tab("a\tb", Err(InputField::FileName))]
#[case::newline("a\nb", Err(InputField::FileName))]
#[case::longest_allowed(&repeated('a', FILE_NAME_MAX_CHARS), Ok(repeated('a', FILE_NAME_MAX_CHARS)))]
#[case::longest_multibyte(&repeated('é', FILE_NAME_MAX_CHARS), Ok(repeated('é', FILE_NAME_MAX_CHARS)))]
#[case::one_too_long(&repeated('a', FILE_NAME_MAX_CHARS + 1), Err(InputField::FileName))]
fn file_name_normalization(#[case] raw: &str, #[case] expected: Result<String, InputField>) {
    assert_eq!(normalize_file_name(raw), expected);
}

#[rstest]
#[case::image("image/png", Ok("image/png".to_owned()))]
#[case::lowercased("IMAGE/PNG", Ok("image/png".to_owned()))]
#[case::parameters_stripped("text/plain; charset=utf-8", Ok("text/plain".to_owned()))]
#[case::padded("  application/pdf  ", Ok("application/pdf".to_owned()))]
#[case::suffix_token("application/vnd.api+json", Ok("application/vnd.api+json".to_owned()))]
#[case::empty("", Err(InputField::ContentType))]
#[case::no_subtype("image", Err(InputField::ContentType))]
#[case::empty_subtype("image/", Err(InputField::ContentType))]
#[case::empty_type("/png", Err(InputField::ContentType))]
#[case::space_inside("image /png", Err(InputField::ContentType))]
#[case::extra_slash("image/png/x", Err(InputField::ContentType))]
#[case::only_parameters("; charset=utf-8", Err(InputField::ContentType))]
#[case::longest_allowed(
    &format!("a/{}", repeated('b', CONTENT_TYPE_MAX_CHARS - 2)),
    Ok(format!("a/{}", repeated('b', CONTENT_TYPE_MAX_CHARS - 2)))
)]
#[case::one_too_long(
    &format!("a/{}", repeated('b', CONTENT_TYPE_MAX_CHARS - 1)),
    Err(InputField::ContentType)
)]
fn content_type_normalization(#[case] raw: &str, #[case] expected: Result<String, InputField>) {
    assert_eq!(normalize_content_type(raw), expected);
}

#[rstest]
#[case::zero(0, Err(InputField::ByteSize))]
#[case::one_byte(1, Ok(1))]
#[case::typical(4096, Ok(4096))]
#[case::exact_cap(ATTACHMENT_MAX_BYTES, Ok(ATTACHMENT_MAX_BYTES))]
#[case::over_cap(ATTACHMENT_MAX_BYTES + 1, Err(InputField::ByteSize))]
#[case::huge(u64::MAX, Err(InputField::ByteSize))]
fn byte_size_validation(#[case] size: u64, #[case] expected: Result<u64, InputField>) {
    assert_eq!(validate_byte_size(size), expected);
}

#[rstest]
#[case::none(&[], Ok(Vec::new()))]
#[case::ordered(&[1, 2], Ok(ids(&[1, 2])))]
#[case::duplicates_dropped(&[1, 1, 2], Ok(ids(&[1, 2])))]
#[case::first_occurrence_order(&[2, 1, 2], Ok(ids(&[2, 1])))]
#[case::exactly_the_maximum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], Ok(ids(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])))]
#[case::one_over_the_maximum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], Err(InputField::Attachments))]
#[case::duplicates_do_not_count(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 1], Ok(ids(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])))]
fn attachment_id_normalization(
    #[case] raw: &[u128],
    #[case] expected: Result<Vec<AttachmentId>, InputField>,
) {
    assert_eq!(normalize_attachment_ids(&ids(raw)), expected);
}

#[rstest]
#[case::body_and_attachment("hi", &[1], Ok(("hi".to_owned(), ids(&[1]))))]
#[case::attachment_only("", &[1], Ok((String::new(), ids(&[1]))))]
#[case::blank_body_with_attachment("   ", &[1], Ok((String::new(), ids(&[1]))))]
#[case::duplicate_attachments_collapse("", &[1, 1], Ok((String::new(), ids(&[1]))))]
#[case::body_is_trimmed("  hi  ", &[], Ok(("hi".to_owned(), Vec::new())))]
#[case::body_and_duplicates("hi", &[1, 1], Ok(("hi".to_owned(), ids(&[1]))))]
#[case::empty_without_attachments("", &[], Err(InputField::MessageBody))]
#[case::blank_without_attachments("  ", &[], Err(InputField::MessageBody))]
#[case::control_in_body_with_attachment("a\u{7}", &[1], Err(InputField::MessageBody))]
#[case::too_many_attachments(
    "hi",
    &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    Err(InputField::Attachments)
)]
fn message_content_normalization(
    #[case] body: &str,
    #[case] raw: &[u128],
    #[case] expected: Result<(String, Vec<AttachmentId>), InputField>,
) {
    assert_eq!(normalize_message_content(body, &ids(raw)), expected);
}

const NOW: u64 = 1_000_000_000;

#[rstest]
#[case::now(NOW, NOW, Ok(NOW))]
#[case::earliest_allowed(NOW - REMIND_AT_PAST_TOLERANCE_MS, NOW, Ok(NOW - REMIND_AT_PAST_TOLERANCE_MS))]
#[case::just_too_early(NOW - REMIND_AT_PAST_TOLERANCE_MS - 1, NOW, Err(InputField::RemindAt))]
#[case::latest_allowed(NOW + REMIND_AT_HORIZON_MS, NOW, Ok(NOW + REMIND_AT_HORIZON_MS))]
#[case::just_too_late(NOW + REMIND_AT_HORIZON_MS + 1, NOW, Err(InputField::RemindAt))]
#[case::clock_near_epoch(0, 100, Ok(0))]
#[case::far_future(u64::MAX, NOW, Err(InputField::RemindAt))]
#[case::saturating_horizon(u64::MAX, u64::MAX, Ok(u64::MAX))]
fn remind_at_window(
    #[case] remind: u64,
    #[case] now: u64,
    #[case] expected: Result<u64, InputField>,
) {
    assert_eq!(validate_remind_at(remind, now), expected);
}

fn update(action: ShelfAction, remind_at_ms: Option<u64>) -> UpdateShelf {
    UpdateShelf {
        room_id: ROOM_A,
        message_id: MESSAGE_A,
        action,
        remind_at_ms,
    }
}

#[rstest]
#[case::save_plain(update(ShelfAction::Save, None), true)]
#[case::save_with_time(update(ShelfAction::Save, Some(5)), true)]
#[case::reschedule_with_time(update(ShelfAction::Reschedule, Some(5)), true)]
#[case::reschedule_without_time(update(ShelfAction::Reschedule, None), false)]
#[case::complete_plain(update(ShelfAction::Complete, None), true)]
#[case::complete_with_time(update(ShelfAction::Complete, Some(5)), false)]
#[case::reopen_plain(update(ShelfAction::Reopen, None), true)]
#[case::reopen_with_time(update(ShelfAction::Reopen, Some(5)), false)]
#[case::remove_plain(update(ShelfAction::Remove, None), true)]
#[case::remove_with_time(update(ShelfAction::Remove, Some(5)), false)]
fn update_shelf_validation(#[case] request: UpdateShelf, #[case] valid: bool) {
    assert_eq!(request.validate().is_ok(), valid);
    if !valid {
        assert_eq!(
            request.validate().err().and_then(|error| error.field),
            Some(InputField::RemindAt)
        );
    }
}

fn upload(file_name: &str, content_type: &str, byte_size: u64) -> CreateUploadRequest {
    CreateUploadRequest {
        file_name: file_name.to_owned(),
        content_type: content_type.to_owned(),
        byte_size,
    }
}

#[rstest]
#[case::valid(upload("a.png", "image/png", 10), None)]
#[case::bad_name_reported_first(upload("..", "nope", 0), Some(InputField::FileName))]
#[case::bad_type_reported_before_size(upload("a.png", "nope", 0), Some(InputField::ContentType))]
#[case::bad_size(upload("a.png", "image/png", 0), Some(InputField::ByteSize))]
#[case::over_cap(upload("a.png", "image/png", ATTACHMENT_MAX_BYTES + 1), Some(InputField::ByteSize))]
fn upload_request_validation(
    #[case] request: CreateUploadRequest,
    #[case] field: Option<InputField>,
) {
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

#[rstest]
#[case::attachments_over_cap(
    SendMessage {
        room_id: ROOM_A,
        client_nonce: NONCE_A,
        body: "hi".to_owned(),
        thread_root_id: None,
        attachment_ids: ids(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
    },
    Some(InputField::Attachments)
)]
#[case::attachment_only(
    SendMessage {
        room_id: ROOM_A,
        client_nonce: NONCE_A,
        body: String::new(),
        thread_root_id: None,
        attachment_ids: ids(&[1]),
    },
    None
)]
#[case::empty_everything(
    SendMessage {
        room_id: ROOM_A,
        client_nonce: NONCE_A,
        body: String::new(),
        thread_root_id: None,
        attachment_ids: Vec::new(),
    },
    Some(InputField::MessageBody)
)]
fn send_message_with_attachments_validation(
    #[case] request: SendMessage,
    #[case] field: Option<InputField>,
) {
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

#[rstest]
#[case::words("Hello, World!", &["hello", "world"])]
#[case::empty("", &[])]
#[case::only_separators("  --  ", &[])]
#[case::case_folded("ÉCOLE école", &["école", "école"])]
#[case::punctuation_splits("rust-lang_2024", &["rust", "lang", "2024"])]
#[case::diacritics_kept_distinct("cafe café", &["cafe", "café"])]
#[case::combining_mark_splits("cafe\u{301}", &["cafe"])]
#[case::cjk_with_space("你好 世界", &["你好", "世界"])]
#[case::cjk_run_is_one_term("你好世界", &["你好世界"])]
#[case::mixed_scripts("rust你好", &["rust你好"])]
#[case::digits("v2 3.14", &["v2", "3", "14"])]
#[case::underscore_splits("snake_case", &["snake", "case"])]
fn search_terms_tokenize(#[case] text: &str, #[case] expected: &[&str]) {
    assert_eq!(search_terms(text), strings(expected));
}

#[test]
fn search_terms_truncate_each_term() {
    let terms = search_terms(&format!("{} b", repeated('a', SEARCH_TERM_MAX_CHARS + 6)));
    assert_eq!(
        terms,
        vec![repeated('a', SEARCH_TERM_MAX_CHARS), "b".to_owned()]
    );
}

#[rstest]
#[case::empty("", Vec::new())]
#[case::words("Rust async", strings(&["rust", "async"]))]
#[case::dedupe_in_order("b a B a", strings(&["b", "a"]))]
#[case::case_dedupe("Rust RUST rust", strings(&["rust"]))]
#[case::cjk("你好 世界", strings(&["你好", "世界"]))]
#[case::term_cap(
    "a b c d e f g h i j",
    strings(&["a", "b", "c", "d", "e", "f", "g", "h"])
)]
#[case::distinct_terms_exactly_at_cap(
    "a b c d e f g h",
    strings(&["a", "b", "c", "d", "e", "f", "g", "h"])
)]
#[case::query_clipped_before_tokenizing(
    &format!("{} tail", repeated('x', SEARCH_QUERY_MAX_CHARS)),
    vec![repeated('x', SEARCH_TERM_MAX_CHARS)]
)]
#[case::clip_keeps_words_inside_the_limit(
    &format!("{}word", "a ".repeat(SEARCH_QUERY_MAX_CHARS / 2)),
    strings(&["a"])
)]
fn search_queries_normalize(#[case] raw: &str, #[case] expected: Vec<String>) {
    let terms = normalize_search_query(raw);
    assert!(terms.len() <= SEARCH_TERMS_MAX);
    assert_eq!(terms, expected);
}

#[rstest]
#[case::png("a.png", "image/png")]
#[case::uppercase_extension("A.PNG", "image/png")]
#[case::jpeg("photo.JPEG", "image/jpeg")]
#[case::video("movie.mov", "video/quicktime")]
#[case::audio("song.mp3", "audio/mpeg")]
#[case::pdf("plan.pdf", "application/pdf")]
#[case::markdown("notes.md", "text/markdown")]
#[case::last_extension_wins("archive.tar.png", "image/png")]
#[case::unknown_extension("archive.tar.gz", "application/octet-stream")]
#[case::no_extension("README", "application/octet-stream")]
#[case::trailing_dot("odd.", "application/octet-stream")]
#[case::dotfile(".png", "image/png")]
#[case::empty("", "application/octet-stream")]
fn content_types_are_guessed_from_extensions(#[case] file_name: &str, #[case] expected: &str) {
    assert_eq!(guess_content_type(file_name), expected);
}

#[rstest]
#[case::image("image/png", AttachmentKind::Image)]
#[case::uppercase("IMAGE/PNG", AttachmentKind::Image)]
#[case::padded(" image/png", AttachmentKind::Image)]
#[case::video("video/mp4", AttachmentKind::Video)]
#[case::audio("audio/mpeg", AttachmentKind::Audio)]
#[case::document("application/pdf", AttachmentKind::File)]
#[case::text("text/plain", AttachmentKind::File)]
#[case::lookalike("images/png", AttachmentKind::File)]
#[case::empty("", AttachmentKind::File)]
fn attachment_kind_follows_the_major_type(
    #[case] content_type: &str,
    #[case] expected: AttachmentKind,
) {
    assert_eq!(AttachmentKind::from_content_type(content_type), expected);
}

#[test]
fn attachment_kinds_round_trip_through_their_names() {
    for kind in AttachmentKind::ALL {
        assert_eq!(kind.as_str().parse::<AttachmentKind>().ok(), Some(kind));
        assert_wire(kind, json!(kind.as_str()));
    }
    assert!("archive".parse::<AttachmentKind>().is_err());
    assert!("Image".parse::<AttachmentKind>().is_err());
}

#[rstest]
#[case::twenty_minutes(ReminderPreset::InTwentyMinutes, 1_200_000)]
#[case::one_hour(ReminderPreset::InOneHour, 3_600_000)]
#[case::three_hours(ReminderPreset::InThreeHours, 10_800_000)]
#[case::tomorrow(ReminderPreset::Tomorrow, 86_400_000)]
#[case::next_week(ReminderPreset::NextWeek, 604_800_000)]
fn reminder_presets_have_fixed_offsets(#[case] preset: ReminderPreset, #[case] expected: u64) {
    assert_eq!(preset.offset_ms(), expected);
    assert!(ReminderPreset::ALL.contains(&preset));
    assert!(validate_remind_at(NOW + preset.offset_ms(), NOW).is_ok());
}

#[test]
fn reminder_presets_are_listed_in_ascending_order() {
    let offsets: Vec<u64> = ReminderPreset::ALL
        .iter()
        .map(|preset| preset.offset_ms())
        .collect();
    let mut sorted = offsets.clone();
    sorted.sort_unstable();
    assert_eq!(offsets, sorted);
    assert_eq!(offsets.len(), 5);
}
