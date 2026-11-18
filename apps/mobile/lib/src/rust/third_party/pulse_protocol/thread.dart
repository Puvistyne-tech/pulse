import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

class SetTopicTitle {
  final RoomId roomId;
  final MessageId rootMessageId;
  final String topicTitle;

  const SetTopicTitle({
    required this.roomId,
    required this.rootMessageId,
    required this.topicTitle,
  });

  Future<void> validate() =>
      RustLib.instance.api.pulseProtocolThreadSetTopicTitleValidate(that: this);

  @override
  int get hashCode =>
      roomId.hashCode ^ rootMessageId.hashCode ^ topicTitle.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SetTopicTitle &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          rootMessageId == other.rootMessageId &&
          topicTitle == other.topicTitle;
}

enum SubscriptionState { following, muted, archived }
