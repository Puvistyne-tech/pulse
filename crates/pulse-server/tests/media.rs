mod common;

use common::{
    ClientSocket, Person, SCENARIO_DEADLINE, Stage, api_error_of, attachment_binding,
    bound_attachment_count, campus_with_owner, count_rows, download_tickets_of, get_bytes,
    message_count, object_key_of, parsed_tickets, put_bytes, request_upload, send_outcome,
    signed_up, storage_ready, synced_page, upload_request, uploaded,
};
use pulse_protocol::{
    ATTACHMENT_MAX_BYTES, ApiError, ApiErrorCode, AttachmentId, AttachmentKind,
    DOWNLOAD_TICKETS_MAX, InputField, MESSAGE_ATTACHMENTS_MAX, MessageRejection, ServerFrame,
    UploadTicket,
};
use pulse_server::storage::ObjectLocation;
use reqwest::StatusCode;
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::timeout;

const PNG: &[u8] = b"\x89PNG-fixture-bytes";
const PDF: &[u8] = b"%PDF-fixture-bytes";

async fn stage() -> Stage {
    let stage = Stage::open().await;
    storage_ready(&stage.state).await;
    stage
}

fn rejection_of(frame: ServerFrame) -> MessageRejection {
    match frame {
        ServerFrame::MessageRejected(rejected) => rejected.reason,
        other => panic!("expected rejection, got {other:?}"),
    }
}

