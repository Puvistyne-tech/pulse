import '../../frb_generated.dart';
import 'api.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<String> normalizeEmail({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeEmail(raw: raw);

Future<String> normalizeDisplayName({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeDisplayName(raw: raw);

Future<String> normalizeMessageBody({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeMessageBody(raw: raw);

Future<void> validateDisplayName({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationValidateDisplayName(raw: raw);

Future<void> validatePassword({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationValidatePassword(raw: raw);

Future<void> validateSlug({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationValidateSlug(raw: raw);

abstract class Validate {
  Future<void> validate();
}
