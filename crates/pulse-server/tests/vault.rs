mod common;

use common::{
    Person, SCENARIO_DEADLINE, Stage, expect_frame, extra_members, frames_until_pong, next_of,
    pin_count, pin_frame, room_activity, seed_attachments, seed_links, seed_messages, seed_pins,
    send, sync_rooms_at, vault_frame,
};
use pulse_protocol::{
    AccountId, AttachmentKind, MessageId, PinRejection, ROOM_PINS_MAX, RoomId, ServerFrame, SetPin,
    VAULT_PAGE_LIMIT, VaultItem, VaultRejection, VaultTab,
};
use pulse_server::pin::{PinFailure, PinOutcome, set_pin};
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::timeout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expect {
    Changed,
    Unchanged,
    Rejected(PinRejection),
}

fn expectation(outcome: Result<PinOutcome, PinFailure>) -> Expect {
    match outcome {
        Ok(PinOutcome::Changed(_)) => Expect::Changed,
        Ok(PinOutcome::Unchanged(_)) => Expect::Unchanged,
        Err(PinFailure::Rejected(reason)) => Expect::Rejected(reason),
        Err(PinFailure::Database(error)) => panic!("database failure: {error}"),
    }
}

async fn pin_as(
    stage: &Stage,
    account_id: AccountId,
    room_id: RoomId,
    message_id: MessageId,
    pinned: bool,
) -> Expect {
    expectation(
        set_pin(
            &stage.state.db,
            stage.campus.id,
            account_id,
            SetPin {
                room_id,
                message_id,
                pinned,
            },
        )
        .await,
    )
}

async fn vault_page(
    stage: &Stage,
    person: Person,
    tab: VaultTab,
    before: Option<MessageId>,
) -> pulse_protocol::VaultSynced {
    let mut socket = stage.plain(person).await;
    send(&mut socket, &vault_frame(stage.room, tab, before)).await;
    match expect_frame(&mut socket).await {
        ServerFrame::VaultSynced(synced) => synced,
        other => panic!("expected vault page, got {other:?}"),
    }
}

fn ids_of(items: &[VaultItem]) -> Vec<MessageId> {
    items.iter().map(|item| item.message.message_id).collect()
}

