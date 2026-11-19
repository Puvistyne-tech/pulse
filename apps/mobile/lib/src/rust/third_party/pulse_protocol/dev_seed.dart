import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'rbac.dart';

Future<List<DevAccount>> devAccounts() =>
    RustLib.instance.api.pulseProtocolDevSeedDevAccounts();

class DevAccount {
  final String email;
  final String displayName;
  final Role role;

  const DevAccount({
    required this.email,
    required this.displayName,
    required this.role,
  });

  @override
  int get hashCode => email.hashCode ^ displayName.hashCode ^ role.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DevAccount &&
          runtimeType == other.runtimeType &&
          email == other.email &&
          displayName == other.displayName &&
          role == other.role;
}
