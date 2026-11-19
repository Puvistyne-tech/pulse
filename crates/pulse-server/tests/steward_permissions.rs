mod common;

use common::{
    AUTH_DEADLINE, ClientSocket, Observed, Outgoing, SCENARIO_DEADLINE, Stage, ack_work_frame,
    call_tool_frame, create_steward_request, issue_steward, join_lounge_frame, joined_socket,
    leave_lounge_frame, list_tools_frame, lounge_heartbeat_frame, manage_frame, mark_read,
    member_session, observe, pin_frame, ping, posted, react, send, send_message, set_subscription,
    set_title, shelf_frame, stream_chunk_frame, sync_inbox, sync_lounges_frame, sync_rooms,
    sync_shelf_frame, sync_stewards_frame, sync_thread, sync_work_frame, vault_frame,
};
use pulse_protocol::{
    AuthToken, ClientNonce, LoungeId, LoungeRejection, MessageId, MessageRejection, ReactionAction,
    ReactionRejection, ReadRoomHistoryArguments, Role, RoomId, ServerFrame, StewardRejection,
    StewardTool, StreamId, StreamRejection, StreamWriter, ThreadRejection, ToolCallId, VaultTab,
    close_code,
};
use rstest::rstest;
use tokio::time::timeout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Actor {
    Owner,
    Admin,
    Member,
    Guest,
    Assigned,
    Unassigned,
}

const ACTORS: [Actor; 6] = [
    Actor::Owner,
    Actor::Admin,
    Actor::Member,
    Actor::Guest,
    Actor::Assigned,
    Actor::Unassigned,
];

#[derive(Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Silent,
    Accepted,
    MessageRejected(MessageRejection),
    ReactionRejected(ReactionRejection),
    RoomSynced,
    RoomUnavailable,
    ThreadSynced,
    ThreadRejected(ThreadRejection),
    StreamRejected(StreamRejection),
    ToolsListed,
    ToolOk,
    ToolError(&'static str),
    WorkSynced(usize),
    Issued,
    StewardRejected(StewardRejection),
    StewardsSynced,
    LoungesSynced,
    LoungeJoined,
    LoungeRejected(LoungeRejection),
    InboxSynced,
    ShelfSynced,
    Close(u16),
}

use Outcome::{
    Accepted, Close, InboxSynced, Issued, LoungeJoined, LoungeRejected, LoungesSynced,
    MessageRejected, ReactionRejected, RoomSynced, RoomUnavailable, ShelfSynced, Silent,
    StewardRejected, StewardsSynced, StreamRejected, ThreadRejected, ThreadSynced, ToolError,
    ToolOk, ToolsListed, WorkSynced,
};

const POLICY: u16 = close_code::POLICY_VIOLATION;

fn classify(frame: &ServerFrame) -> Option<Outcome> {
    Some(match frame {
        ServerFrame::MessageAccepted(_) => Accepted,
        ServerFrame::MessageRejected(rejected) => MessageRejected(rejected.reason),
        ServerFrame::ReactionRejected(rejected) => ReactionRejected(rejected.reason),
        ServerFrame::RoomSynced(_) => RoomSynced,
        ServerFrame::RoomUnavailable(_) => RoomUnavailable,
        ServerFrame::ThreadSynced(_) => ThreadSynced,
        ServerFrame::ThreadRejected(rejected) => ThreadRejected(rejected.reason),
        ServerFrame::StreamRejected(rejected) => StreamRejected(rejected.reason),
        ServerFrame::ToolsListed(_) => ToolsListed,
        ServerFrame::ToolResult(result) if result.is_error => {
            ToolError(match result.content[0].text.as_str() {
                "forbidden" => "forbidden",
                "unknown_room" => "unknown_room",
                _ => "other",
            })
        }
        ServerFrame::ToolResult(_) => ToolOk,
        ServerFrame::WorkSynced(synced) => WorkSynced(synced.items.len()),
        ServerFrame::StewardIssued(_) => Issued,
        ServerFrame::StewardRejected(rejected) => StewardRejected(rejected.reason),
        ServerFrame::StewardsSynced(_) => StewardsSynced,
        ServerFrame::LoungesSynced(_) => LoungesSynced,
        ServerFrame::LoungeJoined(_) => LoungeJoined,
        ServerFrame::LoungeRejected(rejected) => LoungeRejected(rejected.reason),
        ServerFrame::InboxSynced(_) => InboxSynced,
        ServerFrame::ShelfSynced(_) => ShelfSynced,
        _ => return None,
    })
}

async fn probe(socket: &mut ClientSocket, outgoing: &Outgoing) -> Outcome {
    send(socket, outgoing).await;
    send(socket, &ping(77)).await;
    let mut first = None;
    loop {
        match timeout(AUTH_DEADLINE, observe(socket))
            .await
            .expect("probe deadline")
        {
            Observed::Close(code) => return first.unwrap_or(Close(code)),
            Observed::Frame(ServerFrame::Pong(_)) => return first.unwrap_or(Silent),
            Observed::Frame(frame) => {
                if first.is_none() {
                    first = classify(&frame);
                }
            }
        }
    }
}

