mod fixtures;

use fixtures::{
    ACCOUNT_A, CAMPUS_A, CAMPUS_B, ROOM_A, ScriptedApi, ShelfSpec, SummarySpec, accepted_frame,
    chain, chain_message, client_rooms, created_frame, inbox_synced_frame, message_id, nonce,
    own_message, pinned, room_list, settled_client, shelf_changed_frame, shelf_synced_frame,
    signed_in_client, synced_frame, thread_changed_frame, thread_synced_frame, updated_frame,
    vault_item, vault_synced_frame,
};
use futures::executor::block_on;
use pulse_core::{Client, Effect, EffectOutcome, SearchIntent, SessionEvent, StoreError};
use pulse_protocol::{
    ApiErrorCode, CampusId, HighlightSpan, InboxStep, MessageId, RoomId, RoomMessage,
    SEARCH_CANDIDATE_LIMIT, SearchHit, ServerFrame, VaultTab,
};
use rstest::rstest;

fn indexed_ids(effects: &[Effect]) -> Vec<MessageId> {
    let mut ids = effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::IndexDocuments(documents) => Some(documents),
            _ => None,
        })
        .flatten()
        .map(|document| document.message_id)
        .collect::<Vec<_>>();
    ids.sort();
    ids
}

fn sorted(indexes: &[u128]) -> Vec<MessageId> {
    let mut ids = indexes.iter().copied().map(message_id).collect::<Vec<_>>();
    ids.sort();
    ids
}

fn foreign(mut message: RoomMessage) -> RoomMessage {
    message.campus_id = CAMPUS_B;
    message
}

fn reply_of(index: u128, root: u128) -> RoomMessage {
    fixtures::peer_reply(index, root)
}

fn hit_in(room_id: RoomId, index: u128, thread_root: Option<u128>) -> SearchHit {
    SearchHit {
        room_id,
        message_id: message_id(index),
        thread_root_id: thread_root.map(message_id),
        author_display_name: "Grace".to_owned(),
        topic_title: None,
        snippet: vec![HighlightSpan {
            text: "peer".to_owned(),
            matched: true,
        }],
        score: 2,
    }
}

fn hit(index: u128, thread_root: Option<u128>) -> SearchHit {
    hit_in(ROOM_A, index, thread_root)
}

fn run_searches(effects: &[Effect]) -> Vec<(u64, Vec<String>, usize)> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::RunSearch { generation, query } => {
                Some((*generation, query.terms.clone(), query.limit))
            }
            _ => None,
        })
        .collect()
}

fn opened_search(client: &mut Client) {
    client.search_intent(SearchIntent::Toggled);
}

fn complete(client: &mut Client, generation: u64, hits: Vec<SearchHit>) {
    client.effect_finished(EffectOutcome::SearchCompleted {
        generation,
        result: Ok(hits),
    });
}

#[rstest]
#[case::room_page(
    synced_frame(ROOM_A, chain(ROOM_A, 3), false),
    &[1, 2, 3]
)]
#[case::created(created_frame(chain_message(ROOM_A, 1)), &[1])]
#[case::own_accepted(accepted_frame(own_message(ROOM_A, 1, nonce(9))), &[1])]
#[case::updated(updated_frame(pinned(chain_message(ROOM_A, 1), 5)), &[1])]
#[case::thread_page(
    thread_synced_frame(1, vec![chain_message(ROOM_A, 1), reply_of(2, 1)], false),
    &[1, 2]
)]
#[case::shelf_page(shelf_synced_frame(&[ShelfSpec::new(4), ShelfSpec::new(5)]), &[4, 5])]
#[case::shelf_change(shelf_changed_frame(ShelfSpec::new(4), false), &[4])]
#[case::vault_page(
    vault_synced_frame(
        ROOM_A,
        VaultTab::Links,
        vec![vault_item(chain_message(ROOM_A, 6), None, Some("https://example.org/a"))],
        None
    ),
    &[6]
)]
#[case::foreign_room_page(synced_frame(ROOM_A, vec![foreign(chain_message(ROOM_A, 1))], false), &[])]
#[case::foreign_created(created_frame(foreign(chain_message(ROOM_A, 1))), &[])]
#[case::foreign_shelf(
    ServerFrame::ShelfChanged(pulse_protocol::ShelfChanged {
        item: pulse_protocol::ShelfItem {
            message: foreign(chain_message(ROOM_A, 4)),
            ..fixtures::shelf_item(ShelfSpec::new(4))
        },
        removed: false,
    }),
    &[]
)]
#[case::foreign_vault(
    vault_synced_frame(
        ROOM_A,
        VaultTab::Links,
        vec![vault_item(foreign(chain_message(ROOM_A, 6)), None, Some("https://example.org/a"))],
        None
    ),
    &[]
)]
fn ingested_messages_queue_index_documents(#[case] frame: ServerFrame, #[case] expected: &[u128]) {
    let mut client = settled_client();
    client.receive_frame(frame);
    let effects = client.take_effects();
    assert_eq!(indexed_ids(&effects), sorted(expected));
    for effect in &effects {
        if let Effect::IndexDocuments(documents) = effect {
            assert!(documents.iter().all(|document| {
                document.account_id == ACCOUNT_A && document.campus_id == CAMPUS_A
            }));
        }
    }
    assert!(client.take_effects().is_empty());
}