#[rstest]
#[case::media(VaultTab::Media, &[3, 2, 0])]
#[case::files(VaultTab::Files, &[1])]
#[case::links(VaultTab::Links, &[5, 4])]
#[case::pinned(VaultTab::Pinned, &[3, 1])]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn each_tab_lists_its_own_items_newest_first(
    #[case] tab: VaultTab,
    #[case] expected: &[usize],
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (campus_id, room) = (stage.campus.id, stage.room);
        let author = stage.id_of(Person::Ada);
        let seeded = seed_messages(&stage.state, campus_id, room, author, 6).await;
        seed_attachments(
            &stage.state,
            campus_id,
            room,
            author,
            &[
                (seeded[0], "image/png", "image"),
                (seeded[1], "application/pdf", "file"),
                (seeded[2], "audio/mpeg", "audio"),
                (seeded[3], "video/mp4", "video"),
            ],
        )
        .await;
        seed_links(&stage.state, campus_id, room, &seeded[4..]).await;
        seed_pins(&stage.state, campus_id, room, author, &seeded[1..2]).await;
        seed_pins(&stage.state, campus_id, room, author, &seeded[3..4]).await;
        let page = vault_page(&stage, Person::Grace, tab, None).await;
        assert_eq!(page.room_id, room);
        assert_eq!(page.tab, tab);
        assert_eq!(page.next_before, None);
        let wanted: Vec<_> = expected.iter().map(|index| seeded[*index]).collect();
        assert_eq!(ids_of(&page.items), wanted);
        let kinds: Vec<_> = page
            .items
            .iter()
            .map(|item| item.attachment.as_ref().map(|attachment| attachment.kind))
            .collect();
        let links: Vec<_> = page.items.iter().map(|item| item.link.is_some()).collect();
        let (expected_kinds, expected_links) = match tab {
            VaultTab::Media => (
                vec![
                    Some(AttachmentKind::Video),
                    Some(AttachmentKind::Audio),
                    Some(AttachmentKind::Image),
                ],
                vec![false; 3],
            ),
            VaultTab::Files => (vec![Some(AttachmentKind::File)], vec![false]),
            VaultTab::Links => (vec![None, None], vec![true, true]),
            VaultTab::Pinned => (vec![None, None], vec![false, false]),
        };
        assert_eq!(kinds, expected_kinds);
        assert_eq!(links, expected_links);
        if tab == VaultTab::Pinned {
            assert!(page.items.iter().all(|item| item.message.pin.is_some()));
        }
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::two_full_pages_and_a_tail(VAULT_PAGE_LIMIT * 2 + 20)]
#[case::exactly_one_page(VAULT_PAGE_LIMIT)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn paging_walks_every_item_once_without_gaps(#[case] total: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let author = stage.id_of(Person::Ada);
        let seeded = seed_messages(&stage.state, stage.campus.id, stage.room, author, total).await;
        let placements: Vec<_> = seeded
            .iter()
            .map(|id| (*id, "image/png", "image"))
            .collect();
        seed_attachments(
            &stage.state,
            stage.campus.id,
            stage.room,
            author,
            &placements,
        )
        .await;
        let mut seen = Vec::new();
        let mut page_sizes = Vec::new();
        let mut before = None;
        loop {
            let page = vault_page(&stage, Person::Grace, VaultTab::Media, before).await;
            page_sizes.push(page.items.len());
            seen.extend(ids_of(&page.items));
            match page.next_before {
                Some(next) => before = Some(next),
                None => break,
            }
        }
        let mut expected = seeded.clone();
        expected.reverse();
        assert_eq!(seen, expected);
        let full_pages = total / VAULT_PAGE_LIMIT;
        let tail = total % VAULT_PAGE_LIMIT;
        let mut shape = vec![VAULT_PAGE_LIMIT; full_pages];
        if tail > 0 {
            shape.push(tail);
        }
        assert_eq!(page_sizes, shape);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unknown_room_is_rejected_for_every_tab() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut socket = stage.plain(Person::Ada).await;
        let unknown = RoomId::generate();
        for tab in [
            VaultTab::Media,
            VaultTab::Files,
            VaultTab::Links,
            VaultTab::Pinned,
        ] {
            send(&mut socket, &vault_frame(unknown, tab, None)).await;
            match expect_frame(&mut socket).await {
                ServerFrame::VaultRejected(rejected) => {
                    assert_eq!(rejected.room_id, unknown);
                    assert_eq!(rejected.tab, tab);
                    assert_eq!(rejected.reason, VaultRejection::UnknownRoom);
                }
                other => panic!("expected rejection, got {other:?}"),
            }
        }
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Target {
    Message,
    Unknown,
    OtherRoom,
}

