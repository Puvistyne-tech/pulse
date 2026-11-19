mod common;

use std::collections::BTreeSet;

use common::{campus_for, fresh_state, signed_up, state_before};
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
            "create threads reactions subscriptions".to_owned(),
            "create media vault shelf".to_owned(),
            "create stewards lounges".to_owned(),
            "create campus invites".to_owned(),
            "create direct notes".to_owned(),
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
    sibling_room: RoomId,
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
        sibling_room: created.rooms[1].room_id,
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
        ("thread_root_id", "uuid", "YES"),
        ("activity_seq", "bigint", "NO"),
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
        (
            "f",
            "FOREIGN KEY (campus_id, room_id, thread_root_id) REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE",
        ),
        (
            "c",
            "CHECK (((thread_root_id IS NULL) OR (thread_root_id < message_id)))",
        ),
    ]
    .map(|(kind, definition)| (kind.to_owned(), definition.to_owned()))
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(constraints, expected_constraints);
}

type Column = (&'static str, &'static str, &'static str);
type Constraint = (&'static str, &'static str);

const SUBSCRIPTION_ROOM_FK: &str = "FOREIGN KEY (campus_id, room_id, root_message_id) REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE";
const MESSAGE_ROOM_FK: &str = "FOREIGN KEY (campus_id, room_id, message_id) REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE";
const MEMBER_FK: &str = "FOREIGN KEY (campus_id, account_id) REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE";

async fn columns_of(state: &AppState, table: &str) -> Vec<(String, String, String)> {
    sqlx::query_as::<_, (String, String, String)>(
        "SELECT column_name::text, data_type::text, is_nullable::text \
         FROM information_schema.columns WHERE table_name = $1 ORDER BY ordinal_position",
    )
    .bind(table)
    .fetch_all(&state.db)
    .await
    .expect("columns")
}

async fn constraints_of(state: &AppState, table: &str) -> BTreeSet<(String, String)> {
    sqlx::query_as::<_, (String, String)>(
        "SELECT contype::text, pg_get_constraintdef(oid) FROM pg_constraint \
         WHERE conrelid = ($1::text)::regclass AND contype <> 'n'",
    )
    .bind(table)
    .fetch_all(&state.db)
    .await
    .expect("constraints")
    .into_iter()
    .collect()
}

#[rstest]
#[case::threads(
    "threads",
    vec![
        ("campus_id", "uuid", "NO"),
        ("room_id", "uuid", "NO"),
        ("root_message_id", "uuid", "NO"),
        ("topic_title", "text", "YES"),
        ("created_at", "timestamp with time zone", "NO"),
    ],
    vec![
        ("p", "PRIMARY KEY (campus_id, room_id, root_message_id)"),
        ("f", SUBSCRIPTION_ROOM_FK),
    ],
)]
#[case::thread_subscriptions(
    "thread_subscriptions",
    vec![
        ("campus_id", "uuid", "NO"),
        ("room_id", "uuid", "NO"),
        ("root_message_id", "uuid", "NO"),
        ("account_id", "uuid", "NO"),
        ("state", "text", "NO"),
        ("read_through", "uuid", "YES"),
        ("revision", "bigint", "NO"),
        ("updated_at", "timestamp with time zone", "NO"),
    ],
    vec![
        ("p", "PRIMARY KEY (campus_id, root_message_id, account_id)"),
        ("f", SUBSCRIPTION_ROOM_FK),
        ("f", MEMBER_FK),
        (
            "c",
            "CHECK ((state = ANY (ARRAY['following'::text, 'muted'::text, 'archived'::text])))",
        ),
    ],
)]
#[case::message_reactions(
    "message_reactions",
    vec![
        ("campus_id", "uuid", "NO"),
        ("room_id", "uuid", "NO"),
        ("message_id", "uuid", "NO"),
        ("account_id", "uuid", "NO"),
        ("emoji", "text", "NO"),
        ("created_at", "timestamp with time zone", "NO"),
    ],
    vec![
        ("p", "PRIMARY KEY (campus_id, message_id, account_id, emoji)"),
        ("f", MESSAGE_ROOM_FK),
        ("f", MEMBER_FK),
    ],
)]
#[case::message_mentions(
    "message_mentions",
    vec![
        ("campus_id", "uuid", "NO"),
        ("room_id", "uuid", "NO"),
        ("message_id", "uuid", "NO"),
        ("account_id", "uuid", "NO"),
    ],
    vec![
        ("p", "PRIMARY KEY (campus_id, message_id, account_id)"),
        ("f", MESSAGE_ROOM_FK),
        ("f", MEMBER_FK),
    ],
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn thread_tables_match_contract(
    #[case] table: &str,
    #[case] expected_columns: Vec<Column>,
    #[case] expected_constraints: Vec<Constraint>,
) {
    let state = fresh_state().await;
    let expected_columns = expected_columns
        .into_iter()
        .map(|(name, kind, nullable)| (name.to_owned(), kind.to_owned(), nullable.to_owned()))
        .collect::<Vec<_>>();
    let expected_constraints = expected_constraints
        .into_iter()
        .map(|(kind, definition)| (kind.to_owned(), definition.to_owned()))
        .collect::<BTreeSet<_>>();
    assert_eq!(columns_of(&state, table).await, expected_columns);
    assert_eq!(constraints_of(&state, table).await, expected_constraints);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rooms_gain_an_activity_sequence() {
    let state = fresh_state().await;
    let columns = columns_of(&state, "rooms").await;
    assert_eq!(
        columns.last(),
        Some(&(
            "activity_seq".to_owned(),
            "bigint".to_owned(),
            "NO".to_owned()
        ))
    );
}

#[rstest]
#[case::thread_lookup(
    "room_messages_thread_idx",
    "CREATE INDEX room_messages_thread_idx ON public.room_messages USING btree (campus_id, thread_root_id, message_id) WHERE (thread_root_id IS NOT NULL)"
)]
#[case::activity_scan(
    "room_messages_activity_idx",
    "CREATE INDEX room_messages_activity_idx ON public.room_messages USING btree (campus_id, room_id, activity_seq)"
)]
#[case::subscriptions_by_account(
    "thread_subscriptions_account_idx",
    "CREATE INDEX thread_subscriptions_account_idx ON public.thread_subscriptions USING btree (campus_id, account_id)"
)]
#[case::mentions_by_account(
    "message_mentions_account_idx",
    "CREATE INDEX message_mentions_account_idx ON public.message_mentions USING btree (campus_id, account_id, message_id)"
)]
#[case::steward_rooms_by_room(
    "steward_rooms_room_idx",
    "CREATE INDEX steward_rooms_room_idx ON public.steward_rooms USING btree (campus_id, room_id)"
)]
#[case::one_campus_lounge(
    "lounges_campus_scope_idx",
    "CREATE UNIQUE INDEX lounges_campus_scope_idx ON public.lounges USING btree (campus_id) WHERE (room_id IS NULL)"
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn indexes_exist(#[case] name: &str, #[case] definition: &str) {
    let state = fresh_state().await;
    let found =
        sqlx::query_scalar::<_, String>("SELECT indexdef FROM pg_indexes WHERE indexname = $1")
            .bind(name)
            .fetch_optional(&state.db)
            .await
            .expect("index lookup");
    assert_eq!(found.as_deref(), Some(definition));
}

