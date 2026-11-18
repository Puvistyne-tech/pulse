use std::collections::BTreeMap;

use pulse_protocol::{
    AccountId, AgentStreamChunk, ClientNonce, DeliveryState, MessageId, RoomId,
    STREAM_STALE_AFTER_MS, StreamId, StreamPhase, StreamState, TimelineEntry, body_segments,
    mention_token,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LiveStream {
    stream_id: StreamId,
    room_id: RoomId,
    thread_root_id: Option<MessageId>,
    steward_id: AccountId,
    steward_display_name: String,
    next_sequence: u32,
    body: String,
    last_chunk_ms: u64,
    finalizing: bool,
    arrival: u64,
}

impl LiveStream {
    fn stale_at_ms(&self) -> u64 {
        self.last_chunk_ms.saturating_add(STREAM_STALE_AFTER_MS)
    }

    fn is_live(&self, clock_ms: u64) -> bool {
        self.stale_at_ms() > clock_ms
    }

    pub(crate) fn entry(&self) -> TimelineEntry {
        TimelineEntry {
            client_nonce: self.stream_id.client_nonce(),
            message_id: None,
            author_id: self.steward_id,
            author_display_name: self.steward_display_name.clone(),
            author_mention_token: mention_token(self.steward_id),
            from_self: false,
            body: self.body.clone(),
            segments: body_segments(&self.body, &[]),
            thread_root_id: self.thread_root_id,
            reactions: Vec::new(),
            thread: None,
            delivery: DeliveryState::Confirmed,
            rejection: None,
            attachments: Vec::new(),
            pending_drafts: Vec::new(),
            pinned: false,
            shelved: false,
            author_is_steward: true,
            stream: Some(if self.finalizing {
                StreamState::Finalizing
            } else {
                StreamState::Streaming
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct StreamTracker {
    streams: BTreeMap<StreamId, LiveStream>,
    arrivals: u64,
}

impl StreamTracker {
    pub(crate) fn apply(&mut self, chunk: AgentStreamChunk, clock_ms: u64) {
        match chunk.phase {
            StreamPhase::Begin => self.begin(chunk, clock_ms),
            StreamPhase::Delta => self.advance(chunk, clock_ms, false),
            StreamPhase::End => self.advance(chunk, clock_ms, true),
            StreamPhase::Abort => {
                self.streams.remove(&chunk.stream_id);
            }
        }
    }

    fn begin(&mut self, chunk: AgentStreamChunk, clock_ms: u64) {
        if chunk.sequence != 0 || self.streams.contains_key(&chunk.stream_id) {
            return;
        }
        self.arrivals += 1;
        self.streams.insert(
            chunk.stream_id,
            LiveStream {
                stream_id: chunk.stream_id,
                room_id: chunk.room_id,
                thread_root_id: chunk.thread_root_id,
                steward_id: chunk.steward_id,
                steward_display_name: chunk.steward_display_name,
                next_sequence: 1,
                body: chunk.text,
                last_chunk_ms: clock_ms,
                finalizing: false,
                arrival: self.arrivals,
            },
        );
    }

    fn advance(&mut self, chunk: AgentStreamChunk, clock_ms: u64, finalizing: bool) {
        let Some(stream) = self.streams.get_mut(&chunk.stream_id) else {
            return;
        };
        if chunk.sequence != stream.next_sequence {
            self.streams.remove(&chunk.stream_id);
            return;
        }
        stream.next_sequence = stream.next_sequence.saturating_add(1);
        stream.last_chunk_ms = clock_ms;
        stream.body.push_str(&chunk.text);
        stream.finalizing = finalizing;
    }

    pub(crate) fn confirm(&mut self, client_nonce: ClientNonce) {
        self.streams
            .retain(|stream_id, _| stream_id.client_nonce() != client_nonce);
    }

    pub(crate) fn expire(&mut self, clock_ms: u64) {
        self.streams.retain(|_, stream| stream.is_live(clock_ms));
    }

    pub(crate) fn clear(&mut self) {
        self.streams.clear();
    }

    pub(crate) fn next_deadline_ms(&self) -> Option<u64> {
        self.streams.values().map(LiveStream::stale_at_ms).min()
    }

    pub(crate) fn entries_for(
        &self,
        room_id: RoomId,
        thread_root_id: Option<MessageId>,
        clock_ms: u64,
    ) -> Vec<TimelineEntry> {
        let mut live = self
            .streams
            .values()
            .filter(|stream| {
                stream.room_id == room_id
                    && stream.thread_root_id == thread_root_id
                    && stream.is_live(clock_ms)
            })
            .collect::<Vec<_>>();
        live.sort_by_key(|stream| stream.arrival);
        live.into_iter().map(LiveStream::entry).collect()
    }
}
