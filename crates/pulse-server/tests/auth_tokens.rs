mod common;

use std::time::Duration;

use axum::http::{Method, StatusCode};
use common::{
    Observed, api_get, api_raw, campus_for, expire_token, handshake_outcome, sign_in_as, signed_up,
    spawn_server_with,
};
use pulse_protocol::{HandshakeRejected, HandshakeRejection, Role, ServerFrame, api_path};
use rstest::rstest;
use tokio::time::timeout;

const DEADLINE: Duration = Duration::from_secs(30);
const EMAIL: &str = "ada@example.org";

#[derive(Clone, Copy)]
enum Mutation {
    ExpireProbe,
    SignOutProbe,
    ExpireSibling,
}

enum WsOutcome {
    Welcome,
    Unauthenticated,
}

#[rstest]
#[case::expired_probe(
    Mutation::ExpireProbe,
    Some(StatusCode::UNAUTHORIZED),
    WsOutcome::Unauthenticated
)]
#[case::signed_out_probe(Mutation::SignOutProbe, None, WsOutcome::Unauthenticated)]
#[case::expired_sibling(Mutation::ExpireSibling, Some(StatusCode::OK), WsOutcome::Welcome)]
#[tokio::test]
async fn token_lifecycle_is_enforced_on_http_and_websocket(
    #[case] mutation: Mutation,
    #[case] http: Option<StatusCode>,
    #[case] ws: WsOutcome,
) {
    timeout(DEADLINE, async {
        let state = fresh_state_with_account().await;
        let probe = sign_in_as(&state, EMAIL).await;
        let sibling = sign_in_as(&state, EMAIL).await;
        let campus_id = campus_for(&state, &probe.token, "north-quad")
            .await
            .campus
            .campus_id;
        match mutation {
            Mutation::ExpireProbe => expire_token(&state, &probe.token).await,
            Mutation::SignOutProbe => {
                let (status, _) = api_raw(
                    &state,
                    Method::DELETE,
                    api_path::CURRENT_AUTH_TOKEN,
                    Some(&probe.token),
                    None,
                )
                .await;
                assert_eq!(status, StatusCode::NO_CONTENT);
            }
            Mutation::ExpireSibling => expire_token(&state, &sibling.token).await,
        }
        if let Some(expected) = http {
            let (status, _) = api_get(&state, api_path::CAMPUSES, Some(&probe.token)).await;
            assert_eq!(status, expected);
        }
        let addr = spawn_server_with(state).await;
        let outcome = handshake_outcome(addr, campus_id, &probe.token).await;
        match ws {
            WsOutcome::Welcome => {
                let Observed::Frame(ServerFrame::Welcome(welcome)) = outcome else {
                    panic!("expected welcome, got {outcome:?}");
                };
                assert_eq!(welcome.role, Role::Owner);
            }
            WsOutcome::Unauthenticated => assert_eq!(
                outcome,
                Observed::Frame(ServerFrame::HandshakeRejected(HandshakeRejected {
                    reason: HandshakeRejection::Unauthenticated,
                }))
            ),
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test]
async fn repeated_sign_ins_mint_distinct_tokens_that_all_authenticate() {
    timeout(DEADLINE, async {
        let state = fresh_state_with_account().await;
        let first = sign_in_as(&state, EMAIL).await;
        let second = sign_in_as(&state, EMAIL).await;
        assert_ne!(first.token, second.token);
        let campus_id = campus_for(&state, &first.token, "north-quad")
            .await
            .campus
            .campus_id;
        let addr = spawn_server_with(state.clone()).await;
        for token in [&first.token, &second.token] {
            let (status, _) = api_get(&state, api_path::CAMPUSES, Some(token)).await;
            assert_eq!(status, StatusCode::OK);
            let Observed::Frame(ServerFrame::Welcome(welcome)) =
                handshake_outcome(addr, campus_id, token).await
            else {
                panic!("expected welcome");
            };
            assert_eq!(welcome.campus_id, campus_id);
        }
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::three_racers(3)]
#[case::eight_racers(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_same_email_sign_ups_yield_exactly_one_account(#[case] racers: usize) {
    timeout(DEADLINE, async {
        let state = common::fresh_state().await;
        let mut attempts = tokio::task::JoinSet::new();
        for index in 0..racers {
            let state = state.clone();
            attempts.spawn(async move {
                let body = serde_json::json!({
                    "email": EMAIL,
                    "display_name": format!("Racer {index}"),
                    "password": pulse_protocol::fixtures::VALID_PASSWORD,
                })
                .to_string();
                api_raw(&state, Method::POST, api_path::ACCOUNTS, None, Some(body)).await
            });
        }
        let mut created = 0;
        let mut conflicts = 0;
        while let Some(done) = attempts.join_next().await {
            let (status, body) = done.expect("attempt task");
            match status {
                StatusCode::CREATED => created += 1,
                StatusCode::CONFLICT => {
                    conflicts += 1;
                    assert_eq!(
                        body,
                        serde_json::json!({"code": "conflict", "field": "email"})
                    );
                }
                other => panic!("unexpected status {other}: {body}"),
            }
        }
        assert_eq!((created, conflicts), (1, racers - 1));
        let accounts = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM accounts")
            .fetch_one(&state.db)
            .await
            .expect("count");
        assert_eq!(accounts, 1);
    })
    .await
    .expect("scenario deadline");
}

async fn fresh_state_with_account() -> pulse_server::AppState {
    let state = common::fresh_state().await;
    signed_up(&state, EMAIL).await;
    state
}
