mod common;

use std::collections::BTreeSet;

use common::{room_message, room_message_json};
use pulse_protocol::fixtures::{
    CAMPUS_A, LOUNGE_A, MESSAGE_A, MESSAGE_B, NONCE_A, ROOM_A, ROOM_B, STEWARD_A, STEWARD_TOKEN_A,
    STREAM_A, STREAM_B, TOOL_CALL_A, auth_token,
};
use pulse_protocol::{
    AckWork, AgentStreamChunk, CallTool, ClientFrame, HistoryRead, InputField, JoinLounge,
    LeaveLounge, ListRoomsArguments, ListTools, Lounge, LoungeChanged, LoungeHeartbeat,
    LoungeJoined, LoungeRejected, LoungeRejection, LoungeScope, LoungeTicket, LoungesSynced,
    ManageSteward, MessagePosted, PostMessageArguments, REMIND_AT_HORIZON_MS,
    REMINDER_PICKER_STEP_MS, ReadRoomHistoryArguments, ReadThreadArguments, Room, RoomId,
    RoomsListed, STEWARD_ROOMS_MAX, ServerFrame, StewardAction, StewardChanged, StewardIssued,
    StewardProfile, StewardRejected, StewardRejection, StewardTool, StewardWorkItem,
    StewardsSynced, StreamAbortReason, StreamChunk, StreamId, StreamPhase, StreamRejected,
    StreamRejection, StreamWriter, SyncLounges, SyncStewards, SyncWork, ToolContent,
    ToolContentKind, ToolDescriptor, ToolErrorCode, ToolResult, ToolsListed, WorkSynced,
    normalize_room_ids, picker_anchor_ms, picker_latest_ms, tool_descriptors,
};
use rstest::rstest;
use serde::Serialize;
use serde_json::{Value, json};
use uuid::Uuid;

fn room_id(index: u128) -> RoomId {
    RoomId(Uuid::from_u128(
        0x0193a5c0_0000_7000_8000_000000000f00 + index,
    ))
}

fn room_ids(indexes: impl IntoIterator<Item = u128>) -> Vec<RoomId> {
    indexes.into_iter().map(room_id).collect()
}

fn profile() -> StewardProfile {
    StewardProfile {
        steward_id: STEWARD_A,
        display_name: "Scribe".to_owned(),
        room_ids: vec![ROOM_A],
        revoked: false,
        credential_revision: 3,
    }
}

fn lounge() -> Lounge {
    Lounge {
        lounge_id: LOUNGE_A,
        campus_id: CAMPUS_A,
        scope: LoungeScope::Room,
        room_id: Some(ROOM_A),
        participants: Vec::new(),
        revision: 7,
    }
}

fn ticket() -> LoungeTicket {
    LoungeTicket {
        lounge_id: LOUNGE_A,
        livekit_url: "ws://127.0.0.1:7880".to_owned(),
        room_name: "pulse-campus-lounge".to_owned(),
        token: "secret-jwt-value".to_owned(),
        expires_at_ms: 900,
    }
}

fn agent_chunk() -> AgentStreamChunk {
    AgentStreamChunk {
        campus_id: CAMPUS_A,
        stream_id: STREAM_A,
        room_id: ROOM_A,
        thread_root_id: None,
        steward_id: STEWARD_A,
        steward_display_name: "Scribe".to_owned(),
        phase: StreamPhase::Delta,
        sequence: 1,
        text: "hi".to_owned(),
        message_id: None,
        abort_reason: Some(StreamAbortReason::TimedOut),
    }
}

fn manage(action: StewardAction) -> ManageSteward {
    ManageSteward {
        request_nonce: NONCE_A,
        action,
        steward_id: Some(STEWARD_A),
        display_name: None,
        room_ids: Vec::new(),
    }
}

fn chunk(phase: StreamPhase) -> StreamChunk {
    StreamChunk {
        stream_id: STREAM_A,
        room_id: ROOM_A,
        thread_root_id: None,
        phase,
        sequence: 0,
        text: String::new(),
    }
}

fn wire_tag<T: Serialize>(frame: &T) -> String {
    serde_json::to_value(frame).expect("serialize")["type"]
        .as_str()
        .expect("type tag")
        .to_owned()
}

fn json_of<T: Serialize>(value: T) -> Value {
    serde_json::to_value(value).expect("serialize")
}

fn keys(value: &Value) -> BTreeSet<String> {
    value.as_object().expect("object").keys().cloned().collect()
}

