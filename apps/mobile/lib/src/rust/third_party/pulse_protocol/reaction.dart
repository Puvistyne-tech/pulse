import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

class React {
  final RoomId roomId;
  final MessageId messageId;
  final String emoji;
  final ReactionAction action;

  const React({
    required this.roomId,
    required this.messageId,
    required this.emoji,
    required this.action,
  });

  Future<void> validate() =>
      RustLib.instance.api.pulseProtocolReactionReactValidate(that: this);

  @override
  int get hashCode =>
      roomId.hashCode ^ messageId.hashCode ^ emoji.hashCode ^ action.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is React &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          messageId == other.messageId &&
          emoji == other.emoji &&
          action == other.action;
}

enum ReactionAction { add, remove }
