mod common;

use common::{
    SCENARIO_DEADLINE, Stage, join_lounge_frame, joined_socket, lounge_rows, member_session,
    next_of, send, spawn_server_with,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use pulse_protocol::{
    AuthToken, CampusId, LOUNGE_TICKET_TTL_MS, LoungeId, LoungeTicket, Role, ServerFrame,
};
use pulse_server::livekit::{LiveKitClaims, livekit_room_name};
use pulse_server::lounge::LoungeConfig;
use rstest::rstest;
use tokio::time::timeout;

const VALIDATE_URL: &str = "http://127.0.0.1:7880/rtc/validate";

async fn ticket_for(
    addr: std::net::SocketAddr,
    campus_id: CampusId,
    token: &AuthToken,
    lounge_id: LoungeId,
) -> LoungeTicket {
    let (mut socket, _) = joined_socket(addr, campus_id, token).await;
    send(&mut socket, &join_lounge_frame(lounge_id)).await;
    next_of(&mut socket, |frame| match frame {
        ServerFrame::LoungeJoined(joined) => Some(joined.ticket.clone()),
        _ => None,
    })
    .await
}

fn decode_with(ticket: &LoungeTicket, key: &str, issuer: &str) -> Result<LiveKitClaims, String> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[issuer]);
    validation.validate_nbf = true;
    validation.set_required_spec_claims(&["exp", "nbf", "iss", "sub"]);
    decode::<LiveKitClaims>(
        &ticket.token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|error| error.to_string())
}

async fn campus_lounge(stage: &Stage) -> LoungeId {
    lounge_rows(&stage.state, stage.campus.id).await[0].0
}

#[derive(Clone, Copy, Debug)]
enum Who {
    Owner,
    Admin,
    Member,
    Guest,
}

#[rstest]
#[case::owner(Who::Owner, true)]
#[case::admin(Who::Admin, true)]
#[case::member(Who::Member, true)]
#[case::guest(Who::Guest, false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn claims_follow_the_role_and_the_lounge(#[case] who: Who, #[case] can_publish: bool) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let admin = member_session(
            &stage.state,
            "admin@example.org",
            stage.campus.id,
            Role::Admin,
        )
        .await;
        let session = match who {
            Who::Owner => &stage.campus.owner,
            Who::Admin => &admin,
            Who::Member => &stage.ada,
            Who::Guest => &stage.guest,
        };
        let rows = lounge_rows(&stage.state, stage.campus.id).await;
        for (lounge_id, _) in rows {
            let ticket = ticket_for(stage.addr, stage.campus.id, &session.token, lounge_id).await;
            let claims = decode_with(&ticket, "secret", "devkey").expect("valid ticket");
            assert_eq!(claims.iss, "devkey");
            assert_eq!(claims.sub, session.account.account_id.to_string());
            assert_eq!(claims.name, session.account.display_name);
            assert_eq!(claims.exp - claims.nbf, LOUNGE_TICKET_TTL_MS / 1000);
            assert_eq!(ticket.expires_at_ms, claims.exp * 1000);
            assert_eq!(ticket.lounge_id, lounge_id);
            assert_eq!(ticket.livekit_url, "ws://127.0.0.1:7880");
            let room = livekit_room_name(stage.campus.id, lounge_id);
            assert_eq!(ticket.room_name, room);
            assert_eq!(claims.video.room, room);
            assert!(claims.video.room_join);
            assert!(claims.video.can_subscribe);
            assert!(!claims.video.can_publish_data);
            assert_eq!(claims.video.can_publish, can_publish);
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn configured_credentials_and_lifetime_are_honoured() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let config = LoungeConfig {
            livekit_url: "wss://media.example.org".to_owned(),
            api_key: "other-key".to_owned(),
            api_secret: "other-secret".to_owned(),
            presence_ttl_ms: 30_000,
            ticket_ttl_ms: 120_000,
        };
        let addr = spawn_server_with(stage.state.clone().with_lounge(config)).await;
        let lounge_id = campus_lounge(&stage).await;
        let ticket = ticket_for(addr, stage.campus.id, &stage.ada.token, lounge_id).await;
        assert_eq!(ticket.livekit_url, "wss://media.example.org");
        let claims = decode_with(&ticket, "other-secret", "other-key").expect("valid ticket");
        assert_eq!(claims.exp - claims.nbf, 120);
        assert!(decode_with(&ticket, "secret", "other-key").is_err());
        assert!(decode_with(&ticket, "other-secret", "devkey").is_err());
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Tamper {
    WrongSecret,
    FlippedPayload,
    TruncatedSignature,
    WrongIssuer,
}

#[rstest]
#[case::wrong_secret(Tamper::WrongSecret)]
#[case::flipped_payload(Tamper::FlippedPayload)]
#[case::truncated_signature(Tamper::TruncatedSignature)]
#[case::wrong_issuer(Tamper::WrongIssuer)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn altered_tickets_fail_local_verification(#[case] tamper: Tamper) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage).await;
        let ticket = ticket_for(stage.addr, stage.campus.id, &stage.ada.token, lounge_id).await;
        assert!(decode_with(&ticket, "secret", "devkey").is_ok());
        let verdict = match tamper {
            Tamper::WrongSecret => decode_with(&ticket, "not-the-secret", "devkey"),
            Tamper::WrongIssuer => decode_with(&ticket, "secret", "another-key"),
            Tamper::FlippedPayload => decode_with(&tampered(&ticket), "secret", "devkey"),
            Tamper::TruncatedSignature => {
                let mut cut = ticket.clone();
                cut.token.truncate(cut.token.len() - 4);
                decode_with(&cut, "secret", "devkey")
            }
        };
        assert!(verdict.is_err(), "{tamper:?}");
    })
    .await
    .expect("scenario deadline");
}

