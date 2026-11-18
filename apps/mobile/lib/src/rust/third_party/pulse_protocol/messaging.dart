import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

enum MessageRejection { forbidden, invalidBody, unknownRoom }

class SendMessage {
  final RoomId roomId;
  final ClientNonce clientNonce;
  final String body;

  const SendMessage({
    required this.roomId,
    required this.clientNonce,
    required this.body,
  });

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolMessagingSendMessageValidate(that: this);

  @override
  int get hashCode => roomId.hashCode ^ clientNonce.hashCode ^ body.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SendMessage &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          clientNonce == other.clientNonce &&
          body == other.body;
}
