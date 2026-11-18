mod common;

use std::collections::BTreeSet;

use common::{
    Person, SCENARIO_DEADLINE, Stage, campus_with_owner, collect_until, count_rows, expect_frame,
    frames_until_pong, grant_role, ms_from_now, now_ms, seed_messages, seed_shelf, send,
    shelf_frame, shelf_state_of, shelf_update_request, sibling_state, signed_up, sync_shelf_frame,
};
use pulse_protocol::{
    AccountId, MessageId, REMIND_AT_HORIZON_MS, Role, SHELF_ITEMS_MAX, ServerFrame, ShelfAction,
    ShelfChanged, ShelfItemId, ShelfRejection,
};
use pulse_server::shelf::{ShelfFailure, ShelfOutcome, deliver_due_reminders, update_shelf};
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::timeout;

const HOUR: i64 = 3_600_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expect {
    Changed { remind: bool, done: bool },
    Removed,
    Unchanged,
    Rejected(ShelfRejection),
}

#[derive(Clone, Copy, Debug)]
enum Who {
    Member,
    Stranger,
}

#[derive(Clone, Copy, Debug)]
enum Target {
    Message,
    Unknown,
}

fn expectation(outcome: Result<ShelfOutcome, ShelfFailure>) -> Expect {
    match outcome {
        Ok(ShelfOutcome::Changed(item)) => Expect::Changed {
            remind: item.remind_at_ms.is_some(),
            done: item.done_at_ms.is_some(),
        },
        Ok(ShelfOutcome::Removed(_)) => Expect::Removed,
        Ok(ShelfOutcome::Unchanged) => Expect::Unchanged,
        Err(ShelfFailure::Rejected(reason)) => Expect::Rejected(reason),
        Err(ShelfFailure::Database(error)) => panic!("database failure: {error}"),
    }
}

async fn shelve(
    stage: &Stage,
    account_id: AccountId,
    message_id: MessageId,
    action: ShelfAction,
    offset_ms: Option<i64>,
) -> Result<ShelfOutcome, ShelfFailure> {
    update_shelf(
        &stage.state.db,
        stage.campus.id,
        account_id,
        shelf_update_request(stage.room, message_id, action, offset_ms.map(ms_from_now)),
        now_ms(),
    )
    .await
}

const SAVED: (ShelfAction, Option<i64>) = (ShelfAction::Save, None);
const COMPLETED: (ShelfAction, Option<i64>) = (ShelfAction::Complete, None);

