mod fixtures;

use fixtures::{
    CAMPUS_A, NONCE_A, ROOM_A, accepted_frame, attach_draft, attachment_id, attachment_of,
    chain_message, client_rooms, connected_as, created_frame, draft_id, media, message_id,
    own_message, pinned, received, send_requested, settled_client, settled_machine,
    signed_in_client, step_all, timeline_connected, timeline_run, unavailable_error, updated_frame,
    upload_finished, vault_item, vault_synced_frame, with_attachments,
};
use pulse_core::{
    Client, Effect, EffectOutcome, MediaIntent, TimelineEvent, TimelineMachine, UploadJob,
};
use pulse_protocol::{
    ATTACHMENT_MAX_BYTES, ApiError, ApiErrorCode, AttachmentId, AttachmentKind, ClientFrame,
    CreateUploadRequest, DeliveryState, DownloadTicket, DownloadTickets, InputField,
    MessageRejection, PinRejected, PinRejection, Role, RoomId, ServerFrame, SetPin, SyncVault,
    UploadState, VaultRejected, VaultRejection, VaultTab,
};
use rstest::rstest;

#[derive(Debug, Clone, Copy)]
enum Step {
    Attach(u128, &'static str, u64),
    Discard(u128),
    Retry(u128),
    Done(u128),
    Failed(u128),
}

fn upload(index: u128, file_name: &str, content_type: &str, byte_size: u64) -> Effect {
    Effect::Upload(UploadJob {
        draft_id: draft_id(index),
        campus_id: CAMPUS_A,
        room_id: ROOM_A,
        request: CreateUploadRequest {
            file_name: file_name.to_owned(),
            content_type: content_type.to_owned(),
            byte_size,
        },
    })
}

fn apply_step(client: &mut Client, step: Step) {
    let room_id = room_of(client);
    match step {
        Step::Attach(index, file_name, byte_size) => {
            client.media_intent(MediaIntent::DraftAttached {
                draft_id: draft_id(index),
                room_id,
                file_name: file_name.to_owned(),
                byte_size,
            });
        }
        Step::Discard(index) => {
            client.media_intent(MediaIntent::DraftDiscarded(draft_id(index)));
        }
        Step::Retry(index) => {
            client.media_intent(MediaIntent::DraftRetried(draft_id(index)));
        }
        Step::Done(index) => {
            client.effect_finished(EffectOutcome::UploadFinished {
                draft_id: draft_id(index),
                result: Ok(attachment_id(10 + index)),
            });
        }
        Step::Failed(index) => {
            client.effect_finished(EffectOutcome::UploadFinished {
                draft_id: draft_id(index),
                result: Err(unavailable_error()),
            });
        }
    }
}

fn composer_drafts(client: &Client) -> Vec<(u128, UploadState, AttachmentKind)> {
    client
        .snapshot()
        .timeline
        .drafts
        .iter()
        .map(|draft| {
            let index = (1..=9)
                .find(|index| draft_id(*index) == draft.draft_id)
                .expect("known draft");
            (index, draft.upload, draft.kind)
        })
        .collect()
}

#[rstest]
#[case::image_upload(
    &[Step::Attach(1, "a.png", 10)],
    vec![upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::Image],
    None
)]
#[case::document_upload(
    &[Step::Attach(1, "plan.pdf", 10)],
    vec![upload(1, "plan.pdf", "application/pdf", 10)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::File],
    None
)]
#[case::unknown_extension_is_a_file(
    &[Step::Attach(1, "blob.xyz", 10)],
    vec![upload(1, "blob.xyz", "application/octet-stream", 10)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::File],
    None
)]
#[case::path_is_stripped_from_the_name(
    &[Step::Attach(1, "/tmp/secret/a.png", 10)],
    vec![upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::Image],
    None
)]
#[case::two_drafts_upload_independently(
    &[Step::Attach(1, "a.png", 10), Step::Attach(2, "b.mp4", 20)],
    vec![upload(1, "a.png", "image/png", 10), upload(2, "b.mp4", "video/mp4", 20)],
    vec![(1, UploadState::Uploading), (2, UploadState::Uploading)],
    vec![AttachmentKind::Image, AttachmentKind::Video],
    None
)]
#[case::attaching_the_same_draft_twice_uploads_once(
    &[Step::Attach(1, "a.png", 10), Step::Attach(1, "a.png", 10)],
    vec![upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::Image],
    None
)]
#[case::empty_name_is_released(
    &[Step::Attach(1, "", 10)],
    vec![Effect::ReleaseDraft(draft_id(1))],
    vec![],
    vec![],
    Some(ApiError::invalid(InputField::FileName))
)]
#[case::zero_size_is_released(
    &[Step::Attach(1, "a.png", 0)],
    vec![Effect::ReleaseDraft(draft_id(1))],
    vec![],
    vec![],
    Some(ApiError::invalid(InputField::ByteSize))
)]
#[case::over_cap_is_released(
    &[Step::Attach(1, "a.png", ATTACHMENT_MAX_BYTES + 1)],
    vec![Effect::ReleaseDraft(draft_id(1))],
    vec![],
    vec![],
    Some(ApiError::invalid(InputField::ByteSize))
)]
#[case::exact_cap_is_accepted(
    &[Step::Attach(1, "a.png", ATTACHMENT_MAX_BYTES)],
    vec![upload(1, "a.png", "image/png", ATTACHMENT_MAX_BYTES)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::Image],
    None
)]
#[case::finished_upload(
    &[Step::Attach(1, "a.png", 10), Step::Done(1)],
    vec![upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Uploaded)],
    vec![AttachmentKind::Image],
    None
)]
#[case::failed_upload(
    &[Step::Attach(1, "a.png", 10), Step::Failed(1)],
    vec![upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Failed)],
    vec![AttachmentKind::Image],
    None
)]
#[case::retry_restarts_a_failed_upload(
    &[Step::Attach(1, "a.png", 10), Step::Failed(1), Step::Retry(1)],
    vec![upload(1, "a.png", "image/png", 10), upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::Image],
    None
)]
#[case::retry_of_a_running_upload_does_nothing(
    &[Step::Attach(1, "a.png", 10), Step::Retry(1)],
    vec![upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Uploading)],
    vec![AttachmentKind::Image],
    None
)]
#[case::retry_of_a_finished_upload_does_nothing(
    &[Step::Attach(1, "a.png", 10), Step::Done(1), Step::Retry(1)],
    vec![upload(1, "a.png", "image/png", 10)],
    vec![(1, UploadState::Uploaded)],
    vec![AttachmentKind::Image],
    None
)]
#[case::discard_releases_the_draft(
    &[Step::Attach(1, "a.png", 10), Step::Discard(1)],
    vec![upload(1, "a.png", "image/png", 10), Effect::ReleaseDraft(draft_id(1))],
    vec![],
    vec![],
    None
)]
#[case::discard_after_upload_releases_the_draft(
    &[Step::Attach(1, "a.png", 10), Step::Done(1), Step::Discard(1)],
    vec![upload(1, "a.png", "image/png", 10), Effect::ReleaseDraft(draft_id(1))],
    vec![],
    vec![],
    None
)]
#[case::discard_of_an_unknown_draft_does_nothing(&[Step::Discard(9)], vec![], vec![], vec![], None)]
#[case::result_of_an_unknown_draft_is_ignored(&[Step::Done(9), Step::Failed(8)], vec![], vec![], vec![], None)]
fn draft_lifecycle_follows_effects(
    #[case] steps: &[Step],
    #[case] expected_effects: Vec<Effect>,
    #[case] expected_states: Vec<(u128, UploadState)>,
    #[case] expected_kinds: Vec<AttachmentKind>,
    #[case] expected_error: Option<ApiError>,
) {
    let mut client = settled_client();
    steps.iter().for_each(|step| apply_step(&mut client, *step));
    let room = room_of(&client);
    let effects = client
        .take_effects()
        .into_iter()
        .map(|effect| match effect {
            Effect::Upload(job) => {
                assert_eq!(job.room_id, room);
                Effect::Upload(UploadJob {
                    room_id: ROOM_A,
                    ..job
                })
            }
            other => other,
        })
        .collect::<Vec<_>>();
    assert_eq!(effects, expected_effects);
    let drafts = composer_drafts(&client);
    assert_eq!(
        drafts
            .iter()
            .map(|(index, state, _)| (*index, *state))
            .collect::<Vec<_>>(),
        expected_states
    );
    assert_eq!(
        drafts.iter().map(|(_, _, kind)| *kind).collect::<Vec<_>>(),
        expected_kinds
    );
    assert_eq!(client.snapshot().timeline.error, expected_error);
}

