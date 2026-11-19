mod common;

use common::steward::StewardClient;
use common::{
    Person, SCENARIO_DEADLINE, Stage, expect_close, frames_until_pong, issue_steward, manage_on,
    mentioning, owner_socket, posted, steward_request, steward_row, subscribed_socket,
};
use pulse_protocol::{
    AccountId, AuthToken, ClientNonce, MessageId, PostMessageArguments, RoomId, ServerFrame,
    StewardAction, StewardWorkItem, ToolCallId, WORK_SYNC_LIMIT,
};
use rstest::rstest;
use tokio::time::timeout;

async fn mention(stage: &Stage, room_id: RoomId, steward_id: AccountId, text: &str) -> MessageId {
    let mut ada =
        subscribed_socket(stage.addr, stage.campus.id, &stage.ada.token, &[room_id]).await;
    posted(
        &mut ada,
        room_id,
        ClientNonce::generate(),
        &mentioning(steward_id, text),
    )
    .await
    .message_id
}

fn bodies(items: &[StewardWorkItem]) -> Vec<String> {
    items.iter().map(|item| item.message.body.clone()).collect()
}

struct Pair {
    first: (AccountId, AuthToken),
    second: (AccountId, AuthToken),
}

async fn pair(stage: &Stage, first_rooms: &[RoomId]) -> Pair {
    let owner = stage.campus.owner.account.account_id;
    let (first, first_token) =
        issue_steward(&stage.state, stage.campus.id, owner, "Scribe", first_rooms).await;
    let (second, second_token) = issue_steward(
        &stage.state,
        stage.campus.id,
        owner,
        "Herald",
        &stage.campus.rooms,
    )
    .await;
    Pair {
        first: (first.steward_id, first_token),
        second: (second.steward_id, second_token),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_mention_reaches_only_the_targeted_steward() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let pair = pair(&stage, &stage.campus.rooms).await;
        let mut first = StewardClient::connect(stage.addr, stage.campus.id, &pair.first.1).await;
        let mut second = StewardClient::connect(stage.addr, stage.campus.id, &pair.second.1).await;
        let message_id = mention(&stage, stage.room, pair.first.0, "please help").await;
        let item = first.next_work_item().await;
        assert_eq!(item.message.message_id, message_id);
        assert_eq!(item.message.campus_id, stage.campus.id);
        assert_eq!(item.message.room_id, stage.room);
        let quiet = frames_until_pong(&mut second.socket, 5).await;
        assert!(
            quiet
                .iter()
                .all(|frame| !matches!(frame, ServerFrame::StewardWorkItem(_)))
        );
        assert!(second.sync_work().await.items.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Quiet {
    HumanMention,
    UnassignedRoom,
    RevokedSteward,
    StewardAuthor,
}

#[rstest]
#[case::human_mention(Quiet::HumanMention)]
#[case::unassigned_room(Quiet::UnassignedRoom)]
#[case::revoked_steward(Quiet::RevokedSteward)]
#[case::steward_author(Quiet::StewardAuthor)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn some_mentions_never_become_work(#[case] quiet: Quiet) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let pair = pair(&stage, &stage.campus.rooms[..1]).await;
        let mut first = StewardClient::connect(stage.addr, stage.campus.id, &pair.first.1).await;
        match quiet {
            Quiet::HumanMention => {
                mention(&stage, stage.room, stage.id_of(Person::Grace), "hello").await;
            }
            Quiet::UnassignedRoom => {
                mention(&stage, stage.campus.rooms[1], pair.first.0, "elsewhere").await;
            }
            Quiet::RevokedSteward => {
                let mut admin = owner_socket(stage.addr, &stage.campus).await;
                manage_on(
                    &mut admin,
                    steward_request(StewardAction::Revoke, pair.first.0, &[]),
                )
                .await;
                mention(&stage, stage.room, pair.first.0, "too late").await;
            }
            Quiet::StewardAuthor => {
                let mut second =
                    StewardClient::connect(stage.addr, stage.campus.id, &pair.second.1).await;
                let result = second
                    .call_tool(
                        ToolCallId::generate(),
                        "post_message",
                        &PostMessageArguments {
                            room_id: stage.room,
                            client_nonce: ClientNonce::generate(),
                            body: mentioning(pair.first.0, "loop?"),
                            thread_root_id: None,
                        },
                    )
                    .await;
                assert!(!result.is_error, "{result:?}");
            }
        }
        if matches!(quiet, Quiet::RevokedSteward) {
            assert_eq!(
                expect_close(&mut first.socket).await,
                pulse_protocol::close_code::POLICY_VIOLATION
            );
            let backlog = pulse_server::steward::work_items(
                &stage.state.db,
                stage.campus.id,
                pair.first.0,
                WORK_SYNC_LIMIT,
            )
            .await
            .expect("work items");
            assert!(backlog.items.is_empty());
        } else {
            let seen = frames_until_pong(&mut first.socket, 6).await;
            assert!(
                seen.iter()
                    .all(|frame| !matches!(frame, ServerFrame::StewardWorkItem(_)))
            );
            assert!(first.sync_work().await.items.is_empty());
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_offline_steward_catches_up_and_acknowledges_monotonically() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let pair = pair(&stage, &stage.campus.rooms).await;
        let (steward_id, token) = pair.first;
        let mut ids = Vec::new();
        for index in 0..3 {
            ids.push(mention(&stage, stage.room, steward_id, &format!("job {index}")).await);
        }
        let mut steward = StewardClient::connect(stage.addr, stage.campus.id, &token).await;
        let synced = steward.sync_work().await;
        assert_eq!(bodies(&synced.items).len(), 3);
        assert_eq!(
            synced
                .items
                .iter()
                .map(|item| item.message.message_id)
                .collect::<Vec<_>>(),
            ids
        );
        assert!(!synced.has_more);
        steward.ack(ids[1]).await;
        let after_second = steward.sync_work().await;
        assert_eq!(
            after_second
                .items
                .iter()
                .map(|item| item.message.message_id)
                .collect::<Vec<_>>(),
            vec![ids[2]]
        );
        steward.ack(ids[2]).await;
        steward.ack(ids[0]).await;
        assert!(steward.sync_work().await.items.is_empty());
        let row = steward_row(&stage.state, stage.campus.id, steward_id)
            .await
            .expect("row");
        assert_eq!(row.work_cursor, Some(ids[2]));
        drop(steward);
        let mut again = StewardClient::connect(stage.addr, stage.campus.id, &token).await;
        assert!(again.sync_work().await.items.is_empty());
        let fresh = mention(&stage, stage.room, steward_id, "job 3").await;
        assert_eq!(again.next_work_item().await.message.message_id, fresh);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn backlogs_page_by_the_sync_limit() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let pair = pair(&stage, &stage.campus.rooms).await;
        let (steward_id, token) = pair.first;
        let mut ada =
            subscribed_socket(stage.addr, stage.campus.id, &stage.ada.token, &[stage.room]).await;
        let mut ids = Vec::new();
        for index in 0..=WORK_SYNC_LIMIT {
            ids.push(
                posted(
                    &mut ada,
                    stage.room,
                    ClientNonce::generate(),
                    &mentioning(steward_id, &format!("job {index}")),
                )
                .await
                .message_id,
            );
        }
        let mut steward = StewardClient::connect(stage.addr, stage.campus.id, &token).await;
        let first_page = steward.sync_work().await;
        assert_eq!(first_page.items.len(), WORK_SYNC_LIMIT);
        assert!(first_page.has_more);
        steward.ack(ids[WORK_SYNC_LIMIT - 1]).await;
        let second_page = steward.sync_work().await;
        assert_eq!(
            second_page
                .items
                .iter()
                .map(|item| item.message.message_id)
                .collect::<Vec<_>>(),
            vec![ids[WORK_SYNC_LIMIT]]
        );
        assert!(!second_page.has_more);
    })
    .await
    .expect("scenario deadline");
}
