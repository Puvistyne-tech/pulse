import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

class AccountId {
  final UuidValue field0;

  const AccountId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsAccountIdAsUuid(that: this);

  static Future<AccountId> generate() =>
      RustLib.instance.api.pulseProtocolIdsAccountIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AccountId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class AttachmentId {
  final UuidValue field0;

  const AttachmentId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsAttachmentIdAsUuid(that: this);

  static Future<AttachmentId> generate() =>
      RustLib.instance.api.pulseProtocolIdsAttachmentIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AttachmentId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class CampusId {
  final UuidValue field0;

  const CampusId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsCampusIdAsUuid(that: this);

  static Future<CampusId> generate() =>
      RustLib.instance.api.pulseProtocolIdsCampusIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CampusId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class ClientNonce {
  final UuidValue field0;

  const ClientNonce({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsClientNonceAsUuid(that: this);

  static Future<ClientNonce> generate() =>
      RustLib.instance.api.pulseProtocolIdsClientNonceGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ClientNonce &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class DraftId {
  final UuidValue field0;

  const DraftId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsDraftIdAsUuid(that: this);

  static Future<DraftId> generate() =>
      RustLib.instance.api.pulseProtocolIdsDraftIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DraftId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class LoungeId {
  final UuidValue field0;

  const LoungeId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsLoungeIdAsUuid(that: this);

  static Future<LoungeId> generate() =>
      RustLib.instance.api.pulseProtocolIdsLoungeIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoungeId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class MessageId {
  final UuidValue field0;

  const MessageId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsMessageIdAsUuid(that: this);

  static Future<MessageId> generate() =>
      RustLib.instance.api.pulseProtocolIdsMessageIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class RoomId {
  final UuidValue field0;

  const RoomId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsRoomIdAsUuid(that: this);

  static Future<RoomId> generate() =>
      RustLib.instance.api.pulseProtocolIdsRoomIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RoomId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class SessionId {
  final UuidValue field0;

  const SessionId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsSessionIdAsUuid(that: this);

  static Future<SessionId> generate() =>
      RustLib.instance.api.pulseProtocolIdsSessionIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SessionId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class ShelfItemId {
  final UuidValue field0;

  const ShelfItemId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsShelfItemIdAsUuid(that: this);

  static Future<ShelfItemId> generate() =>
      RustLib.instance.api.pulseProtocolIdsShelfItemIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ShelfItemId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class StreamId {
  final UuidValue field0;

  const StreamId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsStreamIdAsUuid(that: this);

  Future<ClientNonce> clientNonce() =>
      RustLib.instance.api.pulseProtocolIdsStreamIdClientNonce(that: this);

  static Future<StreamId> generate() =>
      RustLib.instance.api.pulseProtocolIdsStreamIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StreamId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class ToolCallId {
  final UuidValue field0;

  const ToolCallId({required this.field0});

  Future<UuidValue> asUuid() =>
      RustLib.instance.api.pulseProtocolIdsToolCallIdAsUuid(that: this);

  static Future<ToolCallId> generate() =>
      RustLib.instance.api.pulseProtocolIdsToolCallIdGenerate();

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ToolCallId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}
