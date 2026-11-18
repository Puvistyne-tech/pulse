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
  postMessage,
  react,
  pinMessage,
  uploadAttachment,
  manageStewards,
  streamReply,
  useStewardTools,
  joinLounge,
  speakInLounge,
}

enum Role {
  owner,
  admin,
  member,
  guest,
  steward;

  Future<void> asStr() =>
      RustLib.instance.api.pulseProtocolRbacRoleAsStr(that: this);

  Future<bool> isSteward() =>
      RustLib.instance.api.pulseProtocolRbacRoleIsSteward(that: this);
}
