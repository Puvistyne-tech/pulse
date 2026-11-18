import '../../frb_generated.dart';
import 'frames.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'version.dart';

class ClientSnapshot {
  final ProtocolVersion protocolVersion;
  final ConnectionPhase connection;
  final int? retryInMs;
  final CampusId? campusId;
  final SessionId? sessionId;
  final int attempt;
  final VersionMismatch? rejection;

  const ClientSnapshot({
    required this.protocolVersion,
    required this.connection,
    this.retryInMs,
    this.campusId,
    this.sessionId,
    required this.attempt,
    this.rejection,
  });

  @override
  int get hashCode =>
      protocolVersion.hashCode ^
      connection.hashCode ^
      retryInMs.hashCode ^
      campusId.hashCode ^
      sessionId.hashCode ^
      attempt.hashCode ^
      rejection.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ClientSnapshot &&
          runtimeType == other.runtimeType &&
          protocolVersion == other.protocolVersion &&
          connection == other.connection &&
          retryInMs == other.retryInMs &&
          campusId == other.campusId &&
          sessionId == other.sessionId &&
          attempt == other.attempt &&
          rejection == other.rejection;
}

enum ConnectionPhase {
  disconnected,
  connecting,
  handshaking,
  connected,
  backoff,
}
