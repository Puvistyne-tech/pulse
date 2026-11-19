import '../../frb_generated.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

enum LoungeScope { campus, room }

class LoungeTicket {
  final LoungeId loungeId;
  final String livekitUrl;
  final String roomName;
  final String token;
  final BigInt expiresAtMs;

  const LoungeTicket({
    required this.loungeId,
    required this.livekitUrl,
    required this.roomName,
    required this.token,
    required this.expiresAtMs,
  });

  @override
  int get hashCode =>
      loungeId.hashCode ^
      livekitUrl.hashCode ^
      roomName.hashCode ^
      token.hashCode ^
      expiresAtMs.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoungeTicket &&
          runtimeType == other.runtimeType &&
          loungeId == other.loungeId &&
          livekitUrl == other.livekitUrl &&
          roomName == other.roomName &&
          token == other.token &&
          expiresAtMs == other.expiresAtMs;
}
