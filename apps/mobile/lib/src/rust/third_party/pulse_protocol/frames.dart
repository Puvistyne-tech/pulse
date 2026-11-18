import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'version.dart';

enum HandshakeRejection { unauthenticated, campusUnavailable }

class VersionMismatch {
  final ProtocolVersion supported;
  final ProtocolVersion received;

  const VersionMismatch({required this.supported, required this.received});

  @override
  int get hashCode => supported.hashCode ^ received.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is VersionMismatch &&
          runtimeType == other.runtimeType &&
          supported == other.supported &&
          received == other.received;
}