#[rstest]
#[case::stream_chunk(ClientFrame::StreamChunk(chunk(StreamPhase::Begin)), "stream_chunk")]
#[case::list_tools(ClientFrame::ListTools(ListTools {}), "list_tools")]
#[case::call_tool(
    ClientFrame::CallTool(CallTool {
        call_id: TOOL_CALL_A,
        name: "list_rooms".to_owned(),
        arguments: "{}".to_owned(),
    }),
    "call_tool"
)]
#[case::sync_work(ClientFrame::SyncWork(SyncWork {}), "sync_work")]
#[case::ack_work(ClientFrame::AckWork(AckWork { through: MESSAGE_A }), "ack_work")]
#[case::manage_steward(
    ClientFrame::ManageSteward(manage(StewardAction::RotateToken)),
    "manage_steward"
)]
#[case::sync_stewards(ClientFrame::SyncStewards(SyncStewards {}), "sync_stewards")]
#[case::sync_lounges(ClientFrame::SyncLounges(SyncLounges {}), "sync_lounges")]
#[case::join_lounge(ClientFrame::JoinLounge(JoinLounge { lounge_id: LOUNGE_A }), "join_lounge")]
#[case::leave_lounge(ClientFrame::LeaveLounge(LeaveLounge { lounge_id: LOUNGE_A }), "leave_lounge")]
#[case::lounge_heartbeat(
    ClientFrame::LoungeHeartbeat(LoungeHeartbeat { lounge_id: LOUNGE_A }),
    "lounge_heartbeat"
)]
fn client_frames_round_trip_under_their_tag(#[case] frame: ClientFrame, #[case] tag: &str) {
    let wire = serde_json::to_string(&frame).expect("serialize");
    assert_eq!(wire_tag(&frame), tag);
    assert_eq!(
        serde_json::from_str::<ClientFrame>(&wire).expect("parse"),
        frame
    );
}

#[rstest]
#[case::agent_stream_chunk(ServerFrame::AgentStreamChunk(agent_chunk()), "agent_stream_chunk")]
#[case::stream_rejected(
    ServerFrame::StreamRejected(StreamRejected {
        stream_id: STREAM_A,
        reason: StreamRejection::OutOfOrder,
    }),
    "stream_rejected"
)]
#[case::tools_listed(
    ServerFrame::ToolsListed(ToolsListed { tools: tool_descriptors() }),
    "tools_listed"
)]
#[case::tool_result(
    ServerFrame::ToolResult(ToolResult {
        call_id: TOOL_CALL_A,
        is_error: true,
        content: vec![ToolContent { kind: ToolContentKind::Text, text: "forbidden".to_owned() }],
        structured_content: None,
    }),
    "tool_result"
)]
#[case::steward_work_item(
    ServerFrame::StewardWorkItem(StewardWorkItem { message: room_message(MESSAGE_A, None) }),
    "steward_work_item"
)]
#[case::work_synced(
    ServerFrame::WorkSynced(WorkSynced {
        items: vec![StewardWorkItem { message: room_message(MESSAGE_B, None) }],
        has_more: true,
    }),
    "work_synced"
)]
#[case::steward_issued(
    ServerFrame::StewardIssued(StewardIssued {
        request_nonce: NONCE_A,
        steward: profile(),
        token: auth_token(STEWARD_TOKEN_A),
    }),
    "steward_issued"
)]
#[case::steward_changed(ServerFrame::StewardChanged(StewardChanged { steward: profile() }), "steward_changed")]
#[case::steward_rejected(
    ServerFrame::StewardRejected(StewardRejected {
        request_nonce: NONCE_A,
        action: StewardAction::Revoke,
        reason: StewardRejection::TooManyStewards,
    }),
    "steward_rejected"
)]
#[case::stewards_synced(
    ServerFrame::StewardsSynced(StewardsSynced { stewards: vec![profile()] }),
    "stewards_synced"
)]
#[case::lounges_synced(ServerFrame::LoungesSynced(LoungesSynced { lounges: vec![lounge()] }), "lounges_synced")]
#[case::lounge_joined(
    ServerFrame::LoungeJoined(LoungeJoined { lounge: lounge(), ticket: ticket() }),
    "lounge_joined"
)]
#[case::lounge_changed(ServerFrame::LoungeChanged(LoungeChanged { lounge: lounge() }), "lounge_changed")]
#[case::lounge_rejected(
    ServerFrame::LoungeRejected(LoungeRejected {
        lounge_id: LOUNGE_A,
        reason: LoungeRejection::Unavailable,
    }),
    "lounge_rejected"
)]
fn server_frames_round_trip_under_their_tag(#[case] frame: ServerFrame, #[case] tag: &str) {
    let wire = serde_json::to_string(&frame).expect("serialize");
    assert_eq!(wire_tag(&frame), tag);
    assert_eq!(
        serde_json::from_str::<ServerFrame>(&wire).expect("parse"),
        frame
    );
}