#[rstest]
#[case::member_pins(Some(Person::Ada), false, Target::Message, true, Expect::Changed)]
#[case::owner_pins(None, false, Target::Message, true, Expect::Changed)]
#[case::guest_is_refused(
    Some(Person::Guest),
    false,
    Target::Message,
    true,
    Expect::Rejected(PinRejection::Forbidden)
)]
#[case::guest_cannot_unpin(
    Some(Person::Guest),
    true,
    Target::Message,
    false,
    Expect::Rejected(PinRejection::Forbidden)
)]
#[case::repin_changes_nothing(Some(Person::Ada), true, Target::Message, true, Expect::Unchanged)]
#[case::unpin_of_pinned(Some(Person::Grace), true, Target::Message, false, Expect::Changed)]
#[case::unpin_of_unpinned(Some(Person::Ada), false, Target::Message, false, Expect::Unchanged)]
#[case::unknown_message(
    Some(Person::Ada),
    false,
    Target::Unknown,
    true,
    Expect::Rejected(PinRejection::UnknownMessage)
)]
#[case::message_of_another_room(
    Some(Person::Ada),
    false,
    Target::OtherRoom,
    true,
    Expect::Rejected(PinRejection::UnknownMessage)
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pinning_follows_role_and_message_state(
    #[case] person: Option<Person>,
    #[case] already_pinned: bool,
    #[case] target: Target,
    #[case] pinned: bool,
    #[case] expected: Expect,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let ada = stage.id_of(Person::Ada);
        let message = seed_messages(&stage.state, stage.campus.id, stage.room, ada, 1).await[0];
        let elsewhere =
            seed_messages(&stage.state, stage.campus.id, stage.campus.rooms[1], ada, 1).await[0];
        if already_pinned {
            seed_pins(&stage.state, stage.campus.id, stage.room, owner, &[message]).await;
        }
        let actor = person.map_or(owner, |person| stage.id_of(person));
        let aimed = match target {
            Target::Message => message,
            Target::Unknown => MessageId::generate(),
            Target::OtherRoom => elsewhere,
        };
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        let outcome = pin_as(&stage, actor, stage.room, aimed, pinned).await;
        assert_eq!(outcome, expected, "{target:?}");
        let after = room_activity(&stage.state, stage.campus.id, stage.room).await;
        assert_eq!(after - before, i64::from(expected == Expect::Changed));
        let rows = pin_count(&stage.state, stage.campus.id, stage.room).await;
        let expected_rows = match (expected, pinned) {
            (Expect::Changed, true) => 1,
            (Expect::Changed, false) => 0,
            _ => i64::from(already_pinned),
        };
        assert_eq!(rows, expected_rows);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::four_pinners(4)]
#[case::sixteen_pinners(16)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_pins_of_one_message_leave_one_row(#[case] pinners: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let message = seed_messages(&stage.state, stage.campus.id, stage.room, ada, 1).await[0];
        let members = extra_members(&stage.state, stage.campus.id, pinners).await;
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        let mut racing = JoinSet::new();
        for member in members {
            let db = stage.state.db.clone();
            let (campus_id, room) = (stage.campus.id, stage.room);
            racing.spawn(async move {
                set_pin(
                    &db,
                    campus_id,
                    member.account.account_id,
                    SetPin {
                        room_id: room,
                        message_id: message,
                        pinned: true,
                    },
                )
                .await
            });
        }
        let mut outcomes = Vec::new();
        while let Some(done) = racing.join_next().await {
            outcomes.push(expectation(done.expect("pin task")));
        }
        assert_eq!(
            outcomes.iter().filter(|o| **o == Expect::Changed).count(),
            1
        );
        assert_eq!(
            outcomes.iter().filter(|o| **o == Expect::Unchanged).count(),
            pinners - 1
        );
        assert_eq!(
            pin_count(&stage.state, stage.campus.id, stage.room).await,
            1
        );
        let after = room_activity(&stage.state, stage.campus.id, stage.room).await;
        assert_eq!(after - before, 1);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::one_free_slot(1, 8)]