#[derive(Clone, Copy)]
enum Bind {
    Campus,
    ForeignCampus,
    Room,
    SiblingRoom,
    Message,
    Fresh,
    Nonce,
    Nil,
    Owner,
    Outsider,
    Null,
    Text(&'static str),
}

type Statement = (&'static str, &'static [Bind]);

fn stmt(sql: &'static str, binds: &'static [Bind]) -> Statement {
    (sql, binds)
}

const INSERT_REPLY: &str = "INSERT INTO room_messages \
    (campus_id, room_id, message_id, previous_message_id, author_id, client_nonce, body, thread_root_id) \
    VALUES ($1, $2, $3, $4, $5, $6, 'reply', $7)";
const INSERT_THREAD: &str = "INSERT INTO threads (campus_id, room_id, root_message_id, topic_title) VALUES ($1, $2, $3, $4)";
const INSERT_SUBSCRIPTION: &str = "INSERT INTO thread_subscriptions \
    (campus_id, room_id, root_message_id, account_id, state) VALUES ($1, $2, $3, $4, $5)";
const INSERT_REACTION: &str = "INSERT INTO message_reactions \
    (campus_id, room_id, message_id, account_id, emoji) VALUES ($1, $2, $3, $4, $5)";
const INSERT_MENTION: &str = "INSERT INTO message_mentions \
    (campus_id, room_id, message_id, account_id) VALUES ($1, $2, $3, $4)";

const OWNER_SUBSCRIBES: Statement = (
    INSERT_SUBSCRIPTION,
    &[
        Bind::Campus,
        Bind::Room,
        Bind::Message,
        Bind::Owner,
        Bind::Text("following"),
    ],
);
const OWNER_REACTS: Statement = (
    INSERT_REACTION,
    &[
        Bind::Campus,
        Bind::Room,
        Bind::Message,
        Bind::Owner,
        Bind::Text("👍"),
    ],
);
const OWNER_MENTIONED: Statement = (
    INSERT_MENTION,
    &[Bind::Campus, Bind::Room, Bind::Message, Bind::Owner],
);
const THREAD_TITLED: Statement = (
    INSERT_THREAD,
    &[Bind::Campus, Bind::Room, Bind::Message, Bind::Text("Plans")],
);

const INSERT_ACCOUNT: &str = "INSERT INTO accounts (account_id, email, display_name, password_hash, kind) \
    VALUES ($1, $2, 'Fixture', $3, $4)";
const INSERT_ACCOUNT_DEFAULT_KIND: &str = "INSERT INTO accounts (account_id, email, display_name, password_hash) \
    VALUES ($1, 'plain@example.org', 'Fixture', 'hash')";
const INSERT_MEMBER: &str =
    "INSERT INTO campus_members (campus_id, account_id, role) VALUES ($1, $2, $3)";
const INSERT_STEWARD_FIRST: &str = "INSERT INTO stewards (campus_id, steward_id, token_hash) \
    VALUES ($1, $2, decode('aa01', 'hex'))";
const INSERT_STEWARD_SECOND: &str = "INSERT INTO stewards (campus_id, steward_id, token_hash) \
    VALUES ($1, $2, decode('aa02', 'hex'))";
const INSERT_ASSIGNMENT: &str =
    "INSERT INTO steward_rooms (campus_id, steward_id, room_id) VALUES ($1, $2, $3)";
const INSERT_LOUNGE: &str = "INSERT INTO lounges (campus_id, lounge_id) VALUES ($1, $2)";
const INSERT_ROOM_LOUNGE: &str =
    "INSERT INTO lounges (campus_id, lounge_id, room_id) VALUES ($1, $2, $3)";

fn robot_account() -> Statement {
    stmt(
        INSERT_ACCOUNT,
        &[Bind::Fresh, Bind::Null, Bind::Null, Bind::Text("steward")],
    )
}

fn steward_member(campus: Bind) -> Statement {
    match campus {
        Bind::ForeignCampus => stmt(
            INSERT_MEMBER,
            &[Bind::ForeignCampus, Bind::Fresh, Bind::Text("steward")],
        ),
        _ => stmt(
            INSERT_MEMBER,
            &[Bind::Campus, Bind::Fresh, Bind::Text("steward")],
        ),
    }
}

fn steward_row(sql: &'static str, campus: Bind) -> Statement {
    match campus {
        Bind::ForeignCampus => stmt(sql, &[Bind::ForeignCampus, Bind::Fresh]),
        _ => stmt(sql, &[Bind::Campus, Bind::Fresh]),
    }
}

fn onboarded_steward() -> Vec<Statement> {
    vec![
        robot_account(),
        steward_member(Bind::Campus),
        steward_row(INSERT_STEWARD_FIRST, Bind::Campus),
    ]
}

fn onboarded_then(extras: Vec<Statement>) -> Vec<Statement> {
    [onboarded_steward(), extras].concat()
}

fn resolve(bind: Bind, context: &Context, fresh_id: MessageId) -> Param {
    match bind {
        Bind::Campus => Param::Id(context.stored.campus_id.as_uuid()),
        Bind::ForeignCampus => Param::Id(context.foreign_campus.as_uuid()),
        Bind::Room => Param::Id(context.stored.room_id.as_uuid()),
        Bind::SiblingRoom => Param::Id(context.sibling_room.as_uuid()),
        Bind::Message => Param::Id(context.stored.message_id.as_uuid()),
        Bind::Fresh => Param::Id(fresh_id.as_uuid()),
        Bind::Nonce => Param::Id(ClientNonce::generate().as_uuid()),
        Bind::Nil => Param::Id(Uuid::nil()),
        Bind::Owner => Param::Id(context.stored.author_id.as_uuid()),
        Bind::Outsider => Param::Id(context.other_author.as_uuid()),
        Bind::Text(text) => Param::Text(text),
        Bind::Null => Param::Null,
    }
}

enum Param {
    Id(Uuid),
    Text(&'static str),
    Null,
}

async fn run_statement(
    state: &AppState,
    context: &Context,
    (sql, binds): Statement,
) -> Result<(), sqlx::Error> {
    let fresh_id = fresh(context).message_id;
    let mut query = sqlx::query(sql);
    for bind in binds {
        query = match resolve(*bind, context, fresh_id) {
            Param::Id(id) => query.bind(id),
            Param::Text(text) => query.bind(text),
            Param::Null => query.bind(None::<&str>),
        };
    }
    query.execute(&state.db).await.map(|_| ())
}

fn violation_code(outcome: Result<(), sqlx::Error>) -> Option<String> {
    outcome.err().and_then(|error| {
        error
            .as_database_error()
            .and_then(|database| database.code().map(|code| code.to_string()))
    })
}

const INSERT_ATTACHMENT: &str = "INSERT INTO attachments \
    (campus_id, room_id, attachment_id, uploader_id, object_key, file_name, content_type, kind, byte_size) \
    VALUES ($1, $2, $3, $4, 'campuses/' || $1::text || '/rooms/' || $2::text || '/attachments/' || $3::text, 'a.png', 'image/png', 'image', 10)";
const INSERT_ATTACHMENT_KEYED: &str = "INSERT INTO attachments \
    (campus_id, room_id, attachment_id, uploader_id, object_key, file_name, content_type, kind, byte_size) \
    VALUES ($1, $2, $3, $4, $5, 'a.png', 'image/png', 'image', 10)";
const INSERT_ATTACHMENT_SIZED: &str = "INSERT INTO attachments \
    (campus_id, room_id, attachment_id, uploader_id, object_key, file_name, content_type, kind, byte_size) \
    VALUES ($1, $2, $3, $4, 'campuses/' || $1::text || '/rooms/' || $2::text || '/attachments/' || $3::text, 'a.png', 'image/png', $5, 10)";
const INSERT_ATTACHMENT_ATTACHED: &str = "INSERT INTO attachments \
    (campus_id, room_id, attachment_id, uploader_id, object_key, file_name, content_type, kind, byte_size, message_id, position) \
    VALUES ($1, $2, $3, $4, 'campuses/' || $1::text || '/rooms/' || $2::text || '/attachments/' || $3::text, 'a.png', 'image/png', 'image', 10, $5, 0)";
const INSERT_ATTACHMENT_UNPOSITIONED: &str = "INSERT INTO attachments \
    (campus_id, room_id, attachment_id, uploader_id, object_key, file_name, content_type, kind, byte_size, message_id) \
    VALUES ($1, $2, $3, $4, 'campuses/' || $1::text || '/rooms/' || $2::text || '/attachments/' || $3::text, 'a.png', 'image/png', 'image', 10, $5)";
const INSERT_LINK: &str = "INSERT INTO message_links (campus_id, room_id, message_id, position, url) \
    VALUES ($1, $2, $3, 0, 'https://example.org/a')";
const INSERT_PIN: &str = "INSERT INTO message_pins (campus_id, room_id, message_id, pinned_by) \
    VALUES ($1, $2, $3, $4)";
const INSERT_SHELF: &str = "INSERT INTO shelf_items \
    (campus_id, account_id, shelf_item_id, room_id, message_id) VALUES ($1, $4, $5, $2, $3)";
const INSERT_SHELF_REMINDED_EARLY: &str = "INSERT INTO shelf_items \
    (campus_id, account_id, shelf_item_id, room_id, message_id, remind_at, reminded_at) \
    VALUES ($1, $4, $5, $2, $3, now(), now() - interval '1 hour')";
const INSERT_SHELF_REMINDED_WITHOUT_TIME: &str = "INSERT INTO shelf_items \
    (campus_id, account_id, shelf_item_id, room_id, message_id, reminded_at) \
    VALUES ($1, $4, $5, $2, $3, now())";
const INSERT_SHELF_REMINDED_AFTER: &str = "INSERT INTO shelf_items \
    (campus_id, account_id, shelf_item_id, room_id, message_id, remind_at, reminded_at) \
    VALUES ($1, $4, $5, $2, $3, now() - interval '1 hour', now())";

const ATTACHMENT_BY_OWNER: Statement = (
    INSERT_ATTACHMENT,
    &[Bind::Campus, Bind::Room, Bind::Nonce, Bind::Owner],
);
const ATTACHMENT_BY_OUTSIDER: Statement = (
    INSERT_ATTACHMENT,
    &[Bind::Campus, Bind::Room, Bind::Nonce, Bind::Outsider],
);
const ATTACHMENT_IN_FOREIGN_CAMPUS_ROOM: Statement = (
    INSERT_ATTACHMENT,
    &[Bind::ForeignCampus, Bind::Room, Bind::Nonce, Bind::Owner],
);
const ATTACHMENT_WITH_FOREIGN_KEY: Statement = (
    INSERT_ATTACHMENT_KEYED,
    &[
        Bind::Campus,
        Bind::Room,
        Bind::Nonce,
        Bind::Owner,
        Bind::Text("campuses/other/rooms/other/attachments/other"),
    ],
);
const ATTACHMENT_ATTACHED: Statement = (
    INSERT_ATTACHMENT_ATTACHED,
    &[
        Bind::Campus,
        Bind::Room,
        Bind::Nonce,
        Bind::Owner,
        Bind::Message,
    ],
);
const ATTACHMENT_WITHOUT_POSITION: Statement = (
    INSERT_ATTACHMENT_UNPOSITIONED,
    &[
        Bind::Campus,
        Bind::Room,
        Bind::Nonce,
        Bind::Owner,
        Bind::Message,
    ],
);
const ATTACHMENT_OF_KIND_ARCHIVE: Statement = (
    INSERT_ATTACHMENT_SIZED,
    &[
        Bind::Campus,
        Bind::Room,
        Bind::Nonce,
        Bind::Owner,
        Bind::Text("archive"),
    ],
);
const LINK_ON_MESSAGE: Statement = (INSERT_LINK, &[Bind::Campus, Bind::Room, Bind::Message]);
const LINK_ON_SIBLING_ROOM: Statement = (
    INSERT_LINK,
    &[Bind::Campus, Bind::SiblingRoom, Bind::Message],
);
const PIN_BY_OWNER: Statement = (
    INSERT_PIN,
    &[Bind::Campus, Bind::Room, Bind::Message, Bind::Owner],
);
const PIN_BY_OUTSIDER: Statement = (
    INSERT_PIN,
    &[Bind::Campus, Bind::Room, Bind::Message, Bind::Outsider],
);
const PIN_ON_SIBLING_ROOM: Statement = (
    INSERT_PIN,
    &[Bind::Campus, Bind::SiblingRoom, Bind::Message, Bind::Owner],
);

fn shelf(sql: &'static str, account: Bind) -> Statement {
    match account {
        Bind::Outsider => (
            sql,
            &[
                Bind::Campus,
                Bind::Room,
                Bind::Message,
                Bind::Outsider,
                Bind::Nonce,
            ],
        ),
        _ => (
            sql,
            &[
                Bind::Campus,
                Bind::Room,
                Bind::Message,
                Bind::Owner,
                Bind::Nonce,
            ],
        ),
    }
}

const REPLY_TO_STORED: Statement = (
    INSERT_REPLY,
    &[
        Bind::Campus,
        Bind::Room,
        Bind::Fresh,
        Bind::Message,
        Bind::Owner,
        Bind::Nonce,
        Bind::Message,
    ],
);

#[rstest]
#[case::reply_to_stored_root(vec![REPLY_TO_STORED], None)]
#[case::reply_root_equal_to_own_id(
    vec![stmt(INSERT_REPLY, &[Bind::Campus, Bind::Room, Bind::Fresh, Bind::Message, Bind::Owner, Bind::Nonce, Bind::Fresh])],
    Some(CHECK_VIOLATION),
)]
#[case::reply_root_after_own_id(
    vec![stmt(INSERT_REPLY, &[Bind::Campus, Bind::Room, Bind::Fresh, Bind::Message, Bind::Owner, Bind::Nonce, Bind::Nonce])],
    Some(CHECK_VIOLATION),
)]
#[case::reply_root_does_not_exist(
    vec![stmt(INSERT_REPLY, &[Bind::Campus, Bind::Room, Bind::Fresh, Bind::Message, Bind::Owner, Bind::Nonce, Bind::Nil])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::reply_root_in_sibling_room(
    vec![stmt(INSERT_REPLY, &[Bind::Campus, Bind::SiblingRoom, Bind::Fresh, Bind::Message, Bind::Owner, Bind::Nonce, Bind::Message])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::reply_root_in_foreign_campus(
    vec![stmt(INSERT_REPLY, &[Bind::ForeignCampus, Bind::Room, Bind::Fresh, Bind::Message, Bind::Owner, Bind::Nonce, Bind::Message])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::thread_title_row(vec![THREAD_TITLED], None)]
#[case::thread_row_is_unique(vec![THREAD_TITLED, THREAD_TITLED], Some(UNIQUE_VIOLATION))]
#[case::thread_row_needs_its_room(
    vec![stmt(INSERT_THREAD, &[Bind::Campus, Bind::SiblingRoom, Bind::Message, Bind::Text("Plans")])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::subscription_row(vec![OWNER_SUBSCRIBES], None)]
#[case::subscription_is_unique_per_account(vec![OWNER_SUBSCRIBES, OWNER_SUBSCRIBES], Some(UNIQUE_VIOLATION))]
#[case::subscription_state_is_restricted(
    vec![stmt(INSERT_SUBSCRIPTION, &[Bind::Campus, Bind::Room, Bind::Message, Bind::Owner, Bind::Text("snoozed")])],
    Some(CHECK_VIOLATION),
)]
#[case::subscription_needs_campus_member(
    vec![stmt(INSERT_SUBSCRIPTION, &[Bind::Campus, Bind::Room, Bind::Message, Bind::Outsider, Bind::Text("following")])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::subscription_needs_the_roots_room(
    vec![stmt(INSERT_SUBSCRIPTION, &[Bind::Campus, Bind::SiblingRoom, Bind::Message, Bind::Owner, Bind::Text("following")])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::subscription_needs_an_existing_root(
    vec![stmt(INSERT_SUBSCRIPTION, &[Bind::Campus, Bind::Room, Bind::Fresh, Bind::Owner, Bind::Text("following")])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::subscription_in_foreign_campus(
    vec![stmt(INSERT_SUBSCRIPTION, &[Bind::ForeignCampus, Bind::Room, Bind::Message, Bind::Owner, Bind::Text("following")])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::reaction_row(vec![OWNER_REACTS], None)]
#[case::two_emoji_by_one_account(
    vec![
        OWNER_REACTS,
        stmt(INSERT_REACTION, &[Bind::Campus, Bind::Room, Bind::Message, Bind::Owner, Bind::Text("🎉")]),
    ],
    None,
)]
#[case::reaction_is_unique_per_emoji(vec![OWNER_REACTS, OWNER_REACTS], Some(UNIQUE_VIOLATION))]
#[case::reaction_needs_campus_member(
    vec![stmt(INSERT_REACTION, &[Bind::Campus, Bind::Room, Bind::Message, Bind::Outsider, Bind::Text("👍")])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::reaction_needs_the_messages_room(
    vec![stmt(INSERT_REACTION, &[Bind::Campus, Bind::SiblingRoom, Bind::Message, Bind::Owner, Bind::Text("👍")])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::mention_row(vec![OWNER_MENTIONED], None)]
#[case::mention_is_unique_per_account(vec![OWNER_MENTIONED, OWNER_MENTIONED], Some(UNIQUE_VIOLATION))]
#[case::mention_needs_campus_member(
    vec![stmt(INSERT_MENTION, &[Bind::Campus, Bind::Room, Bind::Message, Bind::Outsider])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::attachment_row(vec![ATTACHMENT_BY_OWNER], None)]
#[case::attachment_attached_with_position(vec![ATTACHMENT_ATTACHED], None)]
#[case::attachment_position_is_unique_per_message(
    vec![ATTACHMENT_ATTACHED, ATTACHMENT_ATTACHED],
    Some(UNIQUE_VIOLATION)
)]
#[case::attachment_key_must_match_formula(
    vec![ATTACHMENT_WITH_FOREIGN_KEY],
    Some(CHECK_VIOLATION)
)]
#[case::attachment_message_needs_position(
    vec![ATTACHMENT_WITHOUT_POSITION],
    Some(CHECK_VIOLATION)
)]
#[case::attachment_kind_is_restricted(
    vec![ATTACHMENT_OF_KIND_ARCHIVE],
    Some(CHECK_VIOLATION)
)]
#[case::attachment_needs_campus_member(
    vec![ATTACHMENT_BY_OUTSIDER],
    Some(FOREIGN_KEY_VIOLATION)
)]
#[case::attachment_room_belongs_to_campus(
    vec![ATTACHMENT_IN_FOREIGN_CAMPUS_ROOM],
    Some(FOREIGN_KEY_VIOLATION)
)]
#[case::link_row(vec![LINK_ON_MESSAGE], None)]
#[case::link_position_is_unique(vec![LINK_ON_MESSAGE, LINK_ON_MESSAGE], Some(UNIQUE_VIOLATION))]
#[case::link_needs_the_messages_room(vec![LINK_ON_SIBLING_ROOM], Some(FOREIGN_KEY_VIOLATION))]
#[case::pin_row(vec![PIN_BY_OWNER], None)]
#[case::pin_is_unique_per_message(vec![PIN_BY_OWNER, PIN_BY_OWNER], Some(UNIQUE_VIOLATION))]
#[case::pin_needs_campus_member(vec![PIN_BY_OUTSIDER], Some(FOREIGN_KEY_VIOLATION))]
#[case::pin_needs_the_messages_room(vec![PIN_ON_SIBLING_ROOM], Some(FOREIGN_KEY_VIOLATION))]
#[case::shelf_row(vec![shelf(INSERT_SHELF, Bind::Owner)], None)]
#[case::shelf_is_unique_per_account_and_message(
    vec![shelf(INSERT_SHELF, Bind::Owner), shelf(INSERT_SHELF, Bind::Owner)],
    Some(UNIQUE_VIOLATION)
)]
#[case::shelf_needs_campus_member(
    vec![shelf(INSERT_SHELF, Bind::Outsider)],
    Some(FOREIGN_KEY_VIOLATION)
)]
#[case::shelf_reminded_after_remind_at(
    vec![shelf(INSERT_SHELF_REMINDED_AFTER, Bind::Owner)],
    None
)]
#[case::shelf_reminded_before_remind_at(
    vec![shelf(INSERT_SHELF_REMINDED_EARLY, Bind::Owner)],
    Some(CHECK_VIOLATION)
)]
#[case::shelf_reminded_without_remind_at(
    vec![shelf(INSERT_SHELF_REMINDED_WITHOUT_TIME, Bind::Owner)],
    Some(CHECK_VIOLATION)
)]
#[case::mention_needs_the_messages_room(
    vec![stmt(INSERT_MENTION, &[Bind::Campus, Bind::SiblingRoom, Bind::Message, Bind::Owner])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::steward_account_without_credentials_is_valid(vec![robot_account()], None)]
