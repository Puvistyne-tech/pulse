import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<bool> rolePermits({
  required Role role,
  required Permission permission,
}) => RustLib.instance.api.pulseProtocolRbacRolePermits(
  role: role,
  permission: permission,
);

enum Permission {
  viewCampus,
  viewRooms,
  createRoom,
  manageMembers,
  manageCampus,
}

enum Role {
  owner,
  admin,
  member,
  guest;

  Future<void> asStr() =>
      RustLib.instance.api.pulseProtocolRbacRoleAsStr(that: this);
}
