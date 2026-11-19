use pulse_protocol::{
    AccountId, CampusId, MessageCreated, MessageId, RoomMessage, ServerFrame, ThreadChanged,
};

use crate::fanout::FanoutEnvelope;
use crate::state::AppState;
use crate::steward::mentioned_stewards;
use crate::thread::thread_summaries;

pub async fn publish_frame(
    state: &AppState,
    campus_id: CampusId,
    account_id: Option<AccountId>,
    frame: ServerFrame,
) -> bool {
    let envelope = FanoutEnvelope {
        campus_id,
        account_id,
        frame,
    };
    match state.fanout.publish(&envelope).await {
        Ok(()) => true,
        Err(error) => {
            tracing::warn!(%error, "fanout failed");
            false
        }
    }
}

pub async fn publish_summaries(
    state: &AppState,
    campus_id: CampusId,
    accounts: &[AccountId],
    root: MessageId,
) {
    if accounts.is_empty() {
        return;
    }
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(error) => {
            tracing::warn!(%error, "thread summary connection failed");
            return;
        }
    };
    for account_id in accounts {
        match thread_summaries(&mut connection, campus_id, *account_id, Some(&[root]), 1).await {
            Ok(summaries) => {
                for summary in summaries {
                    publish_frame(
                        state,
                        campus_id,
                        Some(*account_id),
                        ServerFrame::ThreadChanged(ThreadChanged { summary }),
                    )
                    .await;
                }
            }
            Err(error) => tracing::warn!(%error, "thread summary failed"),
        }
    }
}

pub async fn publish_created(
    state: &AppState,
    campus_id: CampusId,
    message: RoomMessage,
    affected: &[AccountId],
) {
    let root = message.thread_root_id.unwrap_or(message.message_id);
    publish_frame(
        state,
        campus_id,
        None,
        ServerFrame::MessageCreated(MessageCreated {
            message: message.clone(),
        }),
    )
    .await;
    publish_summaries(state, campus_id, affected, root).await;
    match mentioned_stewards(&state.db, campus_id, &message).await {
        Ok(stewards) => {
            for steward_id in stewards {
                publish_frame(
                    state,
                    campus_id,
                    Some(steward_id),
                    ServerFrame::StewardWorkItem(pulse_protocol::StewardWorkItem {
                        message: message.clone(),
                    }),
                )
                .await;
            }
        }
        Err(error) => tracing::warn!(%error, "steward work lookup failed"),
    }
}