#[rstest]
#[case::plain_save(&[], ShelfAction::Save, None, Who::Member, Target::Message, Expect::Changed { remind: false, done: false })]
#[case::save_with_reminder(&[], ShelfAction::Save, Some(HOUR), Who::Member, Target::Message, Expect::Changed { remind: true, done: false })]
#[case::reminder_inside_past_tolerance(&[], ShelfAction::Save, Some(-60_000), Who::Member, Target::Message, Expect::Changed { remind: true, done: false })]
#[case::reminder_beyond_past_tolerance(&[], ShelfAction::Save, Some(-HOUR), Who::Member, Target::Message, Expect::Rejected(ShelfRejection::InvalidRemindAt))]
#[case::reminder_beyond_horizon(&[], ShelfAction::Save, Some(i64::try_from(REMIND_AT_HORIZON_MS).unwrap() + HOUR), Who::Member, Target::Message, Expect::Rejected(ShelfRejection::InvalidRemindAt))]
#[case::reschedule_needs_a_time(&[SAVED], ShelfAction::Reschedule, None, Who::Member, Target::Message, Expect::Rejected(ShelfRejection::InvalidRemindAt))]
#[case::reschedule_saved(&[SAVED], ShelfAction::Reschedule, Some(2 * HOUR), Who::Member, Target::Message, Expect::Changed { remind: true, done: false })]
#[case::save_again_clears_reminder(&[(ShelfAction::Save, Some(HOUR))], ShelfAction::Save, None, Who::Member, Target::Message, Expect::Changed { remind: false, done: false })]
#[case::complete_saved(&[SAVED], ShelfAction::Complete, None, Who::Member, Target::Message, Expect::Changed { remind: false, done: true })]
#[case::complete_twice(&[SAVED, COMPLETED], ShelfAction::Complete, None, Who::Member, Target::Message, Expect::Unchanged)]
#[case::complete_rejects_a_time(&[SAVED], ShelfAction::Complete, Some(HOUR), Who::Member, Target::Message, Expect::Rejected(ShelfRejection::InvalidRemindAt))]
#[case::reopen_done(&[SAVED, COMPLETED], ShelfAction::Reopen, None, Who::Member, Target::Message, Expect::Changed { remind: false, done: false })]
#[case::reopen_open(&[SAVED], ShelfAction::Reopen, None, Who::Member, Target::Message, Expect::Unchanged)]
#[case::remove_saved(&[SAVED], ShelfAction::Remove, None, Who::Member, Target::Message, Expect::Removed)]
#[case::complete_unsaved(&[], ShelfAction::Complete, None, Who::Member, Target::Message, Expect::Rejected(ShelfRejection::UnknownItem))]
#[case::reopen_unsaved(&[], ShelfAction::Reopen, None, Who::Member, Target::Message, Expect::Rejected(ShelfRejection::UnknownItem))]
#[case::remove_unsaved(&[], ShelfAction::Remove, None, Who::Member, Target::Message, Expect::Rejected(ShelfRejection::UnknownItem))]
#[case::save_unknown_message(&[], ShelfAction::Save, None, Who::Member, Target::Unknown, Expect::Rejected(ShelfRejection::UnknownMessage))]
#[case::non_member(&[], ShelfAction::Save, None, Who::Stranger, Target::Message, Expect::Rejected(ShelfRejection::Forbidden))]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shelf_actions_follow_the_item_state(
    #[case] priors: &[(ShelfAction, Option<i64>)],
    #[case] action: ShelfAction,
    #[case] offset_ms: Option<i64>,
    #[case] who: Who,
    #[case] target: Target,
    #[case] expected: Expect,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let message = seed_messages(&stage.state, stage.campus.id, stage.room, ada, 1).await[0];
        for (prior, prior_offset) in priors {
            shelve(&stage, ada, message, *prior, *prior_offset)
                .await
                .expect("prior action");
        }
        let actor = match who {
            Who::Member => ada,
            Who::Stranger => {
                signed_up(&stage.state, "stranger@example.org")
                    .await
                    .account
                    .account_id
            }
        };
        let aimed = match target {
            Target::Message => message,
            Target::Unknown => MessageId::generate(),
        };
        let outcome = expectation(shelve(&stage, actor, aimed, action, offset_ms).await);
        assert_eq!(outcome, expected);
        let row = shelf_state_of(&stage.state, stage.campus.id, ada, message).await;
        let expect_row = match expected {
            Expect::Removed => false,
            Expect::Rejected(_) => !priors.is_empty(),
            _ => true,
        };
        assert_eq!(row.is_some(), expect_row);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::fresh_item(false, 8, 8)]
