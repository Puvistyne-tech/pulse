import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'rbac.dart';

class CampusInvite {
  final CampusId campusId;
  final String code;
  final BigInt expiresAtMs;

  const CampusInvite({
    required this.campusId,
    required this.code,
    required this.expiresAtMs,
  });

  @override
  int get hashCode => campusId.hashCode ^ code.hashCode ^ expiresAtMs.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CampusInvite &&
          runtimeType == other.runtimeType &&
          campusId == other.campusId &&
          code == other.code &&
          expiresAtMs == other.expiresAtMs;
}

class CampusSummary {
  final CampusId campusId;
  final String slug;
  final String displayName;
  final Role role;

  const CampusSummary({
    required this.campusId,
    required this.slug,
    required this.displayName,
    required this.role,
  });

  @override
  int get hashCode =>
      campusId.hashCode ^ slug.hashCode ^ displayName.hashCode ^ role.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CampusSummary &&
          runtimeType == other.runtimeType &&
          campusId == other.campusId &&
          slug == other.slug &&
          displayName == other.displayName &&
          role == other.role;
}

class CreateCampusRequest {
  final String slug;
  final String displayName;

  const CreateCampusRequest({required this.slug, required this.displayName});

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolCampusCreateCampusRequestValidate(that: this);

  @override
  int get hashCode => slug.hashCode ^ displayName.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CreateCampusRequest &&
          runtimeType == other.runtimeType &&
          slug == other.slug &&
          displayName == other.displayName;
}

class CreateRoomRequest {
  final String slug;

  const CreateRoomRequest({required this.slug});

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolCampusCreateRoomRequestValidate(that: this);

  @override
  int get hashCode => slug.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CreateRoomRequest &&
          runtimeType == other.runtimeType &&
          slug == other.slug;
}

class InviteCode {
  final CampusId campusId;
  final String secret;

  const InviteCode({required this.campusId, required this.secret});

  Future<String> render() =>
      RustLib.instance.api.pulseProtocolCampusInviteCodeRender(that: this);

  @override
  int get hashCode => campusId.hashCode ^ secret.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is InviteCode &&
          runtimeType == other.runtimeType &&
          campusId == other.campusId &&
          secret == other.secret;
}

class JoinCampusRequest {
  final String code;

  const JoinCampusRequest({required this.code});

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolCampusJoinCampusRequestValidate(that: this);

  @override
  int get hashCode => code.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is JoinCampusRequest &&
          runtimeType == other.runtimeType &&
          code == other.code;
}

class Room {
  final RoomId roomId;
  final CampusId campusId;
  final String slug;

  const Room({
    required this.roomId,
    required this.campusId,
    required this.slug,
  });

  @override
  int get hashCode => roomId.hashCode ^ campusId.hashCode ^ slug.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Room &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          campusId == other.campusId &&
          slug == other.slug;
}
