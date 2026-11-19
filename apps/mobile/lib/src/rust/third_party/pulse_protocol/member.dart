import '../../frb_generated.dart';
import 'api.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'rbac.dart';

Future<bool> memberManageable({required Role actor, required Role target}) =>
    RustLib.instance.api.pulseProtocolMemberMemberManageable(
      actor: actor,
      target: target,
    );

Future<bool> roleAssignable({required Role actor, required Role role}) =>
    RustLib.instance.api.pulseProtocolMemberRoleAssignable(
      actor: actor,
      role: role,
    );

Future<List<Role>> assignableRoles({required Role actor}) =>
    RustLib.instance.api.pulseProtocolMemberAssignableRoles(actor: actor);

class AddMemberRequest {
  final String email;
  final Role role;

  const AddMemberRequest({required this.email, required this.role});

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolMemberAddMemberRequestValidate(that: this);

  @override
  int get hashCode => email.hashCode ^ role.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AddMemberRequest &&
          runtimeType == other.runtimeType &&
          email == other.email &&
          role == other.role;
}

class UpdateMemberRequest {
  final Role role;

  const UpdateMemberRequest({required this.role});

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolMemberUpdateMemberRequestValidate(that: this);

  @override
  int get hashCode => role.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UpdateMemberRequest &&
          runtimeType == other.runtimeType &&
          role == other.role;
}