#[case::already_saved(true, 8, 9)]
#[case::many_saves(true, 16, 17)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_saves_of_one_message_leave_one_row(
    #[case] presaved: bool,
    #[case] savers: usize,
    #[case] revision: i64,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let message = seed_messages(&stage.state, stage.campus.id, stage.room, ada, 1).await[0];
        if presaved {
            shelve(&stage, ada, message, ShelfAction::Save, None)
                .await
                .expect("presave");
        }
        let mut racing = JoinSet::new();
        for _ in 0..savers {
            let db = stage.state.db.clone();
            let (campus_id, room) = (stage.campus.id, stage.room);
            racing.spawn(async move {
                update_shelf(
                    &db,
                    campus_id,
                    ada,
                    shelf_update_request(room, message, ShelfAction::Save, None),
                    now_ms(),
                )
                .await
            });
        }
        while let Some(done) = racing.join_next().await {
            assert!(matches!(
                expectation(done.expect("save task")),
                Expect::Changed { .. }
            ));
        }
        let rows = count_rows(&stage.state, "shelf_items", stage.campus.id).await;
        assert_eq!(rows, 1);
        let state = shelf_state_of(&stage.state, stage.campus.id, ada, message).await;
        assert_eq!(state, Some((false, false, false, revision)));
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::one_free_slot(1, 8)]
#[case::three_free_slots(3, 8)]
#[case::full_shelf(0, 4)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_saves_never_exceed_the_shelf_cap(
    #[case] free_slots: usize,
    #[case] savers: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let held = SHELF_ITEMS_MAX - free_slots;
        let seeded = seed_messages(
            &stage.state,
            stage.campus.id,
            stage.room,
            ada,
            held + savers,
        )
        .await;
        let (shelved, contested) = seeded.split_at(held);
        seed_shelf(
            &stage.state,
            stage.campus.id,
            stage.room,
            ada,
            shelved,
            None,
        )
        .await;
        let mut racing = JoinSet::new();
        for target in contested.iter().copied() {
            let db = stage.state.db.clone();
            let (campus_id, room) = (stage.campus.id, stage.room);
            racing.spawn(async move {
                update_shelf(
                    &db,
                    campus_id,
                    ada,
                    shelf_update_request(room, target, ShelfAction::Save, None),
                    now_ms(),
                )
                .await
            });
        }
        let mut outcomes = Vec::new();
        while let Some(done) = racing.join_next().await {
            outcomes.push(expectation(done.expect("save task")));
        }
        let changed = outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Expect::Changed { .. }))
            .count();
        let full = outcomes
            .iter()
            .filter(|outcome| **outcome == Expect::Rejected(ShelfRejection::ShelfFull))
            .count();
        assert_eq!(changed, free_slots.min(savers));
        assert_eq!(full, savers - changed);
        let rows = count_rows(&stage.state, "shelf_items", stage.campus.id).await;
        assert_eq!(rows, i64::try_from(SHELF_ITEMS_MAX).expect("cap"));
    })
    .await
    .expect("scenario deadline");
}

fn reminders_of(frames: Vec<ServerFrame>) -> Vec<ShelfChanged> {
    frames
        .into_iter()
        .filter_map(|frame| match frame {
            ServerFrame::ShelfChanged(changed) => Some(changed),
            _ => None,
        })
        .collect()
}

