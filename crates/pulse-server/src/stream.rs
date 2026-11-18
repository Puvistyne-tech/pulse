use std::collections::HashMap;

use pulse_protocol::{
    MESSAGE_BODY_MAX_CHARS, MessageId, RoomId, STREAM_DELTA_MAX_CHARS, STREAMS_OPEN_MAX,
    StreamChunk, StreamId, StreamPhase, StreamRejection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamConfig {
    pub idle_timeout_ms: u64,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            idle_timeout_ms: pulse_protocol::STREAM_IDLE_TIMEOUT_MS,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenStream {
    pub room_id: RoomId,
    pub thread_root_id: Option<MessageId>,
    pub next_sequence: u32,
    pub body: String,
    pub last_chunk_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamStep {
    Opened,
    Appended,
    Closing(OpenStream),
    Cancelled(OpenStream),
}

#[derive(Debug, Clone)]
pub struct OpenStreams {
    idle_timeout_ms: u64,
    streams: HashMap<StreamId, OpenStream>,
}

impl OpenStreams {
    pub fn new(idle_timeout_ms: u64) -> Self {
        Self {
            idle_timeout_ms,
            streams: HashMap::new(),
        }
    }

    pub fn peek(&self, stream_id: StreamId) -> Option<&OpenStream> {
        self.streams.get(&stream_id)
    }

    pub fn accept(
        &mut self,
        chunk: &StreamChunk,
        now_ms: u64,
    ) -> Result<StreamStep, StreamRejection> {
        if chunk.phase != StreamPhase::Begin {
            self.check_scope(chunk)?;
        }
        match chunk.phase {
            StreamPhase::Begin => self.begin(chunk, now_ms),
            StreamPhase::Delta => self.append(chunk, now_ms),
            StreamPhase::End => {
                self.check_sequence(chunk)?;
                self.streams
                    .remove(&chunk.stream_id)
                    .map(StreamStep::Closing)
                    .ok_or(StreamRejection::UnknownStream)
            }
            StreamPhase::Abort => self
                .streams
                .remove(&chunk.stream_id)
                .map(StreamStep::Cancelled)
                .ok_or(StreamRejection::UnknownStream),
        }
    }

    fn check_scope(&mut self, chunk: &StreamChunk) -> Result<(), StreamRejection> {
        let Some(stream) = self.streams.get(&chunk.stream_id) else {
            return Ok(());
        };
        if stream.room_id == chunk.room_id && stream.thread_root_id == chunk.thread_root_id {
            Ok(())
        } else {
            self.streams.remove(&chunk.stream_id);
            Err(StreamRejection::Forbidden)
        }
    }

    pub fn discard(&mut self, stream_id: StreamId) {
        self.streams.remove(&stream_id);
    }

    pub fn take_outside(&mut self, rooms: &[RoomId]) -> Vec<(StreamId, OpenStream)> {
        let outside: Vec<StreamId> = self
            .streams
            .iter()
            .filter(|(_, stream)| !rooms.contains(&stream.room_id))
            .map(|(stream_id, _)| *stream_id)
            .collect();
        self.take(outside)
    }

    fn begin(&mut self, chunk: &StreamChunk, now_ms: u64) -> Result<StreamStep, StreamRejection> {
        if self.streams.contains_key(&chunk.stream_id) {
            return Err(StreamRejection::DuplicateStream);
        }
        if self.streams.len() >= STREAMS_OPEN_MAX {
            return Err(StreamRejection::TooManyStreams);
        }
        if chunk.sequence != 0 {
            return Err(StreamRejection::OutOfOrder);
        }
        if chunk.text.chars().count() > STREAM_DELTA_MAX_CHARS {
            return Err(StreamRejection::TooLong);
        }
        self.streams.insert(
            chunk.stream_id,
            OpenStream {
                room_id: chunk.room_id,
                thread_root_id: chunk.thread_root_id,
                next_sequence: 1,
                body: chunk.text.clone(),
                last_chunk_ms: now_ms,
            },
        );
        Ok(StreamStep::Opened)
    }

    fn check_sequence(&mut self, chunk: &StreamChunk) -> Result<(), StreamRejection> {
        let expected = self
            .streams
            .get(&chunk.stream_id)
            .map(|stream| stream.next_sequence)
            .ok_or(StreamRejection::UnknownStream)?;
        if chunk.sequence == expected {
            Ok(())
        } else {
            self.streams.remove(&chunk.stream_id);
            Err(StreamRejection::OutOfOrder)
        }
    }

    fn append(&mut self, chunk: &StreamChunk, now_ms: u64) -> Result<StreamStep, StreamRejection> {
        self.check_sequence(chunk)?;
        let Some(stream) = self.streams.get_mut(&chunk.stream_id) else {
            return Err(StreamRejection::UnknownStream);
        };
        let total = stream
            .body
            .chars()
            .count()
            .saturating_add(chunk.text.chars().count());
        if chunk.text.chars().count() > STREAM_DELTA_MAX_CHARS || total > MESSAGE_BODY_MAX_CHARS {
            self.streams.remove(&chunk.stream_id);
            return Err(StreamRejection::TooLong);
        }
        stream.body.push_str(&chunk.text);
        stream.next_sequence = stream.next_sequence.saturating_add(1);
        stream.last_chunk_ms = now_ms;
        Ok(StreamStep::Appended)
    }

    pub fn expire(&mut self, now_ms: u64) -> Vec<(StreamId, OpenStream)> {
        let idle = self.idle_timeout_ms;
        let expired: Vec<StreamId> = self
            .streams
            .iter()
            .filter(|(_, stream)| stream.last_chunk_ms.saturating_add(idle) <= now_ms)
            .map(|(stream_id, _)| *stream_id)
            .collect();
        self.take(expired)
    }

    pub fn take_all(&mut self) -> Vec<(StreamId, OpenStream)> {
        let all: Vec<StreamId> = self.streams.keys().copied().collect();
        self.take(all)
    }

    fn take(&mut self, mut stream_ids: Vec<StreamId>) -> Vec<(StreamId, OpenStream)> {
        stream_ids.sort();
        stream_ids
            .into_iter()
            .filter_map(|stream_id| {
                self.streams
                    .remove(&stream_id)
                    .map(|stream| (stream_id, stream))
            })
            .collect()
    }

    pub fn next_deadline_ms(&self) -> Option<u64> {
        self.streams
            .values()
            .map(|stream| stream.last_chunk_ms.saturating_add(self.idle_timeout_ms))
            .min()
    }
}
