import 'api/snapshot.dart';

import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;

import 'frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:uuid/uuid.dart';

import 'third_party/pulse_protocol/frames.dart';
import 'third_party/pulse_protocol/ids.dart';
import 'third_party/pulse_protocol/snapshot.dart';
import 'third_party/pulse_protocol/version.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @protected
  UuidValue dco_decode_Uuid(dynamic raw);

  @protected
  CampusId dco_decode_box_autoadd_campus_id(dynamic raw);

  @protected
  SessionId dco_decode_box_autoadd_session_id(dynamic raw);

  @protected
  int dco_decode_box_autoadd_u_32(dynamic raw);

  @protected
  VersionMismatch dco_decode_box_autoadd_version_mismatch(dynamic raw);

  @protected
  CampusId dco_decode_campus_id(dynamic raw);

  @protected
  ClientSnapshot dco_decode_client_snapshot(dynamic raw);

  @protected
  ConnectionPhase dco_decode_connection_phase(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  CampusId? dco_decode_opt_box_autoadd_campus_id(dynamic raw);

  @protected
  SessionId? dco_decode_opt_box_autoadd_session_id(dynamic raw);

  @protected
  int? dco_decode_opt_box_autoadd_u_32(dynamic raw);

  @protected
  VersionMismatch? dco_decode_opt_box_autoadd_version_mismatch(dynamic raw);

  @protected
  ProtocolVersion dco_decode_protocol_version(dynamic raw);

  @protected
  SessionId dco_decode_session_id(dynamic raw);

  @protected
  int dco_decode_u_16(dynamic raw);

  @protected
  int dco_decode_u_32(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  VersionMismatch dco_decode_version_mismatch(dynamic raw);

  @protected
  UuidValue sse_decode_Uuid(SseDeserializer deserializer);

  @protected
  CampusId sse_decode_box_autoadd_campus_id(SseDeserializer deserializer);

  @protected
  SessionId sse_decode_box_autoadd_session_id(SseDeserializer deserializer);

  @protected
  int sse_decode_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  VersionMismatch sse_decode_box_autoadd_version_mismatch(
    SseDeserializer deserializer,
  );

  @protected
  CampusId sse_decode_campus_id(SseDeserializer deserializer);

  @protected
  ClientSnapshot sse_decode_client_snapshot(SseDeserializer deserializer);

  @protected
  ConnectionPhase sse_decode_connection_phase(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  CampusId? sse_decode_opt_box_autoadd_campus_id(SseDeserializer deserializer);

  @protected
  SessionId? sse_decode_opt_box_autoadd_session_id(
    SseDeserializer deserializer,
  );

  @protected
  int? sse_decode_opt_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  VersionMismatch? sse_decode_opt_box_autoadd_version_mismatch(
    SseDeserializer deserializer,
  );

  @protected
  ProtocolVersion sse_decode_protocol_version(SseDeserializer deserializer);

  @protected
  SessionId sse_decode_session_id(SseDeserializer deserializer);

  @protected
  int sse_decode_u_16(SseDeserializer deserializer);

  @protected
  int sse_decode_u_32(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  VersionMismatch sse_decode_version_mismatch(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  void sse_encode_Uuid(UuidValue self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_campus_id(
    CampusId self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_session_id(
    SessionId self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_u_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_version_mismatch(
    VersionMismatch self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_campus_id(CampusId self, SseSerializer serializer);

  @protected
  void sse_encode_client_snapshot(
    ClientSnapshot self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_connection_phase(
    ConnectionPhase self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
    Uint8List self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_campus_id(
    CampusId? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_session_id(
    SessionId? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_u_32(int? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_version_mismatch(
    VersionMismatch? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_protocol_version(
    ProtocolVersion self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_session_id(SessionId self, SseSerializer serializer);

  @protected
  void sse_encode_u_16(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_version_mismatch(
    VersionMismatch self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);
}

class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
  _lookup;

  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
    : _lookup = dynamicLibrary.lookup;
}
