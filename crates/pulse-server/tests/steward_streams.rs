mod common;

use std::time::Duration;

use common::steward::StewardClient;
use common::{
    ClientSocket, Observed, SCENARIO_DEADLINE, Stage, agent_chunks_of, authored_count,
    drop_steward_room, expect_close, expect_frame, frames_until_pong, issue_steward, manage_on,
    messages_with_nonce, next_of, observe_until, owner_socket, posted, send, spawn_server_with,
    steward_request, stream_chunk_frame, subscribed_socket,
};
use pulse_protocol::{
    AccountId, AgentStreamChunk, ClientNonce, MESSAGE_BODY_MAX_CHARS, MessageId, RoomId,
    STREAM_DELTA_MAX_CHARS, STREAMS_OPEN_MAX, ServerFrame, StewardAction, StewardProfile,
    StreamAbortReason, StreamChunk, StreamId, StreamPhase, StreamRejection, StreamWriter,
    close_code,
};
use pulse_server::stream::StreamConfig;
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::timeout;

struct Rig {
    stage: Stage,
    profile: StewardProfile,
    steward: StewardClient,
    watcher: ClientSocket,
    admin: ClientSocket,
    rooms: Vec<RoomId>,
}

impl Rig {
    async fn open() -> Self {
        Self::with_stage(Stage::open().await, None).await
    }

    async fn with_stage(stage: Stage, streams: Option<StreamConfig>) -> Self {
        let rooms = stage.campus.rooms[..2].to_vec();
        let (profile, token) = issue_steward(
            &stage.state,
            stage.campus.id,
            stage.campus.owner.account.account_id,
            "Scribe",
            &rooms,
        )
        .await;
        let addr = match streams {
            Some(config) => spawn_server_with(stage.state.clone().with_streams(config)).await,
            None => stage.addr,
        };
        let steward = StewardClient::connect(addr, stage.campus.id, &token).await;
        let watcher = subscribed_socket(addr, stage.campus.id, &stage.ada.token, &rooms).await;
        let admin = owner_socket(addr, &stage.campus).await;
        Self {
            stage,
            profile,
            steward,
            watcher,
            admin,
            rooms,
        }
    }

    fn room(&self) -> RoomId {
        self.rooms[0]
    }

    fn writer(&self, stream_id: StreamId) -> StreamWriter {
        StreamWriter::new(stream_id, self.room(), None)
    }

    async fn push(&mut self, chunk: StreamChunk) {
        send(&mut self.steward.socket, &stream_chunk_frame(chunk)).await;
    }

    async fn rejection(&mut self, stream_id: StreamId) -> StreamRejection {
        next_of(&mut self.steward.socket, |frame| match frame {
            ServerFrame::StreamRejected(rejected) if rejected.stream_id == stream_id => {
                Some(rejected.reason)
            }
            _ => None,
        })
        .await
    }

    async fn through_terminal(&mut self, stream_id: StreamId) -> Vec<ServerFrame> {
        through_terminal(&mut self.watcher, stream_id).await
    }

    async fn await_sequence(&mut self, stream_id: StreamId, sequence: u32) {
        await_sequence(&mut self.watcher, &[stream_id], sequence).await;
    }

    async fn authored(&self) -> i64 {
        authored_count(
            &self.stage.state,
            self.stage.campus.id,
            self.room(),
            self.profile.steward_id,
        )
        .await
    }
}

async fn await_sequence(socket: &mut ClientSocket, stream_ids: &[StreamId], sequence: u32) {
    let mut pending = stream_ids.to_vec();
    while !pending.is_empty() {
        let chunk = next_of(socket, |frame| match frame {
            ServerFrame::AgentStreamChunk(chunk) => Some(chunk.clone()),
            _ => None,
        })
        .await;
        if chunk.sequence >= sequence {
            pending.retain(|stream_id| *stream_id != chunk.stream_id);
        }
    }
}

async fn through_all_terminals(
    socket: &mut ClientSocket,
    stream_ids: &[StreamId],
) -> Vec<ServerFrame> {
    let mut seen = Vec::new();
    let mut pending = stream_ids.to_vec();
    while !pending.is_empty() {
        let frame = expect_frame(socket).await;
        if let ServerFrame::AgentStreamChunk(chunk) = &frame
            && matches!(chunk.phase, StreamPhase::End | StreamPhase::Abort)
        {
            pending.retain(|stream_id| *stream_id != chunk.stream_id);
        }
        seen.push(frame);
    }
    seen
}