fn tampered(ticket: &LoungeTicket) -> LoungeTicket {
    let mut parts = ticket
        .token
        .split('.')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let flipped = if parts[1].starts_with('e') { 'f' } else { 'e' };
    parts[1].replace_range(0..1, &flipped.to_string());
    LoungeTicket {
        token: parts.join("."),
        ..ticket.clone()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn tickets_are_scoped_to_their_campus_and_lounge() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let other =
            common::campus_with_owner(&stage.state, "other@example.org", "south-quad").await;
        let ours = campus_lounge(&stage).await;
        let theirs = lounge_rows(&stage.state, other.id).await[0].0;
        let mine = ticket_for(stage.addr, stage.campus.id, &stage.ada.token, ours).await;
        let foreign = ticket_for(stage.addr, other.id, &other.owner.token, theirs).await;
        assert_ne!(mine.room_name, foreign.room_name);
        assert!(mine.room_name.contains(&stage.campus.id.to_string()));
        assert!(foreign.room_name.contains(&other.id.to_string()));
        let rooms = lounge_rows(&stage.state, stage.campus.id).await;
        let sibling = ticket_for(stage.addr, stage.campus.id, &stage.ada.token, rooms[1].0).await;
        assert_ne!(sibling.room_name, mine.room_name);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_media_server_accepts_valid_tickets_and_refuses_tampered_ones() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage).await;
        let ticket = ticket_for(stage.addr, stage.campus.id, &stage.ada.token, lounge_id).await;
        let client = reqwest::Client::new();
        let verdict = |token: String| {
            let client = client.clone();
            async move {
                client
                    .get(VALIDATE_URL)
                    .query(&[("access_token", token)])
                    .send()
                    .await
                    .expect("livekit reachable")
                    .status()
            }
        };
        assert_eq!(verdict(ticket.token.clone()).await.as_u16(), 200);
        assert_eq!(verdict(tampered(&ticket).token).await.as_u16(), 401);
        let mut forged = ticket.clone();
        forged.token.truncate(forged.token.len() - 4);
        assert_eq!(verdict(forged.token).await.as_u16(), 401);
    })
    .await
    .expect("scenario deadline");
}