fn send_frames(frames: &[ClientFrame]) -> Vec<Vec<AttachmentId>> {
    frames
        .iter()
        .filter_map(|frame| match frame {
            ClientFrame::SendMessage(send) => Some(send.attachment_ids.clone()),
            _ => None,
        })
        .collect()
}

fn ids_of(indexes: &[u128]) -> Vec<AttachmentId> {
    indexes
        .iter()
        .copied()
        .map(|index| attachment_id(10 + index))
        .collect()
}

fn room_of(client: &Client) -> RoomId {
    client_rooms(client)[0]
}

fn attach_in(client: &mut Client, room_id: RoomId, index: u128) {
    client.media_intent(MediaIntent::DraftAttached {
        draft_id: draft_id(index),
        room_id,
        file_name: format!("file-{index}.png"),
        byte_size: 10,
    });
}

#[rstest]
#[case::single_draft(&[1], &[1], &[1], &[1])]
#[case::two_drafts_in_order(&[1, 2], &[1, 2], &[0, 1], &[1, 2])]
#[case::two_drafts_reversed(&[1, 2], &[2, 1], &[0, 1], &[1, 2])]
#[case::three_drafts_shuffled(&[1, 2, 3], &[2, 3, 1], &[0, 0, 1], &[1, 2, 3])]
fn a_send_waits_for_every_draft_and_keeps_draft_order(
    #[case] drafts: &[u128],
    #[case] finishing: &[u128],
    #[case] frames_after_each: &[usize],
    #[case] expected_ids: &[u128],
) {
    let mut client = settled_client();
    let room = room_of(&client);
    drafts
        .iter()
        .for_each(|index| attach_in(&mut client, room, *index));
    let sent = client.send_message(room, NONCE_A, String::new());
    assert!(client.snapshot().timeline.drafts.is_empty());
    let pending = sent
        .timeline
        .entries
        .iter()
        .find(|entry| entry.client_nonce == NONCE_A)
        .expect("pending entry");
    assert_eq!(pending.delivery, DeliveryState::Pending);
    assert_eq!(pending.pending_drafts.len(), drafts.len());
    assert_eq!(send_frames(&client.drain_outbound()).len(), 0);
    let mut collected = Vec::new();
    let mut counts = Vec::new();
    for index in finishing {
        apply_step(&mut client, Step::Done(*index));
        collected.extend(send_frames(&client.drain_outbound()));
        counts.push(collected.len());
    }
    assert_eq!(counts, frames_after_each);
    assert_eq!(collected, vec![ids_of(expected_ids)]);
}