async fn through_terminal(socket: &mut ClientSocket, stream_id: StreamId) -> Vec<ServerFrame> {
    let mut seen = Vec::new();
    loop {
        let frame = expect_frame(socket).await;
        let terminal = matches!(
            &frame,
            ServerFrame::AgentStreamChunk(chunk)
                if chunk.stream_id == stream_id
                    && matches!(chunk.phase, StreamPhase::End | StreamPhase::Abort)
        );
        seen.push(frame);
        if terminal {
            return seen;
        }
    }
}

fn own_chunks(frames: &[ServerFrame], stream_id: StreamId) -> Vec<AgentStreamChunk> {
    agent_chunks_of(frames)
        .into_iter()
        .filter(|chunk| chunk.stream_id == stream_id)
        .collect()
}

fn aborted_with(frames: &[ServerFrame], stream_id: StreamId) -> Option<StreamAbortReason> {
    own_chunks(frames, stream_id)
        .into_iter()
        .find(|chunk| chunk.phase == StreamPhase::Abort)
        .and_then(|chunk| chunk.abort_reason)
}

fn created_nonces(frames: &[ServerFrame]) -> Vec<ClientNonce> {
    frames
        .iter()
        .filter_map(|frame| match frame {
            ServerFrame::MessageCreated(created) => Some(created.message.client_nonce),
            _ => None,
        })
        .collect()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_finished_stream_becomes_one_message_in_order() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        let room = rig.room();
        rig.steward
            .stream_reply(stream_id, room, None, &["he", "llo"])
            .await;
        let frames = rig.through_terminal(stream_id).await;
        let chunks = own_chunks(&frames, stream_id);
        let phases = chunks
            .iter()
            .map(|chunk| (chunk.phase, chunk.sequence, chunk.text.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            phases,
            vec![
                (StreamPhase::Begin, 0, ""),
                (StreamPhase::Delta, 1, "he"),
                (StreamPhase::Delta, 2, "llo"),
                (StreamPhase::End, 3, ""),
            ]
        );
        assert!(chunks.iter().all(|chunk| chunk.steward_id == rig.profile.steward_id
            && chunk.steward_display_name == "Scribe"
            && chunk.campus_id == rig.stage.campus.id));
        let created_at = frames
            .iter()
            .position(|frame| matches!(frame, ServerFrame::MessageCreated(_)))
            .expect("message created");
        let ServerFrame::MessageCreated(created) = &frames[created_at] else {
            unreachable!()
        };
        let end_at = frames
            .iter()
            .position(|frame| {
                matches!(frame, ServerFrame::AgentStreamChunk(chunk) if chunk.phase == StreamPhase::End)
            })
            .expect("end frame");
        assert!(created_at < end_at);
        assert_eq!(created.message.body, "hello");
        assert_eq!(created.message.author_id, rig.profile.steward_id);
        assert_eq!(created.message.client_nonce, stream_id.client_nonce());
        assert_eq!(chunks[3].message_id, Some(created.message.message_id));
        assert_eq!(rig.authored().await, 1);
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Step {
    Begin,
    Delta(u32),
    End(u32),
    BeginAt(u32),
}

#[rstest]
#[case::gap(&[Step::Begin, Step::Delta(2)], StreamRejection::OutOfOrder, true)]
#[case::replayed_delta(&[Step::Begin, Step::Delta(1), Step::Delta(1)], StreamRejection::OutOfOrder, true)]
#[case::end_with_a_gap(&[Step::Begin, Step::End(5)], StreamRejection::OutOfOrder, true)]
#[case::begin_off_zero(&[Step::BeginAt(1)], StreamRejection::OutOfOrder, false)]
#[case::delta_without_begin(&[Step::Delta(1)], StreamRejection::UnknownStream, false)]
#[case::end_without_begin(&[Step::End(1)], StreamRejection::UnknownStream, false)]
#[case::duplicate_begin(&[Step::Begin, Step::Begin], StreamRejection::DuplicateStream, false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn broken_sequences_are_rejected_and_never_post(
    #[case] script: &[Step],
    #[case] reason: StreamRejection,
    #[case] aborts: bool,
) {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        for step in script {
            let mut chunk = match step {
                Step::Begin => rig.writer(stream_id).begin(),
                Step::BeginAt(sequence) => StreamChunk {
                    sequence: *sequence,
                    ..rig.writer(stream_id).begin()
                },
                Step::Delta(sequence) => StreamChunk {
                    sequence: *sequence,
                    ..rig.writer(stream_id).delta("x")
                },
                Step::End(sequence) => StreamChunk {
                    sequence: *sequence,
                    ..rig.writer(stream_id).end()
                },
            };
            chunk.sequence = match step {
                Step::Begin => 0,
                Step::BeginAt(sequence) | Step::Delta(sequence) | Step::End(sequence) => *sequence,
            };
            rig.push(chunk).await;
        }
        assert_eq!(rig.rejection(stream_id).await, reason);
        if aborts {
            let frames = rig.through_terminal(stream_id).await;
            assert_eq!(
                aborted_with(&frames, stream_id),
                Some(StreamAbortReason::Rejected)
            );
        }
        let seen = frames_until_pong(&mut rig.watcher, 9).await;
        assert!(created_nonces(&seen).is_empty());
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_duplicate_begin_leaves_the_original_stream_running() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        rig.push(rig.writer(stream_id).begin()).await;
        assert_eq!(
            rig.rejection(stream_id).await,
            StreamRejection::DuplicateStream
        );
        rig.push(writer.delta("ok")).await;
        rig.push(writer.end()).await;
        let frames = rig.through_terminal(stream_id).await;
        assert_eq!(aborted_with(&frames, stream_id), None);
        assert_eq!(created_nonces(&frames), vec![stream_id.client_nonce()]);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_stewards_own_abort_cancels_without_a_message() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        rig.push(writer.delta("half")).await;
        rig.push(writer.abort()).await;
        let frames = rig.through_terminal(stream_id).await;
        assert_eq!(
            aborted_with(&frames, stream_id),
            Some(StreamAbortReason::Cancelled)
        );
        assert!(created_nonces(&frames).is_empty());
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_dropped_connection_aborts_every_open_stream() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let streams = [StreamId::generate(), StreamId::generate()];
        for stream_id in streams {
            let mut writer = rig.writer(stream_id);
            rig.push(writer.begin()).await;
            rig.push(writer.delta("partial")).await;
        }
        let Rig {
            steward,
            mut watcher,
            profile,
            stage,
            rooms,
            ..
        } = rig;
        drop(steward);
        for stream_id in streams {
            let frames = through_terminal(&mut watcher, stream_id).await;
            assert_eq!(
                aborted_with(&frames, stream_id),
                Some(StreamAbortReason::Disconnected)
            );
        }
        assert_eq!(
            authored_count(&stage.state, stage.campus.id, rooms[0], profile.steward_id).await,
            0
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn revoking_mid_stream_aborts_closes_and_posts_nothing() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        rig.push(writer.delta("half")).await;
        rig.await_sequence(stream_id, 1).await;
        manage_on(
            &mut rig.admin,
            steward_request(StewardAction::Revoke, rig.profile.steward_id, &[]),
        )
        .await;
        let frames = rig.through_terminal(stream_id).await;
        assert_eq!(
            aborted_with(&frames, stream_id),
            Some(StreamAbortReason::Revoked)
        );
        assert_eq!(
            expect_close(&mut rig.steward.socket).await,
            close_code::POLICY_VIOLATION
        );
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::four(4)]
#[case::eight(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ending_races_revocation_to_exactly_one_outcome(#[case] rounds: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let rooms = stage.campus.rooms[..1].to_vec();
        let owner = stage.campus.owner.account.account_id;
        let mut watcher =
            subscribed_socket(stage.addr, stage.campus.id, &stage.ada.token, &rooms).await;
        let mut racers = Vec::new();
        for round in 0..rounds {
            let (profile, token) = issue_steward(
                &stage.state,
                stage.campus.id,
                owner,
                &format!("Racer {round}"),
                &rooms,
            )
            .await;
            let mut client = StewardClient::connect(stage.addr, stage.campus.id, &token).await;
            let stream_id = StreamId::generate();
            let mut writer = StreamWriter::new(stream_id, rooms[0], None);
            send(&mut client.socket, &stream_chunk_frame(writer.begin())).await;
            send(
                &mut client.socket,
                &stream_chunk_frame(writer.delta("race")),
            )
            .await;
            racers.push((profile, client, writer, stream_id));
        }
        let started = racers.iter().map(|racer| racer.3).collect::<Vec<_>>();
        await_sequence(&mut watcher, &started, 1).await;
        let mut contenders = JoinSet::new();
        for (profile, mut client, mut writer, stream_id) in racers {
            let mut admin = owner_socket(stage.addr, &stage.campus).await;
            contenders.spawn(async move {
                let ending = async {
                    send(&mut client.socket, &stream_chunk_frame(writer.end())).await;
                };
                let revoking = async {
                    manage_on(
                        &mut admin,
                        steward_request(StewardAction::Revoke, profile.steward_id, &[]),
                    )
                    .await;
                };
                tokio::join!(ending, revoking);
                (profile.steward_id, stream_id, client)
            });
        }
        let mut settled: Vec<(AccountId, StreamId, StewardClient)> = Vec::new();
        while let Some(done) = contenders.join_next().await {
            settled.push(done.expect("race task"));
        }
        let all_ids = settled
            .iter()
            .map(|(_, stream_id, _)| *stream_id)
            .collect::<Vec<_>>();
        let seen = through_all_terminals(&mut watcher, &all_ids).await;
        for (steward_id, stream_id, _) in &settled {
            let frames = &seen;
            let posted_rows =
                authored_count(&stage.state, stage.campus.id, rooms[0], *steward_id).await;
            let aborted = aborted_with(frames, *stream_id);
            let created = created_nonces(frames).contains(&stream_id.client_nonce());
            match (posted_rows, aborted, created) {
                (1, None, true)
                | (0, Some(StreamAbortReason::Revoked | StreamAbortReason::Rejected), false) => {}
                other => panic!("both outcomes or neither: {other:?}"),
            }
            assert_eq!(
                messages_with_nonce(
                    &stage.state,
                    stage.campus.id,
                    rooms[0],
                    stream_id.client_nonce()
                )
                .await,
                posted_rows
            );
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn replaying_an_end_never_posts_twice() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        rig.push(writer.delta("once")).await;
        let end = writer.end();
        rig.push(end.clone()).await;
        let frames = rig.through_terminal(stream_id).await;
        let first = own_chunks(&frames, stream_id)
            .into_iter()
            .find(|chunk| chunk.phase == StreamPhase::End)
            .and_then(|chunk| chunk.message_id)
            .expect("message id");
        rig.push(end).await;
        assert_eq!(
            rig.rejection(stream_id).await,
            StreamRejection::UnknownStream
        );
        let mut again = rig.writer(stream_id);
        rig.push(again.begin()).await;
        rig.push(again.delta("once")).await;
        rig.push(again.end()).await;
        let replay = rig.through_terminal(stream_id).await;
        let replayed = own_chunks(&replay, stream_id)
            .into_iter()
            .find(|chunk| chunk.phase == StreamPhase::End)
            .and_then(|chunk| chunk.message_id);
        assert_eq!(replayed, Some(first));
        assert_eq!(rig.authored().await, 1);
        assert_eq!(
            messages_with_nonce(
                &rig.stage.state,
                rig.stage.campus.id,
                rig.room(),
                stream_id.client_nonce()
            )
            .await,
            1
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn interleaved_streams_of_two_stewards_stay_ordered_per_stream() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let rooms = stage.campus.rooms[..1].to_vec();
        let owner = stage.campus.owner.account.account_id;
        let mut watchers = Vec::new();
        for session in [&stage.ada, &stage.grace, &stage.linus] {
            watchers
                .push(subscribed_socket(stage.addr, stage.campus.id, &session.token, &rooms).await);
        }
        let mut writers = JoinSet::new();
        let mut stream_ids = Vec::new();
        for name in ["Scribe", "Herald"] {
            let (_, token) =
                issue_steward(&stage.state, stage.campus.id, owner, name, &rooms).await;
            let mut client = StewardClient::connect(stage.addr, stage.campus.id, &token).await;
            let stream_id = StreamId::generate();
            stream_ids.push(stream_id);
            let room = rooms[0];
            writers.spawn(async move {
                let deltas = ["a", "b", "c", "d", "e", "f"];
                client.stream_reply(stream_id, room, None, &deltas).await;
                client
            });
        }
        let mut clients = Vec::new();
        while let Some(done) = writers.join_next().await {
            clients.push(done.expect("writer task"));
        }
        for watcher in &mut watchers {
            let mut ended = 0;
            let mut seen: std::collections::BTreeMap<StreamId, Vec<u32>> = Default::default();
            while ended < stream_ids.len() {
                let chunk = next_of(watcher, |frame| match frame {
                    ServerFrame::AgentStreamChunk(chunk) => Some(chunk.clone()),
                    _ => None,
                })
                .await;
                if chunk.phase == StreamPhase::End {
                    ended += 1;
                }
                seen.entry(chunk.stream_id)
                    .or_default()
                    .push(chunk.sequence);
            }
            for stream_id in &stream_ids {
                assert_eq!(seen[stream_id], (0..=7).collect::<Vec<u32>>());
            }
        }
        for stream_id in &stream_ids {
            assert_eq!(
                messages_with_nonce(
                    &stage.state,
                    stage.campus.id,
                    rooms[0],
                    stream_id.client_nonce()
                )
                .await,
                1
            );
        }
        drop(clients);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn silent_streams_time_out_on_the_server_clock() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut rig = Rig::with_stage(
            stage,
            Some(StreamConfig {
                idle_timeout_ms: 300,
            }),
        )
        .await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        rig.push(writer.delta("quiet")).await;
        let frames = rig.through_terminal(stream_id).await;
        assert_eq!(
            aborted_with(&frames, stream_id),
            Some(StreamAbortReason::TimedOut)
        );
        rig.push(writer.delta("late")).await;
        assert_eq!(
            rig.rejection(stream_id).await,
            StreamRejection::UnknownStream
        );
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn chunks_keep_a_stream_alive_past_the_idle_window() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut rig = Rig::with_stage(
            stage,
            Some(StreamConfig {
                idle_timeout_ms: 600,
            }),
        )
        .await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        for _ in 0..4 {
            tokio::time::sleep(Duration::from_millis(300)).await;
            rig.push(writer.delta("tick")).await;
        }
        rig.push(writer.end()).await;
        let frames = rig.through_terminal(stream_id).await;
        assert_eq!(aborted_with(&frames, stream_id), None);
        assert_eq!(created_nonces(&frames), vec![stream_id.client_nonce()]);
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Overflow {
    LongBegin,
    LongDelta,
    Accumulated,
}

#[rstest]
#[case::long_begin(Overflow::LongBegin, false)]
#[case::long_delta(Overflow::LongDelta, true)]
#[case::accumulated(Overflow::Accumulated, true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn oversized_text_is_rejected(#[case] overflow: Overflow, #[case] aborts: bool) {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        let too_long = "x".repeat(STREAM_DELTA_MAX_CHARS + 1);
        match overflow {
            Overflow::LongBegin => {
                rig.push(StreamChunk {
                    text: too_long,
                    ..writer.begin()
                })
                .await;
            }
            Overflow::LongDelta => {
                rig.push(writer.begin()).await;
                rig.push(writer.delta(&too_long)).await;
            }
            Overflow::Accumulated => {
                rig.push(writer.begin()).await;
                let piece = "y".repeat(STREAM_DELTA_MAX_CHARS);
                for _ in 0..=(MESSAGE_BODY_MAX_CHARS / STREAM_DELTA_MAX_CHARS) {
                    rig.push(writer.delta(&piece)).await;
                }
            }
        }
        assert_eq!(rig.rejection(stream_id).await, StreamRejection::TooLong);
        if aborts {
            let frames = rig.through_terminal(stream_id).await;
            assert_eq!(
                aborted_with(&frames, stream_id),
                Some(StreamAbortReason::Rejected)
            );
        }
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn only_a_few_streams_may_be_open_at_once() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let open = (0..STREAMS_OPEN_MAX)
            .map(|_| StreamId::generate())
            .collect::<Vec<_>>();
        for stream_id in &open {
            rig.push(StreamChunk {
                text: "hi".to_owned(),
                ..rig.writer(*stream_id).begin()
            })
            .await;
        }
        let extra = StreamId::generate();
        rig.push(rig.writer(extra).begin()).await;
        assert_eq!(rig.rejection(extra).await, StreamRejection::TooManyStreams);
        for stream_id in &open {
            let mut writer = rig.writer(*stream_id);
            writer.begin();
            rig.push(writer.end()).await;
        }
        for stream_id in &open {
            let frames = rig.through_terminal(*stream_id).await;
            assert_eq!(aborted_with(&frames, *stream_id), None);
        }
        assert_eq!(
            usize::try_from(rig.authored().await).expect("count"),
            STREAMS_OPEN_MAX
        );
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Drift {
    Room,
    Thread,
}

#[rstest]
#[case::room(Drift::Room)]
#[case::thread(Drift::Thread)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn chunks_cannot_drift_from_where_the_stream_began(#[case] drift: Drift) {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let root = {
            let mut poster = subscribed_socket(
                rig.stage.addr,
                rig.stage.campus.id,
                &rig.stage.grace.token,
                &[rig.room()],
            )
            .await;
            posted(&mut poster, rig.room(), ClientNonce::generate(), "root").await
        };
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        let mut wandering = writer.delta("x");
        match drift {
            Drift::Room => wandering.room_id = rig.rooms[1],
            Drift::Thread => wandering.thread_root_id = Some(root.message_id),
        }
        rig.push(wandering).await;
        assert_eq!(rig.rejection(stream_id).await, StreamRejection::Forbidden);
        let frames = rig.through_terminal(stream_id).await;
        let aborts = own_chunks(&frames, stream_id)
            .into_iter()
            .filter(|chunk| chunk.phase == StreamPhase::Abort)
            .collect::<Vec<_>>();
        assert_eq!(aborts.len(), 1);
        assert_eq!(aborts[0].abort_reason, Some(StreamAbortReason::Rejected));
        assert_eq!(aborts[0].room_id, rig.room());
        assert_eq!(aborts[0].thread_root_id, None);
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reply_streams_land_in_their_thread() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let mut poster = subscribed_socket(
            rig.stage.addr,
            rig.stage.campus.id,
            &rig.stage.grace.token,
            &[rig.room()],
        )
        .await;
        let root = posted(&mut poster, rig.room(), ClientNonce::generate(), "root").await;
        let stream_id = StreamId::generate();
        let room = rig.room();
        rig.steward
            .stream_reply(stream_id, room, Some(root.message_id), &["re"])
            .await;
        let frames = rig.through_terminal(stream_id).await;
        let chunks = own_chunks(&frames, stream_id);
        assert!(
            chunks
                .iter()
                .all(|chunk| chunk.thread_root_id == Some(root.message_id))
        );
        let created = frames
            .iter()
            .find_map(|frame| match frame {
                ServerFrame::MessageCreated(created)
                    if created.message.client_nonce == stream_id.client_nonce() =>
                {
                    Some(created.message.clone())
                }
                _ => None,
            })
            .expect("reply created");
        assert_eq!(created.thread_root_id, Some(root.message_id));
        assert_eq!(created.body, "re");
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_unknown_thread_admission_failure_aborts_open_streams_as_rejected() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let (open, doomed) = (StreamId::generate(), StreamId::generate());
        let mut writer = rig.writer(open);
        rig.push(writer.begin()).await;
        rig.push(writer.delta("half")).await;
        rig.await_sequence(open, 1).await;
        let mut stray = StreamWriter::new(doomed, rig.room(), Some(MessageId::generate()));
        rig.push(stray.begin()).await;
        assert_eq!(rig.rejection(doomed).await, StreamRejection::UnknownThread);
        let frames = rig.through_terminal(open).await;
        assert_eq!(
            aborted_with(&frames, open),
            Some(StreamAbortReason::Rejected)
        );
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::unknown_root(false)]
#[case::reply_as_root(true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reply_streams_need_a_top_level_root(#[case] nested: bool) {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let mut poster = subscribed_socket(
            rig.stage.addr,
            rig.stage.campus.id,
            &rig.stage.grace.token,
            &[rig.room()],
        )
        .await;
        let root = posted(&mut poster, rig.room(), ClientNonce::generate(), "root").await;
        let target = if nested {
            common::replied(
                &mut poster,
                rig.room(),
                root.message_id,
                ClientNonce::generate(),
                "r",
            )
            .await
            .message_id
        } else {
            MessageId::generate()
        };
        let stream_id = StreamId::generate();
        let mut writer = StreamWriter::new(stream_id, rig.room(), Some(target));
        rig.push(writer.begin()).await;
        assert_eq!(
            rig.rejection(stream_id).await,
            StreamRejection::UnknownThread
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn pruning_the_assigned_rooms_aborts_streams_outside_them() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let stream_id = StreamId::generate();
        let mut writer = rig.writer(stream_id);
        rig.push(writer.begin()).await;
        rig.push(writer.delta("half")).await;
        rig.await_sequence(stream_id, 1).await;
        let kept = rig.rooms[1];
        manage_on(
            &mut rig.admin,
            steward_request(StewardAction::AssignRooms, rig.profile.steward_id, &[kept]),
        )
        .await;
        let frames = rig.through_terminal(stream_id).await;
        assert_eq!(
            aborted_with(&frames, stream_id),
            Some(StreamAbortReason::Revoked)
        );
        rig.push(writer.delta("late")).await;
        assert_eq!(
            rig.rejection(stream_id).await,
            StreamRejection::UnknownStream
        );
        let seen = frames_until_pong(&mut rig.steward.socket, 4).await;
        assert!(
            seen.iter()
                .all(|frame| !matches!(frame, ServerFrame::HandshakeRejected(_)))
        );
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn losing_the_room_between_chunks_revokes_every_stream() {
    timeout(SCENARIO_DEADLINE, async {
        let mut rig = Rig::open().await;
        let (first, second) = (StreamId::generate(), StreamId::generate());
        let mut writer_first = rig.writer(first);
        let mut writer_second = rig.writer(second);
        rig.push(writer_first.begin()).await;
        rig.push(writer_second.begin()).await;
        await_sequence(&mut rig.watcher, &[first, second], 0).await;
        drop_steward_room(
            &rig.stage.state,
            rig.stage.campus.id,
            rig.profile.steward_id,
            rig.room(),
        )
        .await;
        rig.push(writer_first.delta("x")).await;
        assert_eq!(rig.rejection(first).await, StreamRejection::Forbidden);
        for stream_id in [first, second] {
            let frames = through_terminal(&mut rig.watcher, stream_id).await;
            assert_eq!(
                aborted_with(&frames, stream_id),
                Some(StreamAbortReason::Revoked)
            );
        }
        assert_eq!(rig.authored().await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::immediately(0)]
#[case::after_a_beat(15)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn rotating_around_the_handshake_always_ends_in_a_policy_close(#[case] delay_ms: u64) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let (profile, token) = issue_steward(
            &stage.state,
            stage.campus.id,
            owner,
            "Scribe",
            &stage.campus.rooms[..1],
        )
        .await;
        let mut attempts = JoinSet::new();
        for _ in 0..6 {
            let campus = &stage.campus;
            let campus_id = campus.id;
            let addr = stage.addr;
            let mut admin = owner_socket(addr, campus).await;
            let token = token.clone();
            let steward_id = profile.steward_id;
            attempts.spawn(async move {
                let mut socket = common::connect(addr).await;
                common::send(
                    &mut socket,
                    &common::hello(
                        campus_id,
                        pulse_protocol::ProtocolVersion::CURRENT.0,
                        &token,
                    ),
                )
                .await;
                let rotated = async {
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    manage_on(
                        &mut admin,
                        steward_request(StewardAction::RotateToken, steward_id, &[]),
                    )
                    .await;
                };
                let welcomed = async {
                    observe_until(&mut socket, |observed| {
                        matches!(
                            observed,
                            Observed::Frame(ServerFrame::Welcome(_)) | Observed::Close(_)
                        )
                    })
                    .await
                };
                let (_, welcome) = tokio::join!(rotated, welcomed);
                (socket, welcome)
            });
        }
        while let Some(done) = attempts.join_next().await {
            let (mut socket, welcome) = done.expect("attempt task");
            match welcome {
                Observed::Close(code) => assert_eq!(code, close_code::POLICY_VIOLATION),
                Observed::Frame(ServerFrame::Welcome(_)) => {
                    assert_eq!(
                        expect_close(&mut socket).await,
                        close_code::POLICY_VIOLATION
                    );
                }
                Observed::Frame(other) => {
                    assert!(
                        matches!(other, ServerFrame::HandshakeRejected(_)),
                        "unexpected {other:?}"
                    );
                }
            }
        }
    })
    .await
    .expect("scenario deadline");
}
