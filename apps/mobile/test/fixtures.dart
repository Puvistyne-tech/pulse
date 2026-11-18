import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/frames.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/version.dart';

const _version = ProtocolVersion(field0: 1);

ClientSnapshot _snapshot(
  ConnectionPhase phase, {
  int? retryInMs,
  int attempt = 0,
  VersionMismatch? rejection,
}) => ClientSnapshot(
  protocolVersion: _version,
  connection: phase,
  retryInMs: retryInMs,
  attempt: attempt,
  rejection: rejection,
);

class SnapshotCase {
  const SnapshotCase(this.name, this.snapshot, this.expected);

  final String name;
  final ClientSnapshot snapshot;
  final String expected;
}

final snapshotCases = <SnapshotCase>[
  SnapshotCase(
    'disconnected',
    _snapshot(ConnectionPhase.disconnected),
    'Disconnected',
  ),
  SnapshotCase(
    'connecting',
    _snapshot(ConnectionPhase.connecting),
    'Connecting',
  ),
  SnapshotCase(
    'handshaking',
    _snapshot(ConnectionPhase.handshaking),
    'Handshaking',
  ),
  SnapshotCase('connected', _snapshot(ConnectionPhase.connected), 'Connected'),
  SnapshotCase(
    'backoff',
    _snapshot(ConnectionPhase.backoff, retryInMs: 1500, attempt: 2),
    'Retrying in 1500 ms',
  ),
  SnapshotCase(
    'rejected',
    _snapshot(
      ConnectionPhase.disconnected,
      rejection: const VersionMismatch(
        supported: _version,
        received: ProtocolVersion(field0: 99),
      ),
    ),
    'Version rejected: supported 1, received 99',
  ),
];