#[test]
fn a_failed_upload_fails_the_send_until_the_draft_is_retried() {
    let mut client = settled_client();
    let room = room_of(&client);
    attach_in(&mut client, room, 1);
    attach_in(&mut client, room, 2);
    client.send_message(room, NONCE_A, "files".to_owned());
    apply_step(&mut client, Step::Done(1));
    apply_step(&mut client, Step::Failed(2));
    let failed = client.snapshot();
    let entry = failed
        .timeline
        .entries
        .iter()
        .find(|entry| entry.client_nonce == NONCE_A)
        .expect("entry");
    assert_eq!(entry.delivery, DeliveryState::Failed);
    assert_eq!(entry.rejection, Some(MessageRejection::InvalidAttachment));
    assert_eq!(
        entry
            .pending_drafts
            .iter()
            .map(|draft| draft.upload)
            .collect::<Vec<_>>(),
        vec![UploadState::Uploaded, UploadState::Failed]
    );
    assert!(send_frames(&client.drain_outbound()).is_empty());
    apply_step(&mut client, Step::Retry(2));
    let retrying = client.snapshot();
    let entry = retrying
        .timeline
        .entries
        .iter()
        .find(|entry| entry.client_nonce == NONCE_A)
        .expect("entry");
    assert_eq!(entry.delivery, DeliveryState::Pending);
    apply_step(&mut client, Step::Done(2));
    assert_eq!(send_frames(&client.drain_outbound()), vec![ids_of(&[1, 2])]);
}

