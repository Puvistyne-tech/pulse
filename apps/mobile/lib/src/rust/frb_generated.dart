import 'api/client.dart';

import 'dart:async';
import 'dart:convert';

import 'frb_generated.dart';
import 'frb_generated.io.dart'
    if (dart.library.js_interop) 'frb_generated.web.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'third_party/pulse_protocol/api.dart';
import 'third_party/pulse_protocol/api/api_path.dart';
import 'third_party/pulse_protocol/auth.dart';
import 'third_party/pulse_protocol/campus.dart';
import 'third_party/pulse_protocol/frames.dart';
import 'third_party/pulse_protocol/ids.dart';
import 'third_party/pulse_protocol/rbac.dart';
import 'third_party/pulse_protocol/snapshot.dart';
import 'third_party/pulse_protocol/validation.dart';
import 'third_party/pulse_protocol/version.dart';

class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
    bool forceSameCodegenVersion = true,
  }) async {
    await instance.initImpl(
      api: api,
      handler: handler,
      externalLibrary: externalLibrary,
      forceSameCodegenVersion: forceSameCodegenVersion,
    );
  }

  static void initMock({required RustLibApi api}) {
    instance.initMockImpl(api: api);
  }

  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  Future<void> executeRustInitializers() async {}

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      kDefaultExternalLibraryLoaderConfig;

  @override
  String get codegenVersion => '2.13.0';

  @override
  int get rustContentHash => 2029780457;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
        stem: 'pulse_mobile_bridge',
        ioDirectory: 'rust/target/release/',
        webPrefix: 'pkg/',
        wasmBindgenName: 'wasm_bindgen',
      );
}

abstract class RustLibApi extends BaseApi {
  Future<UuidValue> pulseProtocolIdsAccountIdAsUuid({required AccountId that});

  Future<AccountId> pulseProtocolIdsAccountIdGenerate();

  Future<ApiErrorCode> pulseProtocolApiApiErrorCodeFromHttpStatus({
    required int status,
  });

  Future<int> pulseProtocolApiApiErrorCodeHttpStatus({
    required ApiErrorCode that,
  });

  Future<ApiError> pulseProtocolApiApiErrorConflict({
    required InputField field,
  });

  Future<ApiError> pulseProtocolApiApiErrorInvalid({required InputField field});

  Future<ApiError> pulseProtocolApiApiErrorNew({required ApiErrorCode code});

  Future<UuidValue> pulseProtocolIdsCampusIdAsUuid({required CampusId that});

  Future<CampusId> pulseProtocolIdsCampusIdGenerate();

  Future<String> pulseProtocolApiApiPathCampusRooms({
    required CampusId campusId,
  });

  Future<ClientSnapshot> crateApiClientClientSnapshot();

  Future<ClientSnapshot> crateApiClientCreateCampus({
    required CreateCampusRequest request,
  });

  Future<void> pulseProtocolCampusCreateCampusRequestValidate({
    required CreateCampusRequest that,
  });

  Future<void> pulseProtocolCampusCreateRoomRequestValidate({
    required CreateRoomRequest that,
  });

  Future<String> pulseProtocolValidationNormalizeEmail({required String raw});

  Future<ClientSnapshot> crateApiClientRefreshCampuses();

  Future<void> pulseProtocolRbacRoleAsStr({required Role that});

  Future<bool> pulseProtocolRbacRolePermits({
    required Role role,
    required Permission permission,
  });

  Future<UuidValue> pulseProtocolIdsRoomIdAsUuid({required RoomId that});

  Future<RoomId> pulseProtocolIdsRoomIdGenerate();

  Future<ClientSnapshot> crateApiClientSelectCampus({
    required CampusId campusId,
  });

  Future<UuidValue> pulseProtocolIdsSessionIdAsUuid({required SessionId that});

  Future<SessionId> pulseProtocolIdsSessionIdGenerate();

  Future<ClientSnapshot> crateApiClientSignIn({required SignInRequest request});

  Future<void> pulseProtocolAuthSignInRequestValidate({
    required SignInRequest that,
  });

  Future<ClientSnapshot> crateApiClientSignOut();

  Future<ClientSnapshot> crateApiClientSignUp({required SignUpRequest request});

  Future<void> pulseProtocolAuthSignUpRequestValidate({
    required SignUpRequest that,
  });

  Future<void> pulseProtocolValidationValidateDisplayName({
    required String raw,
  });

  Future<void> pulseProtocolValidationValidatePassword({required String raw});

  Future<void> pulseProtocolValidationValidateSlug({required String raw});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  Future<UuidValue> pulseProtocolIdsAccountIdAsUuid({required AccountId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_account_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 2,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsAccountIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsAccountIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "account_id_as_uuid", argNames: ["that"]);

  @override
  Future<AccountId> pulseProtocolIdsAccountIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 3,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_account_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsAccountIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsAccountIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "account_id_generate", argNames: []);