#[rstest]
#[case::four_scanners_six_items(4, 6)]
#[case::eight_scanners_three_items(8, 3)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_scans_deliver_each_reminder_exactly_once(
    #[case] scanners: usize,
    #[case] items: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let seeded = seed_messages(&stage.state, stage.campus.id, stage.room, ada, items).await;
        seed_shelf(
            &stage.state,
            stage.campus.id,
            stage.room,
            ada,
            &seeded,
            Some(-1_000),
        )
        .await;
        let mut socket = stage.plain(Person::Ada).await;
        let mut racing = JoinSet::new();
        for _ in 0..scanners {
            let sibling = sibling_state(&stage.state);
            racing.spawn(async move { deliver_due_reminders(&sibling, 100).await });
        }
        let mut delivered = 0;
        while let Some(done) = racing.join_next().await {
            delivered += done.expect("scan task").expect("scan");
        }
        assert_eq!(delivered, items);
        let received = collect_until(
            &mut socket,
            |frame| match frame {
                ServerFrame::ShelfChanged(changed) => Some(changed),
                _ => None,
            },
            items,
        )
        .await;
        let mut extra = reminders_of(frames_until_pong(&mut socket, 31).await);
        let mut everything = received;
        everything.append(&mut extra);
        assert_eq!(everything.len(), items);
        let ids: BTreeSet<ShelfItemId> = everything
            .iter()
            .map(|changed| changed.item.shelf_item_id)
            .collect();
        assert_eq!(ids.len(), items);
        for changed in &everything {
            assert!(!changed.removed);
            assert!(changed.item.reminded_at_ms.is_some());
            assert_eq!(changed.item.revision, 2);
        }
        for message in seeded {
            assert_eq!(
                shelf_state_of(&stage.state, stage.campus.id, ada, message).await,
                Some((true, true, false, 2))
            );
        }
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::six_rounds(6)]
#[case::twenty_rounds(20)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn rescheduling_while_scanning_never_breaks_the_reminder_invariant(#[case] rounds: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let seeded = seed_messages(&stage.state, stage.campus.id, stage.room, ada, rounds).await;
        seed_shelf(
            &stage.state,
            stage.campus.id,
            stage.room,
            ada,
            &seeded,
            Some(-1_000),
        )
        .await;
        let mut racing = JoinSet::new();
        for message in seeded.iter().copied() {
            let db = stage.state.db.clone();
            let (campus_id, room) = (stage.campus.id, stage.room);
            racing.spawn(async move {
                update_shelf(
                    &db,
                    campus_id,
                    ada,
                    shelf_update_request(
                        room,
                        message,
                        ShelfAction::Reschedule,
                        Some(ms_from_now(HOUR)),
                    ),
                    now_ms(),
                )
                .await
                .expect("reschedule");
            });
        }
        let scans: Vec<_> = (0..rounds)
            .map(|_| {
                let sibling = sibling_state(&stage.state);
                tokio::spawn(async move { deliver_due_reminders(&sibling, 100).await })
            })
            .collect();
        while racing.join_next().await.is_some() {}
        let mut delivered = 0;
        for scan in scans {
            delivered += scan.await.expect("scan task").expect("scan");
        }
        let mut claimed_before_reschedule = 0;
        for message in seeded {
            let (remind, reminded, done, revision) =
                shelf_state_of(&stage.state, stage.campus.id, ada, message)
                    .await
                    .expect("item");
            assert!(remind && !reminded && !done);
            assert!(revision == 2 || revision == 3);
            if revision == 3 {
                claimed_before_reschedule += 1;
            }
        }
        assert_eq!(delivered, claimed_before_reschedule);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reminders_reach_only_the_owning_account_inside_its_campus() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let other = campus_with_owner(&stage.state, "far@example.org", "south-quad").await;
        grant_role(
            &stage.state,
            other.id,
            stage.id_of(Person::Ada),
            Role::Member,
        )
        .await;
        let (ada, grace) = (stage.id_of(Person::Ada), stage.id_of(Person::Grace));
        let messages = seed_messages(&stage.state, stage.campus.id, stage.room, ada, 2).await;
        let far_messages = seed_messages(&stage.state, other.id, other.rooms[0], ada, 1).await;
        seed_shelf(
            &stage.state,
            stage.campus.id,
            stage.room,
            ada,
            &messages[..1],
            Some(-1_000),
        )
        .await;
        seed_shelf(
            &stage.state,
            stage.campus.id,
            stage.room,
            grace,
            &messages[1..],
            Some(-1_000),
        )
        .await;
        seed_shelf(
            &stage.state,
            other.id,
            other.rooms[0],
            ada,
            &far_messages,
            Some(HOUR),
        )
        .await;
        let mut ada_here = stage.plain(Person::Ada).await;
        let mut grace_here = stage.plain(Person::Grace).await;
        let (mut ada_there, _) =
            common::joined_socket(stage.addr, other.id, &stage.ada.token).await;
        let sibling = sibling_state(&stage.state);
        assert_eq!(deliver_due_reminders(&sibling, 100).await.expect("scan"), 2);
        let mine = collect_until(
            &mut ada_here,
            |frame| match frame {
                ServerFrame::ShelfChanged(changed) => Some(changed),
                _ => None,
            },
            1,
        )
        .await;
        let hers = collect_until(
            &mut grace_here,
            |frame| match frame {
                ServerFrame::ShelfChanged(changed) => Some(changed),
                _ => None,
            },
            1,
        )
        .await;
        assert_eq!(mine[0].item.message.message_id, messages[0]);
        assert_eq!(hers[0].item.message.message_id, messages[1]);
        assert!(reminders_of(frames_until_pong(&mut ada_here, 41).await).is_empty());
        assert!(reminders_of(frames_until_pong(&mut grace_here, 42).await).is_empty());
        assert!(reminders_of(frames_until_pong(&mut ada_there, 43).await).is_empty());
        assert_eq!(
            shelf_state_of(&stage.state, other.id, ada, far_messages[0]).await,
            Some((true, false, false, 1))
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_offline_account_finds_its_reminder_stamped_on_the_next_sync() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (grace, linus) = (stage.id_of(Person::Grace), stage.id_of(Person::Linus));
        let message = seed_messages(&stage.state, stage.campus.id, stage.room, grace, 1).await;
        seed_shelf(
            &stage.state,
            stage.campus.id,
            stage.room,
            grace,
            &message,
            Some(-1_000),
        )
        .await;
        let sibling = sibling_state(&stage.state);
        assert_eq!(deliver_due_reminders(&sibling, 100).await.expect("scan"), 1);
        assert_eq!(deliver_due_reminders(&sibling, 100).await.expect("scan"), 0);
        let mut returning = stage.plain(Person::Grace).await;
        send(&mut returning, &sync_shelf_frame()).await;
        let synced = match expect_frame(&mut returning).await {
            ServerFrame::ShelfSynced(synced) => synced,
            other => panic!("expected shelf sync, got {other:?}"),
        };
        assert_eq!(synced.items.len(), 1);
        let item = &synced.items[0];
        assert_eq!(item.message.message_id, message[0]);
        assert!(item.reminded_at_ms.is_some());
        assert_eq!(item.revision, 2);
        let mut bystander = stage.plain(Person::Linus).await;
        send(&mut bystander, &sync_shelf_frame()).await;
        match expect_frame(&mut bystander).await {
            ServerFrame::ShelfSynced(synced) => assert!(synced.items.is_empty()),
            other => panic!("expected shelf sync, got {other:?}"),
        }
        assert_eq!(
            shelf_state_of(&stage.state, stage.campus.id, linus, message[0]).await,
            None
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn socket_actions_echo_to_the_same_account_only() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let message = seed_messages(&stage.state, stage.campus.id, stage.room, ada, 1).await[0];
        let mut origin = stage.plain(Person::Ada).await;
        let mut twin = stage.plain(Person::Ada).await;
        let mut other = stage.plain(Person::Grace).await;
        let room = stage.room;
        send(
            &mut origin,
            &shelf_frame(room, message, ShelfAction::Save, Some(ms_from_now(HOUR))),
        )
        .await;
        let saved = collect_until(
            &mut twin,
            |frame| match frame {
                ServerFrame::ShelfChanged(changed) => Some(changed),
                _ => None,
            },
            1,
        )
        .await;
        assert!(!saved[0].removed);
        assert_eq!(saved[0].item.revision, 1);
        assert!(saved[0].item.remind_at_ms.is_some());
        send(
            &mut origin,
            &shelf_frame(room, message, ShelfAction::Remove, None),
        )
        .await;
        let removed = collect_until(
            &mut twin,
            |frame| match frame {
                ServerFrame::ShelfChanged(changed) => Some(changed),
                _ => None,
            },
            1,
        )
        .await;
        assert!(removed[0].removed);
        send(
            &mut origin,
            &shelf_frame(room, MessageId::generate(), ShelfAction::Save, None),
        )
        .await;
        let rejection = common::next_of(&mut origin, |frame| match frame {
            ServerFrame::ShelfRejected(rejected) => Some(rejected.reason),
            _ => None,
        })
        .await;
        assert_eq!(rejection, ShelfRejection::UnknownMessage);
        assert!(reminders_of(frames_until_pong(&mut other, 51).await).is_empty());
        assert!(reminders_of(frames_until_pong(&mut twin, 52).await).is_empty());
    })
    .await
    .expect("scenario deadline");
}