#[test]
fn discarding_a_failed_send_releases_every_claimed_draft() {
    let mut client = settled_client();
    let room = room_of(&client);
    attach_in(&mut client, room, 1);
    attach_in(&mut client, room, 2);
    client.take_effects();
    client.send_message(room, NONCE_A, String::new());
    apply_step(&mut client, Step::Done(1));
    apply_step(&mut client, Step::Failed(2));
    client.discard_message(NONCE_A);
    let mut released: Vec<_> = client
        .take_effects()
        .into_iter()
        .filter_map(|effect| match effect {
            Effect::ReleaseDraft(draft) => Some(draft),
            _ => None,
        })
        .collect();
    released.sort();
    let mut expected = vec![draft_id(1), draft_id(2)];
    expected.sort();
    assert_eq!(released, expected);
    assert!(client.live_draft_ids().is_empty());
}

#[test]
fn the_server_echo_releases_the_claimed_drafts() {
    let mut client = settled_client();
    let room = room_of(&client);
    attach_in(&mut client, room, 1);
    client.send_message(room, NONCE_A, String::new());
    apply_step(&mut client, Step::Done(1));
    client.take_effects();
    assert_eq!(client.live_draft_ids(), vec![draft_id(1)]);
    let echoed = with_attachments(
        own_message(room, 1, NONCE_A),
        vec![attachment_of(11, "file-1.png", AttachmentKind::Image)],
    );
    client.receive_frame(accepted_frame(echoed));
    let released = client
        .take_effects()
        .into_iter()
        .filter(|effect| matches!(effect, Effect::ReleaseDraft(_)))
        .count();
    assert_eq!(released, 1);
    assert!(client.live_draft_ids().is_empty());
    assert!(
        client
            .snapshot()
            .timeline
            .entries
            .iter()
            .all(|entry| entry.delivery == DeliveryState::Confirmed)
    );
}

#[test]
fn a_draft_of_another_room_is_not_claimed_by_the_send() {
    let mut client = settled_client();
    let rooms = client_rooms(&client);
    attach_in(&mut client, rooms[1], 1);
    client.send_message(rooms[0], NONCE_A, "plain".to_owned());
    assert_eq!(send_frames(&client.drain_outbound()), vec![Vec::new()]);
    assert_eq!(client.live_draft_ids(), vec![draft_id(1)]);
    assert!(composer_drafts(&client).is_empty());
}

#[test]
fn an_empty_send_without_drafts_is_refused_locally() {
    let mut client = settled_client();
    let room = room_of(&client);
    let snapshot = client.send_message(room, NONCE_A, "  ".to_owned());
    assert_eq!(
        snapshot.timeline.error,
        Some(ApiError::invalid(InputField::MessageBody))
    );
    assert!(client.drain_outbound().is_empty());
}

#[rstest]
#[case::uploaded_before_connect(&[0, 1, 2, 3], 1)]
#[case::uploaded_after_connect(&[0, 1, 3, 2], 1)]
#[case::still_uploading_at_connect(&[0, 1, 3], 0)]
fn offline_sends_wait_for_both_the_link_and_the_uploads(
    #[case] order: &[usize],
    #[case] expected_sends: usize,
) {
    let events: Vec<TimelineEvent> = vec![
        attach_draft(1, "a.png", 10),
        send_requested(ROOM_A, NONCE_A, ""),
        upload_finished(1, Ok(attachment_id(11))),
        timeline_connected(&[ROOM_A]),
    ];
    let ordered = order.iter().map(|index| events[*index].clone()).collect();
    let (_, frames) = timeline_run(ordered);
    assert_eq!(send_frames(&frames), vec![ids_of(&[1]); expected_sends]);
}

#[rstest]
#[case::owner(Role::Owner, true, true)]
#[case::admin(Role::Admin, true, true)]
#[case::member(Role::Member, true, true)]
#[case::guest(Role::Guest, false, false)]
fn pin_and_upload_follow_the_welcome_role(
    #[case] role: Role,
    #[case] can_pin: bool,
    #[case] can_upload: bool,
) {
    let snapshot = connected_as(role).snapshot().timeline;
    assert_eq!(snapshot.can_pin, can_pin);
    assert_eq!(snapshot.can_upload, can_upload);
}

#[test]
fn pin_and_upload_are_unavailable_before_the_welcome() {
    let (client, _) = signed_in_client();
    let snapshot = client.snapshot().timeline;
    assert!(!snapshot.can_pin);
    assert!(!snapshot.can_upload);
}

fn pin_intent(index: u128, pinned: bool) -> TimelineEvent {
    media(MediaIntent::PinRequested {
        room_id: ROOM_A,
        message_id: message_id(index),
        pinned,
    })
}

