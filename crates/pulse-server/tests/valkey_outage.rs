mod common;

use common::{
    Observed, SCENARIO_DEADLINE, ValkeyProxy, campus_with_owner, created_messages, fresh_state,
    joined_socket, observe_until, posted, proxied_state, sibling_state, spawn_server_with,
    subscribed_socket, synced_page,
};
use pulse_protocol::{ClientNonce, close_code};
use tokio::time::timeout;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn valkey_stream_end_closes_sockets_and_delta_sync_recovers_the_outage() {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let proxy = ValkeyProxy::start().await;
        let proxied_addr = spawn_server_with(proxied_state(&state, &proxy)).await;
        let direct_addr = spawn_server_with(sibling_state(&state)).await;
        let room_id = campus.rooms[0];
        let mut watcher =
            subscribed_socket(proxied_addr, campus.id, &campus.owner.token, &[room_id]).await;
        let (mut sender, _) = joined_socket(direct_addr, campus.id, &campus.owner.token).await;
        let seen = posted(&mut sender, room_id, ClientNonce::generate(), "before").await;
        assert_eq!(created_messages(&mut watcher, 1).await, vec![seen.clone()]);
        proxy.cut();
        assert_eq!(
            observe_until(&mut watcher, |observed| matches!(
                observed,
                Observed::Close(_)
            ))
            .await,
            Observed::Close(close_code::TRY_AGAIN_LATER)
        );
        let mut missed = Vec::new();
        for index in 0..3 {
            let body = format!("outage {index}");
            missed.push(posted(&mut sender, room_id, ClientNonce::generate(), &body).await);
        }
        let (mut direct, _) = joined_socket(direct_addr, campus.id, &campus.owner.token).await;
        let page = synced_page(&mut direct, room_id, Some(seen.message_id)).await;
        assert_eq!(page.messages, missed);
        assert!(!page.has_more);
        let (mut restored, _) = joined_socket(proxied_addr, campus.id, &campus.owner.token).await;
        let page = synced_page(&mut restored, room_id, Some(seen.message_id)).await;
        assert_eq!(page.messages, missed);
        let live = posted(&mut sender, room_id, ClientNonce::generate(), "after").await;
        assert_eq!(created_messages(&mut restored, 1).await, vec![live]);
    })
    .await
    .expect("scenario deadline");
}
