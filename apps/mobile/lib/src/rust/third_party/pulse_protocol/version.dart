import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class ProtocolVersion {
  final int field0;

  const ProtocolVersion({required this.field0});

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ProtocolVersion &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}