fn pin_frame(index: u128, pinned: bool) -> ClientFrame {
    ClientFrame::SetPin(SetPin {
        room_id: ROOM_A,
        message_id: message_id(index),
        pinned,
    })
}

#[rstest]
#[case::pin_online(true, true, vec![pin_frame(1, true)])]
#[case::unpin_online(true, false, vec![pin_frame(1, false)])]
#[case::pin_offline_is_dropped(false, true, vec![])]
fn pin_requests_become_frames_only_while_connected(
    #[case] online: bool,
    #[case] pin: bool,
    #[case] expected: Vec<ClientFrame>,
) {
    let mut events = Vec::new();
    if online {
        events.push(timeline_connected(&[ROOM_A]));
    }
    events.push(pin_intent(1, pin));
    let (_, frames) = timeline_run(events);
    let pins: Vec<_> = frames
        .into_iter()
        .filter(|frame| matches!(frame, ClientFrame::SetPin(_)))
        .collect();
    assert_eq!(pins, expected);
}

#[rstest]
#[case::forbidden(PinRejection::Forbidden, ApiErrorCode::Forbidden)]
#[case::unknown_message(PinRejection::UnknownMessage, ApiErrorCode::NotFound)]
#[case::limit_reached(PinRejection::LimitReached, ApiErrorCode::Conflict)]
fn pin_rejections_surface_as_timeline_errors(
    #[case] reason: PinRejection,
    #[case] expected: ApiErrorCode,
) {
    let machine = settled_machine(vec![received(ServerFrame::PinRejected(PinRejected {
        room_id: ROOM_A,
        message_id: message_id(1),
        reason,
    }))]);
    assert_eq!(
        machine.snapshot(Some(ROOM_A)).error.map(|error| error.code),
        Some(expected)
    );
}

#[test]
fn the_pin_mark_follows_message_updates() {
    let root = chain_message(ROOM_A, 1);
    let mut machine = settled_machine(vec![received(created_frame(root.clone()))]);
    let pinned_flags = |machine: &TimelineMachine| {
        machine
            .snapshot(Some(ROOM_A))
            .entries
            .iter()
            .map(|entry| entry.pinned)
            .collect::<Vec<_>>()
    };
    assert_eq!(pinned_flags(&machine), vec![false]);
    let mut updated = pinned(root.clone(), 100);
    updated.activity_seq = 1;
    (machine, _) = step_all(machine, vec![received(updated_frame(updated))]);
    assert_eq!(pinned_flags(&machine), vec![true]);
    let mut cleared = root;
    cleared.activity_seq = 2;
    (machine, _) = step_all(machine, vec![received(updated_frame(cleared))]);
    assert_eq!(pinned_flags(&machine), vec![false]);
}

fn open_vault(tab: VaultTab) -> TimelineEvent {
    media(MediaIntent::VaultOpened {
        room_id: ROOM_A,
        tab,
    })
}

fn more_vault() -> TimelineEvent {
    media(MediaIntent::VaultMoreRequested)
}

fn vault_request(tab: VaultTab, before: Option<u128>) -> ClientFrame {
    ClientFrame::SyncVault(SyncVault {
        room_id: ROOM_A,
        tab,
        before: before.map(message_id),
    })
}

fn media_page(
    messages: &[u128],
    kind: AttachmentKind,
    next_before: Option<u128>,
    tab: VaultTab,
) -> ServerFrame {
    let items = messages
        .iter()
        .map(|index| {
            vault_item(
                chain_message(ROOM_A, *index),
                Some(attachment_of(*index, &format!("f{index}"), kind)),
                None,
            )
        })
        .collect();
    vault_synced_frame(ROOM_A, tab, items, next_before)
}

fn vault_ids(machine: &TimelineMachine) -> Vec<u128> {
    machine
        .vault_snapshot()
        .entries
        .iter()
        .map(|entry| {
            (1..=40)
                .find(|index| message_id(*index) == entry.message_id)
                .expect("known message")
        })
        .collect()
}

