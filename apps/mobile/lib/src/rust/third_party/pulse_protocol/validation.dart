import '../../frb_generated.dart';
import 'api.dart';
import 'campus.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'rbac.dart';

Future<String> normalizeEmail({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeEmail(raw: raw);

Future<String> normalizeDisplayName({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeDisplayName(raw: raw);

Future<String> normalizeMessageBody({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeMessageBody(raw: raw);

Future<String?> normalizeTopicTitle({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeTopicTitle(raw: raw);

Future<String> normalizeReactionEmoji({required String raw}) => RustLib
    .instance
    .api
    .pulseProtocolValidationNormalizeReactionEmoji(raw: raw);

Future<String> normalizeFileName({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeFileName(raw: raw);

Future<String> normalizeContentType({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeContentType(raw: raw);

Future<BigInt> validateByteSize({required BigInt byteSize}) => RustLib
    .instance
    .api
    .pulseProtocolValidationValidateByteSize(byteSize: byteSize);

Future<List<AttachmentId>> normalizeAttachmentIds({
  required List<AttachmentId> ids,
}) => RustLib.instance.api.pulseProtocolValidationNormalizeAttachmentIds(
  ids: ids,
);

Future<List<RoomId>> normalizeRoomIds({required List<RoomId> roomIds}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeRoomIds(
      roomIds: roomIds,
    );

Future<(String, List<AttachmentId>)> normalizeMessageContent({
  required String body,
  required List<AttachmentId> attachmentIds,
}) => RustLib.instance.api.pulseProtocolValidationNormalizeMessageContent(
  body: body,
  attachmentIds: attachmentIds,
);

Future<BigInt> validateRemindAt({
  required BigInt remindAtMs,
  required BigInt nowMs,
}) => RustLib.instance.api.pulseProtocolValidationValidateRemindAt(
  remindAtMs: remindAtMs,
  nowMs: nowMs,
);

Future<List<String>> normalizeSearchQuery({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeSearchQuery(raw: raw);

Future<void> validateDisplayName({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationValidateDisplayName(raw: raw);

Future<void> validatePassword({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationValidatePassword(raw: raw);

Future<void> validateSlug({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationValidateSlug(raw: raw);

Future<InviteCode> normalizeInviteCode({required String raw}) =>
    RustLib.instance.api.pulseProtocolValidationNormalizeInviteCode(raw: raw);

Future<void> validateAssignableRole({required Role role}) => RustLib
    .instance
    .api
    .pulseProtocolValidationValidateAssignableRole(role: role);

abstract class Validate {
  Future<void> validate();
}
