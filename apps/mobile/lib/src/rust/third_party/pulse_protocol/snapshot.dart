import '../../frb_generated.dart';
import 'api.dart';
import 'auth.dart';
import 'campus.dart';
import 'frames.dart';
import 'ids.dart';
import 'messaging.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'rbac.dart';
import 'version.dart';

enum AuthPhase { signedOut, authenticating, signedIn }

class ClientSnapshot {
  final BigInt sequence;
  final ConnectionSnapshot network;
  final SessionSnapshot session;
  final TimelineSnapshot timeline;

  const ClientSnapshot({
    required this.sequence,
    required this.network,
    required this.session,
    required this.timeline,
  });

  @override
  int get hashCode =>
      sequence.hashCode ^
      network.hashCode ^
      session.hashCode ^
      timeline.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ClientSnapshot &&
          runtimeType == other.runtimeType &&
          sequence == other.sequence &&
          network == other.network &&
          session == other.session &&
          timeline == other.timeline;
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

enum DeliveryState { pending, confirmed, failed }

class SessionSnapshot {
  final AuthPhase auth;
  final AccountProfile? account;
  final List<CampusSummary> campuses;
  final CampusId? activeCampusId;
  final List<Room> rooms;
  final RoomId? activeRoomId;
  final bool busy;
  final ApiError? error;

  const SessionSnapshot({
    required this.auth,
    this.account,
    required this.campuses,
    this.activeCampusId,
    required this.rooms,
    this.activeRoomId,
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
      activeRoomId.hashCode ^
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
          activeRoomId == other.activeRoomId &&
          busy == other.busy &&
          error == other.error;
}

class TimelineEntry {
  final ClientNonce clientNonce;
  final MessageId? messageId;
  final AccountId authorId;
  final String authorDisplayName;
  final bool fromSelf;
  final String body;
  final DeliveryState delivery;
  final MessageRejection? rejection;

  const TimelineEntry({
    required this.clientNonce,
    this.messageId,
    required this.authorId,
    required this.authorDisplayName,
    required this.fromSelf,
    required this.body,
    required this.delivery,
    this.rejection,
  });

  @override
  int get hashCode =>
      clientNonce.hashCode ^
      messageId.hashCode ^
      authorId.hashCode ^
      authorDisplayName.hashCode ^
      fromSelf.hashCode ^
      body.hashCode ^
      delivery.hashCode ^
      rejection.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TimelineEntry &&
          runtimeType == other.runtimeType &&
          clientNonce == other.clientNonce &&
          messageId == other.messageId &&
          authorId == other.authorId &&
          authorDisplayName == other.authorDisplayName &&
          fromSelf == other.fromSelf &&
          body == other.body &&
          delivery == other.delivery &&
          rejection == other.rejection;
}

class TimelineSnapshot {
  final RoomId? roomId;
  final List<TimelineEntry> entries;
  final bool syncing;
  final ApiError? error;

  const TimelineSnapshot({
    this.roomId,
    required this.entries,
    required this.syncing,
    this.error,
  });

  @override
  int get hashCode =>
      roomId.hashCode ^ entries.hashCode ^ syncing.hashCode ^ error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TimelineSnapshot &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          entries == other.entries &&
          syncing == other.syncing &&
          error == other.error;
}
