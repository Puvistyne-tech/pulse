import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

enum ReminderPreset {
  inTwentyMinutes,
  inOneHour,
  inThreeHours,
  tomorrow,
  nextWeek;

  Future<BigInt> offsetMs() =>
      RustLib.instance.api.pulseProtocolShelfReminderPresetOffsetMs(that: this);
}

enum ShelfAction { save, reschedule, complete, reopen, remove }

enum ShelfBucket { due, upcoming, done }

class UpdateShelf {
  final RoomId roomId;
  final MessageId messageId;
  final ShelfAction action;
  final BigInt? remindAtMs;

  const UpdateShelf({
    required this.roomId,
    required this.messageId,
    required this.action,
    this.remindAtMs,
  });

  Future<void> validate() =>
      RustLib.instance.api.pulseProtocolShelfUpdateShelfValidate(that: this);

  @override
  int get hashCode =>
      roomId.hashCode ^
      messageId.hashCode ^
      action.hashCode ^
      remindAtMs.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UpdateShelf &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          messageId == other.messageId &&
          action == other.action &&
          remindAtMs == other.remindAtMs;
}