#[rstest]
#[case::member(Person::Ada, StatusCode::CREATED)]
#[case::second_member(Person::Grace, StatusCode::CREATED)]
#[case::guest(Person::Guest, StatusCode::FORBIDDEN)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upload_tickets_follow_the_role(#[case] person: Person, #[case] expected: StatusCode) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let request = upload_request("plan.png", "image/png", 10);
        let (status, body) = request_upload(
            &stage.state,
            &stage.session(person).token,
            stage.campus.id,
            stage.room,
            &request,
        )
        .await;
        assert_eq!(status, expected, "{body}");
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::empty_file_name("", "image/png", 10, Some(InputField::FileName))]
#[case::dot_dot_file_name("..", "image/png", 10, Some(InputField::FileName))]
#[case::malformed_content_type("a.png", "png", 10, Some(InputField::ContentType))]
#[case::zero_bytes("a.png", "image/png", 0, Some(InputField::ByteSize))]
#[case::one_byte("a.png", "image/png", 1, None)]
#[case::exact_cap("a.png", "image/png", ATTACHMENT_MAX_BYTES, None)]
#[case::over_cap("a.png", "image/png", ATTACHMENT_MAX_BYTES + 1, Some(InputField::ByteSize))]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upload_requests_are_validated(
    #[case] file_name: &str,
    #[case] content_type: &str,
    #[case] byte_size: u64,
    #[case] rejected_field: Option<InputField>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let request = upload_request(file_name, content_type, byte_size);
        let (status, body) = request_upload(
            &stage.state,
            &stage.ada.token,
            stage.campus.id,
            stage.room,
            &request,
        )
        .await;
        match rejected_field {
            Some(field) => {
                assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
                assert_eq!(api_error_of(body), ApiError::invalid(field));
            }
            None => assert_eq!(status, StatusCode::CREATED, "{body}"),
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upload_tickets_are_scoped_to_campus_and_room() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let foreign = campus_with_owner(&stage.state, "far@example.org", "south-quad").await;
        let stranger = signed_up(&stage.state, "stranger@example.org").await;
        let request = upload_request("a.png", "image/png", 10);
        let attempts = [
            (&stage.ada.token, stage.campus.id, foreign.rooms[0]),
            (
                &stage.ada.token,
                stage.campus.id,
                pulse_protocol::RoomId::generate(),
            ),
            (&stage.ada.token, foreign.id, foreign.rooms[0]),
            (&stranger.token, stage.campus.id, stage.room),
            (&foreign.owner.token, stage.campus.id, stage.room),
        ];
        for (token, campus_id, room_id) in attempts {
            let (status, body) =
                request_upload(&stage.state, token, campus_id, room_id, &request).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
            assert_eq!(api_error_of(body).code, ApiErrorCode::NotFound);
        }
        for campus_id in [stage.campus.id, foreign.id] {
            assert_eq!(count_rows(&stage.state, "attachments", campus_id).await, 0);
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn presigned_path_equals_the_object_location_key() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let request = upload_request("a.png", "image/png", 10);
        let (status, body) = request_upload(
            &stage.state,
            &stage.ada.token,
            stage.campus.id,
            stage.room,
            &request,
        )
        .await;
        assert_eq!(status, StatusCode::CREATED, "{body}");
        let ticket: UploadTicket = serde_json::from_value(body).expect("ticket");
        let key = ObjectLocation {
            campus_id: stage.campus.id,
            room_id: stage.room,
            attachment_id: ticket.attachment_id,
        }
        .key();
        let url = url::Url::parse(&ticket.put_url).expect("put url");
        assert_eq!(url.path(), format!("/pulse-attachments/{key}"));
        let stored = object_key_of(&stage.state, stage.campus.id, ticket.attachment_id).await;
        assert_eq!(stored, key);
        assert_eq!(ticket.content_type, "image/png");
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::shorter(9)]
#[case::longer(11)]
#[case::empty(0)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn storage_refuses_a_put_whose_size_differs_from_the_declaration(#[case] actual: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let request = upload_request("a.bin", "application/octet-stream", 10);
        let (_, body) = request_upload(
            &stage.state,
            &stage.ada.token,
            stage.campus.id,
            stage.room,
            &request,
        )
        .await;
        let ticket: UploadTicket = serde_json::from_value(body).expect("ticket");
        let status = put_bytes(&ticket.put_url, &ticket.content_type, vec![7; actual]).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let location = ObjectLocation {
            campus_id: stage.campus.id,
            room_id: stage.room,
            attachment_id: ticket.attachment_id,
        };
        assert_eq!(
            stage
                .state
                .storage
                .head_size(&location)
                .await
                .expect("head"),
            None
        );
        let exact = put_bytes(&ticket.put_url, &ticket.content_type, vec![7; 10]).await;
        assert_eq!(exact, StatusCode::OK);
        assert_eq!(
            stage
                .state
                .storage
                .head_size(&location)
                .await
                .expect("head"),
            Some(10)
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn uploaded_attachments_ride_a_message_to_every_viewer() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let image = uploaded(
            &stage.state,
            &stage.ada,
            stage.campus.id,
            stage.room,
            "cover.png",
            "image/png",
            PNG,
        )
        .await;
        let file = uploaded(
            &stage.state,
            &stage.ada,
            stage.campus.id,
            stage.room,
            "plan.pdf",
            "application/pdf",
            PDF,
        )
        .await;
        let mut ada = stage.socket(&stage.ada).await;
        let mut grace = stage.socket(&stage.grace).await;
        let ids = [image.attachment_id, file.attachment_id];
        let accepted = match send_outcome(&mut ada, stage.room, "", &ids).await {
            ServerFrame::MessageAccepted(accepted) => accepted.message,
            other => panic!("expected accepted, got {other:?}"),
        };
        let summary: Vec<_> = accepted
            .attachments
            .iter()
            .map(|attachment| {
                (
                    attachment.attachment_id,
                    attachment.file_name.as_str(),
                    attachment.kind,
                    attachment.byte_size,
                )
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (
                    image.attachment_id,
                    "cover.png",
                    AttachmentKind::Image,
                    PNG.len() as u64
                ),
                (
                    file.attachment_id,
                    "plan.pdf",
                    AttachmentKind::File,
                    PDF.len() as u64
                ),
            ]
        );
        assert_eq!(
            attachment_binding(&stage.state, stage.campus.id, image.attachment_id).await,
            Some((Some(accepted.message_id), Some(0)))
        );
        assert_eq!(
            attachment_binding(&stage.state, stage.campus.id, file.attachment_id).await,
            Some((Some(accepted.message_id), Some(1)))
        );
        let created = common::next_of(&mut grace, |frame| match frame {
            ServerFrame::MessageCreated(created) => Some(created.message.clone()),
            _ => None,
        })
        .await;
        assert_eq!(created.attachments, accepted.attachments);
        let page = synced_page(&mut grace, stage.room, None).await;
        assert_eq!(
            page.messages.last().map(|m| &m.attachments),
            Some(&accepted.attachments)
        );
        let (status, body) = download_tickets_of(
            &stage.state,
            &stage.grace.token,
            stage.campus.id,
            ids.to_vec(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let tickets = parsed_tickets(body).tickets;
        let fetched =
            futures_util::future::join_all(tickets.iter().map(|t| get_bytes(&t.url))).await;
        assert_eq!(
            tickets.iter().map(|t| t.attachment_id).collect::<Vec<_>>(),
            ids.to_vec()
        );
        assert_eq!(fetched[0], (StatusCode::OK, PNG.to_vec()));
        assert_eq!(fetched[1], (StatusCode::OK, PDF.to_vec()));
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Fault {
    NeverUploaded,
    SizeMismatch,
    OtherUploader,
    UnknownId,
    OtherRoom,
    OverTheCap,
    EmptyBodyWithoutAttachments,
}

#[rstest]
#[case::never_uploaded(Fault::NeverUploaded, MessageRejection::InvalidAttachment)]
#[case::size_mismatch(Fault::SizeMismatch, MessageRejection::InvalidAttachment)]
#[case::other_uploader(Fault::OtherUploader, MessageRejection::InvalidAttachment)]
#[case::unknown_id(Fault::UnknownId, MessageRejection::InvalidAttachment)]
#[case::other_room(Fault::OtherRoom, MessageRejection::InvalidAttachment)]
#[case::over_the_cap(Fault::OverTheCap, MessageRejection::InvalidAttachment)]
#[case::empty_body(Fault::EmptyBodyWithoutAttachments, MessageRejection::InvalidBody)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn faulty_attachments_reject_the_send_and_stay_unclaimed(
    #[case] fault: Fault,
    #[case] expected: MessageRejection,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let (campus_id, room) = (stage.campus.id, stage.room);
        let (ids, probe): (Vec<AttachmentId>, Option<AttachmentId>) = match fault {
            Fault::NeverUploaded => {
                let request = upload_request("a.png", "image/png", 10);
                let (_, body) =
                    request_upload(&stage.state, &stage.ada.token, campus_id, room, &request).await;
                let ticket: UploadTicket = serde_json::from_value(body).expect("ticket");
                (vec![ticket.attachment_id], Some(ticket.attachment_id))
            }
            Fault::SizeMismatch => {
                let ticket = uploaded(
                    &stage.state,
                    &stage.ada,
                    campus_id,
                    room,
                    "a.png",
                    "image/png",
                    PNG,
                )
                .await;
                sqlx::query("UPDATE attachments SET byte_size = byte_size + 1")
                    .execute(&stage.state.db)
                    .await
                    .expect("skew declared size");
                (vec![ticket.attachment_id], Some(ticket.attachment_id))
            }
            Fault::OtherUploader => {
                let ticket = uploaded(
                    &stage.state,
                    &stage.grace,
                    campus_id,
                    room,
                    "a.png",
                    "image/png",
                    PNG,
                )
                .await;
                (vec![ticket.attachment_id], Some(ticket.attachment_id))
            }
            Fault::UnknownId => (vec![AttachmentId::generate()], None),
            Fault::OtherRoom => {
                let ticket = uploaded(
                    &stage.state,
                    &stage.ada,
                    campus_id,
                    stage.campus.rooms[1],
                    "a.png",
                    "image/png",
                    PNG,
                )
                .await;
                (vec![ticket.attachment_id], Some(ticket.attachment_id))
            }
            Fault::OverTheCap => (
                (0..=MESSAGE_ATTACHMENTS_MAX)
                    .map(|_| AttachmentId::generate())
                    .collect(),
                None,
            ),
            Fault::EmptyBodyWithoutAttachments => (Vec::new(), None),
        };
        let mut ada = stage.socket(&stage.ada).await;
        let outcome = send_outcome(&mut ada, room, "", &ids).await;
        assert_eq!(rejection_of(outcome), expected, "{fault:?}");
        assert_eq!(message_count(&stage.state, campus_id).await, 0);
        if let Some(attachment_id) = probe {
            assert_eq!(
                attachment_binding(&stage.state, campus_id, attachment_id).await,
                Some((None, None)),
                "{fault:?}"
            );
        }
        if matches!(fault, Fault::SizeMismatch) {
            let location = ObjectLocation {
                campus_id,
                room_id: room,
                attachment_id: probe.expect("probe"),
            };
            assert_eq!(
                stage
                    .state
                    .storage
                    .head_size(&location)
                    .await
                    .expect("head"),
                None
            );
        }
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::four_senders(4)]
#[case::twelve_senders(12)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_sends_claim_an_attachment_exactly_once(#[case] senders: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let ticket = uploaded(
            &stage.state,
            &stage.ada,
            stage.campus.id,
            stage.room,
            "race.png",
            "image/png",
            PNG,
        )
        .await;
        let mut sockets: Vec<ClientSocket> = Vec::new();
        for _ in 0..senders {
            sockets.push(stage.socket(&stage.ada).await);
        }
        let mut racing = JoinSet::new();
        for mut socket in sockets {
            let (room, id) = (stage.room, ticket.attachment_id);
            racing.spawn(async move { send_outcome(&mut socket, room, "race", &[id]).await });
        }
        let mut accepted = Vec::new();
        let mut rejections = Vec::new();
        while let Some(done) = racing.join_next().await {
            match done.expect("sender task") {
                ServerFrame::MessageAccepted(frame) => accepted.push(frame.message),
                other => rejections.push(rejection_of(other)),
            }
        }
        assert_eq!(accepted.len(), 1);
        assert_eq!(
            rejections,
            vec![MessageRejection::InvalidAttachment; senders - 1]
        );
        assert_eq!(
            attachment_binding(&stage.state, stage.campus.id, ticket.attachment_id).await,
            Some((Some(accepted[0].message_id), Some(0)))
        );
        assert_eq!(message_count(&stage.state, stage.campus.id).await, 1);
        assert_eq!(
            bound_attachment_count(&stage.state, stage.campus.id).await,
            1
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn download_tickets_never_cross_a_campus_or_expose_pending_uploads() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = stage().await;
        let foreign = campus_with_owner(&stage.state, "far@example.org", "south-quad").await;
        let near = uploaded(
            &stage.state,
            &stage.ada,
            stage.campus.id,
            stage.room,
            "near.png",
            "image/png",
            PNG,
        )
        .await;
        let far = uploaded(
            &stage.state,
            &foreign.owner,
            foreign.id,
            foreign.rooms[0],
            "far.png",
            "image/png",
            PNG,
        )
        .await;
        let pending = uploaded(
            &stage.state,
            &stage.ada,
            stage.campus.id,
            stage.room,
            "pending.png",
            "image/png",
            PNG,
        )
        .await;
        let mut ada = stage.socket(&stage.ada).await;
        send_outcome(&mut ada, stage.room, "near", &[near.attachment_id]).await;
        let (mut far_socket, _) =
            common::joined_socket(stage.addr, foreign.id, &foreign.owner.token).await;
        common::send(
            &mut far_socket,
            &common::sync_rooms(&[(foreign.rooms[0], None)]),
        )
        .await;
        common::expect_frame(&mut far_socket).await;
        send_outcome(
            &mut far_socket,
            foreign.rooms[0],
            "far",
            &[far.attachment_id],
        )
        .await;
        let asked = vec![
            near.attachment_id,
            far.attachment_id,
            pending.attachment_id,
            AttachmentId::generate(),
        ];
        let (status, body) = download_tickets_of(
            &stage.state,
            &stage.ada.token,
            stage.campus.id,
            asked.clone(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let granted: Vec<_> = parsed_tickets(body)
            .tickets
            .into_iter()
            .map(|ticket| ticket.attachment_id)
            .collect();
        assert_eq!(granted, vec![near.attachment_id]);
        let (status, body) = download_tickets_of(
            &stage.state,
            &foreign.owner.token,
            foreign.id,
            asked.clone(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let granted: Vec<_> = parsed_tickets(body)
            .tickets
            .into_iter()
            .map(|ticket| ticket.attachment_id)
            .collect();
        assert_eq!(granted, vec![far.attachment_id]);
        for (token, campus_id) in [
            (&stage.ada.token, foreign.id),
            (&foreign.owner.token, stage.campus.id),
        ] {
            let (status, body) =
                download_tickets_of(&stage.state, token, campus_id, asked.clone()).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
        }
        let flood: Vec<_> = (0..=DOWNLOAD_TICKETS_MAX)
            .map(|_| AttachmentId::generate())
            .collect();
        let (status, body) =
            download_tickets_of(&stage.state, &stage.ada.token, stage.campus.id, flood).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
        assert_eq!(
            api_error_of(body),
            ApiError::invalid(InputField::Attachments)
        );
    })
    .await
    .expect("scenario deadline");
}