struct Arena {
    stage: Stage,
    admin_token: AuthToken,
    assigned_token: AuthToken,
    unassigned_token: AuthToken,
    room: RoomId,
    root: MessageId,
    campus_lounge: LoungeId,
}

impl Arena {
    async fn open() -> Self {
        let stage = Stage::open().await;
        let room = stage.campus.rooms[0];
        let other_room = stage.campus.rooms[1];
        let owner = stage.campus.owner.account.account_id;
        let admin = member_session(
            &stage.state,
            "admin@example.org",
            stage.campus.id,
            Role::Admin,
        )
        .await;
        let (_, assigned_token) =
            issue_steward(&stage.state, stage.campus.id, owner, "Assigned", &[room]).await;
        let (_, unassigned_token) = issue_steward(
            &stage.state,
            stage.campus.id,
            owner,
            "Unassigned",
            &[other_room],
        )
        .await;
        let mut ada = stage.watching(common::Person::Ada).await;
        let root = posted(&mut ada, room, ClientNonce::generate(), "root")
            .await
            .message_id;
        let campus_lounge = common::campus_lounge(&stage.state, stage.campus.id).await;
        Self {
            admin_token: admin.token,
            stage,
            assigned_token,
            unassigned_token,
            room,
            root,
            campus_lounge,
        }
    }

    async fn socket(&self, actor: Actor) -> ClientSocket {
        let token = match actor {
            Actor::Owner => &self.stage.campus.owner.token,
            Actor::Admin => &self.admin_token,
            Actor::Member => &self.stage.ada.token,
            Actor::Guest => &self.stage.guest.token,
            Actor::Assigned => &self.assigned_token,
            Actor::Unassigned => &self.unassigned_token,
        };
        joined_socket(self.stage.addr, self.stage.campus.id, token)
            .await
            .0
    }
}

#[derive(Clone, Copy, Debug)]
enum Probe {
    Send,
    React,
    SyncRooms,
    SyncThread,
    StreamBegin,
    ListTools,
    ReadHistory,
    SyncWork,
    AckWork,
    ManageCreate,
    SyncStewards,
    SyncLounges,
    Join,
    HeartbeatUnjoined,
    LeaveUnjoined,
    SyncInbox,
    SyncShelf,
    Hello,
}

fn outgoing(probe: Probe, arena: &Arena) -> Outgoing {
    let room = arena.room;
    match probe {
        Probe::Send => send_message(room, ClientNonce::generate(), "probe"),
        Probe::React => react(room, arena.root, "👍", ReactionAction::Add),
        Probe::SyncRooms => sync_rooms(&[(room, None)]),
        Probe::SyncThread => sync_thread(room, arena.root, None),
        Probe::StreamBegin => {
            stream_chunk_frame(StreamWriter::new(StreamId::generate(), room, None).begin())
        }
        Probe::ListTools => list_tools_frame(),
        Probe::ReadHistory => call_tool_frame(
            ToolCallId::generate(),
            StewardTool::ReadRoomHistory.name(),
            &serde_json::to_string(&ReadRoomHistoryArguments {
                room_id: room,
                before: None,
                limit: None,
            })
            .expect("tool arguments"),
        ),
        Probe::SyncWork => sync_work_frame(),
        Probe::AckWork => ack_work_frame(arena.root),
        Probe::ManageCreate => manage_frame(create_steward_request("Probe", &[room])),
        Probe::SyncStewards => sync_stewards_frame(),
        Probe::SyncLounges => sync_lounges_frame(),
        Probe::Join => join_lounge_frame(arena.campus_lounge),
        Probe::HeartbeatUnjoined => lounge_heartbeat_frame(arena.campus_lounge),
        Probe::LeaveUnjoined => leave_lounge_frame(arena.campus_lounge),
        Probe::SyncInbox => sync_inbox(),
        Probe::SyncShelf => sync_shelf_frame(),
        Probe::Hello => common::hello(
            arena.stage.campus.id,
            pulse_protocol::ProtocolVersion::CURRENT.0,
            &arena.stage.ada.token,
        ),
    }
}

const FORBID: StreamRejection = StreamRejection::Forbidden;