#[rstest]
#[case::chunk_without_text(
    json!({
        "type": "stream_chunk",
        "stream_id": STREAM_A.to_string(),
        "room_id": ROOM_A.to_string(),
        "thread_root_id": null,
        "phase": "end",
        "sequence": 4,
    }),
    ClientFrame::StreamChunk(StreamChunk { sequence: 4, ..chunk(StreamPhase::End) })
)]
#[case::manage_without_rooms(
    json!({
        "type": "manage_steward",
        "request_nonce": NONCE_A.to_string(),
        "action": "revoke",
        "steward_id": STEWARD_A.to_string(),
        "display_name": null,
    }),
    ClientFrame::ManageSteward(manage(StewardAction::Revoke))
)]
fn omitted_defaulted_fields_parse(#[case] wire: Value, #[case] expected: ClientFrame) {
    assert_eq!(
        serde_json::from_value::<ClientFrame>(wire).expect("parse"),
        expected
    );
}

#[derive(Clone, Copy)]
enum Op {
    Begin,
    Delta(&'static str),
    End,
    Abort,
}

#[rstest]
#[case::full_stream(
    vec![Op::Begin, Op::Delta("he"), Op::Delta("llo"), Op::End],
    vec![(StreamPhase::Begin, 0, ""), (StreamPhase::Delta, 1, "he"), (StreamPhase::Delta, 2, "llo"), (StreamPhase::End, 3, "")],
    4
)]
#[case::abort_after_begin(
    vec![Op::Begin, Op::Abort],
    vec![(StreamPhase::Begin, 0, ""), (StreamPhase::Abort, 1, "")],
    2
)]
#[case::delta_first_still_counts_from_zero(
    vec![Op::Delta("x")],
    vec![(StreamPhase::Delta, 0, "x")],
    1
)]
#[case::empty_script(vec![], vec![], 0)]
fn stream_writer_sequences_phases(
    #[case] script: Vec<Op>,
    #[case] expected: Vec<(StreamPhase, u32, &str)>,
    #[case] next_sequence: u32,
) {
    let mut writer = StreamWriter::new(STREAM_B, ROOM_B, Some(MESSAGE_A));
    let chunks = script
        .into_iter()
        .map(|op| match op {
            Op::Begin => writer.begin(),
            Op::Delta(text) => writer.delta(text),
            Op::End => writer.end(),
            Op::Abort => writer.abort(),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        chunks
            .iter()
            .map(|chunk| (chunk.phase, chunk.sequence, chunk.text.as_str()))
            .collect::<Vec<_>>(),
        expected
    );
    assert!(chunks.iter().all(|chunk| chunk.stream_id == STREAM_B
        && chunk.room_id == ROOM_B
        && chunk.thread_root_id == Some(MESSAGE_A)));
    assert_eq!(writer.next_sequence(), next_sequence);
}

#[rstest]
#[case::stream_a(STREAM_A)]
#[case::stream_b(STREAM_B)]
fn stream_id_doubles_as_final_nonce(#[case] stream_id: StreamId) {
    assert_eq!(stream_id.client_nonce().0, stream_id.0);
}

#[rstest]
#[case::list_rooms(StewardTool::ListRooms, "list_rooms")]
#[case::read_room_history(StewardTool::ReadRoomHistory, "read_room_history")]
#[case::read_thread(StewardTool::ReadThread, "read_thread")]
#[case::post_message(StewardTool::PostMessage, "post_message")]
fn tool_names_round_trip(#[case] tool: StewardTool, #[case] name: &str) {
    assert_eq!(tool.name(), name);
    assert_eq!(StewardTool::from_name(name), Some(tool));
    assert!(!tool.description().is_empty());
}

#[rstest]
#[case::empty("")]
#[case::unknown("search_messages")]
#[case::wrong_case("List_Rooms")]
#[case::padded(" list_rooms")]
fn unknown_tool_names_do_not_resolve(#[case] name: &str) {
    assert_eq!(StewardTool::from_name(name), None);
}

#[test]
fn tool_catalogue_lists_each_tool_once_in_order() {
    let names = tool_descriptors()
        .into_iter()
        .map(|descriptor| descriptor.name)
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        StewardTool::ALL
            .iter()
            .map(|tool| tool.name().to_owned())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        names.iter().collect::<BTreeSet<_>>().len(),
        StewardTool::ALL.len()
    );
}