#[test]
fn vault_paging_appends_older_pages_without_duplicates() {
    let machine = settled_machine(vec![]);
    let (machine, frames) = step_all(machine, vec![open_vault(VaultTab::Media)]);
    assert_eq!(frames, vec![vault_request(VaultTab::Media, None)]);
    let opened = machine.vault_snapshot();
    assert!(opened.open && opened.loading && !opened.has_more);
    assert_eq!(opened.room_id, Some(ROOM_A));
    let (machine, frames) = step_all(
        machine,
        vec![received(media_page(
            &[5, 4],
            AttachmentKind::Image,
            Some(4),
            VaultTab::Media,
        ))],
    );
    assert!(frames.is_empty());
    let first = machine.vault_snapshot();
    assert_eq!(vault_ids(&machine), vec![5, 4]);
    assert!(first.has_more && !first.loading);
    assert!(first.entries.iter().all(|entry| {
        entry
            .attachment
            .as_ref()
            .is_some_and(|a| a.kind == AttachmentKind::Image)
    }));
    let (machine, frames) = step_all(machine, vec![more_vault(), more_vault()]);
    assert_eq!(frames, vec![vault_request(VaultTab::Media, Some(4))]);
    assert!(machine.vault_snapshot().loading);
    let (machine, _) = step_all(
        machine,
        vec![received(media_page(
            &[4, 3],
            AttachmentKind::Image,
            None,
            VaultTab::Media,
        ))],
    );
    assert_eq!(vault_ids(&machine), vec![5, 4, 3]);
    let last = machine.vault_snapshot();
    assert!(!last.has_more && !last.loading);
    let (machine, frames) = step_all(machine, vec![more_vault()]);
    assert!(frames.is_empty());
    let (machine, _) = step_all(
        machine,
        vec![received(media_page(
            &[9],
            AttachmentKind::Image,
            None,
            VaultTab::Media,
        ))],
    );
    assert_eq!(vault_ids(&machine), vec![5, 4, 3]);
}

#[rstest]
#[case::other_tab(VaultTab::Files, ROOM_A)]
#[case::other_room(VaultTab::Media, RoomId::generate())]
fn pages_for_another_view_are_ignored(#[case] tab: VaultTab, #[case] room_id: RoomId) {
    let machine = settled_machine(vec![open_vault(VaultTab::Media)]);
    let items = vec![vault_item(
        chain_message(room_id, 5),
        Some(attachment_of(5, "f", AttachmentKind::File)),
        None,
    )];
    let (machine, _) = step_all(
        machine,
        vec![received(vault_synced_frame(room_id, tab, items, None))],
    );
    let snapshot = machine.vault_snapshot();
    assert!(snapshot.loading);
    assert!(snapshot.entries.is_empty());
}

#[rstest]
#[case::forbidden(VaultRejection::Forbidden, ApiErrorCode::Forbidden)]
#[case::unknown_room(VaultRejection::UnknownRoom, ApiErrorCode::NotFound)]
fn vault_rejections_stop_loading_and_report(
    #[case] reason: VaultRejection,
    #[case] expected: ApiErrorCode,
) {
    let machine = settled_machine(vec![
        open_vault(VaultTab::Links),
        received(ServerFrame::VaultRejected(VaultRejected {
            room_id: ROOM_A,
            tab: VaultTab::Links,
            reason,
        })),
    ]);
    let snapshot = machine.vault_snapshot();
    assert!(!snapshot.loading);
    assert_eq!(snapshot.error.map(|error| error.code), Some(expected));
}

#[test]
fn closing_or_reopening_the_vault_resets_it() {
    let machine = settled_machine(vec![
        open_vault(VaultTab::Media),
        received(media_page(
            &[5],
            AttachmentKind::Image,
            None,
            VaultTab::Media,
        )),
    ]);
    assert_eq!(vault_ids(&machine), vec![5]);
    let (reopened, frames) = step_all(machine.clone(), vec![open_vault(VaultTab::Files)]);
    assert_eq!(frames, vec![vault_request(VaultTab::Files, None)]);
    let snapshot = reopened.vault_snapshot();
    assert_eq!(snapshot.tab, VaultTab::Files);
    assert!(snapshot.entries.is_empty() && snapshot.loading);
    let (closed, _) = step_all(machine, vec![media(MediaIntent::VaultClosed)]);
    assert_eq!(closed.vault_snapshot(), Default::default());
}