#[rstest]
#[case::inbox_page(inbox_synced_frame(&[SummarySpec::replied(1, 2, 1)]), &[1, 2])]
#[case::thread_change(thread_changed_frame(&SummarySpec::replied(1, 2, 1)), &[1, 2])]
fn inbox_summaries_queue_index_documents(#[case] frame: ServerFrame, #[case] expected: &[u128]) {
    let mut client = settled_client();
    client.receive_frame(frame);
    assert_eq!(indexed_ids(&client.take_effects()), sorted(expected));
}

#[test]
fn indexed_text_is_the_rendered_message_and_terms_are_sorted() {
    let mut client = settled_client();
    let mut message = chain_message(ROOM_A, 1);
    message.body = "Zebra apple zebra".to_owned();
    message.topic_title = Some("Fruit".to_owned());
    client.receive_frame(created_frame(message));
    let effects = client.take_effects();
    let documents = effects
        .iter()
        .find_map(|effect| match effect {
            Effect::IndexDocuments(documents) => Some(documents.clone()),
            _ => None,
        })
        .expect("documents");
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0].text, "Zebra apple zebra");
    assert_eq!(documents[0].terms, vec!["apple", "fruit", "grace", "zebra"]);
}

#[test]
fn reindexing_the_same_message_in_one_step_queues_it_once() {
    let mut client = settled_client();
    client.receive_frame(synced_frame(ROOM_A, chain(ROOM_A, 2), false));
    client.take_effects();
    client.receive_frame(updated_frame(pinned(chain_message(ROOM_A, 2), 3)));
    assert_eq!(indexed_ids(&client.take_effects()), sorted(&[2]));
}

#[rstest]
#[case::while_closed(false, "alpha", 0)]
#[case::blank_query(true, "   ", 0)]
#[case::punctuation_only(true, "?!", 0)]
#[case::single_term(true, "alpha", 1)]
#[case::many_terms_are_clipped(true, "a b c d e f g h i j", 1)]
fn queries_run_one_search_effect_only_when_there_is_something_to_find(
    #[case] open: bool,
    #[case] query: &str,
    #[case] expected_runs: usize,
) {
    let mut client = settled_client();
    if open {
        opened_search(&mut client);
    }
    let snapshot = client.search_intent(SearchIntent::QueryChanged(query.to_owned()));
    let runs = run_searches(&client.take_effects());
    assert_eq!(runs.len(), expected_runs);
    assert_eq!(snapshot.search.searching, expected_runs == 1 && open);
    assert_eq!(snapshot.search.query == query, open);
    if let Some((_, terms, limit)) = runs.first() {
        assert_eq!(*limit, SEARCH_CANDIDATE_LIMIT);
        assert!(terms.len() <= pulse_protocol::SEARCH_TERMS_MAX);
    }
}

#[test]
fn the_search_query_is_scoped_to_the_signed_in_account_and_campus() {
    let mut client = settled_client();
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("Alpha Beta".to_owned()));
    let effects = client.take_effects();
    let Some(Effect::RunSearch { query, .. }) = effects
        .into_iter()
        .find(|effect| matches!(effect, Effect::RunSearch { .. }))
    else {
        panic!("expected a search effect");
    };
    assert_eq!(query.account_id, ACCOUNT_A);
    assert_eq!(query.campus_id, CAMPUS_A);
    assert_eq!(query.terms, vec!["alpha", "beta"]);
}