#[rstest]
#[case::list_rooms(StewardTool::ListRooms, json_of(ListRoomsArguments {}))]
#[case::read_room_history(
    StewardTool::ReadRoomHistory,
    json_of(ReadRoomHistoryArguments { room_id: ROOM_A, before: None, limit: None })
)]
#[case::read_thread(
    StewardTool::ReadThread,
    json_of(ReadThreadArguments { room_id: ROOM_A, root_message_id: MESSAGE_A })
)]
#[case::post_message(
    StewardTool::PostMessage,
    json_of(PostMessageArguments {
        room_id: ROOM_A,
        client_nonce: NONCE_A,
        body: "hi".to_owned(),
        thread_root_id: None,
    })
)]
fn input_schema_matches_argument_struct(#[case] tool: StewardTool, #[case] arguments: Value) {
    let schema = serde_json::from_str::<Value>(tool.input_schema()).expect("schema is json");
    assert_eq!(schema["type"], "object");
    assert_eq!(keys(&schema["properties"]), keys(&arguments));
    let required = schema["required"]
        .as_array()
        .expect("required array")
        .iter()
        .map(|name| name.as_str().expect("name").to_owned())
        .collect::<BTreeSet<_>>();
    let non_optional = arguments
        .as_object()
        .expect("object")
        .iter()
        .filter(|(_, value)| !value.is_null())
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(required, non_optional);
    assert!(required.is_subset(&keys(&schema["properties"])));
}

#[rstest]
#[case::descriptor(
    json_of(ToolDescriptor {
        name: "list_rooms".to_owned(),
        description: "d".to_owned(),
        input_schema: "{}".to_owned(),
    }),
    json!({"name": "list_rooms", "description": "d", "inputSchema": "{}"})
)]
#[case::error_result(
    json_of(ToolResult {
        call_id: TOOL_CALL_A,
        is_error: true,
        content: vec![ToolContent { kind: ToolContentKind::Text, text: "forbidden".to_owned() }],
        structured_content: None,
    }),
    json!({
        "callId": TOOL_CALL_A.to_string(),
        "isError": true,
        "content": [{"type": "text", "text": "forbidden"}],
        "structuredContent": null,
    })
)]
#[case::structured_result(
    json_of(ToolResult {
        call_id: TOOL_CALL_A,
        is_error: false,
        content: vec![ToolContent { kind: ToolContentKind::Text, text: "{}".to_owned() }],
        structured_content: Some("{}".to_owned()),
    }),
    json!({
        "callId": TOOL_CALL_A.to_string(),
        "isError": false,
        "content": [{"type": "text", "text": "{}"}],
        "structuredContent": "{}",
    })
)]
fn tool_payloads_use_mcp_keys(#[case] actual: Value, #[case] expected: Value) {
    assert_eq!(actual, expected);
}

#[rstest]
#[case::rooms_listed(
    json_of(RoomsListed {
        rooms: vec![Room { room_id: ROOM_A, campus_id: CAMPUS_A, slug: "commons".to_owned() }],
    }),
    json!({"rooms": [{"room_id": ROOM_A.to_string(), "campus_id": CAMPUS_A.to_string(), "slug": "commons"}]})
)]
#[case::history_read(
    json_of(HistoryRead { messages: vec![room_message(MESSAGE_A, None)], has_more: false }),
    json!({"messages": [room_message_json(MESSAGE_A, None)], "has_more": false})
)]
#[case::message_posted(
    json_of(MessagePosted { message: room_message(MESSAGE_A, None) }),
    json!({"message": room_message_json(MESSAGE_A, None)})
)]
fn tool_structured_payloads_serialize_flat(#[case] actual: Value, #[case] expected: Value) {
    assert_eq!(actual, expected);
}

