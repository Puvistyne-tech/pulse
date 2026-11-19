import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class ApiError implements FrbException {
  final ApiErrorCode code;
  final InputField? field;

  const ApiError({required this.code, this.field});

  static Future<ApiError> conflict({required InputField field}) =>
      RustLib.instance.api.pulseProtocolApiApiErrorConflict(field: field);

  static Future<ApiError> invalid({required InputField field}) =>
      RustLib.instance.api.pulseProtocolApiApiErrorInvalid(field: field);

  static Future<ApiError> newInstance({required ApiErrorCode code}) =>
      RustLib.instance.api.pulseProtocolApiApiErrorNew(code: code);

  @override
  int get hashCode => code.hashCode ^ field.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApiError &&
          runtimeType == other.runtimeType &&
          code == other.code &&
          field == other.field;
}

enum ApiErrorCode {
  invalidInput,
  unauthenticated,
  forbidden,
  notFound,
  conflict,
  unavailable,
  internal;

  static Future<ApiErrorCode> fromHttpStatus({required int status}) => RustLib
      .instance
      .api
      .pulseProtocolApiApiErrorCodeFromHttpStatus(status: status);

  Future<int> httpStatus() =>
      RustLib.instance.api.pulseProtocolApiApiErrorCodeHttpStatus(that: this);
}

enum InputField {
  email,
  displayName,
  password,
  slug,
  messageBody,
  topicTitle,
  reactionEmoji,
  fileName,
  contentType,
  byteSize,
  attachments,
  remindAt,
  rooms,
  inviteCode,
  role,
}
