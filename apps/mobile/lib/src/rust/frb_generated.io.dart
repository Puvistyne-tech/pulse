import 'api/client.dart';

import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;

import 'frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:uuid/uuid.dart';

import 'third_party/pulse_protocol/api.dart';
import 'third_party/pulse_protocol/api/api_path.dart';
import 'third_party/pulse_protocol/auth.dart';
import 'third_party/pulse_protocol/campus.dart';
import 'third_party/pulse_protocol/frames.dart';
import 'third_party/pulse_protocol/ids.dart';
import 'third_party/pulse_protocol/mention.dart';
import 'third_party/pulse_protocol/messaging.dart';
import 'third_party/pulse_protocol/rbac.dart';
import 'third_party/pulse_protocol/reaction.dart';
import 'third_party/pulse_protocol/snapshot.dart';
import 'third_party/pulse_protocol/thread.dart';
import 'third_party/pulse_protocol/validation.dart';
import 'third_party/pulse_protocol/version.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  RustStreamSink<ClientSnapshot> dco_decode_StreamSink_client_snapshot_Sse(
    dynamic raw,
  );

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  Validate dco_decode_TraitDef_Validate(dynamic raw);

  @protected
  UuidValue dco_decode_Uuid(dynamic raw);

  @protected
  AccountId dco_decode_account_id(dynamic raw);

  @protected
  AccountProfile dco_decode_account_profile(dynamic raw);

  @protected
  ApiError dco_decode_api_error(dynamic raw);

  @protected
  ApiErrorCode dco_decode_api_error_code(dynamic raw);

  @protected
  AuthPhase dco_decode_auth_phase(dynamic raw);

  @protected
  BodySegment dco_decode_body_segment(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  AccountId dco_decode_box_autoadd_account_id(dynamic raw);

  @protected
  AccountProfile dco_decode_box_autoadd_account_profile(dynamic raw);

  @protected
  ApiError dco_decode_box_autoadd_api_error(dynamic raw);

  @protected
  CampusId dco_decode_box_autoadd_campus_id(dynamic raw);

  @protected
  ClientNonce dco_decode_box_autoadd_client_nonce(dynamic raw);

  @protected
  CreateCampusRequest dco_decode_box_autoadd_create_campus_request(dynamic raw);

  @protected
  CreateRoomRequest dco_decode_box_autoadd_create_room_request(dynamic raw);

  @protected
  HandshakeRejection dco_decode_box_autoadd_handshake_rejection(dynamic raw);

  @protected
  InputField dco_decode_box_autoadd_input_field(dynamic raw);

  @protected
  MessageId dco_decode_box_autoadd_message_id(dynamic raw);

  @protected
  MessageRejection dco_decode_box_autoadd_message_rejection(dynamic raw);

  @protected
  React dco_decode_box_autoadd_react(dynamic raw);

  @protected
  ReplyPreview dco_decode_box_autoadd_reply_preview(dynamic raw);

  @protected
  RoomId dco_decode_box_autoadd_room_id(dynamic raw);

  @protected
  SendMessage dco_decode_box_autoadd_send_message(dynamic raw);

  @protected
  SessionId dco_decode_box_autoadd_session_id(dynamic raw);

  @protected
  SetTopicTitle dco_decode_box_autoadd_set_topic_title(dynamic raw);

  @protected
  SignInRequest dco_decode_box_autoadd_sign_in_request(dynamic raw);

  @protected
  SignUpRequest dco_decode_box_autoadd_sign_up_request(dynamic raw);

  @protected
  SubscriptionState dco_decode_box_autoadd_subscription_state(dynamic raw);

  @protected
  ThreadPreview dco_decode_box_autoadd_thread_preview(dynamic raw);

  @protected
  TimelineEntry dco_decode_box_autoadd_timeline_entry(dynamic raw);

  @protected
  int dco_decode_box_autoadd_u_32(dynamic raw);

  @protected
  VersionMismatch dco_decode_box_autoadd_version_mismatch(dynamic raw);

  @protected
  CampusId dco_decode_campus_id(dynamic raw);

  @protected
  CampusSummary dco_decode_campus_summary(dynamic raw);

  @protected
  ClientNonce dco_decode_client_nonce(dynamic raw);

  @protected
  ClientSnapshot dco_decode_client_snapshot(dynamic raw);

  @protected
  ConnectionPhase dco_decode_connection_phase(dynamic raw);

  @protected
  ConnectionSnapshot dco_decode_connection_snapshot(dynamic raw);

  @protected
  CreateCampusRequest dco_decode_create_campus_request(dynamic raw);

  @protected
  CreateRoomRequest dco_decode_create_room_request(dynamic raw);

  @protected
  DeliveryState dco_decode_delivery_state(dynamic raw);

  @protected
  HandshakeRejection dco_decode_handshake_rejection(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  InboxItem dco_decode_inbox_item(dynamic raw);

  @protected
  InboxItemKind dco_decode_inbox_item_kind(dynamic raw);

  @protected
  InboxSnapshot dco_decode_inbox_snapshot(dynamic raw);

  @protected
  InboxStep dco_decode_inbox_step(dynamic raw);

  @protected
  InputField dco_decode_input_field(dynamic raw);

  @protected
  List<String> dco_decode_list_String(dynamic raw);

  @protected
  List<AccountId> dco_decode_list_account_id(dynamic raw);

  @protected
  List<BodySegment> dco_decode_list_body_segment(dynamic raw);

  @protected
  List<CampusSummary> dco_decode_list_campus_summary(dynamic raw);

  @protected
  List<InboxItem> dco_decode_list_inbox_item(dynamic raw);

  @protected
  List<MentionRef> dco_decode_list_mention_ref(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  List<ReactionSummary> dco_decode_list_reaction_summary(dynamic raw);

  @protected
  List<Room> dco_decode_list_room(dynamic raw);

  @protected
  List<TimelineEntry> dco_decode_list_timeline_entry(dynamic raw);

  @protected
  MentionRef dco_decode_mention_ref(dynamic raw);

  @protected
  MessageId dco_decode_message_id(dynamic raw);

  @protected
  MessageRejection dco_decode_message_rejection(dynamic raw);

  @protected
  String? dco_decode_opt_String(dynamic raw);

  @protected
  AccountId? dco_decode_opt_box_autoadd_account_id(dynamic raw);

  @protected
  AccountProfile? dco_decode_opt_box_autoadd_account_profile(dynamic raw);

  @protected
  ApiError? dco_decode_opt_box_autoadd_api_error(dynamic raw);

  @protected
  CampusId? dco_decode_opt_box_autoadd_campus_id(dynamic raw);

  @protected
  HandshakeRejection? dco_decode_opt_box_autoadd_handshake_rejection(
    dynamic raw,
  );

  @protected
  InputField? dco_decode_opt_box_autoadd_input_field(dynamic raw);

  @protected
  MessageId? dco_decode_opt_box_autoadd_message_id(dynamic raw);

  @protected
  MessageRejection? dco_decode_opt_box_autoadd_message_rejection(dynamic raw);

  @protected
  ReplyPreview? dco_decode_opt_box_autoadd_reply_preview(dynamic raw);

  @protected
  RoomId? dco_decode_opt_box_autoadd_room_id(dynamic raw);

  @protected
  SessionId? dco_decode_opt_box_autoadd_session_id(dynamic raw);

  @protected
  SubscriptionState? dco_decode_opt_box_autoadd_subscription_state(dynamic raw);

  @protected
  ThreadPreview? dco_decode_opt_box_autoadd_thread_preview(dynamic raw);

  @protected
  TimelineEntry? dco_decode_opt_box_autoadd_timeline_entry(dynamic raw);

  @protected
  int? dco_decode_opt_box_autoadd_u_32(dynamic raw);

  @protected
  VersionMismatch? dco_decode_opt_box_autoadd_version_mismatch(dynamic raw);

  @protected
  Permission dco_decode_permission(dynamic raw);

  @protected
  ProtocolVersion dco_decode_protocol_version(dynamic raw);

  @protected
  React dco_decode_react(dynamic raw);

  @protected
  ReactionAction dco_decode_reaction_action(dynamic raw);

  @protected
  ReactionSummary dco_decode_reaction_summary(dynamic raw);

  @protected
  ReplyPreview dco_decode_reply_preview(dynamic raw);

  @protected
  Role dco_decode_role(dynamic raw);

  @protected
  Room dco_decode_room(dynamic raw);

  @protected
  RoomId dco_decode_room_id(dynamic raw);

  @protected
  SegmentKind dco_decode_segment_kind(dynamic raw);

  @protected
  SendMessage dco_decode_send_message(dynamic raw);

  @protected
  SessionId dco_decode_session_id(dynamic raw);

  @protected
  SessionSnapshot dco_decode_session_snapshot(dynamic raw);

  @protected
  SetTopicTitle dco_decode_set_topic_title(dynamic raw);

  @protected
  SignInRequest dco_decode_sign_in_request(dynamic raw);

  @protected
  SignUpRequest dco_decode_sign_up_request(dynamic raw);

  @protected
  SubscriptionState dco_decode_subscription_state(dynamic raw);

  @protected
  ThreadPreview dco_decode_thread_preview(dynamic raw);

  @protected
  ThreadSnapshot dco_decode_thread_snapshot(dynamic raw);

  @protected
  TimelineEntry dco_decode_timeline_entry(dynamic raw);

  @protected
  TimelineSnapshot dco_decode_timeline_snapshot(dynamic raw);

  @protected
  int dco_decode_u_16(dynamic raw);

  @protected
  int dco_decode_u_32(dynamic raw);

  @protected
  BigInt dco_decode_u_64(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  VersionMismatch dco_decode_version_mismatch(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  RustStreamSink<ClientSnapshot> sse_decode_StreamSink_client_snapshot_Sse(
    SseDeserializer deserializer,
  );

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  UuidValue sse_decode_Uuid(SseDeserializer deserializer);

  @protected
  AccountId sse_decode_account_id(SseDeserializer deserializer);

  @protected
  AccountProfile sse_decode_account_profile(SseDeserializer deserializer);

  @protected
  ApiError sse_decode_api_error(SseDeserializer deserializer);

  @protected
  ApiErrorCode sse_decode_api_error_code(SseDeserializer deserializer);

  @protected
  AuthPhase sse_decode_auth_phase(SseDeserializer deserializer);

  @protected
  BodySegment sse_decode_body_segment(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  AccountId sse_decode_box_autoadd_account_id(SseDeserializer deserializer);

  @protected
  AccountProfile sse_decode_box_autoadd_account_profile(
    SseDeserializer deserializer,
  );

  @protected
  ApiError sse_decode_box_autoadd_api_error(SseDeserializer deserializer);

  @protected
  CampusId sse_decode_box_autoadd_campus_id(SseDeserializer deserializer);

  @protected
  ClientNonce sse_decode_box_autoadd_client_nonce(SseDeserializer deserializer);

  @protected
  CreateCampusRequest sse_decode_box_autoadd_create_campus_request(
    SseDeserializer deserializer,
  );

  @protected
  CreateRoomRequest sse_decode_box_autoadd_create_room_request(
    SseDeserializer deserializer,
  );

  @protected
  HandshakeRejection sse_decode_box_autoadd_handshake_rejection(
    SseDeserializer deserializer,
  );

  @protected
  InputField sse_decode_box_autoadd_input_field(SseDeserializer deserializer);

  @protected
  MessageId sse_decode_box_autoadd_message_id(SseDeserializer deserializer);

  @protected
  MessageRejection sse_decode_box_autoadd_message_rejection(
    SseDeserializer deserializer,
  );

  @protected
  React sse_decode_box_autoadd_react(SseDeserializer deserializer);

  @protected
  ReplyPreview sse_decode_box_autoadd_reply_preview(
    SseDeserializer deserializer,
  );

  @protected
  RoomId sse_decode_box_autoadd_room_id(SseDeserializer deserializer);

  @protected
  SendMessage sse_decode_box_autoadd_send_message(SseDeserializer deserializer);

  @protected
  SessionId sse_decode_box_autoadd_session_id(SseDeserializer deserializer);

  @protected
  SetTopicTitle sse_decode_box_autoadd_set_topic_title(
    SseDeserializer deserializer,
  );

  @protected
  SignInRequest sse_decode_box_autoadd_sign_in_request(
    SseDeserializer deserializer,
  );

  @protected
  SignUpRequest sse_decode_box_autoadd_sign_up_request(
    SseDeserializer deserializer,
  );

  @protected
  SubscriptionState sse_decode_box_autoadd_subscription_state(
    SseDeserializer deserializer,
  );

  @protected
  ThreadPreview sse_decode_box_autoadd_thread_preview(
    SseDeserializer deserializer,
  );

  @protected
  TimelineEntry sse_decode_box_autoadd_timeline_entry(
    SseDeserializer deserializer,
  );

  @protected
  int sse_decode_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  VersionMismatch sse_decode_box_autoadd_version_mismatch(
    SseDeserializer deserializer,
  );

  @protected
  CampusId sse_decode_campus_id(SseDeserializer deserializer);

  @protected
  CampusSummary sse_decode_campus_summary(SseDeserializer deserializer);

  @protected
  ClientNonce sse_decode_client_nonce(SseDeserializer deserializer);

  @protected
  ClientSnapshot sse_decode_client_snapshot(SseDeserializer deserializer);

  @protected
  ConnectionPhase sse_decode_connection_phase(SseDeserializer deserializer);

  @protected
  ConnectionSnapshot sse_decode_connection_snapshot(
    SseDeserializer deserializer,
  );

  @protected
  CreateCampusRequest sse_decode_create_campus_request(
    SseDeserializer deserializer,
  );

  @protected
  CreateRoomRequest sse_decode_create_room_request(
    SseDeserializer deserializer,
  );

  @protected
  DeliveryState sse_decode_delivery_state(SseDeserializer deserializer);

  @protected
  HandshakeRejection sse_decode_handshake_rejection(
    SseDeserializer deserializer,
  );

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  InboxItem sse_decode_inbox_item(SseDeserializer deserializer);

  @protected
  InboxItemKind sse_decode_inbox_item_kind(SseDeserializer deserializer);

  @protected
  InboxSnapshot sse_decode_inbox_snapshot(SseDeserializer deserializer);

  @protected
  InboxStep sse_decode_inbox_step(SseDeserializer deserializer);

  @protected
  InputField sse_decode_input_field(SseDeserializer deserializer);

  @protected
  List<String> sse_decode_list_String(SseDeserializer deserializer);

  @protected
  List<AccountId> sse_decode_list_account_id(SseDeserializer deserializer);

  @protected
  List<BodySegment> sse_decode_list_body_segment(SseDeserializer deserializer);

  @protected
  List<CampusSummary> sse_decode_list_campus_summary(
    SseDeserializer deserializer,
  );

  @protected
  List<InboxItem> sse_decode_list_inbox_item(SseDeserializer deserializer);

  @protected
  List<MentionRef> sse_decode_list_mention_ref(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  List<ReactionSummary> sse_decode_list_reaction_summary(
    SseDeserializer deserializer,
  );

  @protected
  List<Room> sse_decode_list_room(SseDeserializer deserializer);

  @protected
  List<TimelineEntry> sse_decode_list_timeline_entry(
    SseDeserializer deserializer,
  );

  @protected
  MentionRef sse_decode_mention_ref(SseDeserializer deserializer);

  @protected
  MessageId sse_decode_message_id(SseDeserializer deserializer);

  @protected
  MessageRejection sse_decode_message_rejection(SseDeserializer deserializer);

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer);

  @protected
  AccountId? sse_decode_opt_box_autoadd_account_id(
    SseDeserializer deserializer,
  );

  @protected
  AccountProfile? sse_decode_opt_box_autoadd_account_profile(
    SseDeserializer deserializer,
  );

  @protected
  ApiError? sse_decode_opt_box_autoadd_api_error(SseDeserializer deserializer);

  @protected
  CampusId? sse_decode_opt_box_autoadd_campus_id(SseDeserializer deserializer);

  @protected
  HandshakeRejection? sse_decode_opt_box_autoadd_handshake_rejection(
    SseDeserializer deserializer,
  );

  @protected
  InputField? sse_decode_opt_box_autoadd_input_field(
    SseDeserializer deserializer,
  );

  @protected
  MessageId? sse_decode_opt_box_autoadd_message_id(
    SseDeserializer deserializer,
  );

  @protected
  MessageRejection? sse_decode_opt_box_autoadd_message_rejection(
    SseDeserializer deserializer,
  );

  @protected
  ReplyPreview? sse_decode_opt_box_autoadd_reply_preview(
    SseDeserializer deserializer,
  );

  @protected
  RoomId? sse_decode_opt_box_autoadd_room_id(SseDeserializer deserializer);

  @protected
  SessionId? sse_decode_opt_box_autoadd_session_id(
    SseDeserializer deserializer,
  );

  @protected
  SubscriptionState? sse_decode_opt_box_autoadd_subscription_state(
    SseDeserializer deserializer,
  );

  @protected
  ThreadPreview? sse_decode_opt_box_autoadd_thread_preview(
    SseDeserializer deserializer,
  );

  @protected
  TimelineEntry? sse_decode_opt_box_autoadd_timeline_entry(
    SseDeserializer deserializer,
  );

  @protected
  int? sse_decode_opt_box_autoadd_u_32(SseDeserializer deserializer);

  @protected
  VersionMismatch? sse_decode_opt_box_autoadd_version_mismatch(
    SseDeserializer deserializer,
  );

  @protected
  Permission sse_decode_permission(SseDeserializer deserializer);

  @protected
  ProtocolVersion sse_decode_protocol_version(SseDeserializer deserializer);

  @protected
  React sse_decode_react(SseDeserializer deserializer);

  @protected
  ReactionAction sse_decode_reaction_action(SseDeserializer deserializer);

  @protected
  ReactionSummary sse_decode_reaction_summary(SseDeserializer deserializer);

  @protected
  ReplyPreview sse_decode_reply_preview(SseDeserializer deserializer);

  @protected
  Role sse_decode_role(SseDeserializer deserializer);

  @protected
  Room sse_decode_room(SseDeserializer deserializer);

  @protected
  RoomId sse_decode_room_id(SseDeserializer deserializer);

  @protected
  SegmentKind sse_decode_segment_kind(SseDeserializer deserializer);

  @protected
  SendMessage sse_decode_send_message(SseDeserializer deserializer);

  @protected
  SessionId sse_decode_session_id(SseDeserializer deserializer);

  @protected
  SessionSnapshot sse_decode_session_snapshot(SseDeserializer deserializer);

  @protected
  SetTopicTitle sse_decode_set_topic_title(SseDeserializer deserializer);

  @protected
  SignInRequest sse_decode_sign_in_request(SseDeserializer deserializer);

  @protected
  SignUpRequest sse_decode_sign_up_request(SseDeserializer deserializer);

  @protected
  SubscriptionState sse_decode_subscription_state(SseDeserializer deserializer);

  @protected
  ThreadPreview sse_decode_thread_preview(SseDeserializer deserializer);

  @protected
  ThreadSnapshot sse_decode_thread_snapshot(SseDeserializer deserializer);

  @protected
  TimelineEntry sse_decode_timeline_entry(SseDeserializer deserializer);

  @protected
  TimelineSnapshot sse_decode_timeline_snapshot(SseDeserializer deserializer);

  @protected
  int sse_decode_u_16(SseDeserializer deserializer);

  @protected
  int sse_decode_u_32(SseDeserializer deserializer);

  @protected
  BigInt sse_decode_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  VersionMismatch sse_decode_version_mismatch(SseDeserializer deserializer);

  @protected
  void sse_encode_AnyhowException(
    AnyhowException self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_StreamSink_client_snapshot_Sse(
    RustStreamSink<ClientSnapshot> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_Uuid(UuidValue self, SseSerializer serializer);

  @protected
  void sse_encode_account_id(AccountId self, SseSerializer serializer);

  @protected
  void sse_encode_account_profile(
    AccountProfile self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_api_error(ApiError self, SseSerializer serializer);

  @protected
  void sse_encode_api_error_code(ApiErrorCode self, SseSerializer serializer);

  @protected
  void sse_encode_auth_phase(AuthPhase self, SseSerializer serializer);

  @protected
  void sse_encode_body_segment(BodySegment self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_account_id(
    AccountId self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_account_profile(
    AccountProfile self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_api_error(
    ApiError self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_campus_id(
    CampusId self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_client_nonce(
    ClientNonce self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_create_campus_request(
    CreateCampusRequest self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_create_room_request(
    CreateRoomRequest self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_handshake_rejection(
    HandshakeRejection self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_input_field(
    InputField self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_message_id(
    MessageId self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_message_rejection(
    MessageRejection self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_react(React self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_reply_preview(
    ReplyPreview self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_room_id(RoomId self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_send_message(
    SendMessage self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_session_id(
    SessionId self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_set_topic_title(
    SetTopicTitle self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_sign_in_request(
    SignInRequest self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_sign_up_request(
    SignUpRequest self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_subscription_state(
    SubscriptionState self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_thread_preview(
    ThreadPreview self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_box_autoadd_timeline_entry(
    TimelineEntry self,
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
  void sse_encode_campus_summary(CampusSummary self, SseSerializer serializer);

  @protected
  void sse_encode_client_nonce(ClientNonce self, SseSerializer serializer);

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
  void sse_encode_connection_snapshot(
    ConnectionSnapshot self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_create_campus_request(
    CreateCampusRequest self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_create_room_request(
    CreateRoomRequest self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_delivery_state(DeliveryState self, SseSerializer serializer);

  @protected
  void sse_encode_handshake_rejection(
    HandshakeRejection self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_inbox_item(InboxItem self, SseSerializer serializer);

  @protected
  void sse_encode_inbox_item_kind(InboxItemKind self, SseSerializer serializer);

  @protected
  void sse_encode_inbox_snapshot(InboxSnapshot self, SseSerializer serializer);

  @protected
  void sse_encode_inbox_step(InboxStep self, SseSerializer serializer);

  @protected
  void sse_encode_input_field(InputField self, SseSerializer serializer);

  @protected
  void sse_encode_list_String(List<String> self, SseSerializer serializer);

  @protected
  void sse_encode_list_account_id(
    List<AccountId> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_list_body_segment(
    List<BodySegment> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_list_campus_summary(
    List<CampusSummary> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_list_inbox_item(
    List<InboxItem> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_list_mention_ref(
    List<MentionRef> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_list_prim_u_8_strict(
    Uint8List self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_list_reaction_summary(
    List<ReactionSummary> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_list_room(List<Room> self, SseSerializer serializer);

  @protected
  void sse_encode_list_timeline_entry(
    List<TimelineEntry> self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_mention_ref(MentionRef self, SseSerializer serializer);

  @protected
  void sse_encode_message_id(MessageId self, SseSerializer serializer);

  @protected
  void sse_encode_message_rejection(
    MessageRejection self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer);

  @protected
  void sse_encode_opt_box_autoadd_account_id(
    AccountId? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_account_profile(
    AccountProfile? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_api_error(
    ApiError? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_campus_id(
    CampusId? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_handshake_rejection(
    HandshakeRejection? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_input_field(
    InputField? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_message_id(
    MessageId? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_message_rejection(
    MessageRejection? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_reply_preview(
    ReplyPreview? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_room_id(
    RoomId? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_session_id(
    SessionId? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_subscription_state(
    SubscriptionState? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_thread_preview(
    ThreadPreview? self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_opt_box_autoadd_timeline_entry(
    TimelineEntry? self,
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
  void sse_encode_permission(Permission self, SseSerializer serializer);

  @protected
  void sse_encode_protocol_version(
    ProtocolVersion self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_react(React self, SseSerializer serializer);

  @protected
  void sse_encode_reaction_action(
    ReactionAction self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_reaction_summary(
    ReactionSummary self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_reply_preview(ReplyPreview self, SseSerializer serializer);

  @protected
  void sse_encode_role(Role self, SseSerializer serializer);

  @protected
  void sse_encode_room(Room self, SseSerializer serializer);

  @protected
  void sse_encode_room_id(RoomId self, SseSerializer serializer);

  @protected
  void sse_encode_segment_kind(SegmentKind self, SseSerializer serializer);

  @protected
  void sse_encode_send_message(SendMessage self, SseSerializer serializer);

  @protected
  void sse_encode_session_id(SessionId self, SseSerializer serializer);

  @protected
  void sse_encode_session_snapshot(
    SessionSnapshot self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_set_topic_title(SetTopicTitle self, SseSerializer serializer);

  @protected
  void sse_encode_sign_in_request(SignInRequest self, SseSerializer serializer);

  @protected
  void sse_encode_sign_up_request(SignUpRequest self, SseSerializer serializer);

  @protected
  void sse_encode_subscription_state(
    SubscriptionState self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_thread_preview(ThreadPreview self, SseSerializer serializer);

  @protected
  void sse_encode_thread_snapshot(
    ThreadSnapshot self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_timeline_entry(TimelineEntry self, SseSerializer serializer);

  @protected
  void sse_encode_timeline_snapshot(
    TimelineSnapshot self,
    SseSerializer serializer,
  );

  @protected
  void sse_encode_u_16(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_64(BigInt self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_version_mismatch(
    VersionMismatch self,
    SseSerializer serializer,
  );
}

class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
  _lookup;

  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
    : _lookup = dynamicLibrary.lookup;
}
