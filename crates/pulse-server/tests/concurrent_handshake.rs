mod common;

use std::collections::HashSet;
use std::net::SocketAddr;

use common::{
    CAMPUS_A, CAMPUS_B, Observed, campus, connect, hello, observe, ping, send, spawn_server,
};
use pulse_protocol::{Pong, ServerFrame, SessionId};
use rstest::rstest;

const PINGS_PER_CLIENT: u32 = 4;

fn campus_for(index: u32) -> &'static str {
    if index.is_multiple_of(2) {
        CAMPUS_A
    } else {
        CAMPUS_B
    }
}

fn nonce_for(index: u32, round: u32) -> u32 {
    index * 1_000 + round
}

async fn run_client(addr: SocketAddr, index: u32) -> (SessionId, Vec<u32>) {
    let wanted = campus_for(index);
    let mut socket = connect(addr).await;
    send(&mut socket, &hello(wanted, 1)).await;
    let Observed::Frame(ServerFrame::Welcome(welcome)) = observe(&mut socket).await else {
        panic!("client {index} expected welcome");
    };
    assert_eq!(welcome.campus_id, campus(wanted));
    for round in 0..PINGS_PER_CLIENT {
        send(&mut socket, &ping(nonce_for(index, round))).await;
        tokio::task::yield_now().await;
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

#[rstest]
#[case::sixteen_clients_two_campuses(16)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_clients_keep_campus_session_and_nonce_isolation(#[case] clients: u32) {
    let addr = spawn_server().await;
    let handles: Vec<_> = (0..clients)
        .map(|index| tokio::spawn(run_client(addr, index)))
        .collect();
    let mut sessions = HashSet::new();
    for (index, handle) in (0..clients).zip(handles) {
        let (session_id, nonces) = handle.await.expect("client task");
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
    assert_eq!(sessions.len(), clients as usize);
}
