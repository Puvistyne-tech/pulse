import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

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
