mod fixtures;

use fixtures::{
    ACCOUNT_A, ACCOUNT_B, ACCOUNT_C, EMOJI_A, EMOJI_B, ROOM_A, chain_message, connect_burst_at,
    created_frame, permutations, react_frame, react_intent, reacted, reaction_rejected_frame,
    received, settled_machine, step_all, timeline_connected, updated_frame,
};
use pulse_core::TimelineEvent;
use pulse_protocol::{
    ApiError, ApiErrorCode, ClientFrame, InputField, REACTION_PALETTE, ReactionAction,
    ReactionRejection,
};
use rstest::rstest;

fn first() -> pulse_protocol::RoomMessage {
    chain_message(ROOM_A, 1)
}

#[rstest]
#[case::reactions_only(
    vec![
        received(created_frame(first())),
        received(updated_frame(reacted(first(), &[(EMOJI_A, &[ACCOUNT_B])], 1))),
        received(updated_frame(reacted(first(), &[(EMOJI_A, &[ACCOUNT_B, ACCOUNT_C]), (EMOJI_B, &[ACCOUNT_B])], 2))),
        received(updated_frame(reacted(first(), &[(EMOJI_B, &[ACCOUNT_A])], 3))),
    ],
    vec![(EMOJI_B.to_owned(), 1, true)],
)]
#[case::removal_to_empty_wins_by_activity(
    vec![
        received(created_frame(first())),
        received(updated_frame(reacted(first(), &[(EMOJI_A, &[ACCOUNT_B])], 1))),
        received(updated_frame(reacted(first(), &[], 2))),
        received(updated_frame(reacted(first(), &[(EMOJI_A, &[ACCOUNT_B, ACCOUNT_A])], 1))),
    ],
    vec![],
)]
fn reaction_updates_converge_in_every_delivery_order(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Vec<(String, u32, bool)>,
) {
    permutations(&events).into_iter().for_each(|order| {
        let snapshot = settled_machine(order).snapshot(Some(ROOM_A));
        assert_eq!(snapshot.entries.len(), 1);
        let summaries = snapshot.entries[0]
            .reactions
            .iter()
            .map(|summary| {
                (
                    summary.emoji.clone(),
                    summary.count,
                    summary.reacted_by_self,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(summaries, expected);
    });
}

#[test]
fn snapshot_palette_is_the_protocol_palette() {
    let snapshot = settled_machine(vec![]).snapshot(Some(ROOM_A));
    assert_eq!(
        snapshot.reaction_palette,
        REACTION_PALETTE
            .iter()
            .map(|emoji| (*emoji).to_owned())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        settled_machine(vec![]).snapshot(None).reaction_palette,
        snapshot.reaction_palette
    );
}

#[rstest]
#[case::palette_add(
    EMOJI_A,
    ReactionAction::Add,
    Some(react_frame(1, EMOJI_A, ReactionAction::Add)),
    None
)]
#[case::palette_remove(
    EMOJI_B,
    ReactionAction::Remove,
    Some(react_frame(1, EMOJI_B, ReactionAction::Remove)),
    None
)]
#[case::surrounding_space_is_trimmed(
    " 👍 ",
    ReactionAction::Add,
    Some(react_frame(1, EMOJI_A, ReactionAction::Add)),
    None
)]
#[case::empty_is_invalid(
    "",
    ReactionAction::Add,
    None,
    Some(ApiError::invalid(InputField::ReactionEmoji))
)]
#[case::blank_is_invalid(
    "   ",
    ReactionAction::Add,
    None,
    Some(ApiError::invalid(InputField::ReactionEmoji))
)]
#[case::ascii_is_invalid(
    "a",
    ReactionAction::Add,
    None,
    Some(ApiError::invalid(InputField::ReactionEmoji))
)]
#[case::inner_space_is_invalid(
    "👍 🎉",
    ReactionAction::Add,
    None,
    Some(ApiError::invalid(InputField::ReactionEmoji))
)]
#[case::control_is_invalid(
    "👍\u{7}",
    ReactionAction::Remove,
    None,
    Some(ApiError::invalid(InputField::ReactionEmoji))
)]
#[case::too_long_is_invalid(
    "👍👍👍👍👍👍👍👍👍👍👍👍👍👍👍👍👍",
    ReactionAction::Add,
    None,
    Some(ApiError::invalid(InputField::ReactionEmoji))
)]
fn react_intent_validates_before_any_frame(
    #[case] emoji: &str,
    #[case] action: ReactionAction,
    #[case] expected_frame: Option<ClientFrame>,
    #[case] expected_error: Option<ApiError>,
) {
    let machine = settled_machine(vec![]);
    let (machine, frames) = step_all(machine, vec![react_intent(1, emoji, action)]);
    assert_eq!(frames, expected_frame.into_iter().collect::<Vec<_>>());
    assert_eq!(machine.snapshot(Some(ROOM_A)).error, expected_error);
}

#[test]
fn valid_reaction_clears_an_earlier_error() {
    let machine = settled_machine(vec![react_intent(1, "x", ReactionAction::Add)]);
    assert_eq!(
        machine.snapshot(Some(ROOM_A)).error,
        Some(ApiError::invalid(InputField::ReactionEmoji))
    );
    let (machine, _) = step_all(machine, vec![react_intent(1, EMOJI_A, ReactionAction::Add)]);
    assert_eq!(machine.snapshot(Some(ROOM_A)).error, None);
}

#[test]
fn react_while_disconnected_emits_nothing_and_is_not_queued() {
    let machine = settled_machine(vec![TimelineEvent::Disconnected]);
    let (machine, frames) = step_all(
        machine,
        vec![
            react_intent(1, EMOJI_A, ReactionAction::Add),
            timeline_connected(&[ROOM_A]),
        ],
    );
    assert_eq!(frames, connect_burst_at(&[(ROOM_A, None, Some(0))]));
    assert_eq!(machine.snapshot(Some(ROOM_A)).error, None);
}

#[rstest]
#[case::forbidden(ReactionRejection::Forbidden, ApiError::new(ApiErrorCode::Forbidden))]
#[case::invalid_emoji(
    ReactionRejection::InvalidEmoji,
    ApiError::invalid(InputField::ReactionEmoji)
)]
#[case::unknown_message(
    ReactionRejection::UnknownMessage,
    ApiError::new(ApiErrorCode::NotFound)
)]
#[case::too_many_kinds(
    ReactionRejection::TooManyKinds,
    ApiError::conflict(InputField::ReactionEmoji)
)]
fn reaction_rejections_surface_as_timeline_errors(
    #[case] reason: ReactionRejection,
    #[case] expected: ApiError,
) {
    let machine = settled_machine(vec![received(reaction_rejected_frame(1, reason))]);
    assert_eq!(machine.snapshot(Some(ROOM_A)).error, Some(expected));
}
