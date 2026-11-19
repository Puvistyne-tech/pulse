use std::net::SocketAddr;

use pulse_protocol::{
    AccountId, AuthToken, CampusId, MessageId, RoomId, ServerFrame, StewardWorkItem, StreamId,
    StreamWriter, ToolCallId, ToolDescriptor, ToolResult, WorkSynced,
};
use serde::Serialize;

use super::{
    ClientSocket, ack_work_frame, call_tool_frame, expect_frame, joined_socket, list_tools_frame,
    next_of, send, stream_chunk_frame, sync_rooms, sync_work_frame,
};

pub struct StewardClient {
    pub socket: ClientSocket,
    pub steward_id: AccountId,
}

impl StewardClient {
    pub async fn connect(addr: SocketAddr, campus_id: CampusId, token: &AuthToken) -> Self {
        let (socket, welcome) = joined_socket(addr, campus_id, token).await;
        Self {
            socket,
            steward_id: welcome.account_id,
        }
    }

    pub async fn subscribe(&mut self, rooms: &[RoomId]) {
        let cursors = rooms
            .iter()
            .map(|room_id| (*room_id, None))
            .collect::<Vec<_>>();
        send(&mut self.socket, &sync_rooms(&cursors)).await;
        for _ in rooms {
            match expect_frame(&mut self.socket).await {
                ServerFrame::RoomSynced(_) | ServerFrame::RoomUnavailable(_) => {}
                other => panic!("expected room answer, got {other:?}"),
            }
        }
    }

    pub async fn next_work_item(&mut self) -> StewardWorkItem {
        next_of(&mut self.socket, |frame| match frame {
            ServerFrame::StewardWorkItem(item) => Some(item.clone()),
            _ => None,
        })
        .await
    }

    pub async fn sync_work(&mut self) -> WorkSynced {
        send(&mut self.socket, &sync_work_frame()).await;
        next_of(&mut self.socket, |frame| match frame {
            ServerFrame::WorkSynced(synced) => Some(synced.clone()),
            _ => None,
        })
        .await
    }

    pub async fn ack(&mut self, through: MessageId) {
        send(&mut self.socket, &ack_work_frame(through)).await;
    }

    pub async fn stream_reply(
        &mut self,
        stream_id: StreamId,
        room_id: RoomId,
        thread_root_id: Option<MessageId>,
        deltas: &[&str],
    ) {
        let mut writer = StreamWriter::new(stream_id, room_id, thread_root_id);
        send(&mut self.socket, &stream_chunk_frame(writer.begin())).await;
        for delta in deltas {
            send(&mut self.socket, &stream_chunk_frame(writer.delta(delta))).await;
        }
        send(&mut self.socket, &stream_chunk_frame(writer.end())).await;
    }

    pub async fn call_tool(
        &mut self,
        call_id: ToolCallId,
        name: &str,
        arguments: &impl Serialize,
    ) -> ToolResult {
        let text = serde_json::to_string(arguments).expect("tool arguments");
        self.call_tool_raw(call_id, name, &text).await
    }

    pub async fn call_tool_raw(
        &mut self,
        call_id: ToolCallId,
        name: &str,
        arguments: &str,
    ) -> ToolResult {
        send(&mut self.socket, &call_tool_frame(call_id, name, arguments)).await;
        next_of(&mut self.socket, |frame| match frame {
            ServerFrame::ToolResult(result) if result.call_id == call_id => Some(result.clone()),
            _ => None,
        })
        .await
    }

    pub async fn list_tools(&mut self) -> Vec<ToolDescriptor> {
        send(&mut self.socket, &list_tools_frame()).await;
        next_of(&mut self.socket, |frame| match frame {
            ServerFrame::ToolsListed(listed) => Some(listed.tools.clone()),
            _ => None,
        })
        .await
    }
}
