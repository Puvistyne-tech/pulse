mod common;

use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Duration;

use common::{
    Observed, campus_for, connect, fresh_state, hello, observe, ping, send, signed_up,
    spawn_server_with,
};
use pulse_protocol::{AuthToken, CampusId, Pong, ProtocolVersion, ServerFrame, SessionId};
use tokio::time::timeout;

const CLIENTS: u32 = 16;
const PINGS_PER_CLIENT: u32 = 4;
const DEADLINE: Duration = Duration::from_secs(10);

struct Tenant {
    campus_id: CampusId,
    token: AuthToken,
}

fn tenant_for(tenants: &[Tenant; 2], index: u32) -> &Tenant {
    &tenants[(index % 2) as usize]
}

fn nonce_for(index: u32, round: u32) -> u32 {
    index * 1_000 + round
}

async fn run_client(
    addr: SocketAddr,
    index: u32,
    campus_id: CampusId,
    token: AuthToken,
) -> (SessionId, Vec<u32>) {
    let mut socket = connect(addr).await;
    send(
        &mut socket,
        &hello(campus_id, ProtocolVersion::CURRENT.0, &token),
    )
    .await;
    let Observed::Frame(ServerFrame::Welcome(welcome)) = observe(&mut socket).await else {
        panic!("client {index} expected welcome");
    };
    assert_eq!(welcome.campus_id, campus_id);
    for round in 0..PINGS_PER_CLIENT {
        send(&mut socket, &ping(nonce_for(index, round))).await;
    }
    let mut nonces = Vec::new();
    for _ in 0..PINGS_PER_CLIENT {
        let Observed::Frame(ServerFrame::Pong(Pong { nonce })) = observe(&mut socket).await else {
            panic!("client {index} expected pong");
        };
        nonces.push(nonce);
    }
    (welcome.session_id, nonces)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_clients_keep_campus_session_and_nonce_isolation() {
    let state = fresh_state().await;
    let first = signed_up(&state, "first@example.org").await;
    let second = signed_up(&state, "second@example.org").await;
    let tenants = [
        Tenant {
            campus_id: campus_for(&state, &first.token, "first-campus")
                .await
                .campus
                .campus_id,
            token: first.token,
        },
        Tenant {
            campus_id: campus_for(&state, &second.token, "second-campus")
                .await
                .campus
                .campus_id,
            token: second.token,
        },
    ];
    let addr = spawn_server_with(state).await;
    let handles: Vec<_> = (0..CLIENTS)
        .map(|index| {
            let tenant = tenant_for(&tenants, index);
            let work = run_client(addr, index, tenant.campus_id, tenant.token.clone());
            tokio::spawn(timeout(DEADLINE, work))
        })
        .collect();
    let mut sessions = HashSet::new();
    for (index, handle) in (0..CLIENTS).zip(handles) {
        let (session_id, nonces) = handle
            .await
            .expect("client task")
            .unwrap_or_else(|_| panic!("client {index} timed out"));
        assert!(
            sessions.insert(session_id),
            "duplicate session for client {index}"
        );
        let expected: Vec<u32> = (0..PINGS_PER_CLIENT)
            .map(|round| nonce_for(index, round))
            .collect();
        assert_eq!(
            nonces, expected,
            "client {index} saw foreign or reordered pongs"
        );
    }
    assert_eq!(sessions.len(), CLIENTS as usize);
}