#[case::steward_account_with_full_credentials(
    vec![stmt(INSERT_ACCOUNT, &[Bind::Fresh, Bind::Text("bot@example.org"), Bind::Text("hash"), Bind::Text("steward")])],
    Some(CHECK_VIOLATION),
)]
#[case::steward_account_with_only_an_email(
    vec![stmt(INSERT_ACCOUNT, &[Bind::Fresh, Bind::Text("bot@example.org"), Bind::Null, Bind::Text("steward")])],
    Some(CHECK_VIOLATION),
)]
#[case::steward_account_with_only_a_password(
    vec![stmt(INSERT_ACCOUNT, &[Bind::Fresh, Bind::Null, Bind::Text("hash"), Bind::Text("steward")])],
    Some(CHECK_VIOLATION),
)]
#[case::person_account_without_a_password(
    vec![stmt(INSERT_ACCOUNT, &[Bind::Fresh, Bind::Text("p@example.org"), Bind::Null, Bind::Text("person")])],
    Some(CHECK_VIOLATION),
)]
#[case::person_account_without_an_email(
    vec![stmt(INSERT_ACCOUNT, &[Bind::Fresh, Bind::Null, Bind::Text("hash"), Bind::Text("person")])],
    Some(CHECK_VIOLATION),
)]
#[case::unknown_account_kind(
    vec![stmt(INSERT_ACCOUNT, &[Bind::Fresh, Bind::Text("r@example.org"), Bind::Text("hash"), Bind::Text("robot")])],
    Some(CHECK_VIOLATION),
)]
#[case::accounts_default_to_person(
    vec![stmt(INSERT_ACCOUNT_DEFAULT_KIND, &[Bind::Fresh])],
    None,
)]
#[case::steward_member_role_is_valid(vec![robot_account(), steward_member(Bind::Campus)], None)]
#[case::unknown_member_role(
    vec![robot_account(), stmt(INSERT_MEMBER, &[Bind::Campus, Bind::Fresh, Bind::Text("wizard")])],
    Some(CHECK_VIOLATION),
)]
#[case::steward_row_needs_a_membership(
    vec![robot_account(), steward_row(INSERT_STEWARD_FIRST, Bind::Campus)],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::steward_row_is_valid(onboarded_steward(), None)]
