mod common;

use common::steward::StewardClient;
use common::{
    ClientSocket, Person, SCENARIO_DEADLINE, Stage, authored_count, call_tool_frame, issue_steward,
    list_tools_frame, next_of, posted, replied, send, subscribed_socket,
};
use pulse_protocol::{
    AccountId, ClientNonce, HistoryRead, MessageId, MessagePosted, PostMessageArguments,
    ReadRoomHistoryArguments, ReadThreadArguments, RoomId, RoomMessage, RoomsListed, ServerFrame,
    StewardTool, TOOL_ARGUMENTS_MAX_BYTES, TOOL_HISTORY_LIMIT_MAX, ToolCallId, ToolContentKind,
    ToolResult, tool_descriptors,
};
use rstest::rstest;
use serde::Serialize;
use tokio::time::timeout;

struct Fix {
    stage: Stage,
    steward: StewardClient,
    steward_id: AccountId,
    room: RoomId,
    other_room: RoomId,
    tops: Vec<RoomMessage>,
    reply: RoomMessage,
    foreign_root: RoomMessage,
}

impl Fix {
    async fn open() -> Self {
        let stage = Stage::open().await;
        let room = stage.campus.rooms[0];
        let other_room = stage.campus.rooms[1];
        let (profile, token) = issue_steward(
            &stage.state,
            stage.campus.id,
            stage.campus.owner.account.account_id,
            "Scribe",
            &[room],
        )
        .await;
        let steward = StewardClient::connect(stage.addr, stage.campus.id, &token).await;
        let mut ada = subscribed_socket(
            stage.addr,
            stage.campus.id,
            &stage.ada.token,
            &[room, other_room],
        )
        .await;
        let mut tops = Vec::new();
        for body in ["first", "second", "third"] {
            tops.push(posted(&mut ada, room, ClientNonce::generate(), body).await);
        }
        let reply = replied(
            &mut ada,
            room,
            tops[0].message_id,
            ClientNonce::generate(),
            "answer",
        )
        .await;
        let foreign_root = posted(&mut ada, other_room, ClientNonce::generate(), "hidden").await;
        Self {
            stage,
            steward,
            steward_id: profile.steward_id,
            room,
            other_room,
            tops,
            reply,
            foreign_root,
        }
    }
}

fn json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("arguments")
}

fn history(room_id: RoomId, before: Option<MessageId>, limit: Option<u32>) -> String {
    json(&ReadRoomHistoryArguments {
        room_id,
        before,
        limit,
    })
}