#[rstest]
#[case::first_page_in_flight(false, false, vec![vault_request(VaultTab::Media, None)])]
#[case::older_page_in_flight(true, true, vec![vault_request(VaultTab::Media, Some(4))])]
#[case::loaded_and_idle(true, false, vec![])]
fn reconnect_resumes_only_an_unfinished_vault_request(
    #[case] first_page_arrived: bool,
    #[case] older_requested: bool,
    #[case] expected: Vec<ClientFrame>,
) {
    let mut events = vec![open_vault(VaultTab::Media)];
    if first_page_arrived {
        events.push(received(media_page(
            &[5, 4],
            AttachmentKind::Image,
            Some(4),
            VaultTab::Media,
        )));
    }
    if older_requested {
        events.push(more_vault());
    }
    events.push(TimelineEvent::Disconnected);
    events.push(timeline_connected(&[ROOM_A]));
    let (_, frames) = timeline_run([vec![timeline_connected(&[ROOM_A])], events].concat());
    let resumed: Vec<_> = frames
        .into_iter()
        .rev()
        .take_while(|frame| !matches!(frame, ClientFrame::SyncRooms(_)))
        .filter(|frame| matches!(frame, ClientFrame::SyncVault(_)))
        .collect();
    assert_eq!(resumed, expected);
}

#[rstest]
#[case::new_pin_goes_first(vec![pinned(chain_message(ROOM_A, 12), 200)], vec![12, 10, 11])]
#[case::unpin_removes(vec![chain_message(ROOM_A, 10)], vec![11])]
#[case::repin_moves_to_the_front(vec![pinned(chain_message(ROOM_A, 10), 300)], vec![10, 11])]
#[case::pin_in_another_room_is_ignored(
    vec![pinned(chain_message(RoomId::generate(), 12), 200)],
    vec![10, 11]
)]
fn live_pin_updates_reshape_the_pinned_tab(
    #[case] updates: Vec<pulse_protocol::RoomMessage>,
    #[case] expected: Vec<u128>,
) {
    let page = vault_synced_frame(
        ROOM_A,
        VaultTab::Pinned,
        vec![
            vault_item(pinned(chain_message(ROOM_A, 10), 100), None, None),
            vault_item(pinned(chain_message(ROOM_A, 11), 50), None, None),
        ],
        None,
    );
    let machine = settled_machine(vec![open_vault(VaultTab::Pinned), received(page)]);
    assert_eq!(vault_ids(&machine), vec![10, 11]);
    let events = updates
        .into_iter()
        .map(|message| received(updated_frame(message)))
        .collect();
    let (machine, _) = step_all(machine, events);
    assert_eq!(vault_ids(&machine), expected);
}

#[test]
fn pin_updates_do_not_touch_the_media_tab() {
    let machine = settled_machine(vec![
        open_vault(VaultTab::Media),
        received(media_page(
            &[5],
            AttachmentKind::Image,
            None,
            VaultTab::Media,
        )),
    ]);
    let (machine, _) = step_all(
        machine,
        vec![received(updated_frame(pinned(
            chain_message(ROOM_A, 12),
            9,
        )))],
    );
    assert_eq!(vault_ids(&machine), vec![5]);
}

const EXPIRY_MS: u64 = 1_000_000;

fn ticket_for(index: u128, url: &str, expires_at_ms: u64) -> DownloadTicket {
    DownloadTicket {
        attachment_id: attachment_id(index),
        url: url.to_owned(),
        expires_at_ms,
    }
}

fn fetches(effects: &[Effect]) -> Vec<(u64, Vec<AttachmentId>)> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::FetchDownloadTickets {
                generation,
                campus_id,
                attachment_ids,
            } => {
                assert_eq!(*campus_id, CAMPUS_A);
                Some((*generation, attachment_ids.clone()))
            }
            _ => None,
        })
        .collect()
}

fn client_with_ticket() -> (Client, RoomId) {
    let mut client = settled_client();
    let room = room_of(&client);
    let message = with_attachments(
        chain_message(room, 1),
        vec![attachment_of(1, "a.png", AttachmentKind::Image)],
    );
    client.receive_frame(created_frame(message));
    let requested = fetches(&client.take_effects());
    assert_eq!(requested.len(), 1);
    assert_eq!(requested[0].1, vec![attachment_id(1)]);
    client.effect_finished(EffectOutcome::TicketsReceived {
        generation: requested[0].0,
        result: Ok(DownloadTickets {
            tickets: vec![ticket_for(1, "https://files.example.org/a", EXPIRY_MS)],
        }),
    });
    (client, room)
}

fn visible_url(client: &Client) -> Option<String> {
    client
        .snapshot()
        .timeline
        .entries
        .iter()
        .flat_map(|entry| entry.attachments.iter())
        .find_map(|attachment| attachment.url.clone())
}

