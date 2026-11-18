import 'api/snapshot.dart';

import 'dart:async';
import 'dart:convert';

import 'frb_generated.dart';
import 'frb_generated.io.dart'
    if (dart.library.js_interop) 'frb_generated.web.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'third_party/pulse_protocol/frames.dart';
import 'third_party/pulse_protocol/ids.dart';
import 'third_party/pulse_protocol/snapshot.dart';
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
  int get rustContentHash => -190957032;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
        stem: 'pulse_mobile_bridge',
        ioDirectory: 'rust/target/release/',
        webPrefix: 'pkg/',
        wasmBindgenName: 'wasm_bindgen',
      );
}

abstract class RustLibApi extends BaseApi {
  Future<UuidValue> pulseProtocolIdsCampusIdAsUuid({required CampusId that});

  Future<CampusId> pulseProtocolIdsCampusIdGenerate();

  Future<ClientSnapshot> crateApiSnapshotInitialSnapshot();

  Future<UuidValue> pulseProtocolIdsSessionIdAsUuid({required SessionId that});

  Future<SessionId> pulseProtocolIdsSessionIdGenerate();
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

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
            funcId: 1,
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
            funcId: 2,
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
  Future<ClientSnapshot> crateApiSnapshotInitialSnapshot() {
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
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiSnapshotInitialSnapshotConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiSnapshotInitialSnapshotConstMeta =>
      const TaskConstMeta(debugName: "initial_snapshot", argNames: []);

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
            funcId: 4,
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
            funcId: 5,
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

  @protected
  UuidValue dco_decode_Uuid(dynamic raw) {
    return UuidValue.fromByteList(dco_decode_list_prim_u_8_strict(raw));
  }

  @protected
  CampusId dco_decode_box_autoadd_campus_id(dynamic raw) {
    return dco_decode_campus_id(raw);
  }

  @protected
  SessionId dco_decode_box_autoadd_session_id(dynamic raw) {
    return dco_decode_session_id(raw);
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
  ClientSnapshot dco_decode_client_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 7)
      throw Exception('unexpected arr length: expect 7 but see ${arr.length}');
    return ClientSnapshot(
      protocolVersion: dco_decode_protocol_version(arr[0]),
      connection: dco_decode_connection_phase(arr[1]),
      retryInMs: dco_decode_opt_box_autoadd_u_32(arr[2]),
      campusId: dco_decode_opt_box_autoadd_campus_id(arr[3]),
      sessionId: dco_decode_opt_box_autoadd_session_id(arr[4]),
      attempt: dco_decode_u_32(arr[5]),
      rejection: dco_decode_opt_box_autoadd_version_mismatch(arr[6]),
    );
  }

  @protected
  ConnectionPhase dco_decode_connection_phase(dynamic raw) {
    return ConnectionPhase.values[raw as int];
  }

  @protected
  int dco_decode_i_32(dynamic raw) {
    return raw as int;
  }

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw) {
    return raw as Uint8List;
  }

  @protected
  CampusId? dco_decode_opt_box_autoadd_campus_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_campus_id(raw);
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
  ProtocolVersion dco_decode_protocol_version(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ProtocolVersion(field0: dco_decode_u_16(arr[0]));
  }

  @protected
  SessionId dco_decode_session_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return SessionId(field0: dco_decode_Uuid(arr[0]));
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
  UuidValue sse_decode_Uuid(SseDeserializer deserializer) {
    var inner = sse_decode_list_prim_u_8_strict(deserializer);
    return UuidValue.fromByteList(inner);
  }

  @protected
  CampusId sse_decode_box_autoadd_campus_id(SseDeserializer deserializer) {
    return (sse_decode_campus_id(deserializer));
  }

  @protected
  SessionId sse_decode_box_autoadd_session_id(SseDeserializer deserializer) {
    return (sse_decode_session_id(deserializer));
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
  ClientSnapshot sse_decode_client_snapshot(SseDeserializer deserializer) {
    var var_protocolVersion = sse_decode_protocol_version(deserializer);
    var var_connection = sse_decode_connection_phase(deserializer);
    var var_retryInMs = sse_decode_opt_box_autoadd_u_32(deserializer);
    var var_campusId = sse_decode_opt_box_autoadd_campus_id(deserializer);
    var var_sessionId = sse_decode_opt_box_autoadd_session_id(deserializer);
    var var_attempt = sse_decode_u_32(deserializer);
    var var_rejection = sse_decode_opt_box_autoadd_version_mismatch(
      deserializer,
    );
    return ClientSnapshot(
      protocolVersion: var_protocolVersion,
      connection: var_connection,
      retryInMs: var_retryInMs,
      campusId: var_campusId,
      sessionId: var_sessionId,
      attempt: var_attempt,
      rejection: var_rejection,
    );
  }

  @protected
  ConnectionPhase sse_decode_connection_phase(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ConnectionPhase.values[inner];
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    return deserializer.buffer.getInt32();
  }

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
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
  ProtocolVersion sse_decode_protocol_version(SseDeserializer deserializer) {
    var var_field0 = sse_decode_u_16(deserializer);
    return ProtocolVersion(field0: var_field0);
  }

  @protected
  SessionId sse_decode_session_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return SessionId(field0: var_field0);
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
  bool sse_decode_bool(SseDeserializer deserializer) {
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  void sse_encode_Uuid(UuidValue self, SseSerializer serializer) {
    sse_encode_list_prim_u_8_strict(self.toBytes(), serializer);
  }

  @protected
  void sse_encode_box_autoadd_campus_id(
    CampusId self,
    SseSerializer serializer,
  ) {
    sse_encode_campus_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_session_id(
    SessionId self,
    SseSerializer serializer,
  ) {
    sse_encode_session_id(self, serializer);
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
  void sse_encode_client_snapshot(
    ClientSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_protocol_version(self.protocolVersion, serializer);
    sse_encode_connection_phase(self.connection, serializer);
    sse_encode_opt_box_autoadd_u_32(self.retryInMs, serializer);
    sse_encode_opt_box_autoadd_campus_id(self.campusId, serializer);
    sse_encode_opt_box_autoadd_session_id(self.sessionId, serializer);
    sse_encode_u_32(self.attempt, serializer);
    sse_encode_opt_box_autoadd_version_mismatch(self.rejection, serializer);
  }

  @protected
  void sse_encode_connection_phase(
    ConnectionPhase self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    serializer.buffer.putInt32(self);
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
  void sse_encode_protocol_version(
    ProtocolVersion self,
    SseSerializer serializer,
  ) {
    sse_encode_u_16(self.field0, serializer);
  }

  @protected
  void sse_encode_session_id(SessionId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
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

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    serializer.buffer.putUint8(self ? 1 : 0);
  }
}