#[rstest]
#[case::send(Probe::Send, [Accepted, Accepted, Accepted, MessageRejected(MessageRejection::Forbidden), Accepted, MessageRejected(MessageRejection::Forbidden)])]
#[case::react(Probe::React, [Silent, Silent, Silent, Silent, Silent, ReactionRejected(ReactionRejection::Forbidden)])]
#[case::sync_rooms(Probe::SyncRooms, [RoomSynced, RoomSynced, RoomSynced, RoomSynced, RoomSynced, RoomUnavailable])]
#[case::sync_thread(Probe::SyncThread, [ThreadSynced, ThreadSynced, ThreadSynced, ThreadSynced, ThreadSynced, ThreadRejected(ThreadRejection::Forbidden)])]
#[case::stream_begin(Probe::StreamBegin, [StreamRejected(FORBID), StreamRejected(FORBID), StreamRejected(FORBID), StreamRejected(FORBID), Silent, StreamRejected(FORBID)])]
#[case::list_tools(Probe::ListTools, [ToolsListed, ToolsListed, ToolsListed, ToolsListed, ToolsListed, ToolsListed])]
#[case::read_history(Probe::ReadHistory, [ToolError("forbidden"), ToolError("forbidden"), ToolError("forbidden"), ToolError("forbidden"), ToolOk, ToolError("unknown_room")])]
#[case::sync_work(Probe::SyncWork, [WorkSynced(0), WorkSynced(0), WorkSynced(0), WorkSynced(0), WorkSynced(0), WorkSynced(0)])]
#[case::ack_work(Probe::AckWork, [Silent, Silent, Silent, Silent, Silent, Silent])]
#[case::manage_create(Probe::ManageCreate, [Issued, Issued, StewardRejected(StewardRejection::Forbidden), StewardRejected(StewardRejection::Forbidden), Close(POLICY), Close(POLICY)])]
#[case::sync_stewards(Probe::SyncStewards, [StewardsSynced, StewardsSynced, StewardsSynced, StewardsSynced, Close(POLICY), Close(POLICY)])]
#[case::sync_lounges(Probe::SyncLounges, [LoungesSynced, LoungesSynced, LoungesSynced, LoungesSynced, Close(POLICY), Close(POLICY)])]
#[case::join(Probe::Join, [LoungeJoined, LoungeJoined, LoungeJoined, LoungeJoined, Close(POLICY), Close(POLICY)])]
#[case::heartbeat_unjoined(Probe::HeartbeatUnjoined, [LoungeRejected(LoungeRejection::UnknownLounge), LoungeRejected(LoungeRejection::UnknownLounge), LoungeRejected(LoungeRejection::UnknownLounge), LoungeRejected(LoungeRejection::UnknownLounge), Close(POLICY), Close(POLICY)])]
#[case::leave_unjoined(Probe::LeaveUnjoined, [Silent, Silent, Silent, Silent, Close(POLICY), Close(POLICY)])]
#[case::sync_inbox(Probe::SyncInbox, [InboxSynced, InboxSynced, InboxSynced, InboxSynced, Close(POLICY), Close(POLICY)])]
#[case::sync_shelf(Probe::SyncShelf, [ShelfSynced, ShelfSynced, ShelfSynced, ShelfSynced, Close(POLICY), Close(POLICY)])]
#[case::hello_again(Probe::Hello, [Close(POLICY), Close(POLICY), Close(POLICY), Close(POLICY), Close(POLICY), Close(POLICY)])]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn every_frame_gets_its_exact_outcome_per_role(
    #[case] frame: Probe,
    #[case] expected: [Outcome; 6],
) {
    timeout(SCENARIO_DEADLINE, async {
        let arena = Arena::open().await;
        for (actor, want) in ACTORS.into_iter().zip(expected) {
            let mut socket = arena.socket(actor).await;
            let got = probe(&mut socket, &outgoing(frame, &arena)).await;
            assert_eq!(got, want, "{frame:?} as {actor:?}");
        }
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Outside {
    SetTitle,
    SetSubscription,
    MarkRead,
    SetPin,
    SyncVault,
    UpdateShelf,
}

#[rstest]
#[case::set_title(Outside::SetTitle)]
#[case::set_subscription(Outside::SetSubscription)]
#[case::mark_read(Outside::MarkRead)]
#[case::set_pin(Outside::SetPin)]
#[case::sync_vault(Outside::SyncVault)]
#[case::update_shelf(Outside::UpdateShelf)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn stewards_are_closed_on_frames_outside_their_allowlist(#[case] frame: Outside) {
    timeout(AUTH_DEADLINE, async {
        let arena = Arena::open().await;
        let (room, root) = (arena.room, arena.root);
        for actor in [Actor::Assigned, Actor::Unassigned] {
            let outgoing = match frame {
                Outside::SetTitle => set_title(room, root, "title"),
                Outside::SetSubscription => set_subscription(
                    room,
                    root,
                    pulse_protocol::SubscriptionState::Following,
                    pulse_protocol::SubscriptionOrigin::Explicit,
                ),
                Outside::MarkRead => mark_read(room, root, root),
                Outside::SetPin => pin_frame(room, root, true),
                Outside::SyncVault => vault_frame(room, VaultTab::Files, None),
                Outside::UpdateShelf => {
                    shelf_frame(room, root, pulse_protocol::ShelfAction::Save, None)
                }
            };
            let mut socket = arena.socket(actor).await;
            assert_eq!(
                probe(&mut socket, &outgoing).await,
                Close(POLICY),
                "{frame:?} as {actor:?}"
            );
        }
    })
    .await
    .expect("scenario deadline");
}