#[rstest]
#[case::forbidden(ToolErrorCode::Forbidden, "forbidden")]
#[case::unknown_tool(ToolErrorCode::UnknownTool, "unknown_tool")]
#[case::invalid_arguments(ToolErrorCode::InvalidArguments, "invalid_arguments")]
#[case::unknown_room(ToolErrorCode::UnknownRoom, "unknown_room")]
#[case::unknown_thread(ToolErrorCode::UnknownThread, "unknown_thread")]
#[case::rejected(ToolErrorCode::Rejected, "rejected")]
fn tool_error_codes_match_their_wire_form(#[case] code: ToolErrorCode, #[case] text: &str) {
    assert_eq!(code.as_str(), text);
    assert_eq!(json_of(code), json!(text));
}

#[rstest]
#[case::ticket(format!("{:?}", ticket()), "secret-jwt-value")]
#[case::joined_frame(
    format!(
        "{:?}",
        ServerFrame::LoungeJoined(LoungeJoined { lounge: lounge(), ticket: ticket() })
    ),
    "secret-jwt-value"
)]
#[case::issued_frame(
    format!(
        "{:?}",
        ServerFrame::StewardIssued(StewardIssued {
            request_nonce: NONCE_A,
            steward: profile(),
            token: auth_token(STEWARD_TOKEN_A),
        })
    ),
    STEWARD_TOKEN_A
)]
fn debug_output_redacts_secrets(#[case] rendered: String, #[case] secret: &str) {
    assert!(!rendered.contains(secret));
    assert!(rendered.contains("redacted"));
}

#[rstest]
#[case::empty(vec![], Ok(vec![]))]
#[case::sorted_and_deduped(room_ids([3, 1, 2, 1, 3]), Ok(room_ids([1, 2, 3])))]
#[case::already_sorted(room_ids([1, 2]), Ok(room_ids([1, 2])))]
#[case::at_the_cap(
    room_ids(0..STEWARD_ROOMS_MAX as u128),
    Ok(room_ids(0..STEWARD_ROOMS_MAX as u128))
)]
#[case::over_the_cap(room_ids(0..=STEWARD_ROOMS_MAX as u128), Err(InputField::Rooms))]
#[case::duplicates_collapse_below_the_cap(
    room_ids((0..STEWARD_ROOMS_MAX as u128).chain(0..STEWARD_ROOMS_MAX as u128)),
    Ok(room_ids(0..STEWARD_ROOMS_MAX as u128))
)]
fn room_ids_normalize(
    #[case] input: Vec<RoomId>,
    #[case] expected: Result<Vec<RoomId>, InputField>,
) {
    assert_eq!(normalize_room_ids(&input), expected);
}

#[rstest]
#[case::zero_clock(0, None, None)]
#[case::first_millisecond(1, Some(REMINDER_PICKER_STEP_MS), Some(1 + REMIND_AT_HORIZON_MS))]
#[case::just_below_step(
    REMINDER_PICKER_STEP_MS - 1,
    Some(REMINDER_PICKER_STEP_MS),
    Some(REMINDER_PICKER_STEP_MS - 1 + REMIND_AT_HORIZON_MS)
)]
#[case::on_step(
    REMINDER_PICKER_STEP_MS,
    Some(REMINDER_PICKER_STEP_MS),
    Some(REMINDER_PICKER_STEP_MS + REMIND_AT_HORIZON_MS)
)]
#[case::just_past_step(
    REMINDER_PICKER_STEP_MS + 1,
    Some(2 * REMINDER_PICKER_STEP_MS),
    Some(REMINDER_PICKER_STEP_MS + 1 + REMIND_AT_HORIZON_MS)
)]
#[case::many_steps(
    7 * REMINDER_PICKER_STEP_MS + 12_345,
    Some(8 * REMINDER_PICKER_STEP_MS),
    Some(7 * REMINDER_PICKER_STEP_MS + 12_345 + REMIND_AT_HORIZON_MS)
)]
#[case::overflow(u64::MAX, None, None)]
fn picker_bounds_follow_the_clock(
    #[case] clock_ms: u64,
    #[case] anchor: Option<u64>,
    #[case] latest: Option<u64>,
) {
    assert_eq!(picker_anchor_ms(clock_ms), anchor);
    assert_eq!(picker_latest_ms(clock_ms), latest);
}

#[rstest]
#[case::first(1)]
#[case::mid(123_456_789)]
#[case::large(u64::MAX / 2)]
fn picker_anchor_is_a_step_multiple_within_one_step(#[case] clock_ms: u64) {
    let anchor = picker_anchor_ms(clock_ms).expect("anchor");
    assert_eq!(anchor % REMINDER_PICKER_STEP_MS, 0);
    assert!(anchor >= clock_ms);
    assert!(anchor - clock_ms < REMINDER_PICKER_STEP_MS);
}