#[case::token_digest_is_unique(
    onboarded_then(vec![stmt(INSERT_STEWARD_FIRST, &[Bind::Campus, Bind::Owner])]),
    Some(UNIQUE_VIOLATION),
)]
#[case::steward_belongs_to_one_campus(
    onboarded_then(vec![
        steward_member(Bind::ForeignCampus),
        steward_row(INSERT_STEWARD_SECOND, Bind::ForeignCampus),
    ]),
    Some(UNIQUE_VIOLATION),
)]
#[case::assignment_is_valid(
    onboarded_then(vec![stmt(INSERT_ASSIGNMENT, &[Bind::Campus, Bind::Fresh, Bind::Room])]),
    None,
)]
#[case::assignment_is_unique(
    onboarded_then(vec![
        stmt(INSERT_ASSIGNMENT, &[Bind::Campus, Bind::Fresh, Bind::Room]),
        stmt(INSERT_ASSIGNMENT, &[Bind::Campus, Bind::Fresh, Bind::Room]),
    ]),
    Some(UNIQUE_VIOLATION),
)]
#[case::assignment_needs_an_existing_room(
    onboarded_then(vec![stmt(INSERT_ASSIGNMENT, &[Bind::Campus, Bind::Fresh, Bind::Nil])]),
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::assignment_needs_a_steward_of_the_campus(
    onboarded_then(vec![stmt(INSERT_ASSIGNMENT, &[Bind::ForeignCampus, Bind::Fresh, Bind::Room])]),
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::one_lounge_per_campus_scope(
    vec![stmt(INSERT_LOUNGE, &[Bind::Campus, Bind::Nonce])],
    Some(UNIQUE_VIOLATION),
)]
#[case::one_lounge_per_room(
    vec![stmt(INSERT_ROOM_LOUNGE, &[Bind::Campus, Bind::Nonce, Bind::Room])],
    Some(UNIQUE_VIOLATION),
)]
#[case::lounge_room_must_share_the_campus(
    vec![stmt(INSERT_ROOM_LOUNGE, &[Bind::ForeignCampus, Bind::Nonce, Bind::Room])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[case::lounge_needs_a_campus(
    vec![stmt(INSERT_LOUNGE, &[Bind::Nil, Bind::Nonce])],
    Some(FOREIGN_KEY_VIOLATION),
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn table_constraints_hold(
    #[case] statements: Vec<Statement>,
    #[case] violation: Option<&str>,
) {
    let (state, context) = seeded().await;
    let mut outcome = Ok(());
    for statement in statements {
        outcome = run_statement(&state, &context, statement).await;
        if outcome.is_err() {
            break;
        }
    }
    assert_eq!(violation_code(outcome).as_deref(), violation);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn deleting_a_root_cascades_to_its_thread_rows() {
    let (state, context) = seeded().await;
    for statement in [
        REPLY_TO_STORED,
        THREAD_TITLED,
        OWNER_SUBSCRIBES,
        OWNER_REACTS,
        OWNER_MENTIONED,
    ] {
        run_statement(&state, &context, statement)
            .await
            .expect("seed thread rows");
    }
    let remaining = |table: &'static str| {
        let state = state.clone();
        let campus = context.stored.campus_id;
        async move { common::count_rows(&state, table, campus).await }
    };
    assert_eq!(remaining("room_messages").await, 2);
    sqlx::query("DELETE FROM room_messages WHERE campus_id = $1 AND message_id = $2")
        .bind(context.stored.campus_id.as_uuid())
        .bind(context.stored.message_id.as_uuid())
        .execute(&state.db)
        .await
        .expect("delete root");
    for table in [
        "room_messages",
        "threads",
        "thread_subscriptions",
        "message_reactions",
        "message_mentions",
    ] {
        assert_eq!(remaining(table).await, 0, "{table}");
    }
}

#[rstest]
#[case::attachments(
    "attachments",
    &[
        "campus_id", "room_id", "attachment_id", "uploader_id", "object_key", "file_name",
        "content_type", "kind", "byte_size", "message_id", "position", "created_at",
    ]
)]
#[case::message_links(
    "message_links",
    &["campus_id", "room_id", "message_id", "position", "url"]
)]
#[case::message_pins(
    "message_pins",
    &["campus_id", "room_id", "message_id", "pinned_by", "pinned_at"]
)]
#[case::shelf_items(
    "shelf_items",
    &[
        "campus_id", "account_id", "shelf_item_id", "room_id", "message_id", "remind_at",
        "reminded_at", "done_at", "revision", "created_at", "updated_at",
    ]
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn media_vault_shelf_tables_match_contract(#[case] table: &str, #[case] expected: &[&str]) {
    let state = fresh_state().await;
    let names = columns_of(&state, table)
        .await
        .into_iter()
        .map(|(name, _, _)| name)
        .collect::<Vec<_>>();
    assert_eq!(names, expected);
}

