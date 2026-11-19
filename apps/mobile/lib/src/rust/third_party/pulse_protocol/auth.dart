import '../../frb_generated.dart';
import 'api.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

class AccountProfile {
  final AccountId accountId;
  final String email;
  final String displayName;

  const AccountProfile({
    required this.accountId,
    required this.email,
    required this.displayName,
  });

  @override
  int get hashCode =>
      accountId.hashCode ^ email.hashCode ^ displayName.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AccountProfile &&
          runtimeType == other.runtimeType &&
          accountId == other.accountId &&
          email == other.email &&
          displayName == other.displayName;
}

class AuthToken {
  final String field0;

  const AuthToken({required this.field0});

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AuthToken &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class SignInRequest {
  final String email;
  final String password;

  const SignInRequest({required this.email, required this.password});

  Future<void> validate() =>
      RustLib.instance.api.pulseProtocolAuthSignInRequestValidate(that: this);

  @override
  int get hashCode => email.hashCode ^ password.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SignInRequest &&
          runtimeType == other.runtimeType &&
          email == other.email &&
          password == other.password;
}

class SignUpRequest {
  final String email;
  final String displayName;
  final String password;

  const SignUpRequest({
    required this.email,
    required this.displayName,
    required this.password,
  });

  Future<void> validate() =>
      RustLib.instance.api.pulseProtocolAuthSignUpRequestValidate(that: this);

  @override
  int get hashCode => email.hashCode ^ displayName.hashCode ^ password.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SignUpRequest &&
          runtimeType == other.runtimeType &&
          email == other.email &&
          displayName == other.displayName &&
          password == other.password;
}