#[test]
fn only_the_latest_generation_may_complete_a_search() {
    let mut client = settled_client();
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("pe".to_owned()));
    client.search_intent(SearchIntent::QueryChanged("peer".to_owned()));
    let runs = run_searches(&client.take_effects());
    assert_eq!(runs.len(), 2);
    let (first, second) = (runs[0].0, runs[1].0);
    assert!(second > first);
    complete(&mut client, first, vec![hit(1, None)]);
    let stale = client.snapshot().search;
    assert!(stale.searching);
    assert!(stale.hits.is_empty());
    assert_eq!(stale.selected_index, None);
    complete(&mut client, second, vec![hit(2, None), hit(1, None)]);
    let fresh = client.snapshot().search;
    assert!(!fresh.searching);
    assert_eq!(
        fresh
            .hits
            .iter()
            .map(|hit| hit.message_id)
            .collect::<Vec<_>>(),
        vec![message_id(2), message_id(1)]
    );
    assert_eq!(fresh.selected_index, Some(0));
    complete(&mut client, first, vec![hit(3, None)]);
    assert_eq!(client.snapshot().search.hits.len(), 2);
}

#[rstest]
#[case::closed_before_result(SearchIntent::Closed)]
#[case::toggled_away(SearchIntent::Toggled)]
fn results_arriving_after_the_palette_closed_are_ignored(#[case] closing: SearchIntent) {
    let mut client = settled_client();
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("peer".to_owned()));
    let generation = run_searches(&client.take_effects())[0].0;
    client.search_intent(closing);
    complete(&mut client, generation, vec![hit(1, None)]);
    let snapshot = client.snapshot().search;
    assert!(snapshot.hits.is_empty());
    assert!(!snapshot.searching);
}

#[test]
fn a_failed_search_reports_unavailable_and_clears_hits() {
    let mut client = settled_client();
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("peer".to_owned()));
    let generation = run_searches(&client.take_effects())[0].0;
    client.effect_finished(EffectOutcome::SearchCompleted {
        generation,
        result: Err(StoreError::Unavailable("disk".to_owned())),
    });
    let snapshot = client.snapshot().search;
    assert_eq!(
        snapshot.error.map(|error| error.code),
        Some(ApiErrorCode::Unavailable)
    );
    assert!(snapshot.hits.is_empty());
    assert!(!snapshot.searching);
}

#[rstest]
#[case::next_moves_down(&[InboxStep::Next], Some(1))]
#[case::next_clamps_at_the_end(&[InboxStep::Next, InboxStep::Next, InboxStep::Next, InboxStep::Next], Some(2))]
#[case::previous_clamps_at_the_start(&[InboxStep::Previous], Some(0))]
#[case::down_then_up(&[InboxStep::Next, InboxStep::Next, InboxStep::Previous], Some(1))]
#[case::no_movement(&[], Some(0))]
fn selection_moves_within_the_hits(#[case] steps: &[InboxStep], #[case] expected: Option<u32>) {
    let mut client = settled_client();
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("peer".to_owned()));
    let generation = run_searches(&client.take_effects())[0].0;
    complete(
        &mut client,
        generation,
        vec![hit(3, None), hit(2, None), hit(1, None)],
    );
    for step in steps {
        client.search_intent(SearchIntent::SelectionMoved(*step));
    }
    let snapshot = client.snapshot().search;
    assert_eq!(snapshot.selected_index, expected);
    let selected = usize::try_from(expected.expect("selection")).expect("index");
    assert_eq!(
        client.selected_search_hit(),
        Some(snapshot.hits[selected].message_id)
    );
}

#[test]
fn selection_does_nothing_without_hits() {
    let mut client = settled_client();
    opened_search(&mut client);
    let snapshot = client.search_intent(SearchIntent::SelectionMoved(InboxStep::Next));
    assert_eq!(snapshot.search.selected_index, None);
    assert_eq!(client.selected_search_hit(), None);
}

type HitSpec = (usize, u128, Option<u128>);

fn client_with_hits(specs: &[HitSpec]) -> (Client, ScriptedApi, Vec<RoomId>) {
    let (mut client, api) = signed_in_client();
    let rooms = client_rooms(&client);
    rooms.iter().for_each(|room_id| {
        client.receive_frame(synced_frame(*room_id, vec![], false));
    });
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("peer".to_owned()));
    let generation = run_searches(&client.take_effects())[0].0;
    let hits = specs
        .iter()
        .map(|(slot, index, root)| hit_in(rooms[*slot], *index, *root))
        .collect();
    complete(&mut client, generation, hits);
    (client, api, rooms)
}