fn post(room_id: RoomId, body: &str, thread_root_id: Option<MessageId>) -> String {
    json(&PostMessageArguments {
        room_id,
        client_nonce: ClientNonce::generate(),
        body: body.to_owned(),
        thread_root_id,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expect {
    Error(&'static str),
    Rooms(usize),
    History(usize, bool),
    Thread(usize),
    Posted,
}

#[rstest]
#[case::list_rooms_happy("list_rooms", |_: &Fix| "{}".to_owned(), Expect::Rooms(1))]
#[case::list_rooms_wrong_shape("list_rooms", |_: &Fix| "[1]".to_owned(), Expect::Error("invalid_arguments"))]
#[case::list_rooms_not_json("list_rooms", |_: &Fix| "nope".to_owned(), Expect::Error("invalid_arguments"))]
#[case::history_happy("read_room_history", |f: &Fix| history(f.room, None, None), Expect::History(3, false))]
#[case::history_limited("read_room_history", |f: &Fix| history(f.room, None, Some(2)), Expect::History(2, true))]
#[case::history_limit_is_clamped(
    "read_room_history",
    |f: &Fix| history(f.room, None, Some(TOOL_HISTORY_LIMIT_MAX + 450)),
    Expect::History(3, false)
)]
#[case::history_before(
    "read_room_history",
    |f: &Fix| history(f.room, Some(f.tops[2].message_id), None),
    Expect::History(2, false)
)]
#[case::history_zero_limit("read_room_history", |f: &Fix| history(f.room, None, Some(0)), Expect::Error("invalid_arguments"))]
#[case::history_missing_room("read_room_history", |_: &Fix| "{}".to_owned(), Expect::Error("invalid_arguments"))]
#[case::history_unassigned_room(
    "read_room_history",
    |f: &Fix| history(f.other_room, None, None),
    Expect::Error("unknown_room")
)]
#[case::history_unknown_room(
    "read_room_history",
    |_: &Fix| history(RoomId::generate(), None, None),
    Expect::Error("unknown_room")
)]
#[case::thread_happy(
    "read_thread",
    |f: &Fix| json(&ReadThreadArguments { room_id: f.room, root_message_id: f.tops[0].message_id }),
    Expect::Thread(2)
)]
#[case::thread_unknown_root(
    "read_thread",
    |f: &Fix| json(&ReadThreadArguments { room_id: f.room, root_message_id: MessageId::generate() }),
    Expect::Error("unknown_thread")
)]
#[case::thread_root_of_another_room(
    "read_thread",
    |f: &Fix| json(&ReadThreadArguments { room_id: f.room, root_message_id: f.foreign_root.message_id }),
    Expect::Error("unknown_thread")
)]
#[case::thread_unassigned_room(
    "read_thread",
    |f: &Fix| json(&ReadThreadArguments { room_id: f.other_room, root_message_id: f.foreign_root.message_id }),
    Expect::Error("unknown_room")
)]
#[case::thread_missing_root(
    "read_thread",
    |f: &Fix| format!("{{\"room_id\":\"{}\"}}", f.room),
    Expect::Error("invalid_arguments")
)]
#[case::post_happy("post_message", |f: &Fix| post(f.room, "hello there", None), Expect::Posted)]
#[case::post_reply("post_message", |f: &Fix| post(f.room, "in thread", Some(f.tops[1].message_id)), Expect::Posted)]
#[case::post_empty_body("post_message", |f: &Fix| post(f.room, "   ", None), Expect::Error("invalid_arguments"))]
#[case::post_unassigned_room("post_message", |f: &Fix| post(f.other_room, "hi", None), Expect::Error("unknown_room"))]
#[case::post_unknown_thread(
    "post_message",
    |f: &Fix| post(f.room, "hi", Some(MessageId::generate())),
    Expect::Error("unknown_thread")
)]
#[case::post_missing_body(
    "post_message",
    |f: &Fix| format!("{{\"room_id\":\"{}\",\"client_nonce\":\"{}\"}}", f.room, ClientNonce::generate()),
    Expect::Error("invalid_arguments")
)]
#[case::unknown_tool("search_messages", |_: &Fix| "{}".to_owned(), Expect::Error("unknown_tool"))]
#[case::empty_tool_name("", |_: &Fix| "{}".to_owned(), Expect::Error("unknown_tool"))]
#[case::oversized_arguments(
    "post_message",
    |f: &Fix| post(f.room, &"x".repeat(TOOL_ARGUMENTS_MAX_BYTES), None),
    Expect::Error("invalid_arguments")
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn steward_calls_return_mcp_shaped_results(
    #[case] name: &str,
    #[case] arguments: fn(&Fix) -> String,
    #[case] expect: Expect,
) {
    timeout(SCENARIO_DEADLINE, async {
        let mut fix = Fix::open().await;
        let call_id = ToolCallId::generate();
        let before = authored_count(
            &fix.stage.state,
            fix.stage.campus.id,
            fix.room,
            fix.steward_id,
        )
        .await;
        let text = arguments(&fix);
        let result = fix.steward.call_tool_raw(call_id, name, &text).await;
        assert_eq!(result.call_id, call_id);
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.content[0].kind, ToolContentKind::Text);
        let after = authored_count(
            &fix.stage.state,
            fix.stage.campus.id,
            fix.room,
            fix.steward_id,
        )
        .await;
        match expect {
            Expect::Error(code) => {
                assert!(result.is_error, "{result:?}");
                assert_eq!(result.content[0].text, code);
                assert_eq!(result.structured_content, None);
                assert_eq!(after, before);
            }
            ok => {
                assert!(!result.is_error, "{result:?}");
                assert_eq!(
                    result.structured_content.as_deref(),
                    Some(result.content[0].text.as_str())
                );
                check_ok(&fix, ok, &result, after - before);
            }
        }
    })
    .await
    .expect("scenario deadline");
}

