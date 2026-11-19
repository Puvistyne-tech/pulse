import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<String> noteMemberKey({required List<AccountId> members}) =>
    RustLib.instance.api.pulseProtocolNoteNoteMemberKey(members: members);

Future<String> noteTitle({required Note note, required AccountId viewer}) =>
    RustLib.instance.api.pulseProtocolNoteNoteTitle(note: note, viewer: viewer);

class Note {
  final CampusId campusId;
  final RoomId roomId;
  final List<NoteMember> members;

  const Note({
    required this.campusId,
    required this.roomId,
    required this.members,
  });

  @override
  int get hashCode => campusId.hashCode ^ roomId.hashCode ^ members.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Note &&
          runtimeType == other.runtimeType &&
          campusId == other.campusId &&
          roomId == other.roomId &&
          members == other.members;
}

class NoteMember {
  final AccountId accountId;
  final String displayName;

  const NoteMember({required this.accountId, required this.displayName});

  @override
  int get hashCode => accountId.hashCode ^ displayName.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NoteMember &&
          runtimeType == other.runtimeType &&
          accountId == other.accountId &&
          displayName == other.displayName;
}

class StartNoteRequest {
  final List<AccountId> accountIds;

  const StartNoteRequest({required this.accountIds});

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolNoteStartNoteRequestValidate(that: this);

  @override
  int get hashCode => accountIds.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StartNoteRequest &&
          runtimeType == other.runtimeType &&
          accountIds == other.accountIds;
}