#[rstest]
#[case::attachment_position("attachments_message_position_idx")]
#[case::attachment_vault("attachments_vault_idx")]
#[case::link_room("message_links_room_idx")]
#[case::pin_room("message_pins_room_idx")]
#[case::shelf_account("shelf_items_account_idx")]
#[case::shelf_due("shelf_items_due_idx")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn media_vault_shelf_indexes_exist(#[case] name: &str) {
    let state = fresh_state().await;
    let found = sqlx::query_scalar::<_, String>(
        "SELECT indexname::text FROM pg_indexes WHERE indexname = $1",
    )
    .bind(name)
    .fetch_optional(&state.db)
    .await
    .expect("index lookup");
    assert_eq!(found.as_deref(), Some(name));
}

#[rstest]
#[case::accounts(
    "accounts",
    &[
        ("account_id", "uuid", "NO"),
        ("email", "text", "YES"),
        ("display_name", "text", "NO"),
        ("password_hash", "text", "YES"),
        ("created_at", "timestamp with time zone", "NO"),
        ("kind", "text", "NO"),
    ]
)]
#[case::stewards(
    "stewards",
    &[
        ("campus_id", "uuid", "NO"),
        ("steward_id", "uuid", "NO"),
        ("token_hash", "bytea", "NO"),
        ("credential_revision", "bigint", "NO"),
        ("work_cursor", "uuid", "YES"),
        ("revoked_at", "timestamp with time zone", "YES"),
        ("created_at", "timestamp with time zone", "NO"),
    ]
)]
#[case::steward_rooms(
    "steward_rooms",
    &[("campus_id", "uuid", "NO"), ("steward_id", "uuid", "NO"), ("room_id", "uuid", "NO")]
)]
#[case::lounges(
    "lounges",
    &[
        ("campus_id", "uuid", "NO"),
        ("lounge_id", "uuid", "NO"),
        ("room_id", "uuid", "YES"),
        ("created_at", "timestamp with time zone", "NO"),
    ]
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn steward_and_lounge_tables_match_contract(
    #[case] table: &str,
    #[case] expected: &[Column],
) {
    let state = fresh_state().await;
    let columns = columns_of(&state, table).await;
    let expected = expected
        .iter()
        .map(|(name, kind, nullable)| {
            (
                (*name).to_owned(),
                (*kind).to_owned(),
                (*nullable).to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(columns, expected);
}

#[rstest]
#[case::two_campuses(&[("alpha", 3), ("bravo", 1)])]
#[case::campus_without_rooms(&[("alpha", 2), ("empty", 0)])]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn lounges_are_backfilled_for_existing_campuses_and_rooms(
    #[case] campuses: &[(&str, usize)],
) {
    let state = state_before("create stewards lounges").await;
    let account_id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO accounts (account_id, email, display_name, password_hash) \
         VALUES ($1, 'old@example.org', 'Old', 'hash')",
    )
    .bind(account_id)
    .execute(&state.db)
    .await
    .expect("seed account");
    let mut seeded = Vec::new();
    for (slug, room_count) in campuses {
        let campus_id = Uuid::now_v7();
        sqlx::query("INSERT INTO campuses (campus_id, slug, display_name) VALUES ($1, $2, $2)")
            .bind(campus_id)
            .bind(*slug)
            .execute(&state.db)
            .await
            .expect("seed campus");
        sqlx::query(
            "INSERT INTO campus_members (campus_id, account_id, role) VALUES ($1, $2, 'owner')",
        )
        .bind(campus_id)
        .bind(account_id)
        .execute(&state.db)
        .await
        .expect("seed membership");
        let mut rooms = BTreeSet::new();
        for index in 0..*room_count {
            let room_id = Uuid::now_v7();
            sqlx::query("INSERT INTO rooms (campus_id, room_id, slug) VALUES ($1, $2, $3)")
                .bind(campus_id)
                .bind(room_id)
                .bind(format!("room-{index}"))
                .execute(&state.db)
                .await
                .expect("seed room");
            rooms.insert(room_id);
        }
        seeded.push((campus_id, rooms));
    }
    MIGRATOR
        .run(&state.db)
        .await
        .expect("apply the remaining migrations");
    let mut every_lounge = BTreeSet::new();
    for (campus_id, rooms) in &seeded {
        let rows = sqlx::query_as::<_, (Uuid, Option<Uuid>)>(
            "SELECT lounge_id, room_id FROM lounges WHERE campus_id = $1",
        )
        .bind(campus_id)
        .fetch_all(&state.db)
        .await
        .expect("lounges");
        assert_eq!(rows.len(), rooms.len() + 1);
        assert_eq!(rows.iter().filter(|(_, room)| room.is_none()).count(), 1);
        let scoped = rows
            .iter()
            .filter_map(|(_, room)| *room)
            .collect::<BTreeSet<_>>();
        assert_eq!(&scoped, rooms);
        every_lounge.extend(rows.iter().map(|(lounge_id, _)| *lounge_id));
    }
    let expected_total = seeded
        .iter()
        .map(|(_, rooms)| rooms.len() + 1)
        .sum::<usize>();
    assert_eq!(every_lounge.len(), expected_total);
    let kind = sqlx::query_scalar::<_, String>("SELECT kind FROM accounts WHERE account_id = $1")
        .bind(account_id)
        .fetch_one(&state.db)
        .await
        .expect("kind");
    assert_eq!(kind, "person");
}
