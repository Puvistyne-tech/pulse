import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

class MentionRef {
  final AccountId accountId;
  final String displayName;

  const MentionRef({required this.accountId, required this.displayName});

  @override
  int get hashCode => accountId.hashCode ^ displayName.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MentionRef &&
          runtimeType == other.runtimeType &&
          accountId == other.accountId &&
          displayName == other.displayName;
}

enum MessageRejection {
  forbidden,
  invalidBody,
  unknownRoom,
  unknownThread,
  invalidAttachment,
}

class SendMessage {
  final RoomId roomId;
  final ClientNonce clientNonce;
  final String body;
  final MessageId? threadRootId;
  final List<AttachmentId> attachmentIds;

  const SendMessage({
    required this.roomId,
    required this.clientNonce,
    required this.body,
    this.threadRootId,
    required this.attachmentIds,
  });

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolMessagingSendMessageValidate(that: this);

  @override
  int get hashCode =>
      roomId.hashCode ^
      clientNonce.hashCode ^
      body.hashCode ^
      threadRootId.hashCode ^
      attachmentIds.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SendMessage &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          clientNonce == other.clientNonce &&
          body == other.body &&
          threadRootId == other.threadRootId &&
          attachmentIds == other.attachmentIds;
}