#[case::three_free_slots(3, 8)]
#[case::no_free_slot(0, 4)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_pins_never_exceed_the_room_cap(
    #[case] free_slots: usize,
    #[case] pinners: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada = stage.id_of(Person::Ada);
        let owner = stage.campus.owner.account.account_id;
        let seeded = seed_messages(
            &stage.state,
            stage.campus.id,
            stage.room,
            ada,
            ROOM_PINS_MAX - free_slots + pinners,
        )
        .await;
        let (held, contested) = seeded.split_at(ROOM_PINS_MAX - free_slots);
        seed_pins(&stage.state, stage.campus.id, stage.room, owner, held).await;
        let mut racing = JoinSet::new();
        for target in contested.iter().copied() {
            let db = stage.state.db.clone();
            let (campus_id, room) = (stage.campus.id, stage.room);
            racing.spawn(async move {
                set_pin(
                    &db,
                    campus_id,
                    ada,
                    SetPin {
                        room_id: room,
                        message_id: target,
                        pinned: true,
                    },
                )
                .await
            });
        }
        let mut outcomes = Vec::new();
        while let Some(done) = racing.join_next().await {
            outcomes.push(expectation(done.expect("pin task")));
        }
        let changed = outcomes.iter().filter(|o| **o == Expect::Changed).count();
        let refused = outcomes
            .iter()
            .filter(|o| **o == Expect::Rejected(PinRejection::LimitReached))
            .count();
        assert_eq!(changed, free_slots.min(pinners));
        assert_eq!(refused, pinners - changed);
        assert_eq!(
            pin_count(&stage.state, stage.campus.id, stage.room).await,
            i64::try_from(ROOM_PINS_MAX).expect("cap")
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn pin_changes_fan_out_and_reach_a_returning_client_through_delta_sync() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.socket(&stage.ada).await;
        let mut grace = stage.socket(&stage.grace).await;
        let root = stage.root(&mut ada, "keep this").await;
        next_of(&mut grace, |frame| {
            matches!(frame, ServerFrame::MessageCreated(_)).then_some(())
        })
        .await;
        let baseline = root.activity_seq;
        send(&mut ada, &pin_frame(stage.room, root.message_id, true)).await;
        let pinned = next_of(&mut grace, |frame| match frame {
            ServerFrame::MessageUpdated(updated) => Some(updated.message.clone()),
            _ => None,
        })
        .await;
        let mark = pinned.pin.expect("pin mark");
        assert_eq!(mark.pinned_by, stage.id_of(Person::Ada));
        assert!(pinned.activity_seq > root.activity_seq);
        send(&mut ada, &pin_frame(stage.room, root.message_id, true)).await;
        let mut sentinel = frames_until_pong(&mut grace, 21).await;
        sentinel.retain(|frame| matches!(frame, ServerFrame::MessageUpdated(_)));
        assert!(sentinel.is_empty());
        let mut returning = common::joined_socket(stage.addr, stage.campus.id, &stage.linus.token)
            .await
            .0;
        send(
            &mut returning,
            &sync_rooms_at(&[(stage.room, Some(root.message_id), Some(baseline))]),
        )
        .await;
        let page = match expect_frame(&mut returning).await {
            ServerFrame::RoomSynced(page) => page,
            other => panic!("expected room sync, got {other:?}"),
        };
        assert!(page.messages.is_empty());
        let delta: Vec<_> = page
            .updates
            .iter()
            .map(|message| (message.message_id, message.pin.is_some()))
            .collect();
        assert_eq!(delta, vec![(root.message_id, true)]);
        send(&mut ada, &pin_frame(stage.room, root.message_id, false)).await;
        let unpinned = next_of(&mut grace, |frame| match frame {
            ServerFrame::MessageUpdated(updated) => Some(updated.message.clone()),
            _ => None,
        })
        .await;
        assert_eq!(unpinned.pin, None);
        assert!(unpinned.activity_seq > pinned.activity_seq);
        send(
            &mut returning,
            &sync_rooms_at(&[(stage.room, Some(root.message_id), Some(pinned.activity_seq))]),
        )
        .await;
        let page = next_of(&mut returning, |frame| match frame {
            ServerFrame::RoomSynced(page) => Some(page.clone()),
            _ => None,
        })
        .await;
        assert_eq!(
            page.updates
                .iter()
                .map(|message| (message.message_id, message.pin.is_some()))
                .collect::<Vec<_>>(),
            vec![(root.message_id, false)]
        );
        assert!(page.messages.is_empty());
        assert_eq!(page.activity_seq, unpinned.activity_seq);
    })
    .await
    .expect("scenario deadline");
}