#[rstest]
#[case::fresh_ticket(0, true, false)]
#[case::just_outside_the_margin(EXPIRY_MS - 60_001, true, false)]
#[case::exactly_at_the_margin(EXPIRY_MS - 60_000, true, true)]
#[case::inside_the_margin(EXPIRY_MS - 1_000, true, true)]
#[case::last_valid_millisecond(EXPIRY_MS - 1, true, true)]
#[case::expired(EXPIRY_MS, false, true)]
#[case::long_expired(EXPIRY_MS * 2, false, true)]
fn tickets_refresh_inside_the_margin_and_stop_working_at_expiry(
    #[case] clock_ms: u64,
    #[case] url_visible: bool,
    #[case] refetched: bool,
) {
    let (mut client, _) = client_with_ticket();
    assert!(visible_url(&client).is_some());
    client.advance_clock(clock_ms);
    assert_eq!(visible_url(&client).is_some(), url_visible);
    let requested = fetches(&client.take_effects());
    assert_eq!(requested.len(), usize::from(refetched));
    if let Some((_, ids)) = requested.first() {
        assert_eq!(ids, &vec![attachment_id(1)]);
    }
}

#[test]
fn a_refresh_in_flight_is_not_requested_twice() {
    let (mut client, _) = client_with_ticket();
    client.advance_clock(EXPIRY_MS * 2);
    client.advance_clock(EXPIRY_MS * 2 + 1);
    assert_eq!(fetches(&client.take_effects()).len(), 1);
}

#[test]
fn a_failed_refresh_waits_before_trying_again() {
    let (mut client, _) = client_with_ticket();
    let clock = EXPIRY_MS * 2;
    client.advance_clock(clock);
    let generation = fetches(&client.take_effects())[0].0;
    client.effect_finished(EffectOutcome::TicketsReceived {
        generation,
        result: Err(unavailable_error()),
    });
    client.advance_clock(clock + 29_999);
    assert!(fetches(&client.take_effects()).is_empty());
    client.advance_clock(clock + 30_000);
    assert_eq!(fetches(&client.take_effects()).len(), 1);
}

#[rstest]
#[case::wrong_generation(false, 1)]
#[case::unrequested_attachment(true, 7)]
fn tickets_that_were_not_asked_for_are_ignored(
    #[case] correct_generation: bool,
    #[case] attachment_index: u128,
) {
    let mut client = settled_client();
    let room = room_of(&client);
    let message = with_attachments(
        chain_message(room, 1),
        vec![attachment_of(1, "a.png", AttachmentKind::Image)],
    );
    client.receive_frame(created_frame(message));
    let generation = fetches(&client.take_effects())[0].0;
    let sent_generation = if correct_generation {
        generation
    } else {
        generation + 100
    };
    client.effect_finished(EffectOutcome::TicketsReceived {
        generation: sent_generation,
        result: Ok(DownloadTickets {
            tickets: vec![ticket_for(
                attachment_index,
                "https://files.example.org/x",
                EXPIRY_MS,
            )],
        }),
    });
    assert_eq!(visible_url(&client), None);
}

#[test]
fn re_ingesting_a_message_with_a_fresh_ticket_does_not_refetch() {
    let (mut client, room) = client_with_ticket();
    let again = with_attachments(
        pinned(chain_message(room, 1), 5),
        vec![attachment_of(1, "a.png", AttachmentKind::Image)],
    );
    client.receive_frame(updated_frame(again));
    assert!(fetches(&client.take_effects()).is_empty());
    assert!(visible_url(&client).is_some());
}

#[test]
fn many_attachments_are_fetched_in_chunks_of_the_ticket_limit() {
    let mut client = settled_client();
    let room = room_of(&client);
    let count = u128::try_from(pulse_protocol::DOWNLOAD_TICKETS_MAX).expect("limit") + 20;
    let attachments = (1..=count)
        .map(|index| attachment_of(index, "a.png", AttachmentKind::Image))
        .collect();
    client.receive_frame(created_frame(with_attachments(
        chain_message(room, 1),
        attachments,
    )));
    let requested = fetches(&client.take_effects());
    let mut sizes: Vec<usize> = requested.iter().map(|(_, ids)| ids.len()).collect();
    sizes.sort_unstable();
    assert_eq!(sizes, vec![20, pulse_protocol::DOWNLOAD_TICKETS_MAX]);
}
