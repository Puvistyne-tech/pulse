import '../../frb_generated.dart';
import 'api.dart';
import 'auth.dart';
import 'campus.dart';
import 'frames.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'rbac.dart';
import 'version.dart';

enum AuthPhase { signedOut, authenticating, signedIn }

class ClientSnapshot {
  final ConnectionSnapshot network;
  final SessionSnapshot session;

  const ClientSnapshot({required this.network, required this.session});

  @override
  int get hashCode => network.hashCode ^ session.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ClientSnapshot &&
          runtimeType == other.runtimeType &&
          network == other.network &&
          session == other.session;
}

enum ConnectionPhase {
  disconnected,
  connecting,
  handshaking,
  connected,
  backoff,
}

class ConnectionSnapshot {
  final ProtocolVersion protocolVersion;
  final ConnectionPhase connection;
  final int? retryInMs;
  final CampusId? campusId;
  final SessionId? sessionId;
  final int attempt;
  final VersionMismatch? rejection;
  final HandshakeRejection? handshakeRejection;

  const ConnectionSnapshot({
    required this.protocolVersion,
    required this.connection,
    this.retryInMs,
    this.campusId,
    this.sessionId,
    required this.attempt,
    this.rejection,
    this.handshakeRejection,
  });

  @override
  int get hashCode =>
      protocolVersion.hashCode ^
      connection.hashCode ^
      retryInMs.hashCode ^
      campusId.hashCode ^
      sessionId.hashCode ^
      attempt.hashCode ^
      rejection.hashCode ^
      handshakeRejection.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConnectionSnapshot &&
          runtimeType == other.runtimeType &&
          protocolVersion == other.protocolVersion &&
          connection == other.connection &&
          retryInMs == other.retryInMs &&
          campusId == other.campusId &&
          sessionId == other.sessionId &&
          attempt == other.attempt &&
          rejection == other.rejection &&
          handshakeRejection == other.handshakeRejection;
}

class SessionSnapshot {
  final AuthPhase auth;
  final AccountProfile? account;
  final List<CampusSummary> campuses;
  final CampusId? activeCampusId;
  final List<Room> rooms;
  final bool busy;
  final ApiError? error;

  const SessionSnapshot({
    required this.auth,
    this.account,
    required this.campuses,
    this.activeCampusId,
    required this.rooms,
    required this.busy,
    this.error,
  });

  @override
  int get hashCode =>
      auth.hashCode ^
      account.hashCode ^
      campuses.hashCode ^
      activeCampusId.hashCode ^
      rooms.hashCode ^
      busy.hashCode ^
      error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SessionSnapshot &&
          runtimeType == other.runtimeType &&
          auth == other.auth &&
          account == other.account &&
          campuses == other.campuses &&
          activeCampusId == other.activeCampusId &&
          rooms == other.rooms &&
          busy == other.busy &&
          error == other.error;
}