fn check_ok(fix: &Fix, expect: Expect, result: &ToolResult, new_rows: i64) {
    let text = &result.content[0].text;
    match expect {
        Expect::Rooms(count) => {
            let listed = serde_json::from_str::<RoomsListed>(text).expect("rooms");
            assert_eq!(listed.rooms.len(), count);
            assert!(
                listed
                    .rooms
                    .iter()
                    .all(|room| room.room_id == fix.room && room.campus_id == fix.stage.campus.id)
            );
            assert_eq!(new_rows, 0);
        }
        Expect::History(count, has_more) => {
            let read = serde_json::from_str::<HistoryRead>(text).expect("history");
            assert_eq!(read.messages.len(), count);
            assert_eq!(read.has_more, has_more);
            assert!(
                read.messages
                    .iter()
                    .all(|message| message.room_id == fix.room && message.thread_root_id.is_none())
            );
            assert_eq!(new_rows, 0);
        }
        Expect::Thread(count) => {
            let read = serde_json::from_str::<HistoryRead>(text).expect("thread");
            assert_eq!(read.messages.len(), count);
            assert!(
                read.messages
                    .iter()
                    .any(|message| message.message_id == fix.reply.message_id)
            );
            assert_eq!(new_rows, 0);
        }
        Expect::Posted => {
            let posted = serde_json::from_str::<MessagePosted>(text).expect("posted");
            assert_eq!(posted.message.author_id, fix.steward_id);
            assert_eq!(posted.message.campus_id, fix.stage.campus.id);
            assert_eq!(new_rows, 1);
        }
        Expect::Error(_) => unreachable!(),
    }
}

async fn human_call(socket: &mut ClientSocket, call_id: ToolCallId, name: &str) -> ToolResult {
    send(socket, &call_tool_frame(call_id, name, "{}")).await;
    next_of(socket, |frame| match frame {
        ServerFrame::ToolResult(result) if result.call_id == call_id => Some(result.clone()),
        _ => None,
    })
    .await
}

#[rstest]
#[case::list_rooms(StewardTool::ListRooms)]
#[case::read_room_history(StewardTool::ReadRoomHistory)]
#[case::read_thread(StewardTool::ReadThread)]
#[case::post_message(StewardTool::PostMessage)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn humans_cannot_call_steward_tools(#[case] tool: StewardTool) {
    timeout(SCENARIO_DEADLINE, async {
        let fix = Fix::open().await;
        for person in [Person::Ada, Person::Guest] {
            let mut socket = fix.stage.plain(person).await;
            let call_id = ToolCallId::generate();
            let result = human_call(&mut socket, call_id, tool.name()).await;
            assert!(result.is_error);
            assert_eq!(result.content[0].text, "forbidden");
        }
        let mut owner = fix.stage.poster().await;
        let result = human_call(&mut owner, ToolCallId::generate(), tool.name()).await;
        assert!(result.is_error);
        assert_eq!(result.content[0].text, "forbidden");
        assert_eq!(
            authored_count(
                &fix.stage.state,
                fix.stage.campus.id,
                fix.room,
                fix.stage.id_of(Person::Ada)
            )
            .await,
            4
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn listing_matches_the_shared_catalogue_for_everyone() {
    timeout(SCENARIO_DEADLINE, async {
        let mut fix = Fix::open().await;
        assert_eq!(fix.steward.list_tools().await, tool_descriptors());
        let mut ada = fix.stage.plain(Person::Ada).await;
        send(&mut ada, &list_tools_frame()).await;
        let tools = next_of(&mut ada, |frame| match frame {
            ServerFrame::ToolsListed(listed) => Some(listed.tools.clone()),
            _ => None,
        })
        .await;
        assert_eq!(tools, tool_descriptors());
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_posted_message_reaches_room_subscribers_once() {
    timeout(SCENARIO_DEADLINE, async {
        let mut fix = Fix::open().await;
        let mut watcher = subscribed_socket(
            fix.stage.addr,
            fix.stage.campus.id,
            &fix.stage.grace.token,
            &[fix.room],
        )
        .await;
        let arguments = PostMessageArguments {
            room_id: fix.room,
            client_nonce: ClientNonce::generate(),
            body: "from a steward".to_owned(),
            thread_root_id: None,
        };
        let first = fix
            .steward
            .call_tool(ToolCallId::generate(), "post_message", &arguments)
            .await;
        let second = fix
            .steward
            .call_tool(ToolCallId::generate(), "post_message", &arguments)
            .await;
        assert_eq!(first.content, second.content);
        let created = next_of(&mut watcher, |frame| match frame {
            ServerFrame::MessageCreated(created) => Some(created.message.clone()),
            _ => None,
        })
        .await;
        assert_eq!(created.body, "from a steward");
        assert_eq!(created.client_nonce, arguments.client_nonce);
        let rest = common::frames_until_pong(&mut watcher, 2).await;
        assert!(
            rest.iter()
                .all(|frame| !matches!(frame, ServerFrame::MessageCreated(_)))
        );
    })
    .await
    .expect("scenario deadline");
}
