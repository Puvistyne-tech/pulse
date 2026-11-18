mod common;

use std::collections::BTreeSet;

use common::{campus_for, fresh_state, signed_up};
use pulse_protocol::{AccountId, CampusId, ClientNonce, MessageId, RoomId};
use pulse_server::{AppState, MIGRATOR};
use rstest::rstest;
use uuid::Uuid;

const UNIQUE_VIOLATION: &str = "23505";
const FOREIGN_KEY_VIOLATION: &str = "23503";
const CHECK_VIOLATION: &str = "23514";

#[test]
fn migrations_apply_in_order() {
    let descriptions: Vec<_> = MIGRATOR
        .iter()
        .map(|migration| migration.description.to_string())
        .collect();
    assert_eq!(
        descriptions,
        vec![
            "create campuses".to_owned(),
            "create accounts memberships rooms".to_owned(),
            "create room messages".to_owned(),
        ]
    );
}

#[derive(Clone, Copy)]
struct Row {
    campus_id: CampusId,
    room_id: RoomId,
    message_id: MessageId,
    previous: Option<MessageId>,
    author_id: AccountId,
    client_nonce: ClientNonce,
}

struct Context {
    stored: Row,
    foreign_campus: CampusId,
    other_author: AccountId,
}

async fn insert(state: &AppState, row: Row) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO room_messages \
         (campus_id, room_id, message_id, previous_message_id, author_id, client_nonce, body) \
         VALUES ($1, $2, $3, $4, $5, $6, 'body')",
    )
    .bind(row.campus_id.as_uuid())
    .bind(row.room_id.as_uuid())
    .bind(row.message_id.as_uuid())
    .bind(row.previous.map(|id| id.as_uuid()))
    .bind(row.author_id.as_uuid())
    .bind(row.client_nonce.as_uuid())
    .execute(&state.db)
    .await
    .map(|_| ())
}

async fn seeded() -> (AppState, Context) {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let other = signed_up(&state, "other@example.org").await;
    let created = campus_for(&state, &owner.token, "north-quad").await;
    let foreign = campus_for(&state, &other.token, "south-quad").await;
    let stored = Row {
        campus_id: created.campus.campus_id,
        room_id: created.rooms[0].room_id,
        message_id: MessageId::generate(),
        previous: None,
        author_id: owner.account.account_id,
        client_nonce: ClientNonce::generate(),
    };
    insert(&state, stored).await.expect("seed message");
    let context = Context {
        stored,
        foreign_campus: foreign.campus.campus_id,
        other_author: other.account.account_id,
    };
    (state, context)
}

fn fresh(context: &Context) -> Row {
    Row {
        message_id: MessageId(Uuid::from_u128(
            context.stored.message_id.as_uuid().as_u128() + 1,
        )),
        client_nonce: ClientNonce::generate(),
        previous: Some(context.stored.message_id),
        ..context.stored
    }
}

#[rstest]
#[case::chained_message_is_valid(|context: &Context| fresh(context), None)]
#[case::same_nonce_other_author_is_valid(
    |context: &Context| Row {
        author_id: context.other_author,
        client_nonce: context.stored.client_nonce,
        ..fresh(context)
    },
    None,
)]
#[case::duplicate_nonce_for_author(
    |context: &Context| Row { client_nonce: context.stored.client_nonce, ..fresh(context) },
    Some(UNIQUE_VIOLATION),
)]
#[case::duplicate_message_id(
    |context: &Context| Row {
        message_id: context.stored.message_id,
        previous: None,
        ..fresh(context)
    },
    Some(UNIQUE_VIOLATION),
)]
#[case::previous_equal_to_own_id(
    |context: &Context| {
        let row = fresh(context);
        Row { previous: Some(row.message_id), ..row }
    },
    Some(CHECK_VIOLATION),
)]
#[case::previous_after_own_id(
    |context: &Context| Row {
        previous: Some(MessageId(Uuid::from_u128(context.stored.message_id.as_uuid().as_u128() + 5))),
        ..fresh(context)
    },
    Some(CHECK_VIOLATION),
)]
#[case::room_belongs_to_other_campus(
    |context: &Context| Row { campus_id: context.foreign_campus, ..fresh(context) },
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::unknown_room(
    |context: &Context| Row { room_id: RoomId::generate(), ..fresh(context) },
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::unknown_author(
    |context: &Context| Row { author_id: AccountId::generate(), ..fresh(context) },
    Some(FOREIGN_KEY_VIOLATION),
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn room_messages_constraints_hold(
    #[case] build: fn(&Context) -> Row,
    #[case] violation: Option<&str>,
) {
    let (state, context) = seeded().await;
    let outcome = insert(&state, build(&context)).await;
    let code = outcome.err().and_then(|error| {
        error
            .as_database_error()
            .and_then(|database| database.code().map(|code| code.to_string()))
    });
    assert_eq!(code.as_deref(), violation);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn room_messages_schema_matches_contract() {
    let state = fresh_state().await;
    let columns = sqlx::query_as::<_, (String, String, String)>(
        "SELECT column_name::text, data_type::text, is_nullable::text \
         FROM information_schema.columns WHERE table_name = 'room_messages' \
         ORDER BY ordinal_position",
    )
    .fetch_all(&state.db)
    .await
    .expect("columns");
    let expected_columns = [
        ("campus_id", "uuid", "NO"),
        ("room_id", "uuid", "NO"),
        ("message_id", "uuid", "NO"),
        ("previous_message_id", "uuid", "YES"),
        ("author_id", "uuid", "NO"),
        ("client_nonce", "uuid", "NO"),
        ("body", "text", "NO"),
        ("created_at", "timestamp with time zone", "NO"),
    ]
    .map(|(name, kind, nullable)| (name.to_owned(), kind.to_owned(), nullable.to_owned()));
    assert_eq!(columns, expected_columns);
    let constraints = sqlx::query_as::<_, (String, String)>(
        "SELECT contype::text, pg_get_constraintdef(oid) FROM pg_constraint \
         WHERE conrelid = 'room_messages'::regclass AND contype <> 'n'",
    )
    .fetch_all(&state.db)
    .await
    .expect("constraints")
    .into_iter()
    .collect::<BTreeSet<_>>();
    let expected_constraints = [
        ("p", "PRIMARY KEY (campus_id, room_id, message_id)"),
        (
            "f",
            "FOREIGN KEY (campus_id, room_id) REFERENCES rooms(campus_id, room_id) ON DELETE CASCADE",
        ),
        (
            "f",
            "FOREIGN KEY (author_id) REFERENCES accounts(account_id) ON DELETE CASCADE",
        ),
        ("u", "UNIQUE (campus_id, author_id, client_nonce)"),
        (
            "c",
            "CHECK (((previous_message_id IS NULL) OR (previous_message_id < message_id)))",
        ),
    ]
    .map(|(kind, definition)| (kind.to_owned(), definition.to_owned()))
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(constraints, expected_constraints);
}