  @override
  Future<ApiErrorCode> pulseProtocolApiApiErrorCodeFromHttpStatus({
    required int status,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_u_16(status, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 4,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_api_error_code,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiErrorCodeFromHttpStatusConstMeta,
        argValues: [status],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiErrorCodeFromHttpStatusConstMeta =>
      const TaskConstMeta(
        debugName: "api_error_code_from_http_status",
        argNames: ["status"],
      );

  @override
  Future<int> pulseProtocolApiApiErrorCodeHttpStatus({
    required ApiErrorCode that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_api_error_code(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 5,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_u_16,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiErrorCodeHttpStatusConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiErrorCodeHttpStatusConstMeta =>
      const TaskConstMeta(
        debugName: "api_error_code_http_status",
        argNames: ["that"],
      );

  @override
  Future<ApiError> pulseProtocolApiApiErrorConflict({
    required InputField field,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_input_field(field, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 6,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_api_error,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiErrorConflictConstMeta,
        argValues: [field],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiErrorConflictConstMeta =>
      const TaskConstMeta(debugName: "api_error_conflict", argNames: ["field"]);

  @override
  Future<ApiError> pulseProtocolApiApiErrorInvalid({
    required InputField field,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_input_field(field, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 7,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_api_error,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiErrorInvalidConstMeta,
        argValues: [field],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiErrorInvalidConstMeta =>
      const TaskConstMeta(debugName: "api_error_invalid", argNames: ["field"]);

  @override
  Future<ApiError> pulseProtocolApiApiErrorNew({required ApiErrorCode code}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_api_error_code(code, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 8,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_api_error,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiErrorNewConstMeta,
        argValues: [code],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiErrorNewConstMeta =>
      const TaskConstMeta(debugName: "api_error_new", argNames: ["code"]);

  @override
  Future<UuidValue> pulseProtocolIdsCampusIdAsUuid({required CampusId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_campus_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 9,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsCampusIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsCampusIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "campus_id_as_uuid", argNames: ["that"]);

  @override
  Future<CampusId> pulseProtocolIdsCampusIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 10,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_campus_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsCampusIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsCampusIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "campus_id_generate", argNames: []);

  @override
  Future<String> pulseProtocolApiApiPathCampusRooms({
    required CampusId campusId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_campus_id(campusId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 11,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiPathCampusRoomsConstMeta,
        argValues: [campusId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiPathCampusRoomsConstMeta =>
      const TaskConstMeta(debugName: "campus_rooms", argNames: ["campusId"]);

  @override
  Future<ClientSnapshot> crateApiClientClientSnapshot() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 12,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientClientSnapshotConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientClientSnapshotConstMeta =>
      const TaskConstMeta(debugName: "client_snapshot", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientCreateCampus({
    required CreateCampusRequest request,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_create_campus_request(request, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 13,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientCreateCampusConstMeta,
        argValues: [request],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientCreateCampusConstMeta =>
      const TaskConstMeta(debugName: "create_campus", argNames: ["request"]);

  @override
  Future<void> pulseProtocolCampusCreateCampusRequestValidate({
    required CreateCampusRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_create_campus_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 14,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolCampusCreateCampusRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolCampusCreateCampusRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "create_campus_request_validate",
        argNames: ["that"],
      );

  @override
  Future<void> pulseProtocolCampusCreateRoomRequestValidate({
    required CreateRoomRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_create_room_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 15,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolCampusCreateRoomRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolCampusCreateRoomRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "create_room_request_validate",
        argNames: ["that"],
      );

  @override
  Future<String> pulseProtocolValidationNormalizeEmail({required String raw}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(raw, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 16,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeEmailConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeEmailConstMeta =>
      const TaskConstMeta(debugName: "normalize_email", argNames: ["raw"]);

  @override
  Future<ClientSnapshot> crateApiClientRefreshCampuses() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 17,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientRefreshCampusesConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientRefreshCampusesConstMeta =>
      const TaskConstMeta(debugName: "refresh_campuses", argNames: []);

  @override
  Future<void> pulseProtocolRbacRoleAsStr({required Role that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_role(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 18,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolRbacRoleAsStrConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolRbacRoleAsStrConstMeta =>
      const TaskConstMeta(debugName: "role_as_str", argNames: ["that"]);

  @override
  Future<bool> pulseProtocolRbacRolePermits({
    required Role role,
    required Permission permission,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_role(role, serializer);
          sse_encode_permission(permission, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 19,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_bool,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolRbacRolePermitsConstMeta,
        argValues: [role, permission],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolRbacRolePermitsConstMeta =>
      const TaskConstMeta(
        debugName: "role_permits",
        argNames: ["role", "permission"],
      );

  @override
  Future<UuidValue> pulseProtocolIdsRoomIdAsUuid({required RoomId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 20,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsRoomIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsRoomIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "room_id_as_uuid", argNames: ["that"]);

  @override
  Future<RoomId> pulseProtocolIdsRoomIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 21,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_room_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsRoomIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsRoomIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "room_id_generate", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientSelectCampus({
    required CampusId campusId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_campus_id(campusId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 22,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSelectCampusConstMeta,
        argValues: [campusId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSelectCampusConstMeta =>
      const TaskConstMeta(debugName: "select_campus", argNames: ["campusId"]);

  @override
  Future<UuidValue> pulseProtocolIdsSessionIdAsUuid({required SessionId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_session_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 23,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsSessionIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsSessionIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "session_id_as_uuid", argNames: ["that"]);

  @override
  Future<SessionId> pulseProtocolIdsSessionIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 24,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_session_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsSessionIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsSessionIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "session_id_generate", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientSignIn({
    required SignInRequest request,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_sign_in_request(request, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 25,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSignInConstMeta,
        argValues: [request],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSignInConstMeta =>
      const TaskConstMeta(debugName: "sign_in", argNames: ["request"]);

  @override
  Future<void> pulseProtocolAuthSignInRequestValidate({
    required SignInRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_sign_in_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 26,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolAuthSignInRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolAuthSignInRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "sign_in_request_validate",
        argNames: ["that"],
      );

  @override
  Future<ClientSnapshot> crateApiClientSignOut() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 27,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSignOutConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSignOutConstMeta =>
      const TaskConstMeta(debugName: "sign_out", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientSignUp({
    required SignUpRequest request,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_sign_up_request(request, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 28,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSignUpConstMeta,
        argValues: [request],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSignUpConstMeta =>
      const TaskConstMeta(debugName: "sign_up", argNames: ["request"]);

  @override
  Future<void> pulseProtocolAuthSignUpRequestValidate({
    required SignUpRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_sign_up_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 29,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolAuthSignUpRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolAuthSignUpRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "sign_up_request_validate",
        argNames: ["that"],
      );

  @override
  Future<void> pulseProtocolValidationValidateDisplayName({
    required String raw,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(raw, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 30,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationValidateDisplayNameConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationValidateDisplayNameConstMeta =>
      const TaskConstMeta(
        debugName: "validate_display_name",
        argNames: ["raw"],
      );

  @override
  Future<void> pulseProtocolValidationValidatePassword({required String raw}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(raw, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 31,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationValidatePasswordConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationValidatePasswordConstMeta =>
      const TaskConstMeta(debugName: "validate_password", argNames: ["raw"]);

  @override
  Future<void> pulseProtocolValidationValidateSlug({required String raw}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(raw, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 32,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationValidateSlugConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationValidateSlugConstMeta =>
      const TaskConstMeta(debugName: "validate_slug", argNames: ["raw"]);

  @protected
  String dco_decode_String(dynamic raw) {
    return raw as String;
  }

  @protected
  Validate dco_decode_TraitDef_Validate(dynamic raw) {
    throw UnimplementedError();
  }

  @protected
  UuidValue dco_decode_Uuid(dynamic raw) {
    return UuidValue.fromByteList(dco_decode_list_prim_u_8_strict(raw));
  }

  @protected
  AccountId dco_decode_account_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return AccountId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  AccountProfile dco_decode_account_profile(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return AccountProfile(
      accountId: dco_decode_account_id(arr[0]),
      email: dco_decode_String(arr[1]),
      displayName: dco_decode_String(arr[2]),
    );
  }

  @protected
  ApiError dco_decode_api_error(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return ApiError(
      code: dco_decode_api_error_code(arr[0]),
      field: dco_decode_opt_box_autoadd_input_field(arr[1]),
    );
  }

  @protected
  ApiErrorCode dco_decode_api_error_code(dynamic raw) {
    return ApiErrorCode.values[raw as int];
  }

  @protected
  AuthPhase dco_decode_auth_phase(dynamic raw) {
    return AuthPhase.values[raw as int];
  }

  @protected
  bool dco_decode_bool(dynamic raw) {
    return raw as bool;
  }

  @protected
  AccountId dco_decode_box_autoadd_account_id(dynamic raw) {
    return dco_decode_account_id(raw);
  }

  @protected
  AccountProfile dco_decode_box_autoadd_account_profile(dynamic raw) {
    return dco_decode_account_profile(raw);
  }

  @protected
  ApiError dco_decode_box_autoadd_api_error(dynamic raw) {
    return dco_decode_api_error(raw);
  }

  @protected
  CampusId dco_decode_box_autoadd_campus_id(dynamic raw) {
    return dco_decode_campus_id(raw);
  }

  @protected
  CreateCampusRequest dco_decode_box_autoadd_create_campus_request(
    dynamic raw,
  ) {
    return dco_decode_create_campus_request(raw);
  }

  @protected
  CreateRoomRequest dco_decode_box_autoadd_create_room_request(dynamic raw) {
    return dco_decode_create_room_request(raw);
  }

  @protected
  HandshakeRejection dco_decode_box_autoadd_handshake_rejection(dynamic raw) {
    return dco_decode_handshake_rejection(raw);
  }

  @protected
  InputField dco_decode_box_autoadd_input_field(dynamic raw) {
    return dco_decode_input_field(raw);
  }

  @protected
  RoomId dco_decode_box_autoadd_room_id(dynamic raw) {
    return dco_decode_room_id(raw);
  }

  @protected
  SessionId dco_decode_box_autoadd_session_id(dynamic raw) {
    return dco_decode_session_id(raw);
  }

  @protected
  SignInRequest dco_decode_box_autoadd_sign_in_request(dynamic raw) {
    return dco_decode_sign_in_request(raw);
  }

  @protected
  SignUpRequest dco_decode_box_autoadd_sign_up_request(dynamic raw) {
    return dco_decode_sign_up_request(raw);
  }

  @protected
  int dco_decode_box_autoadd_u_32(dynamic raw) {
    return raw as int;
  }

  @protected
  VersionMismatch dco_decode_box_autoadd_version_mismatch(dynamic raw) {
    return dco_decode_version_mismatch(raw);
  }

  @protected
  CampusId dco_decode_campus_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CampusId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  CampusSummary dco_decode_campus_summary(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return CampusSummary(
      campusId: dco_decode_campus_id(arr[0]),
      slug: dco_decode_String(arr[1]),
      displayName: dco_decode_String(arr[2]),
      role: dco_decode_role(arr[3]),
    );
  }

  @protected
  ClientSnapshot dco_decode_client_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return ClientSnapshot(
      network: dco_decode_connection_snapshot(arr[0]),
      session: dco_decode_session_snapshot(arr[1]),
    );
  }

  @protected
  ConnectionPhase dco_decode_connection_phase(dynamic raw) {
    return ConnectionPhase.values[raw as int];
  }

  @protected
  ConnectionSnapshot dco_decode_connection_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 8)
      throw Exception('unexpected arr length: expect 8 but see ${arr.length}');
    return ConnectionSnapshot(
      protocolVersion: dco_decode_protocol_version(arr[0]),
      connection: dco_decode_connection_phase(arr[1]),
      retryInMs: dco_decode_opt_box_autoadd_u_32(arr[2]),
      campusId: dco_decode_opt_box_autoadd_campus_id(arr[3]),
      sessionId: dco_decode_opt_box_autoadd_session_id(arr[4]),
      attempt: dco_decode_u_32(arr[5]),
      rejection: dco_decode_opt_box_autoadd_version_mismatch(arr[6]),
      handshakeRejection: dco_decode_opt_box_autoadd_handshake_rejection(
        arr[7],
      ),
    );
  }

  @protected
  CreateCampusRequest dco_decode_create_campus_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return CreateCampusRequest(
      slug: dco_decode_String(arr[0]),
      displayName: dco_decode_String(arr[1]),
    );
  }

  @protected
  CreateRoomRequest dco_decode_create_room_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CreateRoomRequest(slug: dco_decode_String(arr[0]));
  }

  @protected
  HandshakeRejection dco_decode_handshake_rejection(dynamic raw) {
    return HandshakeRejection.values[raw as int];
  }

  @protected
  int dco_decode_i_32(dynamic raw) {
    return raw as int;
  }

  @protected
  InputField dco_decode_input_field(dynamic raw) {
    return InputField.values[raw as int];
  }

  @protected
  List<CampusSummary> dco_decode_list_campus_summary(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_campus_summary).toList();
  }

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw) {
    return raw as Uint8List;
  }

  @protected
  List<Room> dco_decode_list_room(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_room).toList();
  }

  @protected
  AccountProfile? dco_decode_opt_box_autoadd_account_profile(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_account_profile(raw);
  }

  @protected
  ApiError? dco_decode_opt_box_autoadd_api_error(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_api_error(raw);
  }

  @protected
  CampusId? dco_decode_opt_box_autoadd_campus_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_campus_id(raw);
  }

  @protected
  HandshakeRejection? dco_decode_opt_box_autoadd_handshake_rejection(
    dynamic raw,
  ) {
    return raw == null ? null : dco_decode_box_autoadd_handshake_rejection(raw);
  }

  @protected
  InputField? dco_decode_opt_box_autoadd_input_field(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_input_field(raw);
  }

  @protected
  SessionId? dco_decode_opt_box_autoadd_session_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_session_id(raw);
  }

  @protected
  int? dco_decode_opt_box_autoadd_u_32(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_u_32(raw);
  }

  @protected
  VersionMismatch? dco_decode_opt_box_autoadd_version_mismatch(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_version_mismatch(raw);
  }

  @protected
  Permission dco_decode_permission(dynamic raw) {
    return Permission.values[raw as int];
  }

  @protected
  ProtocolVersion dco_decode_protocol_version(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ProtocolVersion(field0: dco_decode_u_16(arr[0]));
  }

  @protected
  Role dco_decode_role(dynamic raw) {
    return Role.values[raw as int];
  }

  @protected
  Room dco_decode_room(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return Room(
      roomId: dco_decode_room_id(arr[0]),
      campusId: dco_decode_campus_id(arr[1]),
      slug: dco_decode_String(arr[2]),
    );
  }

  @protected
  RoomId dco_decode_room_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return RoomId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  SessionId dco_decode_session_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return SessionId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  SessionSnapshot dco_decode_session_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 7)
      throw Exception('unexpected arr length: expect 7 but see ${arr.length}');
    return SessionSnapshot(
      auth: dco_decode_auth_phase(arr[0]),
      account: dco_decode_opt_box_autoadd_account_profile(arr[1]),
      campuses: dco_decode_list_campus_summary(arr[2]),
      activeCampusId: dco_decode_opt_box_autoadd_campus_id(arr[3]),
      rooms: dco_decode_list_room(arr[4]),
      busy: dco_decode_bool(arr[5]),
      error: dco_decode_opt_box_autoadd_api_error(arr[6]),
    );
  }

  @protected
  SignInRequest dco_decode_sign_in_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return SignInRequest(
      email: dco_decode_String(arr[0]),
      password: dco_decode_String(arr[1]),
    );
  }

  @protected
  SignUpRequest dco_decode_sign_up_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return SignUpRequest(
      email: dco_decode_String(arr[0]),
      displayName: dco_decode_String(arr[1]),
      password: dco_decode_String(arr[2]),
    );
  }

  @protected
  int dco_decode_u_16(dynamic raw) {
    return raw as int;
  }

  @protected
  int dco_decode_u_32(dynamic raw) {
    return raw as int;
  }

  @protected
  int dco_decode_u_8(dynamic raw) {
    return raw as int;
  }

  @protected
  void dco_decode_unit(dynamic raw) {
    return;
  }

  @protected
  VersionMismatch dco_decode_version_mismatch(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return VersionMismatch(
      supported: dco_decode_protocol_version(arr[0]),
      received: dco_decode_protocol_version(arr[1]),
    );
  }

  @protected
  String sse_decode_String(SseDeserializer deserializer) {
    var inner = sse_decode_list_prim_u_8_strict(deserializer);
    return utf8.decoder.convert(inner);
  }

  @protected
  UuidValue sse_decode_Uuid(SseDeserializer deserializer) {
    var inner = sse_decode_list_prim_u_8_strict(deserializer);
    return UuidValue.fromByteList(inner);
  }

  @protected
  AccountId sse_decode_account_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return AccountId(field0: var_field0);
  }

  @protected
  AccountProfile sse_decode_account_profile(SseDeserializer deserializer) {
    var var_accountId = sse_decode_account_id(deserializer);
    var var_email = sse_decode_String(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    return AccountProfile(
      accountId: var_accountId,
      email: var_email,
      displayName: var_displayName,
    );
  }

  @protected
  ApiError sse_decode_api_error(SseDeserializer deserializer) {
    var var_code = sse_decode_api_error_code(deserializer);
    var var_field = sse_decode_opt_box_autoadd_input_field(deserializer);
    return ApiError(code: var_code, field: var_field);
  }

  @protected
  ApiErrorCode sse_decode_api_error_code(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ApiErrorCode.values[inner];
  }

  @protected
  AuthPhase sse_decode_auth_phase(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return AuthPhase.values[inner];
  }

  @protected
  bool sse_decode_bool(SseDeserializer deserializer) {
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  AccountId sse_decode_box_autoadd_account_id(SseDeserializer deserializer) {
    return (sse_decode_account_id(deserializer));
  }

  @protected
  AccountProfile sse_decode_box_autoadd_account_profile(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_account_profile(deserializer));
  }

  @protected
  ApiError sse_decode_box_autoadd_api_error(SseDeserializer deserializer) {
    return (sse_decode_api_error(deserializer));
  }

  @protected
  CampusId sse_decode_box_autoadd_campus_id(SseDeserializer deserializer) {
    return (sse_decode_campus_id(deserializer));
  }

  @protected
  CreateCampusRequest sse_decode_box_autoadd_create_campus_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_create_campus_request(deserializer));
  }

  @protected
  CreateRoomRequest sse_decode_box_autoadd_create_room_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_create_room_request(deserializer));
  }

  @protected
  HandshakeRejection sse_decode_box_autoadd_handshake_rejection(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_handshake_rejection(deserializer));
  }

  @protected
  InputField sse_decode_box_autoadd_input_field(SseDeserializer deserializer) {
    return (sse_decode_input_field(deserializer));
  }

  @protected
  RoomId sse_decode_box_autoadd_room_id(SseDeserializer deserializer) {
    return (sse_decode_room_id(deserializer));
  }

  @protected
  SessionId sse_decode_box_autoadd_session_id(SseDeserializer deserializer) {
    return (sse_decode_session_id(deserializer));
  }

  @protected
  SignInRequest sse_decode_box_autoadd_sign_in_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_sign_in_request(deserializer));
  }

  @protected
  SignUpRequest sse_decode_box_autoadd_sign_up_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_sign_up_request(deserializer));
  }

  @protected
  int sse_decode_box_autoadd_u_32(SseDeserializer deserializer) {
    return (sse_decode_u_32(deserializer));
  }

  @protected
  VersionMismatch sse_decode_box_autoadd_version_mismatch(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_version_mismatch(deserializer));
  }

  @protected
  CampusId sse_decode_campus_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return CampusId(field0: var_field0);
  }

  @protected
  CampusSummary sse_decode_campus_summary(SseDeserializer deserializer) {
    var var_campusId = sse_decode_campus_id(deserializer);
    var var_slug = sse_decode_String(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    var var_role = sse_decode_role(deserializer);
    return CampusSummary(
      campusId: var_campusId,
      slug: var_slug,
      displayName: var_displayName,
      role: var_role,
    );
  }

  @protected
  ClientSnapshot sse_decode_client_snapshot(SseDeserializer deserializer) {
    var var_network = sse_decode_connection_snapshot(deserializer);
    var var_session = sse_decode_session_snapshot(deserializer);
    return ClientSnapshot(network: var_network, session: var_session);
  }

  @protected
  ConnectionPhase sse_decode_connection_phase(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ConnectionPhase.values[inner];
  }

  @protected
  ConnectionSnapshot sse_decode_connection_snapshot(
    SseDeserializer deserializer,
  ) {
    var var_protocolVersion = sse_decode_protocol_version(deserializer);
    var var_connection = sse_decode_connection_phase(deserializer);
    var var_retryInMs = sse_decode_opt_box_autoadd_u_32(deserializer);
    var var_campusId = sse_decode_opt_box_autoadd_campus_id(deserializer);
    var var_sessionId = sse_decode_opt_box_autoadd_session_id(deserializer);
    var var_attempt = sse_decode_u_32(deserializer);
    var var_rejection = sse_decode_opt_box_autoadd_version_mismatch(
      deserializer,
    );
    var var_handshakeRejection = sse_decode_opt_box_autoadd_handshake_rejection(
      deserializer,
    );
    return ConnectionSnapshot(
      protocolVersion: var_protocolVersion,
      connection: var_connection,
      retryInMs: var_retryInMs,
      campusId: var_campusId,
      sessionId: var_sessionId,
      attempt: var_attempt,
      rejection: var_rejection,
      handshakeRejection: var_handshakeRejection,
    );
  }

  @protected
  CreateCampusRequest sse_decode_create_campus_request(
    SseDeserializer deserializer,
  ) {
    var var_slug = sse_decode_String(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    return CreateCampusRequest(slug: var_slug, displayName: var_displayName);
  }

  @protected
  CreateRoomRequest sse_decode_create_room_request(
    SseDeserializer deserializer,
  ) {
    var var_slug = sse_decode_String(deserializer);
    return CreateRoomRequest(slug: var_slug);
  }

  @protected
  HandshakeRejection sse_decode_handshake_rejection(
    SseDeserializer deserializer,
  ) {
    var inner = sse_decode_i_32(deserializer);
    return HandshakeRejection.values[inner];
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    return deserializer.buffer.getInt32();
  }

  @protected
  InputField sse_decode_input_field(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return InputField.values[inner];
  }

  @protected
  List<CampusSummary> sse_decode_list_campus_summary(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <CampusSummary>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_campus_summary(deserializer));
    }
    return ans_;
  }

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  List<Room> sse_decode_list_room(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <Room>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_room(deserializer));
    }
    return ans_;
  }

  @protected
  AccountProfile? sse_decode_opt_box_autoadd_account_profile(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_account_profile(deserializer));
    } else {
      return null;
    }
  }

  @protected
  ApiError? sse_decode_opt_box_autoadd_api_error(SseDeserializer deserializer) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_api_error(deserializer));
    } else {
      return null;
    }
  }

  @protected
  CampusId? sse_decode_opt_box_autoadd_campus_id(SseDeserializer deserializer) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_campus_id(deserializer));
    } else {
      return null;
    }
  }

  @protected
  HandshakeRejection? sse_decode_opt_box_autoadd_handshake_rejection(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_handshake_rejection(deserializer));
    } else {
      return null;
    }
  }

  @protected
  InputField? sse_decode_opt_box_autoadd_input_field(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_input_field(deserializer));
    } else {
      return null;
    }
  }

  @protected
  SessionId? sse_decode_opt_box_autoadd_session_id(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_session_id(deserializer));
    } else {
      return null;
    }
  }

  @protected
  int? sse_decode_opt_box_autoadd_u_32(SseDeserializer deserializer) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_u_32(deserializer));
    } else {
      return null;
    }
  }

  @protected
  VersionMismatch? sse_decode_opt_box_autoadd_version_mismatch(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_version_mismatch(deserializer));
    } else {
      return null;
    }
  }

  @protected
  Permission sse_decode_permission(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return Permission.values[inner];
  }

  @protected
  ProtocolVersion sse_decode_protocol_version(SseDeserializer deserializer) {
    var var_field0 = sse_decode_u_16(deserializer);
    return ProtocolVersion(field0: var_field0);
  }

  @protected
  Role sse_decode_role(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return Role.values[inner];
  }

  @protected
  Room sse_decode_room(SseDeserializer deserializer) {
    var var_roomId = sse_decode_room_id(deserializer);
    var var_campusId = sse_decode_campus_id(deserializer);
    var var_slug = sse_decode_String(deserializer);
    return Room(roomId: var_roomId, campusId: var_campusId, slug: var_slug);
  }

  @protected
  RoomId sse_decode_room_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return RoomId(field0: var_field0);
  }

  @protected
  SessionId sse_decode_session_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return SessionId(field0: var_field0);
  }

  @protected
  SessionSnapshot sse_decode_session_snapshot(SseDeserializer deserializer) {
    var var_auth = sse_decode_auth_phase(deserializer);
    var var_account = sse_decode_opt_box_autoadd_account_profile(deserializer);
    var var_campuses = sse_decode_list_campus_summary(deserializer);
    var var_activeCampusId = sse_decode_opt_box_autoadd_campus_id(deserializer);
    var var_rooms = sse_decode_list_room(deserializer);
    var var_busy = sse_decode_bool(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    return SessionSnapshot(
      auth: var_auth,
      account: var_account,
      campuses: var_campuses,
      activeCampusId: var_activeCampusId,
      rooms: var_rooms,
      busy: var_busy,
      error: var_error,
    );
  }

  @protected
  SignInRequest sse_decode_sign_in_request(SseDeserializer deserializer) {
    var var_email = sse_decode_String(deserializer);
    var var_password = sse_decode_String(deserializer);
    return SignInRequest(email: var_email, password: var_password);
  }

  @protected
  SignUpRequest sse_decode_sign_up_request(SseDeserializer deserializer) {
    var var_email = sse_decode_String(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    var var_password = sse_decode_String(deserializer);
    return SignUpRequest(
      email: var_email,
      displayName: var_displayName,
      password: var_password,
    );
  }

  @protected
  int sse_decode_u_16(SseDeserializer deserializer) {
    return deserializer.buffer.getUint16();
  }

  @protected
  int sse_decode_u_32(SseDeserializer deserializer) {
    return deserializer.buffer.getUint32();
  }

  @protected
  int sse_decode_u_8(SseDeserializer deserializer) {
    return deserializer.buffer.getUint8();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {}

  @protected
  VersionMismatch sse_decode_version_mismatch(SseDeserializer deserializer) {
    var var_supported = sse_decode_protocol_version(deserializer);
    var var_received = sse_decode_protocol_version(deserializer);
    return VersionMismatch(supported: var_supported, received: var_received);
  }

  @protected
  void sse_encode_String(String self, SseSerializer serializer) {
    sse_encode_list_prim_u_8_strict(utf8.encoder.convert(self), serializer);
  }

  @protected
  void sse_encode_Uuid(UuidValue self, SseSerializer serializer) {
    sse_encode_list_prim_u_8_strict(self.toBytes(), serializer);
  }

  @protected
  void sse_encode_account_id(AccountId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_account_profile(
    AccountProfile self,
    SseSerializer serializer,
  ) {
    sse_encode_account_id(self.accountId, serializer);
    sse_encode_String(self.email, serializer);
    sse_encode_String(self.displayName, serializer);
  }

  @protected
  void sse_encode_api_error(ApiError self, SseSerializer serializer) {
    sse_encode_api_error_code(self.code, serializer);
    sse_encode_opt_box_autoadd_input_field(self.field, serializer);
  }

  @protected
  void sse_encode_api_error_code(ApiErrorCode self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_auth_phase(AuthPhase self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    serializer.buffer.putUint8(self ? 1 : 0);
  }

  @protected
  void sse_encode_box_autoadd_account_id(
    AccountId self,
    SseSerializer serializer,
  ) {
    sse_encode_account_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_account_profile(
    AccountProfile self,
    SseSerializer serializer,
  ) {
    sse_encode_account_profile(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_api_error(
    ApiError self,
    SseSerializer serializer,
  ) {
    sse_encode_api_error(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_campus_id(
    CampusId self,
    SseSerializer serializer,
  ) {
    sse_encode_campus_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_create_campus_request(
    CreateCampusRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_create_campus_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_create_room_request(
    CreateRoomRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_create_room_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_handshake_rejection(
    HandshakeRejection self,
    SseSerializer serializer,
  ) {
    sse_encode_handshake_rejection(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_input_field(
    InputField self,
    SseSerializer serializer,
  ) {
    sse_encode_input_field(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_room_id(RoomId self, SseSerializer serializer) {
    sse_encode_room_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_session_id(
    SessionId self,
    SseSerializer serializer,
  ) {
    sse_encode_session_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_sign_in_request(
    SignInRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_sign_in_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_sign_up_request(
    SignUpRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_sign_up_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_u_32(int self, SseSerializer serializer) {
    sse_encode_u_32(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_version_mismatch(
    VersionMismatch self,
    SseSerializer serializer,
  ) {
    sse_encode_version_mismatch(self, serializer);
  }

  @protected
  void sse_encode_campus_id(CampusId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_campus_summary(CampusSummary self, SseSerializer serializer) {
    sse_encode_campus_id(self.campusId, serializer);
    sse_encode_String(self.slug, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_role(self.role, serializer);
  }

  @protected
  void sse_encode_client_snapshot(
    ClientSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_connection_snapshot(self.network, serializer);
    sse_encode_session_snapshot(self.session, serializer);
  }

  @protected
  void sse_encode_connection_phase(
    ConnectionPhase self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_connection_snapshot(
    ConnectionSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_protocol_version(self.protocolVersion, serializer);
    sse_encode_connection_phase(self.connection, serializer);
    sse_encode_opt_box_autoadd_u_32(self.retryInMs, serializer);
    sse_encode_opt_box_autoadd_campus_id(self.campusId, serializer);
    sse_encode_opt_box_autoadd_session_id(self.sessionId, serializer);
    sse_encode_u_32(self.attempt, serializer);
    sse_encode_opt_box_autoadd_version_mismatch(self.rejection, serializer);
    sse_encode_opt_box_autoadd_handshake_rejection(
      self.handshakeRejection,
      serializer,
    );
  }

  @protected
  void sse_encode_create_campus_request(
    CreateCampusRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.slug, serializer);
    sse_encode_String(self.displayName, serializer);
  }

  @protected
  void sse_encode_create_room_request(
    CreateRoomRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.slug, serializer);
  }

  @protected
  void sse_encode_handshake_rejection(
    HandshakeRejection self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    serializer.buffer.putInt32(self);
  }

  @protected
  void sse_encode_input_field(InputField self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_list_campus_summary(
    List<CampusSummary> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_campus_summary(item, serializer);
    }
  }

  @protected
  void sse_encode_list_prim_u_8_strict(
    Uint8List self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    serializer.buffer.putUint8List(self);
  }

  @protected
  void sse_encode_list_room(List<Room> self, SseSerializer serializer) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_room(item, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_account_profile(
    AccountProfile? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_account_profile(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_api_error(
    ApiError? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_api_error(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_campus_id(
    CampusId? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_campus_id(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_handshake_rejection(
    HandshakeRejection? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_handshake_rejection(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_input_field(
    InputField? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_input_field(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_session_id(
    SessionId? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_session_id(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_u_32(int? self, SseSerializer serializer) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_u_32(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_version_mismatch(
    VersionMismatch? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_version_mismatch(self, serializer);
    }
  }

  @protected
  void sse_encode_permission(Permission self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_protocol_version(
    ProtocolVersion self,
    SseSerializer serializer,
  ) {
    sse_encode_u_16(self.field0, serializer);
  }

  @protected
  void sse_encode_role(Role self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_room(Room self, SseSerializer serializer) {
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_campus_id(self.campusId, serializer);
    sse_encode_String(self.slug, serializer);
  }

  @protected
  void sse_encode_room_id(RoomId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_session_id(SessionId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_session_snapshot(
    SessionSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_auth_phase(self.auth, serializer);
    sse_encode_opt_box_autoadd_account_profile(self.account, serializer);
    sse_encode_list_campus_summary(self.campuses, serializer);
    sse_encode_opt_box_autoadd_campus_id(self.activeCampusId, serializer);
    sse_encode_list_room(self.rooms, serializer);
    sse_encode_bool(self.busy, serializer);
    sse_encode_opt_box_autoadd_api_error(self.error, serializer);
  }

  @protected
  void sse_encode_sign_in_request(
    SignInRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.email, serializer);
    sse_encode_String(self.password, serializer);
  }

  @protected
  void sse_encode_sign_up_request(
    SignUpRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.email, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_String(self.password, serializer);
  }

  @protected
  void sse_encode_u_16(int self, SseSerializer serializer) {
    serializer.buffer.putUint16(self);
  }

  @protected
  void sse_encode_u_32(int self, SseSerializer serializer) {
    serializer.buffer.putUint32(self);
  }

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer) {
    serializer.buffer.putUint8(self);
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {}

  @protected
  void sse_encode_version_mismatch(
    VersionMismatch self,
    SseSerializer serializer,
  ) {
    sse_encode_protocol_version(self.supported, serializer);
    sse_encode_protocol_version(self.received, serializer);
  }
}