#[rstest]
#[case::room_message((0, 1, None), None)]
#[case::thread_reply((0, 2, Some(1)), Some(message_id(1)))]
#[case::message_in_second_room((1, 3, None), None)]
#[case::reply_in_second_room((1, 4, Some(3)), Some(message_id(3)))]
fn opening_a_hit_selects_its_room_focuses_it_and_closes_search(
    #[case] spec: HitSpec,
    #[case] expected_thread_root: Option<MessageId>,
) {
    let (mut client, mut api, rooms) = client_with_hits(&[spec]);
    let snapshot = block_on(client.open_search_hit(&mut api, message_id(spec.1), 7_000));
    assert!(!snapshot.search.open);
    assert_eq!(snapshot.timeline.focus_message_id, Some(message_id(spec.1)));
    assert_eq!(snapshot.session.active_room_id, Some(rooms[spec.0]));
    assert_eq!(snapshot.thread.root_message_id, expected_thread_root);
    assert_eq!(snapshot.timeline.error, None);
}

#[test]
fn opening_a_message_that_is_not_a_hit_changes_nothing() {
    let (mut client, mut api, _) = client_with_hits(&[(0, 1, None)]);
    let before = client.snapshot();
    let snapshot = block_on(client.open_search_hit(&mut api, message_id(9), 1_000));
    assert_eq!(snapshot.search, before.search);
    assert_eq!(snapshot.timeline.focus_message_id, None);
    assert_eq!(
        snapshot.session.active_room_id,
        before.session.active_room_id
    );
}

#[test]
fn sending_a_message_clears_the_focus() {
    let (mut client, mut api, rooms) = client_with_hits(&[(0, 1, None)]);
    block_on(client.open_search_hit(&mut api, message_id(1), 1_000));
    let snapshot = client.send_message(rooms[0], nonce(1), "hello".to_owned());
    assert_eq!(snapshot.timeline.focus_message_id, None);
}

fn campus_b_client() -> (Client, ScriptedApi) {
    let (mut client, mut api) = signed_in_client();
    let rooms = client_rooms(&client);
    rooms.iter().for_each(|room_id| {
        client.receive_frame(synced_frame(*room_id, chain(*room_id, 2), false));
    });
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("peer".to_owned()));
    api.room_lists
        .push_back(Ok(room_list(CAMPUS_B, &["lab", "annex"])));
    (client, api)
}

#[test]
fn switching_campus_clears_search_and_drops_queued_effects() {
    let (mut client, mut api) = campus_b_client();
    let snapshot = block_on(client.apply(&mut api, SessionEvent::CampusSelected(CAMPUS_B)));
    assert!(!snapshot.search.open);
    assert!(snapshot.search.hits.is_empty());
    assert!(snapshot.search.query.is_empty());
    assert!(!snapshot.search.searching);
    assert_eq!(client.take_effects(), Vec::new());
    assert_eq!(client.selected_search_hit(), None);
}

#[test]
fn results_of_the_previous_campus_cannot_land_in_the_new_one() {
    let (mut client, mut api) = campus_b_client();
    let stale_generation = run_searches(&client.take_effects())[0].0;
    block_on(client.apply(&mut api, SessionEvent::CampusSelected(CAMPUS_B)));
    opened_search(&mut client);
    client.search_intent(SearchIntent::QueryChanged("lab".to_owned()));
    let fresh_generation = run_searches(&client.take_effects())[0].0;
    assert!(fresh_generation > stale_generation);
    complete(&mut client, stale_generation, vec![hit(1, None)]);
    let snapshot = client.snapshot().search;
    assert!(snapshot.hits.is_empty());
    assert!(snapshot.searching);
    complete(&mut client, fresh_generation, vec![hit(2, None)]);
    assert_eq!(client.snapshot().search.hits.len(), 1);
}

#[rstest]
#[case::previous_campus(CAMPUS_A, 0)]
#[case::current_campus(CAMPUS_B, 1)]
fn documents_follow_the_active_campus_only(#[case] campus: CampusId, #[case] expected: usize) {
    let (mut client, mut api) = campus_b_client();
    block_on(client.apply(&mut api, SessionEvent::CampusSelected(CAMPUS_B)));
    client.take_effects();
    let mut message = chain_message(ROOM_A, 1);
    message.campus_id = campus;
    client.receive_frame(created_frame(message));
    assert_eq!(indexed_ids(&client.take_effects()).len(), expected);
}
