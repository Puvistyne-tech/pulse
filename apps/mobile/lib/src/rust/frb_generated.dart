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
import 'third_party/pulse_protocol/dev_seed.dart';
import 'third_party/pulse_protocol/frames.dart';
import 'third_party/pulse_protocol/ids.dart';
import 'third_party/pulse_protocol/link.dart';
import 'third_party/pulse_protocol/lounge.dart';
import 'third_party/pulse_protocol/media.dart';
import 'third_party/pulse_protocol/member.dart';
import 'third_party/pulse_protocol/mention.dart';
import 'third_party/pulse_protocol/messaging.dart';
import 'third_party/pulse_protocol/note.dart';
import 'third_party/pulse_protocol/rbac.dart';
import 'third_party/pulse_protocol/reaction.dart';
import 'third_party/pulse_protocol/search.dart';
import 'third_party/pulse_protocol/shelf.dart';
import 'third_party/pulse_protocol/snapshot.dart';
import 'third_party/pulse_protocol/steward.dart';
import 'third_party/pulse_protocol/thread.dart';
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
  int get rustContentHash => 796307434;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
        stem: 'pulse_mobile_bridge',
        ioDirectory: 'rust/target/release/',
        webPrefix: 'pkg/',
        wasmBindgenName: 'wasm_bindgen',
      );
}

abstract class RustLibApi extends BaseApi {
  Future<StreamChunk> pulseProtocolStewardStreamWriterAbort({
    required StreamWriter that,
  });

  Future<StreamChunk> pulseProtocolStewardStreamWriterBegin({
    required StreamWriter that,
  });

  Future<StreamChunk> pulseProtocolStewardStreamWriterDelta({
    required StreamWriter that,
    required String text,
  });

  Future<StreamChunk> pulseProtocolStewardStreamWriterEnd({
    required StreamWriter that,
  });

  Future<StreamWriter> pulseProtocolStewardStreamWriterNew({
    required StreamId streamId,
    required RoomId roomId,
    MessageId? threadRootId,
  });

  Future<int> pulseProtocolStewardStreamWriterNextSequence({
    required StreamWriter that,
  });

  Future<UuidValue> pulseProtocolIdsAccountIdAsUuid({required AccountId that});

  Future<AccountId> pulseProtocolIdsAccountIdGenerate();

  Future<void> pulseProtocolMemberAddMemberRequestValidate({
    required AddMemberRequest that,
  });

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

  Future<ClientSnapshot> crateApiClientArchiveInboxSelection();

  Future<List<Role>> pulseProtocolMemberAssignableRoles({required Role actor});

  Future<ClientSnapshot> crateApiClientAttachFile({
    required RoomId roomId,
    required String fileName,
    required List<int> bytes,
  });

  Future<UuidValue> pulseProtocolIdsAttachmentIdAsUuid({
    required AttachmentId that,
  });

  Future<AttachmentId> pulseProtocolIdsAttachmentIdGenerate();

  Future<void> pulseProtocolMediaAttachmentKindAsStr({
    required AttachmentKind that,
  });

  Future<AttachmentKind> pulseProtocolMediaAttachmentKindFromContentType({
    required String contentType,
  });

  Future<List<BodySegment>> pulseProtocolMentionBodySegments({
    required String body,
    required List<MentionRef> mentions,
  });

  Future<String> pulseProtocolApiApiPathCampusDownloads({
    required CampusId campusId,
  });

  Future<UuidValue> pulseProtocolIdsCampusIdAsUuid({required CampusId that});

  Future<CampusId> pulseProtocolIdsCampusIdGenerate();

  Future<String> pulseProtocolApiApiPathCampusInvites({
    required CampusId campusId,
  });

  Future<String> pulseProtocolApiApiPathCampusMember({
    required CampusId campusId,
    required AccountId accountId,
  });

  Future<String> pulseProtocolApiApiPathCampusMembers({
    required CampusId campusId,
  });

  Future<String> pulseProtocolApiApiPathCampusNotes({
    required CampusId campusId,
  });

  Future<String> pulseProtocolApiApiPathCampusRoomUploads({
    required CampusId campusId,
    required RoomId roomId,
  });

  Future<String> pulseProtocolApiApiPathCampusRooms({
    required CampusId campusId,
  });

  Future<ClientSnapshot> crateApiClientCancelInboxReply();

  Future<UuidValue> pulseProtocolIdsClientNonceAsUuid({
    required ClientNonce that,
  });

  Future<ClientNonce> pulseProtocolIdsClientNonceGenerate();

  Future<ClientSnapshot> crateApiClientClientSnapshot();

  Future<ClientSnapshot> crateApiClientCloseThread();

  Future<ClientSnapshot> crateApiClientCloseVault();

  Future<ClientSnapshot> crateApiClientCreateCampus({
    required CreateCampusRequest request,
  });

  Future<void> pulseProtocolCampusCreateCampusRequestValidate({
    required CreateCampusRequest that,
  });

  Future<void> pulseProtocolCampusCreateRoomRequestValidate({
    required CreateRoomRequest that,
  });

  Future<void> pulseProtocolMediaCreateUploadRequestValidate({
    required CreateUploadRequest that,
  });

  Future<List<DevAccount>> pulseProtocolDevSeedDevAccounts();

  Future<ClientSnapshot> crateApiClientDiscardDraft({required DraftId draftId});

  Future<ClientSnapshot> crateApiClientDiscardMessage({
    required ClientNonce clientNonce,
  });

  Future<ClientSnapshot> crateApiClientDismissReminder();

  Future<UuidValue> pulseProtocolIdsDraftIdAsUuid({required DraftId that});

  Future<DraftId> pulseProtocolIdsDraftIdGenerate();

  Future<List<String>> pulseProtocolLinkExtractLinks({required String body});

  Future<List<AccountId>> pulseProtocolMentionExtractMentions({
    required String body,
  });

  Future<void> pulseProtocolMediaGuessContentType({required String fileName});

  Future<InboxSnapshot> pulseProtocolSnapshotInboxSnapshotDefault();

  Future<void> crateApiClientInitStorage({required String dataDir});

  Future<String> pulseProtocolCampusInviteCodeRender({
    required InviteCode that,
  });

  Future<void> pulseProtocolCampusJoinCampusRequestValidate({
    required JoinCampusRequest that,
  });

  Future<ClientSnapshot> crateApiClientLoadMoreVault();

  Future<UuidValue> pulseProtocolIdsLoungeIdAsUuid({required LoungeId that});

  Future<LoungeId> pulseProtocolIdsLoungeIdGenerate();

  Future<LoungeMediaState> pulseProtocolSnapshotLoungeMediaStateDefault();

  Future<LoungeSnapshot> pulseProtocolSnapshotLoungeSnapshotDefault();

  Future<bool> pulseProtocolMemberMemberManageable({
    required Role actor,
    required Role target,
  });

  Future<String> pulseProtocolMentionMentionToken({
    required AccountId accountId,
  });

  Future<UuidValue> pulseProtocolIdsMessageIdAsUuid({required MessageId that});

  Future<MessageId> pulseProtocolIdsMessageIdGenerate();

  Future<ClientSnapshot> crateApiClientMoveInboxSelection({
    required InboxStep step,
  });

  Future<ClientSnapshot> crateApiClientMoveSearchSelection({
    required InboxStep step,
  });

  Future<List<AttachmentId>> pulseProtocolValidationNormalizeAttachmentIds({
    required List<AttachmentId> ids,
  });

  Future<String> pulseProtocolValidationNormalizeContentType({
    required String raw,
  });

  Future<String> pulseProtocolValidationNormalizeDisplayName({
    required String raw,
  });

  Future<String> pulseProtocolValidationNormalizeEmail({required String raw});

  Future<String> pulseProtocolValidationNormalizeFileName({
    required String raw,
  });

  Future<InviteCode> pulseProtocolValidationNormalizeInviteCode({
    required String raw,
  });

  Future<String> pulseProtocolValidationNormalizeMessageBody({
    required String raw,
  });

  Future<(String, List<AttachmentId>)>
  pulseProtocolValidationNormalizeMessageContent({
    required String body,
    required List<AttachmentId> attachmentIds,
  });

  Future<List<AccountId>> pulseProtocolValidationNormalizeNoteMembers({
    required List<AccountId> requested,
    required AccountId starter,
  });

  Future<String> pulseProtocolValidationNormalizeReactionEmoji({
    required String raw,
  });

  Future<List<RoomId>> pulseProtocolValidationNormalizeRoomIds({
    required List<RoomId> roomIds,
  });

  Future<List<String>> pulseProtocolValidationNormalizeSearchQuery({
    required String raw,
  });

  Future<String?> pulseProtocolValidationNormalizeTopicTitle({
    required String raw,
  });

  Future<String> pulseProtocolNoteNoteMemberKey({
    required List<AccountId> members,
  });

  Future<String> pulseProtocolNoteNoteTitle({
    required Note note,
    required AccountId viewer,
  });

  Future<ClientSnapshot> crateApiClientOpenSearchHit({
    required MessageId messageId,
  });

  Future<ClientSnapshot> crateApiClientOpenSearchSelection();

  Future<ClientSnapshot> crateApiClientOpenThread({
    required RoomId roomId,
    required MessageId rootMessageId,
  });

  Future<ClientSnapshot> crateApiClientOpenVault({
    required RoomId roomId,
    required VaultTab tab,
  });

  Future<BigInt?> pulseProtocolShelfPickerAnchorMs({required BigInt clockMs});

  Future<BigInt?> pulseProtocolShelfPickerLatestMs({required BigInt clockMs});

  Future<ClientSnapshot> crateApiClientReact({
    required RoomId roomId,
    required MessageId messageId,
    required String emoji,
    required ReactionAction action,
  });

  Future<void> pulseProtocolReactionReactValidate({required React that});

  Future<ClientSnapshot> crateApiClientRefreshCampuses();

  Future<BigInt> pulseProtocolShelfReminderPresetOffsetMs({
    required ReminderPreset that,
  });

  Future<ClientSnapshot> crateApiClientRetryDraft({required DraftId draftId});

  Future<void> pulseProtocolRbacRoleAsStr({required Role that});

  Future<bool> pulseProtocolMemberRoleAssignable({
    required Role actor,
    required Role role,
  });

  Future<bool> pulseProtocolRbacRoleIsSteward({required Role that});

  Future<bool> pulseProtocolRbacRolePermits({
    required Role role,
    required Permission permission,
  });

  Future<UuidValue> pulseProtocolIdsRoomIdAsUuid({required RoomId that});

  Future<RoomId> pulseProtocolIdsRoomIdGenerate();

  Future<ClientSnapshot> crateApiClientSearch({required String query});

  Future<SearchSnapshot> pulseProtocolSnapshotSearchSnapshotDefault();

  Future<List<String>> pulseProtocolSearchSearchTerms({required String text});

  Future<ClientSnapshot> crateApiClientSelectCampus({
    required CampusId campusId,
  });

  Future<ClientSnapshot> crateApiClientSelectRoom({required RoomId roomId});

  Future<ClientSnapshot> crateApiClientSendMessage({
    required RoomId roomId,
    required String body,
  });

  Future<void> pulseProtocolMessagingSendMessageValidate({
    required SendMessage that,
  });

  Future<ClientSnapshot> crateApiClientSendReply({
    required RoomId roomId,
    required MessageId rootMessageId,
    required String body,
  });

  Future<UuidValue> pulseProtocolIdsSessionIdAsUuid({required SessionId that});

  Future<SessionId> pulseProtocolIdsSessionIdGenerate();

  Future<ClientSnapshot> crateApiClientSetPin({
    required RoomId roomId,
    required MessageId messageId,
    required bool pinned,
  });

  Future<ClientSnapshot> crateApiClientSetThreadSubscription({
    required RoomId roomId,
    required MessageId rootMessageId,
    required SubscriptionState state,
  });

  Future<ClientSnapshot> crateApiClientSetTopicTitle({
    required RoomId roomId,
    required MessageId rootMessageId,
    required String topicTitle,
  });

  Future<void> pulseProtocolThreadSetTopicTitleValidate({
    required SetTopicTitle that,
  });

  Future<UuidValue> pulseProtocolIdsShelfItemIdAsUuid({
    required ShelfItemId that,
  });

  Future<ShelfItemId> pulseProtocolIdsShelfItemIdGenerate();

  Future<ShelfSnapshot> pulseProtocolSnapshotShelfSnapshotDefault();

  Future<ClientSnapshot> crateApiClientSignIn({required SignInRequest request});

  Future<void> pulseProtocolAuthSignInRequestValidate({
    required SignInRequest that,
  });

  Future<ClientSnapshot> crateApiClientSignOut();

  Future<ClientSnapshot> crateApiClientSignUp({required SignUpRequest request});

  Future<void> pulseProtocolAuthSignUpRequestValidate({
    required SignUpRequest that,
  });

  Future<ClientSnapshot> crateApiClientStartInboxReply();

  Future<void> pulseProtocolNoteStartNoteRequestValidate({
    required StartNoteRequest that,
  });

  Future<StewardSnapshot> pulseProtocolSnapshotStewardSnapshotDefault();

  Future<void> pulseProtocolStewardStewardToolDescription({
    required StewardTool that,
  });

  Future<StewardTool?> pulseProtocolStewardStewardToolFromName({
    required String name,
  });

  Future<void> pulseProtocolStewardStewardToolInputSchema({
    required StewardTool that,
  });

  Future<void> pulseProtocolStewardStewardToolName({required StewardTool that});

  Future<UuidValue> pulseProtocolIdsStreamIdAsUuid({required StreamId that});

  Future<ClientNonce> pulseProtocolIdsStreamIdClientNonce({
    required StreamId that,
  });

  Future<StreamId> pulseProtocolIdsStreamIdGenerate();

  Future<void> pulseProtocolThreadSubscriptionStateAsStr({
    required SubscriptionState that,
  });

  Future<ThreadSnapshot> pulseProtocolSnapshotThreadSnapshotDefault();

  Future<ClientSnapshot> crateApiClientToggleInbox();

  Future<ClientSnapshot> crateApiClientToggleSearch();

  Future<ClientSnapshot> crateApiClientToggleShelf();

  Future<UuidValue> pulseProtocolIdsToolCallIdAsUuid({
    required ToolCallId that,
  });

  Future<ToolCallId> pulseProtocolIdsToolCallIdGenerate();

  Future<List<ToolDescriptor>> pulseProtocolStewardToolDescriptors();

  Future<void> pulseProtocolStewardToolErrorCodeAsStr({
    required ToolErrorCode that,
  });

  Future<void> pulseProtocolMemberUpdateMemberRequestValidate({
    required UpdateMemberRequest that,
  });

  Future<ClientSnapshot> crateApiClientUpdateShelf({
    required RoomId roomId,
    required MessageId messageId,
    required ShelfAction action,
    ReminderPreset? preset,
    BigInt? remindAtMs,
  });

  Future<void> pulseProtocolShelfUpdateShelfValidate({
    required UpdateShelf that,
  });

  Future<void> pulseProtocolValidationValidateAssignableRole({
    required Role role,
  });

  Future<BigInt> pulseProtocolValidationValidateByteSize({
    required BigInt byteSize,
  });

  Future<void> pulseProtocolValidationValidateDisplayName({
    required String raw,
  });

  Future<void> pulseProtocolValidationValidatePassword({required String raw});

  Future<BigInt> pulseProtocolValidationValidateRemindAt({
    required BigInt remindAtMs,
    required BigInt nowMs,
  });

  Future<void> pulseProtocolValidationValidateSlug({required String raw});

  Future<VaultSnapshot> pulseProtocolSnapshotVaultSnapshotDefault();

  Future<VaultTab> pulseProtocolMediaVaultTabDefault();

  Stream<ClientSnapshot> crateApiClientWatchSnapshots();

  RustArcIncrementStrongCountFnType
  get rust_arc_increment_strong_count_StreamWriter;

  RustArcDecrementStrongCountFnType
  get rust_arc_decrement_strong_count_StreamWriter;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_StreamWriterPtr;
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  Future<StreamChunk> pulseProtocolStewardStreamWriterAbort({
    required StreamWriter that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
            that,
            serializer,
          );
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 1,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_stream_chunk,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStreamWriterAbortConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStreamWriterAbortConstMeta =>
      const TaskConstMeta(debugName: "StreamWriter_abort", argNames: ["that"]);

  @override
  Future<StreamChunk> pulseProtocolStewardStreamWriterBegin({
    required StreamWriter that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
            that,
            serializer,
          );
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 2,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_stream_chunk,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStreamWriterBeginConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStreamWriterBeginConstMeta =>
      const TaskConstMeta(debugName: "StreamWriter_begin", argNames: ["that"]);

  @override
  Future<StreamChunk> pulseProtocolStewardStreamWriterDelta({
    required StreamWriter that,
    required String text,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
            that,
            serializer,
          );
          sse_encode_String(text, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 3,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_stream_chunk,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStreamWriterDeltaConstMeta,
        argValues: [that, text],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStreamWriterDeltaConstMeta =>
      const TaskConstMeta(
        debugName: "StreamWriter_delta",
        argNames: ["that", "text"],
      );

  @override
  Future<StreamChunk> pulseProtocolStewardStreamWriterEnd({
    required StreamWriter that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
            that,
            serializer,
          );
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 4,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_stream_chunk,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStreamWriterEndConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStreamWriterEndConstMeta =>
      const TaskConstMeta(debugName: "StreamWriter_end", argNames: ["that"]);

  @override
  Future<StreamWriter> pulseProtocolStewardStreamWriterNew({
    required StreamId streamId,
    required RoomId roomId,
    MessageId? threadRootId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_stream_id(streamId, serializer);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_opt_box_autoadd_message_id(threadRootId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 5,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData:
              sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStreamWriterNewConstMeta,
        argValues: [streamId, roomId, threadRootId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStreamWriterNewConstMeta =>
      const TaskConstMeta(
        debugName: "StreamWriter_new",
        argNames: ["streamId", "roomId", "threadRootId"],
      );

  @override
  Future<int> pulseProtocolStewardStreamWriterNextSequence({
    required StreamWriter that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
            that,
            serializer,
          );
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 6,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_u_32,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStreamWriterNextSequenceConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStreamWriterNextSequenceConstMeta =>
      const TaskConstMeta(
        debugName: "StreamWriter_next_sequence",
        argNames: ["that"],
      );

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
            funcId: 8,
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
            funcId: 9,
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
  Future<void> pulseProtocolMemberAddMemberRequestValidate({
    required AddMemberRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_add_member_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 10,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolMemberAddMemberRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMemberAddMemberRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "add_member_request_validate",
        argNames: ["that"],
      );

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
            funcId: 11,
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
            funcId: 12,
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
            funcId: 13,
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
            funcId: 14,
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
            funcId: 15,
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
  Future<ClientSnapshot> crateApiClientArchiveInboxSelection() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 16,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientArchiveInboxSelectionConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientArchiveInboxSelectionConstMeta =>
      const TaskConstMeta(debugName: "archive_inbox_selection", argNames: []);

  @override
  Future<List<Role>> pulseProtocolMemberAssignableRoles({required Role actor}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_role(actor, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 17,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_role,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMemberAssignableRolesConstMeta,
        argValues: [actor],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMemberAssignableRolesConstMeta =>
      const TaskConstMeta(debugName: "assignable_roles", argNames: ["actor"]);

  @override
  Future<ClientSnapshot> crateApiClientAttachFile({
    required RoomId roomId,
    required String fileName,
    required List<int> bytes,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_String(fileName, serializer);
          sse_encode_list_prim_u_8_loose(bytes, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 18,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientAttachFileConstMeta,
        argValues: [roomId, fileName, bytes],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientAttachFileConstMeta => const TaskConstMeta(
    debugName: "attach_file",
    argNames: ["roomId", "fileName", "bytes"],
  );

  @override
  Future<UuidValue> pulseProtocolIdsAttachmentIdAsUuid({
    required AttachmentId that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_attachment_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 19,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsAttachmentIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsAttachmentIdAsUuidConstMeta =>
      const TaskConstMeta(
        debugName: "attachment_id_as_uuid",
        argNames: ["that"],
      );

  @override
  Future<AttachmentId> pulseProtocolIdsAttachmentIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 20,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_attachment_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsAttachmentIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsAttachmentIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "attachment_id_generate", argNames: []);

  @override
  Future<void> pulseProtocolMediaAttachmentKindAsStr({
    required AttachmentKind that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_attachment_kind(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 21,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMediaAttachmentKindAsStrConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMediaAttachmentKindAsStrConstMeta =>
      const TaskConstMeta(
        debugName: "attachment_kind_as_str",
        argNames: ["that"],
      );

  @override
  Future<AttachmentKind> pulseProtocolMediaAttachmentKindFromContentType({
    required String contentType,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(contentType, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 22,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_attachment_kind,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMediaAttachmentKindFromContentTypeConstMeta,
        argValues: [contentType],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMediaAttachmentKindFromContentTypeConstMeta =>
      const TaskConstMeta(
        debugName: "attachment_kind_from_content_type",
        argNames: ["contentType"],
      );

  @override
  Future<List<BodySegment>> pulseProtocolMentionBodySegments({
    required String body,
    required List<MentionRef> mentions,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(body, serializer);
          sse_encode_list_mention_ref(mentions, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 23,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_body_segment,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMentionBodySegmentsConstMeta,
        argValues: [body, mentions],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMentionBodySegmentsConstMeta =>
      const TaskConstMeta(
        debugName: "body_segments",
        argNames: ["body", "mentions"],
      );

  @override
  Future<String> pulseProtocolApiApiPathCampusDownloads({
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
            funcId: 24,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiPathCampusDownloadsConstMeta,
        argValues: [campusId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiPathCampusDownloadsConstMeta =>
      const TaskConstMeta(
        debugName: "campus_downloads",
        argNames: ["campusId"],
      );

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
            funcId: 25,
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
            funcId: 26,
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
  Future<String> pulseProtocolApiApiPathCampusInvites({
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
            funcId: 27,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiPathCampusInvitesConstMeta,
        argValues: [campusId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiPathCampusInvitesConstMeta =>
      const TaskConstMeta(debugName: "campus_invites", argNames: ["campusId"]);

  @override
  Future<String> pulseProtocolApiApiPathCampusMember({
    required CampusId campusId,
    required AccountId accountId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_campus_id(campusId, serializer);
          sse_encode_box_autoadd_account_id(accountId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 28,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiPathCampusMemberConstMeta,
        argValues: [campusId, accountId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiPathCampusMemberConstMeta =>
      const TaskConstMeta(
        debugName: "campus_member",
        argNames: ["campusId", "accountId"],
      );

  @override
  Future<String> pulseProtocolApiApiPathCampusMembers({
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
            funcId: 29,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiPathCampusMembersConstMeta,
        argValues: [campusId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiPathCampusMembersConstMeta =>
      const TaskConstMeta(debugName: "campus_members", argNames: ["campusId"]);

  @override
  Future<String> pulseProtocolApiApiPathCampusNotes({
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
            funcId: 30,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiPathCampusNotesConstMeta,
        argValues: [campusId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiPathCampusNotesConstMeta =>
      const TaskConstMeta(debugName: "campus_notes", argNames: ["campusId"]);

  @override
  Future<String> pulseProtocolApiApiPathCampusRoomUploads({
    required CampusId campusId,
    required RoomId roomId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_campus_id(campusId, serializer);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 31,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolApiApiPathCampusRoomUploadsConstMeta,
        argValues: [campusId, roomId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolApiApiPathCampusRoomUploadsConstMeta =>
      const TaskConstMeta(
        debugName: "campus_room_uploads",
        argNames: ["campusId", "roomId"],
      );

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
            funcId: 32,
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
  Future<ClientSnapshot> crateApiClientCancelInboxReply() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 33,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientCancelInboxReplyConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientCancelInboxReplyConstMeta =>
      const TaskConstMeta(debugName: "cancel_inbox_reply", argNames: []);

  @override
  Future<UuidValue> pulseProtocolIdsClientNonceAsUuid({
    required ClientNonce that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_client_nonce(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 34,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsClientNonceAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsClientNonceAsUuidConstMeta =>
      const TaskConstMeta(
        debugName: "client_nonce_as_uuid",
        argNames: ["that"],
      );

  @override
  Future<ClientNonce> pulseProtocolIdsClientNonceGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 35,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_nonce,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsClientNonceGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsClientNonceGenerateConstMeta =>
      const TaskConstMeta(debugName: "client_nonce_generate", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientClientSnapshot() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 36,
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
  Future<ClientSnapshot> crateApiClientCloseThread() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 37,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientCloseThreadConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientCloseThreadConstMeta =>
      const TaskConstMeta(debugName: "close_thread", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientCloseVault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 38,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientCloseVaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientCloseVaultConstMeta =>
      const TaskConstMeta(debugName: "close_vault", argNames: []);

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
            funcId: 39,
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
            funcId: 40,
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
            funcId: 41,
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
  Future<void> pulseProtocolMediaCreateUploadRequestValidate({
    required CreateUploadRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_create_upload_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 42,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolMediaCreateUploadRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMediaCreateUploadRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "create_upload_request_validate",
        argNames: ["that"],
      );

  @override
  Future<List<DevAccount>> pulseProtocolDevSeedDevAccounts() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 43,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_dev_account,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolDevSeedDevAccountsConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolDevSeedDevAccountsConstMeta =>
      const TaskConstMeta(debugName: "dev_accounts", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientDiscardDraft({
    required DraftId draftId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_draft_id(draftId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 44,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientDiscardDraftConstMeta,
        argValues: [draftId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientDiscardDraftConstMeta =>
      const TaskConstMeta(debugName: "discard_draft", argNames: ["draftId"]);

  @override
  Future<ClientSnapshot> crateApiClientDiscardMessage({
    required ClientNonce clientNonce,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_client_nonce(clientNonce, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 45,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientDiscardMessageConstMeta,
        argValues: [clientNonce],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientDiscardMessageConstMeta =>
      const TaskConstMeta(
        debugName: "discard_message",
        argNames: ["clientNonce"],
      );

  @override
  Future<ClientSnapshot> crateApiClientDismissReminder() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 46,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientDismissReminderConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientDismissReminderConstMeta =>
      const TaskConstMeta(debugName: "dismiss_reminder", argNames: []);

  @override
  Future<UuidValue> pulseProtocolIdsDraftIdAsUuid({required DraftId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_draft_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 47,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsDraftIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsDraftIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "draft_id_as_uuid", argNames: ["that"]);

  @override
  Future<DraftId> pulseProtocolIdsDraftIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 48,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_draft_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsDraftIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsDraftIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "draft_id_generate", argNames: []);

  @override
  Future<List<String>> pulseProtocolLinkExtractLinks({required String body}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(body, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 49,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolLinkExtractLinksConstMeta,
        argValues: [body],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolLinkExtractLinksConstMeta =>
      const TaskConstMeta(debugName: "extract_links", argNames: ["body"]);

  @override
  Future<List<AccountId>> pulseProtocolMentionExtractMentions({
    required String body,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(body, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 50,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_account_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMentionExtractMentionsConstMeta,
        argValues: [body],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMentionExtractMentionsConstMeta =>
      const TaskConstMeta(debugName: "extract_mentions", argNames: ["body"]);

  @override
  Future<void> pulseProtocolMediaGuessContentType({required String fileName}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(fileName, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 51,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMediaGuessContentTypeConstMeta,
        argValues: [fileName],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMediaGuessContentTypeConstMeta =>
      const TaskConstMeta(
        debugName: "guess_content_type",
        argNames: ["fileName"],
      );

  @override
  Future<InboxSnapshot> pulseProtocolSnapshotInboxSnapshotDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 52,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_inbox_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotInboxSnapshotDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotInboxSnapshotDefaultConstMeta =>
      const TaskConstMeta(debugName: "inbox_snapshot_default", argNames: []);

  @override
  Future<void> crateApiClientInitStorage({required String dataDir}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(dataDir, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 53,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientInitStorageConstMeta,
        argValues: [dataDir],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientInitStorageConstMeta =>
      const TaskConstMeta(debugName: "init_storage", argNames: ["dataDir"]);

  @override
  Future<String> pulseProtocolCampusInviteCodeRender({
    required InviteCode that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_invite_code(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 54,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolCampusInviteCodeRenderConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolCampusInviteCodeRenderConstMeta =>
      const TaskConstMeta(debugName: "invite_code_render", argNames: ["that"]);

  @override
  Future<void> pulseProtocolCampusJoinCampusRequestValidate({
    required JoinCampusRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_join_campus_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 55,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolCampusJoinCampusRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolCampusJoinCampusRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "join_campus_request_validate",
        argNames: ["that"],
      );

  @override
  Future<ClientSnapshot> crateApiClientLoadMoreVault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 56,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientLoadMoreVaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientLoadMoreVaultConstMeta =>
      const TaskConstMeta(debugName: "load_more_vault", argNames: []);

  @override
  Future<UuidValue> pulseProtocolIdsLoungeIdAsUuid({required LoungeId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_lounge_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 57,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsLoungeIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsLoungeIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "lounge_id_as_uuid", argNames: ["that"]);

  @override
  Future<LoungeId> pulseProtocolIdsLoungeIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 58,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_lounge_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsLoungeIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsLoungeIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "lounge_id_generate", argNames: []);

  @override
  Future<LoungeMediaState> pulseProtocolSnapshotLoungeMediaStateDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 59,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_lounge_media_state,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotLoungeMediaStateDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotLoungeMediaStateDefaultConstMeta =>
      const TaskConstMeta(
        debugName: "lounge_media_state_default",
        argNames: [],
      );

  @override
  Future<LoungeSnapshot> pulseProtocolSnapshotLoungeSnapshotDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 60,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_lounge_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotLoungeSnapshotDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotLoungeSnapshotDefaultConstMeta =>
      const TaskConstMeta(debugName: "lounge_snapshot_default", argNames: []);

  @override
  Future<bool> pulseProtocolMemberMemberManageable({
    required Role actor,
    required Role target,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_role(actor, serializer);
          sse_encode_role(target, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 61,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_bool,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMemberMemberManageableConstMeta,
        argValues: [actor, target],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMemberMemberManageableConstMeta =>
      const TaskConstMeta(
        debugName: "member_manageable",
        argNames: ["actor", "target"],
      );

  @override
  Future<String> pulseProtocolMentionMentionToken({
    required AccountId accountId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_account_id(accountId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 62,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMentionMentionTokenConstMeta,
        argValues: [accountId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMentionMentionTokenConstMeta =>
      const TaskConstMeta(debugName: "mention_token", argNames: ["accountId"]);

  @override
  Future<UuidValue> pulseProtocolIdsMessageIdAsUuid({required MessageId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_message_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 63,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsMessageIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsMessageIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "message_id_as_uuid", argNames: ["that"]);

  @override
  Future<MessageId> pulseProtocolIdsMessageIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 64,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_message_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsMessageIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsMessageIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "message_id_generate", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientMoveInboxSelection({
    required InboxStep step,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_inbox_step(step, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 65,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientMoveInboxSelectionConstMeta,
        argValues: [step],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientMoveInboxSelectionConstMeta =>
      const TaskConstMeta(
        debugName: "move_inbox_selection",
        argNames: ["step"],
      );

  @override
  Future<ClientSnapshot> crateApiClientMoveSearchSelection({
    required InboxStep step,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_inbox_step(step, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 66,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientMoveSearchSelectionConstMeta,
        argValues: [step],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientMoveSearchSelectionConstMeta =>
      const TaskConstMeta(
        debugName: "move_search_selection",
        argNames: ["step"],
      );

  @override
  Future<List<AttachmentId>> pulseProtocolValidationNormalizeAttachmentIds({
    required List<AttachmentId> ids,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_list_attachment_id(ids, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 67,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_attachment_id,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeAttachmentIdsConstMeta,
        argValues: [ids],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeAttachmentIdsConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_attachment_ids",
        argNames: ["ids"],
      );

  @override
  Future<String> pulseProtocolValidationNormalizeContentType({
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
            funcId: 68,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeContentTypeConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeContentTypeConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_content_type",
        argNames: ["raw"],
      );

  @override
  Future<String> pulseProtocolValidationNormalizeDisplayName({
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
            funcId: 69,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeDisplayNameConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeDisplayNameConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_display_name",
        argNames: ["raw"],
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
            funcId: 70,
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
  Future<String> pulseProtocolValidationNormalizeFileName({
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
            funcId: 71,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeFileNameConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeFileNameConstMeta =>
      const TaskConstMeta(debugName: "normalize_file_name", argNames: ["raw"]);

  @override
  Future<InviteCode> pulseProtocolValidationNormalizeInviteCode({
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
            funcId: 72,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_invite_code,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeInviteCodeConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeInviteCodeConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_invite_code",
        argNames: ["raw"],
      );

  @override
  Future<String> pulseProtocolValidationNormalizeMessageBody({
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
            funcId: 73,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeMessageBodyConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeMessageBodyConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_message_body",
        argNames: ["raw"],
      );

  @override
  Future<(String, List<AttachmentId>)>
  pulseProtocolValidationNormalizeMessageContent({
    required String body,
    required List<AttachmentId> attachmentIds,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(body, serializer);
          sse_encode_list_attachment_id(attachmentIds, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 74,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_record_string_list_attachment_id,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeMessageContentConstMeta,
        argValues: [body, attachmentIds],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeMessageContentConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_message_content",
        argNames: ["body", "attachmentIds"],
      );

  @override
  Future<List<AccountId>> pulseProtocolValidationNormalizeNoteMembers({
    required List<AccountId> requested,
    required AccountId starter,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_list_account_id(requested, serializer);
          sse_encode_box_autoadd_account_id(starter, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 75,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_account_id,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeNoteMembersConstMeta,
        argValues: [requested, starter],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeNoteMembersConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_note_members",
        argNames: ["requested", "starter"],
      );

  @override
  Future<String> pulseProtocolValidationNormalizeReactionEmoji({
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
            funcId: 76,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeReactionEmojiConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeReactionEmojiConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_reaction_emoji",
        argNames: ["raw"],
      );

  @override
  Future<List<RoomId>> pulseProtocolValidationNormalizeRoomIds({
    required List<RoomId> roomIds,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_list_room_id(roomIds, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 77,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_room_id,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeRoomIdsConstMeta,
        argValues: [roomIds],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeRoomIdsConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_room_ids",
        argNames: ["roomIds"],
      );

  @override
  Future<List<String>> pulseProtocolValidationNormalizeSearchQuery({
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
            funcId: 78,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolValidationNormalizeSearchQueryConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeSearchQueryConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_search_query",
        argNames: ["raw"],
      );

  @override
  Future<String?> pulseProtocolValidationNormalizeTopicTitle({
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
            funcId: 79,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_opt_String,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationNormalizeTopicTitleConstMeta,
        argValues: [raw],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationNormalizeTopicTitleConstMeta =>
      const TaskConstMeta(
        debugName: "normalize_topic_title",
        argNames: ["raw"],
      );

  @override
  Future<String> pulseProtocolNoteNoteMemberKey({
    required List<AccountId> members,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_list_account_id(members, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 80,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolNoteNoteMemberKeyConstMeta,
        argValues: [members],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolNoteNoteMemberKeyConstMeta =>
      const TaskConstMeta(debugName: "note_member_key", argNames: ["members"]);

  @override
  Future<String> pulseProtocolNoteNoteTitle({
    required Note note,
    required AccountId viewer,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_note(note, serializer);
          sse_encode_box_autoadd_account_id(viewer, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 81,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolNoteNoteTitleConstMeta,
        argValues: [note, viewer],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolNoteNoteTitleConstMeta => const TaskConstMeta(
    debugName: "note_title",
    argNames: ["note", "viewer"],
  );

  @override
  Future<ClientSnapshot> crateApiClientOpenSearchHit({
    required MessageId messageId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_message_id(messageId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 82,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientOpenSearchHitConstMeta,
        argValues: [messageId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientOpenSearchHitConstMeta =>
      const TaskConstMeta(
        debugName: "open_search_hit",
        argNames: ["messageId"],
      );

  @override
  Future<ClientSnapshot> crateApiClientOpenSearchSelection() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 83,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientOpenSearchSelectionConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientOpenSearchSelectionConstMeta =>
      const TaskConstMeta(debugName: "open_search_selection", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientOpenThread({
    required RoomId roomId,
    required MessageId rootMessageId,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_box_autoadd_message_id(rootMessageId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 84,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientOpenThreadConstMeta,
        argValues: [roomId, rootMessageId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientOpenThreadConstMeta => const TaskConstMeta(
    debugName: "open_thread",
    argNames: ["roomId", "rootMessageId"],
  );

  @override
  Future<ClientSnapshot> crateApiClientOpenVault({
    required RoomId roomId,
    required VaultTab tab,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_vault_tab(tab, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 85,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientOpenVaultConstMeta,
        argValues: [roomId, tab],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientOpenVaultConstMeta =>
      const TaskConstMeta(debugName: "open_vault", argNames: ["roomId", "tab"]);

  @override
  Future<BigInt?> pulseProtocolShelfPickerAnchorMs({required BigInt clockMs}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_u_64(clockMs, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 86,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_opt_box_autoadd_u_64,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolShelfPickerAnchorMsConstMeta,
        argValues: [clockMs],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolShelfPickerAnchorMsConstMeta =>
      const TaskConstMeta(debugName: "picker_anchor_ms", argNames: ["clockMs"]);

  @override
  Future<BigInt?> pulseProtocolShelfPickerLatestMs({required BigInt clockMs}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_u_64(clockMs, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 87,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_opt_box_autoadd_u_64,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolShelfPickerLatestMsConstMeta,
        argValues: [clockMs],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolShelfPickerLatestMsConstMeta =>
      const TaskConstMeta(debugName: "picker_latest_ms", argNames: ["clockMs"]);

  @override
  Future<ClientSnapshot> crateApiClientReact({
    required RoomId roomId,
    required MessageId messageId,
    required String emoji,
    required ReactionAction action,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_box_autoadd_message_id(messageId, serializer);
          sse_encode_String(emoji, serializer);
          sse_encode_reaction_action(action, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 88,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientReactConstMeta,
        argValues: [roomId, messageId, emoji, action],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientReactConstMeta => const TaskConstMeta(
    debugName: "react",
    argNames: ["roomId", "messageId", "emoji", "action"],
  );

  @override
  Future<void> pulseProtocolReactionReactValidate({required React that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_react(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 89,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolReactionReactValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolReactionReactValidateConstMeta =>
      const TaskConstMeta(debugName: "react_validate", argNames: ["that"]);

  @override
  Future<ClientSnapshot> crateApiClientRefreshCampuses() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 90,
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
  Future<BigInt> pulseProtocolShelfReminderPresetOffsetMs({
    required ReminderPreset that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_reminder_preset(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 91,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_u_64,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolShelfReminderPresetOffsetMsConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolShelfReminderPresetOffsetMsConstMeta =>
      const TaskConstMeta(
        debugName: "reminder_preset_offset_ms",
        argNames: ["that"],
      );

  @override
  Future<ClientSnapshot> crateApiClientRetryDraft({required DraftId draftId}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_draft_id(draftId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 92,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientRetryDraftConstMeta,
        argValues: [draftId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientRetryDraftConstMeta =>
      const TaskConstMeta(debugName: "retry_draft", argNames: ["draftId"]);

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
            funcId: 93,
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
  Future<bool> pulseProtocolMemberRoleAssignable({
    required Role actor,
    required Role role,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_role(actor, serializer);
          sse_encode_role(role, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 94,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_bool,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMemberRoleAssignableConstMeta,
        argValues: [actor, role],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMemberRoleAssignableConstMeta =>
      const TaskConstMeta(
        debugName: "role_assignable",
        argNames: ["actor", "role"],
      );

  @override
  Future<bool> pulseProtocolRbacRoleIsSteward({required Role that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_role(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 95,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_bool,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolRbacRoleIsStewardConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolRbacRoleIsStewardConstMeta =>
      const TaskConstMeta(debugName: "role_is_steward", argNames: ["that"]);

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
            funcId: 96,
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
            funcId: 97,
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
            funcId: 98,
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
  Future<ClientSnapshot> crateApiClientSearch({required String query}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(query, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 99,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSearchConstMeta,
        argValues: [query],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSearchConstMeta =>
      const TaskConstMeta(debugName: "search", argNames: ["query"]);

  @override
  Future<SearchSnapshot> pulseProtocolSnapshotSearchSnapshotDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 100,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_search_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotSearchSnapshotDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotSearchSnapshotDefaultConstMeta =>
      const TaskConstMeta(debugName: "search_snapshot_default", argNames: []);

  @override
  Future<List<String>> pulseProtocolSearchSearchTerms({required String text}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(text, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 101,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_String,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSearchSearchTermsConstMeta,
        argValues: [text],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSearchSearchTermsConstMeta =>
      const TaskConstMeta(debugName: "search_terms", argNames: ["text"]);

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
            funcId: 102,
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
  Future<ClientSnapshot> crateApiClientSelectRoom({required RoomId roomId}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 103,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSelectRoomConstMeta,
        argValues: [roomId],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSelectRoomConstMeta =>
      const TaskConstMeta(debugName: "select_room", argNames: ["roomId"]);

  @override
  Future<ClientSnapshot> crateApiClientSendMessage({
    required RoomId roomId,
    required String body,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_String(body, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 104,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSendMessageConstMeta,
        argValues: [roomId, body],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSendMessageConstMeta => const TaskConstMeta(
    debugName: "send_message",
    argNames: ["roomId", "body"],
  );

  @override
  Future<void> pulseProtocolMessagingSendMessageValidate({
    required SendMessage that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_send_message(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 105,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolMessagingSendMessageValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMessagingSendMessageValidateConstMeta =>
      const TaskConstMeta(
        debugName: "send_message_validate",
        argNames: ["that"],
      );

  @override
  Future<ClientSnapshot> crateApiClientSendReply({
    required RoomId roomId,
    required MessageId rootMessageId,
    required String body,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_box_autoadd_message_id(rootMessageId, serializer);
          sse_encode_String(body, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 106,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSendReplyConstMeta,
        argValues: [roomId, rootMessageId, body],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSendReplyConstMeta => const TaskConstMeta(
    debugName: "send_reply",
    argNames: ["roomId", "rootMessageId", "body"],
  );

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
            funcId: 107,
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
            funcId: 108,
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
  Future<ClientSnapshot> crateApiClientSetPin({
    required RoomId roomId,
    required MessageId messageId,
    required bool pinned,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_box_autoadd_message_id(messageId, serializer);
          sse_encode_bool(pinned, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 109,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSetPinConstMeta,
        argValues: [roomId, messageId, pinned],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSetPinConstMeta => const TaskConstMeta(
    debugName: "set_pin",
    argNames: ["roomId", "messageId", "pinned"],
  );

  @override
  Future<ClientSnapshot> crateApiClientSetThreadSubscription({
    required RoomId roomId,
    required MessageId rootMessageId,
    required SubscriptionState state,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_box_autoadd_message_id(rootMessageId, serializer);
          sse_encode_subscription_state(state, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 110,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSetThreadSubscriptionConstMeta,
        argValues: [roomId, rootMessageId, state],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSetThreadSubscriptionConstMeta =>
      const TaskConstMeta(
        debugName: "set_thread_subscription",
        argNames: ["roomId", "rootMessageId", "state"],
      );

  @override
  Future<ClientSnapshot> crateApiClientSetTopicTitle({
    required RoomId roomId,
    required MessageId rootMessageId,
    required String topicTitle,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_box_autoadd_message_id(rootMessageId, serializer);
          sse_encode_String(topicTitle, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 111,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientSetTopicTitleConstMeta,
        argValues: [roomId, rootMessageId, topicTitle],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientSetTopicTitleConstMeta =>
      const TaskConstMeta(
        debugName: "set_topic_title",
        argNames: ["roomId", "rootMessageId", "topicTitle"],
      );

  @override
  Future<void> pulseProtocolThreadSetTopicTitleValidate({
    required SetTopicTitle that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_set_topic_title(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 112,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolThreadSetTopicTitleValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolThreadSetTopicTitleValidateConstMeta =>
      const TaskConstMeta(
        debugName: "set_topic_title_validate",
        argNames: ["that"],
      );

  @override
  Future<UuidValue> pulseProtocolIdsShelfItemIdAsUuid({
    required ShelfItemId that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_shelf_item_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 113,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsShelfItemIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsShelfItemIdAsUuidConstMeta =>
      const TaskConstMeta(
        debugName: "shelf_item_id_as_uuid",
        argNames: ["that"],
      );

  @override
  Future<ShelfItemId> pulseProtocolIdsShelfItemIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 114,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_shelf_item_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsShelfItemIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsShelfItemIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "shelf_item_id_generate", argNames: []);

  @override
  Future<ShelfSnapshot> pulseProtocolSnapshotShelfSnapshotDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 115,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_shelf_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotShelfSnapshotDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotShelfSnapshotDefaultConstMeta =>
      const TaskConstMeta(debugName: "shelf_snapshot_default", argNames: []);

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
            funcId: 116,
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
            funcId: 117,
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
            funcId: 118,
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
            funcId: 119,
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
            funcId: 120,
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
  Future<ClientSnapshot> crateApiClientStartInboxReply() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 121,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientStartInboxReplyConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientStartInboxReplyConstMeta =>
      const TaskConstMeta(debugName: "start_inbox_reply", argNames: []);

  @override
  Future<void> pulseProtocolNoteStartNoteRequestValidate({
    required StartNoteRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_start_note_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 122,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolNoteStartNoteRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolNoteStartNoteRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "start_note_request_validate",
        argNames: ["that"],
      );

  @override
  Future<StewardSnapshot> pulseProtocolSnapshotStewardSnapshotDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 123,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_steward_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotStewardSnapshotDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotStewardSnapshotDefaultConstMeta =>
      const TaskConstMeta(debugName: "steward_snapshot_default", argNames: []);

  @override
  Future<void> pulseProtocolStewardStewardToolDescription({
    required StewardTool that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_steward_tool(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 124,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStewardToolDescriptionConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStewardToolDescriptionConstMeta =>
      const TaskConstMeta(
        debugName: "steward_tool_description",
        argNames: ["that"],
      );

  @override
  Future<StewardTool?> pulseProtocolStewardStewardToolFromName({
    required String name,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_String(name, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 125,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_opt_box_autoadd_steward_tool,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStewardToolFromNameConstMeta,
        argValues: [name],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStewardToolFromNameConstMeta =>
      const TaskConstMeta(
        debugName: "steward_tool_from_name",
        argNames: ["name"],
      );

  @override
  Future<void> pulseProtocolStewardStewardToolInputSchema({
    required StewardTool that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_steward_tool(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 126,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStewardToolInputSchemaConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStewardToolInputSchemaConstMeta =>
      const TaskConstMeta(
        debugName: "steward_tool_input_schema",
        argNames: ["that"],
      );

  @override
  Future<void> pulseProtocolStewardStewardToolName({
    required StewardTool that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_steward_tool(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 127,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardStewardToolNameConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardStewardToolNameConstMeta =>
      const TaskConstMeta(debugName: "steward_tool_name", argNames: ["that"]);

  @override
  Future<UuidValue> pulseProtocolIdsStreamIdAsUuid({required StreamId that}) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_stream_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 128,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsStreamIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsStreamIdAsUuidConstMeta =>
      const TaskConstMeta(debugName: "stream_id_as_uuid", argNames: ["that"]);

  @override
  Future<ClientNonce> pulseProtocolIdsStreamIdClientNonce({
    required StreamId that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_stream_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 129,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_nonce,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsStreamIdClientNonceConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsStreamIdClientNonceConstMeta =>
      const TaskConstMeta(
        debugName: "stream_id_client_nonce",
        argNames: ["that"],
      );

  @override
  Future<StreamId> pulseProtocolIdsStreamIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 130,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_stream_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsStreamIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsStreamIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "stream_id_generate", argNames: []);

  @override
  Future<void> pulseProtocolThreadSubscriptionStateAsStr({
    required SubscriptionState that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_subscription_state(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 131,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolThreadSubscriptionStateAsStrConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolThreadSubscriptionStateAsStrConstMeta =>
      const TaskConstMeta(
        debugName: "subscription_state_as_str",
        argNames: ["that"],
      );

  @override
  Future<ThreadSnapshot> pulseProtocolSnapshotThreadSnapshotDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 132,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_thread_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotThreadSnapshotDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotThreadSnapshotDefaultConstMeta =>
      const TaskConstMeta(debugName: "thread_snapshot_default", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientToggleInbox() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 133,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientToggleInboxConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientToggleInboxConstMeta =>
      const TaskConstMeta(debugName: "toggle_inbox", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientToggleSearch() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 134,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientToggleSearchConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientToggleSearchConstMeta =>
      const TaskConstMeta(debugName: "toggle_search", argNames: []);

  @override
  Future<ClientSnapshot> crateApiClientToggleShelf() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 135,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientToggleShelfConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientToggleShelfConstMeta =>
      const TaskConstMeta(debugName: "toggle_shelf", argNames: []);

  @override
  Future<UuidValue> pulseProtocolIdsToolCallIdAsUuid({
    required ToolCallId that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_tool_call_id(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 136,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_Uuid,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsToolCallIdAsUuidConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsToolCallIdAsUuidConstMeta =>
      const TaskConstMeta(
        debugName: "tool_call_id_as_uuid",
        argNames: ["that"],
      );

  @override
  Future<ToolCallId> pulseProtocolIdsToolCallIdGenerate() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 137,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_tool_call_id,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolIdsToolCallIdGenerateConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolIdsToolCallIdGenerateConstMeta =>
      const TaskConstMeta(debugName: "tool_call_id_generate", argNames: []);

  @override
  Future<List<ToolDescriptor>> pulseProtocolStewardToolDescriptors() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 138,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_list_tool_descriptor,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardToolDescriptorsConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardToolDescriptorsConstMeta =>
      const TaskConstMeta(debugName: "tool_descriptors", argNames: []);

  @override
  Future<void> pulseProtocolStewardToolErrorCodeAsStr({
    required ToolErrorCode that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_tool_error_code(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 139,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolStewardToolErrorCodeAsStrConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolStewardToolErrorCodeAsStrConstMeta =>
      const TaskConstMeta(
        debugName: "tool_error_code_as_str",
        argNames: ["that"],
      );

  @override
  Future<void> pulseProtocolMemberUpdateMemberRequestValidate({
    required UpdateMemberRequest that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_update_member_request(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 140,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolMemberUpdateMemberRequestValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMemberUpdateMemberRequestValidateConstMeta =>
      const TaskConstMeta(
        debugName: "update_member_request_validate",
        argNames: ["that"],
      );

  @override
  Future<ClientSnapshot> crateApiClientUpdateShelf({
    required RoomId roomId,
    required MessageId messageId,
    required ShelfAction action,
    ReminderPreset? preset,
    BigInt? remindAtMs,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_room_id(roomId, serializer);
          sse_encode_box_autoadd_message_id(messageId, serializer);
          sse_encode_shelf_action(action, serializer);
          sse_encode_opt_box_autoadd_reminder_preset(preset, serializer);
          sse_encode_opt_box_autoadd_u_64(remindAtMs, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 141,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kCrateApiClientUpdateShelfConstMeta,
        argValues: [roomId, messageId, action, preset, remindAtMs],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kCrateApiClientUpdateShelfConstMeta => const TaskConstMeta(
    debugName: "update_shelf",
    argNames: ["roomId", "messageId", "action", "preset", "remindAtMs"],
  );

  @override
  Future<void> pulseProtocolShelfUpdateShelfValidate({
    required UpdateShelf that,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_box_autoadd_update_shelf(that, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 142,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_api_error,
        ),
        constMeta: kPulseProtocolShelfUpdateShelfValidateConstMeta,
        argValues: [that],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolShelfUpdateShelfValidateConstMeta =>
      const TaskConstMeta(
        debugName: "update_shelf_validate",
        argNames: ["that"],
      );

  @override
  Future<void> pulseProtocolValidationValidateAssignableRole({
    required Role role,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_role(role, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 143,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationValidateAssignableRoleConstMeta,
        argValues: [role],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationValidateAssignableRoleConstMeta =>
      const TaskConstMeta(
        debugName: "validate_assignable_role",
        argNames: ["role"],
      );

  @override
  Future<BigInt> pulseProtocolValidationValidateByteSize({
    required BigInt byteSize,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_u_64(byteSize, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 144,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_u_64,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationValidateByteSizeConstMeta,
        argValues: [byteSize],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationValidateByteSizeConstMeta =>
      const TaskConstMeta(
        debugName: "validate_byte_size",
        argNames: ["byteSize"],
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
            funcId: 145,
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
            funcId: 146,
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
  Future<BigInt> pulseProtocolValidationValidateRemindAt({
    required BigInt remindAtMs,
    required BigInt nowMs,
  }) {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          sse_encode_u_64(remindAtMs, serializer);
          sse_encode_u_64(nowMs, serializer);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 147,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_u_64,
          decodeErrorData: sse_decode_input_field,
        ),
        constMeta: kPulseProtocolValidationValidateRemindAtConstMeta,
        argValues: [remindAtMs, nowMs],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolValidationValidateRemindAtConstMeta =>
      const TaskConstMeta(
        debugName: "validate_remind_at",
        argNames: ["remindAtMs", "nowMs"],
      );

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
            funcId: 148,
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

  @override
  Future<VaultSnapshot> pulseProtocolSnapshotVaultSnapshotDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 149,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_vault_snapshot,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolSnapshotVaultSnapshotDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolSnapshotVaultSnapshotDefaultConstMeta =>
      const TaskConstMeta(debugName: "vault_snapshot_default", argNames: []);

  @override
  Future<VaultTab> pulseProtocolMediaVaultTabDefault() {
    return handler.executeNormal(
      NormalTask(
        callFfi: (port_) {
          final serializer = SseSerializer(generalizedFrbRustBinding);
          pdeCallFfi(
            generalizedFrbRustBinding,
            serializer,
            funcId: 150,
            port: port_,
          );
        },
        codec: SseCodec(
          decodeSuccessData: sse_decode_vault_tab,
          decodeErrorData: null,
        ),
        constMeta: kPulseProtocolMediaVaultTabDefaultConstMeta,
        argValues: [],
        apiImpl: this,
      ),
    );
  }

  TaskConstMeta get kPulseProtocolMediaVaultTabDefaultConstMeta =>
      const TaskConstMeta(debugName: "vault_tab_default", argNames: []);

  @override
  Stream<ClientSnapshot> crateApiClientWatchSnapshots() {
    final sink = RustStreamSink<ClientSnapshot>();
    unawaited(
      handler.executeNormal(
        NormalTask(
          callFfi: (port_) {
            final serializer = SseSerializer(generalizedFrbRustBinding);
            sse_encode_StreamSink_client_snapshot_Sse(sink, serializer);
            pdeCallFfi(
              generalizedFrbRustBinding,
              serializer,
              funcId: 151,
              port: port_,
            );
          },
          codec: SseCodec(
            decodeSuccessData: sse_decode_unit,
            decodeErrorData: null,
          ),
          constMeta: kCrateApiClientWatchSnapshotsConstMeta,
          argValues: [sink],
          apiImpl: this,
        ),
      ),
    );
    return sink.stream;
  }

  TaskConstMeta get kCrateApiClientWatchSnapshotsConstMeta =>
      const TaskConstMeta(debugName: "watch_snapshots", argNames: ["sink"]);

  RustArcIncrementStrongCountFnType
  get rust_arc_increment_strong_count_StreamWriter => wire
      .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter;

  RustArcDecrementStrongCountFnType
  get rust_arc_decrement_strong_count_StreamWriter => wire
      .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw) {
    return AnyhowException(raw as String);
  }

  @protected
  StreamWriter
  dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    dynamic raw,
  ) {
    return StreamWriterImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  StreamWriter
  dco_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    dynamic raw,
  ) {
    return StreamWriterImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  StreamWriter
  dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    dynamic raw,
  ) {
    return StreamWriterImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  StreamWriter
  dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    dynamic raw,
  ) {
    return StreamWriterImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  RustStreamSink<ClientSnapshot> dco_decode_StreamSink_client_snapshot_Sse(
    dynamic raw,
  ) {
    throw UnimplementedError();
  }

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
  AddMemberRequest dco_decode_add_member_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return AddMemberRequest(
      email: dco_decode_String(arr[0]),
      role: dco_decode_role(arr[1]),
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
  AttachmentId dco_decode_attachment_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return AttachmentId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  AttachmentKind dco_decode_attachment_kind(dynamic raw) {
    return AttachmentKind.values[raw as int];
  }

  @protected
  AttachmentView dco_decode_attachment_view(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return AttachmentView(
      attachmentId: dco_decode_attachment_id(arr[0]),
      fileName: dco_decode_String(arr[1]),
      contentType: dco_decode_String(arr[2]),
      kind: dco_decode_attachment_kind(arr[3]),
      byteSize: dco_decode_u_64(arr[4]),
      url: dco_decode_opt_String(arr[5]),
    );
  }

  @protected
  AuthPhase dco_decode_auth_phase(dynamic raw) {
    return AuthPhase.values[raw as int];
  }

  @protected
  AuthToken dco_decode_auth_token(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return AuthToken(field0: dco_decode_String(arr[0]));
  }

  @protected
  BodySegment dco_decode_body_segment(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return BodySegment(
      kind: dco_decode_segment_kind(arr[0]),
      text: dco_decode_String(arr[1]),
      accountId: dco_decode_opt_box_autoadd_account_id(arr[2]),
      url: dco_decode_opt_String(arr[3]),
    );
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
  AddMemberRequest dco_decode_box_autoadd_add_member_request(dynamic raw) {
    return dco_decode_add_member_request(raw);
  }

  @protected
  ApiError dco_decode_box_autoadd_api_error(dynamic raw) {
    return dco_decode_api_error(raw);
  }

  @protected
  AttachmentId dco_decode_box_autoadd_attachment_id(dynamic raw) {
    return dco_decode_attachment_id(raw);
  }

  @protected
  AttachmentView dco_decode_box_autoadd_attachment_view(dynamic raw) {
    return dco_decode_attachment_view(raw);
  }

  @protected
  CampusId dco_decode_box_autoadd_campus_id(dynamic raw) {
    return dco_decode_campus_id(raw);
  }

  @protected
  CampusInvite dco_decode_box_autoadd_campus_invite(dynamic raw) {
    return dco_decode_campus_invite(raw);
  }

  @protected
  ClientNonce dco_decode_box_autoadd_client_nonce(dynamic raw) {
    return dco_decode_client_nonce(raw);
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
  CreateUploadRequest dco_decode_box_autoadd_create_upload_request(
    dynamic raw,
  ) {
    return dco_decode_create_upload_request(raw);
  }

  @protected
  DraftId dco_decode_box_autoadd_draft_id(dynamic raw) {
    return dco_decode_draft_id(raw);
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
  InviteCode dco_decode_box_autoadd_invite_code(dynamic raw) {
    return dco_decode_invite_code(raw);
  }

  @protected
  IssuedStewardToken dco_decode_box_autoadd_issued_steward_token(dynamic raw) {
    return dco_decode_issued_steward_token(raw);
  }

  @protected
  JoinCampusRequest dco_decode_box_autoadd_join_campus_request(dynamic raw) {
    return dco_decode_join_campus_request(raw);
  }

  @protected
  LoungeId dco_decode_box_autoadd_lounge_id(dynamic raw) {
    return dco_decode_lounge_id(raw);
  }

  @protected
  LoungeTicket dco_decode_box_autoadd_lounge_ticket(dynamic raw) {
    return dco_decode_lounge_ticket(raw);
  }

  @protected
  MessageId dco_decode_box_autoadd_message_id(dynamic raw) {
    return dco_decode_message_id(raw);
  }

  @protected
  MessageRejection dco_decode_box_autoadd_message_rejection(dynamic raw) {
    return dco_decode_message_rejection(raw);
  }

  @protected
  Note dco_decode_box_autoadd_note(dynamic raw) {
    return dco_decode_note(raw);
  }

  @protected
  React dco_decode_box_autoadd_react(dynamic raw) {
    return dco_decode_react(raw);
  }

  @protected
  ReminderPreset dco_decode_box_autoadd_reminder_preset(dynamic raw) {
    return dco_decode_reminder_preset(raw);
  }

  @protected
  ReplyPreview dco_decode_box_autoadd_reply_preview(dynamic raw) {
    return dco_decode_reply_preview(raw);
  }

  @protected
  RoomId dco_decode_box_autoadd_room_id(dynamic raw) {
    return dco_decode_room_id(raw);
  }

  @protected
  SendMessage dco_decode_box_autoadd_send_message(dynamic raw) {
    return dco_decode_send_message(raw);
  }

  @protected
  SessionId dco_decode_box_autoadd_session_id(dynamic raw) {
    return dco_decode_session_id(raw);
  }

  @protected
  SetTopicTitle dco_decode_box_autoadd_set_topic_title(dynamic raw) {
    return dco_decode_set_topic_title(raw);
  }

  @protected
  ShelfEntry dco_decode_box_autoadd_shelf_entry(dynamic raw) {
    return dco_decode_shelf_entry(raw);
  }

  @protected
  ShelfItemId dco_decode_box_autoadd_shelf_item_id(dynamic raw) {
    return dco_decode_shelf_item_id(raw);
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
  StartNoteRequest dco_decode_box_autoadd_start_note_request(dynamic raw) {
    return dco_decode_start_note_request(raw);
  }

  @protected
  StewardTool dco_decode_box_autoadd_steward_tool(dynamic raw) {
    return dco_decode_steward_tool(raw);
  }

  @protected
  StreamId dco_decode_box_autoadd_stream_id(dynamic raw) {
    return dco_decode_stream_id(raw);
  }

  @protected
  StreamState dco_decode_box_autoadd_stream_state(dynamic raw) {
    return dco_decode_stream_state(raw);
  }

  @protected
  SubscriptionState dco_decode_box_autoadd_subscription_state(dynamic raw) {
    return dco_decode_subscription_state(raw);
  }

  @protected
  ThreadPreview dco_decode_box_autoadd_thread_preview(dynamic raw) {
    return dco_decode_thread_preview(raw);
  }

  @protected
  TimelineEntry dco_decode_box_autoadd_timeline_entry(dynamic raw) {
    return dco_decode_timeline_entry(raw);
  }

  @protected
  ToolCallId dco_decode_box_autoadd_tool_call_id(dynamic raw) {
    return dco_decode_tool_call_id(raw);
  }

  @protected
  int dco_decode_box_autoadd_u_32(dynamic raw) {
    return raw as int;
  }

  @protected
  BigInt dco_decode_box_autoadd_u_64(dynamic raw) {
    return dco_decode_u_64(raw);
  }

  @protected
  UpdateMemberRequest dco_decode_box_autoadd_update_member_request(
    dynamic raw,
  ) {
    return dco_decode_update_member_request(raw);
  }

  @protected
  UpdateShelf dco_decode_box_autoadd_update_shelf(dynamic raw) {
    return dco_decode_update_shelf(raw);
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
  CampusInvite dco_decode_campus_invite(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return CampusInvite(
      campusId: dco_decode_campus_id(arr[0]),
      code: dco_decode_String(arr[1]),
      expiresAtMs: dco_decode_u_64(arr[2]),
    );
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
  ClientNonce dco_decode_client_nonce(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ClientNonce(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  ClientSnapshot dco_decode_client_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 11)
      throw Exception('unexpected arr length: expect 11 but see ${arr.length}');
    return ClientSnapshot(
      sequence: dco_decode_u_64(arr[0]),
      network: dco_decode_connection_snapshot(arr[1]),
      session: dco_decode_session_snapshot(arr[2]),
      timeline: dco_decode_timeline_snapshot(arr[3]),
      thread: dco_decode_thread_snapshot(arr[4]),
      inbox: dco_decode_inbox_snapshot(arr[5]),
      search: dco_decode_search_snapshot(arr[6]),
      vault: dco_decode_vault_snapshot(arr[7]),
      shelf: dco_decode_shelf_snapshot(arr[8]),
      lounges: dco_decode_lounge_snapshot(arr[9]),
      stewards: dco_decode_steward_snapshot(arr[10]),
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
  CreateUploadRequest dco_decode_create_upload_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return CreateUploadRequest(
      fileName: dco_decode_String(arr[0]),
      contentType: dco_decode_String(arr[1]),
      byteSize: dco_decode_u_64(arr[2]),
    );
  }

  @protected
  DeliveryState dco_decode_delivery_state(dynamic raw) {
    return DeliveryState.values[raw as int];
  }

  @protected
  DevAccount dco_decode_dev_account(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return DevAccount(
      email: dco_decode_String(arr[0]),
      displayName: dco_decode_String(arr[1]),
      role: dco_decode_role(arr[2]),
    );
  }

  @protected
  DraftAttachmentView dco_decode_draft_attachment_view(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return DraftAttachmentView(
      draftId: dco_decode_draft_id(arr[0]),
      roomId: dco_decode_room_id(arr[1]),
      fileName: dco_decode_String(arr[2]),
      kind: dco_decode_attachment_kind(arr[3]),
      byteSize: dco_decode_u_64(arr[4]),
      upload: dco_decode_upload_state(arr[5]),
    );
  }

  @protected
  DraftId dco_decode_draft_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return DraftId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  HandshakeRejection dco_decode_handshake_rejection(dynamic raw) {
    return HandshakeRejection.values[raw as int];
  }

  @protected
  HighlightSpan dco_decode_highlight_span(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return HighlightSpan(
      text: dco_decode_String(arr[0]),
      matched: dco_decode_bool(arr[1]),
    );
  }

  @protected
  int dco_decode_i_32(dynamic raw) {
    return raw as int;
  }

  @protected
  InboxItem dco_decode_inbox_item(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 12)
      throw Exception('unexpected arr length: expect 12 but see ${arr.length}');
    return InboxItem(
      kind: dco_decode_inbox_item_kind(arr[0]),
      roomId: dco_decode_room_id(arr[1]),
      rootMessageId: dco_decode_message_id(arr[2]),
      topicTitle: dco_decode_opt_String(arr[3]),
      rootAuthorDisplayName: dco_decode_String(arr[4]),
      rootPreview: dco_decode_String(arr[5]),
      latestReply: dco_decode_opt_box_autoadd_reply_preview(arr[6]),
      replyCount: dco_decode_u_32(arr[7]),
      unreadReplies: dco_decode_u_32(arr[8]),
      unreadMentions: dco_decode_u_32(arr[9]),
      subscription: dco_decode_subscription_state(arr[10]),
      activityMessageId: dco_decode_message_id(arr[11]),
    );
  }

  @protected
  InboxItemKind dco_decode_inbox_item_kind(dynamic raw) {
    return InboxItemKind.values[raw as int];
  }

  @protected
  InboxSnapshot dco_decode_inbox_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 5)
      throw Exception('unexpected arr length: expect 5 but see ${arr.length}');
    return InboxSnapshot(
      open: dco_decode_bool(arr[0]),
      items: dco_decode_list_inbox_item(arr[1]),
      unreadThreads: dco_decode_u_32(arr[2]),
      selectedRootMessageId: dco_decode_opt_box_autoadd_message_id(arr[3]),
      replying: dco_decode_bool(arr[4]),
    );
  }

  @protected
  InboxStep dco_decode_inbox_step(dynamic raw) {
    return InboxStep.values[raw as int];
  }

  @protected
  InputField dco_decode_input_field(dynamic raw) {
    return InputField.values[raw as int];
  }

  @protected
  InviteCode dco_decode_invite_code(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return InviteCode(
      campusId: dco_decode_campus_id(arr[0]),
      secret: dco_decode_String(arr[1]),
    );
  }

  @protected
  IssuedStewardToken dco_decode_issued_steward_token(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return IssuedStewardToken(
      stewardId: dco_decode_account_id(arr[0]),
      displayName: dco_decode_String(arr[1]),
      token: dco_decode_auth_token(arr[2]),
    );
  }

  @protected
  JoinCampusRequest dco_decode_join_campus_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return JoinCampusRequest(code: dco_decode_String(arr[0]));
  }

  @protected
  List<String> dco_decode_list_String(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_String).toList();
  }

  @protected
  List<AccountId> dco_decode_list_account_id(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_account_id).toList();
  }

  @protected
  List<AttachmentId> dco_decode_list_attachment_id(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_attachment_id).toList();
  }

  @protected
  List<AttachmentView> dco_decode_list_attachment_view(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_attachment_view).toList();
  }

  @protected
  List<BodySegment> dco_decode_list_body_segment(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_body_segment).toList();
  }

  @protected
  List<CampusSummary> dco_decode_list_campus_summary(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_campus_summary).toList();
  }

  @protected
  List<DevAccount> dco_decode_list_dev_account(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_dev_account).toList();
  }

  @protected
  List<DraftAttachmentView> dco_decode_list_draft_attachment_view(dynamic raw) {
    return (raw as List<dynamic>)
        .map(dco_decode_draft_attachment_view)
        .toList();
  }

  @protected
  List<HighlightSpan> dco_decode_list_highlight_span(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_highlight_span).toList();
  }

  @protected
  List<InboxItem> dco_decode_list_inbox_item(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_inbox_item).toList();
  }

  @protected
  List<LoungeEntry> dco_decode_list_lounge_entry(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_lounge_entry).toList();
  }

  @protected
  List<LoungeParticipantView> dco_decode_list_lounge_participant_view(
    dynamic raw,
  ) {
    return (raw as List<dynamic>)
        .map(dco_decode_lounge_participant_view)
        .toList();
  }

  @protected
  List<MemberEntry> dco_decode_list_member_entry(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_member_entry).toList();
  }

  @protected
  List<MentionRef> dco_decode_list_mention_ref(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_mention_ref).toList();
  }

  @protected
  List<NoteEntry> dco_decode_list_note_entry(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_note_entry).toList();
  }

  @protected
  List<NoteMember> dco_decode_list_note_member(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_note_member).toList();
  }

  @protected
  List<int> dco_decode_list_prim_u_8_loose(dynamic raw) {
    return raw as List<int>;
  }

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw) {
    return raw as Uint8List;
  }

  @protected
  List<ReactionSummary> dco_decode_list_reaction_summary(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_reaction_summary).toList();
  }

  @protected
  List<ReminderPreset> dco_decode_list_reminder_preset(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_reminder_preset).toList();
  }

  @protected
  List<Role> dco_decode_list_role(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_role).toList();
  }

  @protected
  List<Room> dco_decode_list_room(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_room).toList();
  }

  @protected
  List<RoomId> dco_decode_list_room_id(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_room_id).toList();
  }

  @protected
  List<SearchHit> dco_decode_list_search_hit(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_search_hit).toList();
  }

  @protected
  List<ShelfEntry> dco_decode_list_shelf_entry(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_shelf_entry).toList();
  }

  @protected
  List<StewardEntry> dco_decode_list_steward_entry(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_steward_entry).toList();
  }

  @protected
  List<TimelineEntry> dco_decode_list_timeline_entry(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_timeline_entry).toList();
  }

  @protected
  List<ToolDescriptor> dco_decode_list_tool_descriptor(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_tool_descriptor).toList();
  }

  @protected
  List<VaultEntry> dco_decode_list_vault_entry(dynamic raw) {
    return (raw as List<dynamic>).map(dco_decode_vault_entry).toList();
  }

  @protected
  LoungeEntry dco_decode_lounge_entry(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return LoungeEntry(
      loungeId: dco_decode_lounge_id(arr[0]),
      scope: dco_decode_lounge_scope(arr[1]),
      roomId: dco_decode_opt_box_autoadd_room_id(arr[2]),
      roomSlug: dco_decode_opt_String(arr[3]),
      participants: dco_decode_list_lounge_participant_view(arr[4]),
      joined: dco_decode_bool(arr[5]),
    );
  }

  @protected
  LoungeId dco_decode_lounge_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return LoungeId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  LoungeMediaState dco_decode_lounge_media_state(dynamic raw) {
    return LoungeMediaState.values[raw as int];
  }

  @protected
  LoungeParticipantView dco_decode_lounge_participant_view(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return LoungeParticipantView(
      accountId: dco_decode_account_id(arr[0]),
      displayName: dco_decode_String(arr[1]),
      fromSelf: dco_decode_bool(arr[2]),
      canSpeak: dco_decode_bool(arr[3]),
    );
  }

  @protected
  LoungeScope dco_decode_lounge_scope(dynamic raw) {
    return LoungeScope.values[raw as int];
  }

  @protected
  LoungeSnapshot dco_decode_lounge_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 8)
      throw Exception('unexpected arr length: expect 8 but see ${arr.length}');
    return LoungeSnapshot(
      lounges: dco_decode_list_lounge_entry(arr[0]),
      joinedLoungeId: dco_decode_opt_box_autoadd_lounge_id(arr[1]),
      joining: dco_decode_bool(arr[2]),
      ticket: dco_decode_opt_box_autoadd_lounge_ticket(arr[3]),
      media: dco_decode_lounge_media_state(arr[4]),
      canJoin: dco_decode_bool(arr[5]),
      canSpeak: dco_decode_bool(arr[6]),
      error: dco_decode_opt_box_autoadd_api_error(arr[7]),
    );
  }

  @protected
  LoungeTicket dco_decode_lounge_ticket(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 5)
      throw Exception('unexpected arr length: expect 5 but see ${arr.length}');
    return LoungeTicket(
      loungeId: dco_decode_lounge_id(arr[0]),
      livekitUrl: dco_decode_String(arr[1]),
      roomName: dco_decode_String(arr[2]),
      token: dco_decode_String(arr[3]),
      expiresAtMs: dco_decode_u_64(arr[4]),
    );
  }

  @protected
  MemberEntry dco_decode_member_entry(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return MemberEntry(
      accountId: dco_decode_account_id(arr[0]),
      displayName: dco_decode_String(arr[1]),
      email: dco_decode_String(arr[2]),
      role: dco_decode_role(arr[3]),
      manageable: dco_decode_bool(arr[4]),
      isSelf: dco_decode_bool(arr[5]),
    );
  }

  @protected
  MentionRef dco_decode_mention_ref(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return MentionRef(
      accountId: dco_decode_account_id(arr[0]),
      displayName: dco_decode_String(arr[1]),
    );
  }

  @protected
  MessageId dco_decode_message_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return MessageId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  MessageRejection dco_decode_message_rejection(dynamic raw) {
    return MessageRejection.values[raw as int];
  }

  @protected
  Note dco_decode_note(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return Note(
      campusId: dco_decode_campus_id(arr[0]),
      roomId: dco_decode_room_id(arr[1]),
      members: dco_decode_list_note_member(arr[2]),
    );
  }

  @protected
  NoteEntry dco_decode_note_entry(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return NoteEntry(
      roomId: dco_decode_room_id(arr[0]),
      title: dco_decode_String(arr[1]),
      memberCount: dco_decode_u_32(arr[2]),
    );
  }

  @protected
  NoteMember dco_decode_note_member(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return NoteMember(
      accountId: dco_decode_account_id(arr[0]),
      displayName: dco_decode_String(arr[1]),
    );
  }

  @protected
  String? dco_decode_opt_String(dynamic raw) {
    return raw == null ? null : dco_decode_String(raw);
  }

  @protected
  AccountId? dco_decode_opt_box_autoadd_account_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_account_id(raw);
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
  AttachmentView? dco_decode_opt_box_autoadd_attachment_view(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_attachment_view(raw);
  }

  @protected
  CampusId? dco_decode_opt_box_autoadd_campus_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_campus_id(raw);
  }

  @protected
  CampusInvite? dco_decode_opt_box_autoadd_campus_invite(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_campus_invite(raw);
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
  IssuedStewardToken? dco_decode_opt_box_autoadd_issued_steward_token(
    dynamic raw,
  ) {
    return raw == null
        ? null
        : dco_decode_box_autoadd_issued_steward_token(raw);
  }

  @protected
  LoungeId? dco_decode_opt_box_autoadd_lounge_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_lounge_id(raw);
  }

  @protected
  LoungeTicket? dco_decode_opt_box_autoadd_lounge_ticket(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_lounge_ticket(raw);
  }

  @protected
  MessageId? dco_decode_opt_box_autoadd_message_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_message_id(raw);
  }

  @protected
  MessageRejection? dco_decode_opt_box_autoadd_message_rejection(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_message_rejection(raw);
  }

  @protected
  ReminderPreset? dco_decode_opt_box_autoadd_reminder_preset(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_reminder_preset(raw);
  }

  @protected
  ReplyPreview? dco_decode_opt_box_autoadd_reply_preview(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_reply_preview(raw);
  }

  @protected
  RoomId? dco_decode_opt_box_autoadd_room_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_room_id(raw);
  }

  @protected
  SessionId? dco_decode_opt_box_autoadd_session_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_session_id(raw);
  }

  @protected
  ShelfEntry? dco_decode_opt_box_autoadd_shelf_entry(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_shelf_entry(raw);
  }

  @protected
  ShelfItemId? dco_decode_opt_box_autoadd_shelf_item_id(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_shelf_item_id(raw);
  }

  @protected
  StewardTool? dco_decode_opt_box_autoadd_steward_tool(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_steward_tool(raw);
  }

  @protected
  StreamState? dco_decode_opt_box_autoadd_stream_state(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_stream_state(raw);
  }

  @protected
  SubscriptionState? dco_decode_opt_box_autoadd_subscription_state(
    dynamic raw,
  ) {
    return raw == null ? null : dco_decode_box_autoadd_subscription_state(raw);
  }

  @protected
  ThreadPreview? dco_decode_opt_box_autoadd_thread_preview(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_thread_preview(raw);
  }

  @protected
  TimelineEntry? dco_decode_opt_box_autoadd_timeline_entry(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_timeline_entry(raw);
  }

  @protected
  int? dco_decode_opt_box_autoadd_u_32(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_u_32(raw);
  }

  @protected
  BigInt? dco_decode_opt_box_autoadd_u_64(dynamic raw) {
    return raw == null ? null : dco_decode_box_autoadd_u_64(raw);
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
  React dco_decode_react(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return React(
      roomId: dco_decode_room_id(arr[0]),
      messageId: dco_decode_message_id(arr[1]),
      emoji: dco_decode_String(arr[2]),
      action: dco_decode_reaction_action(arr[3]),
    );
  }

  @protected
  ReactionAction dco_decode_reaction_action(dynamic raw) {
    return ReactionAction.values[raw as int];
  }

  @protected
  ReactionSummary dco_decode_reaction_summary(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return ReactionSummary(
      emoji: dco_decode_String(arr[0]),
      count: dco_decode_u_32(arr[1]),
      reactedBySelf: dco_decode_bool(arr[2]),
    );
  }

  @protected
  (String, List<AttachmentId>) dco_decode_record_string_list_attachment_id(
    dynamic raw,
  ) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2) {
      throw Exception('Expected 2 elements, got ${arr.length}');
    }
    return (dco_decode_String(arr[0]), dco_decode_list_attachment_id(arr[1]));
  }

  @protected
  ReminderPreset dco_decode_reminder_preset(dynamic raw) {
    return ReminderPreset.values[raw as int];
  }

  @protected
  ReplyPreview dco_decode_reply_preview(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return ReplyPreview(
      messageId: dco_decode_message_id(arr[0]),
      authorId: dco_decode_account_id(arr[1]),
      authorDisplayName: dco_decode_String(arr[2]),
      preview: dco_decode_String(arr[3]),
    );
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
  SearchHit dco_decode_search_hit(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 7)
      throw Exception('unexpected arr length: expect 7 but see ${arr.length}');
    return SearchHit(
      roomId: dco_decode_room_id(arr[0]),
      messageId: dco_decode_message_id(arr[1]),
      threadRootId: dco_decode_opt_box_autoadd_message_id(arr[2]),
      authorDisplayName: dco_decode_String(arr[3]),
      topicTitle: dco_decode_opt_String(arr[4]),
      snippet: dco_decode_list_highlight_span(arr[5]),
      score: dco_decode_u_32(arr[6]),
    );
  }

  @protected
  SearchSnapshot dco_decode_search_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return SearchSnapshot(
      open: dco_decode_bool(arr[0]),
      query: dco_decode_String(arr[1]),
      hits: dco_decode_list_search_hit(arr[2]),
      selectedIndex: dco_decode_opt_box_autoadd_u_32(arr[3]),
      searching: dco_decode_bool(arr[4]),
      error: dco_decode_opt_box_autoadd_api_error(arr[5]),
    );
  }

  @protected
  SegmentKind dco_decode_segment_kind(dynamic raw) {
    return SegmentKind.values[raw as int];
  }

  @protected
  SendMessage dco_decode_send_message(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 5)
      throw Exception('unexpected arr length: expect 5 but see ${arr.length}');
    return SendMessage(
      roomId: dco_decode_room_id(arr[0]),
      clientNonce: dco_decode_client_nonce(arr[1]),
      body: dco_decode_String(arr[2]),
      threadRootId: dco_decode_opt_box_autoadd_message_id(arr[3]),
      attachmentIds: dco_decode_list_attachment_id(arr[4]),
    );
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
    if (arr.length != 14)
      throw Exception('unexpected arr length: expect 14 but see ${arr.length}');
    return SessionSnapshot(
      auth: dco_decode_auth_phase(arr[0]),
      account: dco_decode_opt_box_autoadd_account_profile(arr[1]),
      campuses: dco_decode_list_campus_summary(arr[2]),
      activeCampusId: dco_decode_opt_box_autoadd_campus_id(arr[3]),
      rooms: dco_decode_list_room(arr[4]),
      activeRoomId: dco_decode_opt_box_autoadd_room_id(arr[5]),
      invite: dco_decode_opt_box_autoadd_campus_invite(arr[6]),
      canInvite: dco_decode_bool(arr[7]),
      members: dco_decode_list_member_entry(arr[8]),
      assignableRoles: dco_decode_list_role(arr[9]),
      quickAccounts: dco_decode_list_dev_account(arr[10]),
      notes: dco_decode_list_note_entry(arr[11]),
      busy: dco_decode_bool(arr[12]),
      error: dco_decode_opt_box_autoadd_api_error(arr[13]),
    );
  }

  @protected
  SetTopicTitle dco_decode_set_topic_title(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return SetTopicTitle(
      roomId: dco_decode_room_id(arr[0]),
      rootMessageId: dco_decode_message_id(arr[1]),
      topicTitle: dco_decode_String(arr[2]),
    );
  }

  @protected
  ShelfAction dco_decode_shelf_action(dynamic raw) {
    return ShelfAction.values[raw as int];
  }

  @protected
  ShelfBucket dco_decode_shelf_bucket(dynamic raw) {
    return ShelfBucket.values[raw as int];
  }

  @protected
  ShelfEntry dco_decode_shelf_entry(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 9)
      throw Exception('unexpected arr length: expect 9 but see ${arr.length}');
    return ShelfEntry(
      shelfItemId: dco_decode_opt_box_autoadd_shelf_item_id(arr[0]),
      roomId: dco_decode_room_id(arr[1]),
      messageId: dco_decode_message_id(arr[2]),
      authorDisplayName: dco_decode_String(arr[3]),
      preview: dco_decode_String(arr[4]),
      remindAtMs: dco_decode_opt_box_autoadd_u_64(arr[5]),
      doneAtMs: dco_decode_opt_box_autoadd_u_64(arr[6]),
      bucket: dco_decode_shelf_bucket(arr[7]),
      pending: dco_decode_bool(arr[8]),
    );
  }

  @protected
  ShelfItemId dco_decode_shelf_item_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ShelfItemId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  ShelfSnapshot dco_decode_shelf_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 10)
      throw Exception('unexpected arr length: expect 10 but see ${arr.length}');
    return ShelfSnapshot(
      open: dco_decode_bool(arr[0]),
      due: dco_decode_list_shelf_entry(arr[1]),
      upcoming: dco_decode_list_shelf_entry(arr[2]),
      done: dco_decode_list_shelf_entry(arr[3]),
      dueCount: dco_decode_u_32(arr[4]),
      latestReminder: dco_decode_opt_box_autoadd_shelf_entry(arr[5]),
      presets: dco_decode_list_reminder_preset(arr[6]),
      pickerAnchorMs: dco_decode_opt_box_autoadd_u_64(arr[7]),
      pickerLatestMs: dco_decode_opt_box_autoadd_u_64(arr[8]),
      error: dco_decode_opt_box_autoadd_api_error(arr[9]),
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
  StartNoteRequest dco_decode_start_note_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return StartNoteRequest(accountIds: dco_decode_list_account_id(arr[0]));
  }

  @protected
  StewardEntry dco_decode_steward_entry(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return StewardEntry(
      stewardId: dco_decode_account_id(arr[0]),
      displayName: dco_decode_String(arr[1]),
      mentionToken: dco_decode_String(arr[2]),
      roomIds: dco_decode_list_room_id(arr[3]),
      revoked: dco_decode_bool(arr[4]),
      manageable: dco_decode_bool(arr[5]),
    );
  }

  @protected
  StewardSnapshot dco_decode_steward_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return StewardSnapshot(
      open: dco_decode_bool(arr[0]),
      stewards: dco_decode_list_steward_entry(arr[1]),
      canManage: dco_decode_bool(arr[2]),
      issued: dco_decode_opt_box_autoadd_issued_steward_token(arr[3]),
      pending: dco_decode_bool(arr[4]),
      error: dco_decode_opt_box_autoadd_api_error(arr[5]),
    );
  }

  @protected
  StewardTool dco_decode_steward_tool(dynamic raw) {
    return StewardTool.values[raw as int];
  }

  @protected
  StreamChunk dco_decode_stream_chunk(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return StreamChunk(
      streamId: dco_decode_stream_id(arr[0]),
      roomId: dco_decode_room_id(arr[1]),
      threadRootId: dco_decode_opt_box_autoadd_message_id(arr[2]),
      phase: dco_decode_stream_phase(arr[3]),
      sequence: dco_decode_u_32(arr[4]),
      text: dco_decode_String(arr[5]),
    );
  }

  @protected
  StreamId dco_decode_stream_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return StreamId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  StreamPhase dco_decode_stream_phase(dynamic raw) {
    return StreamPhase.values[raw as int];
  }

  @protected
  StreamState dco_decode_stream_state(dynamic raw) {
    return StreamState.values[raw as int];
  }

  @protected
  SubscriptionState dco_decode_subscription_state(dynamic raw) {
    return SubscriptionState.values[raw as int];
  }

  @protected
  ThreadPreview dco_decode_thread_preview(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 6)
      throw Exception('unexpected arr length: expect 6 but see ${arr.length}');
    return ThreadPreview(
      topicTitle: dco_decode_opt_String(arr[0]),
      replyCount: dco_decode_u_32(arr[1]),
      latestReply: dco_decode_opt_box_autoadd_reply_preview(arr[2]),
      showLatestReply: dco_decode_bool(arr[3]),
      subscription: dco_decode_opt_box_autoadd_subscription_state(arr[4]),
      unreadReplies: dco_decode_u_32(arr[5]),
    );
  }

  @protected
  ThreadSnapshot dco_decode_thread_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 8)
      throw Exception('unexpected arr length: expect 8 but see ${arr.length}');
    return ThreadSnapshot(
      roomId: dco_decode_opt_box_autoadd_room_id(arr[0]),
      rootMessageId: dco_decode_opt_box_autoadd_message_id(arr[1]),
      root: dco_decode_opt_box_autoadd_timeline_entry(arr[2]),
      replies: dco_decode_list_timeline_entry(arr[3]),
      topicTitle: dco_decode_opt_String(arr[4]),
      subscription: dco_decode_opt_box_autoadd_subscription_state(arr[5]),
      syncing: dco_decode_bool(arr[6]),
      mentionableStewards: dco_decode_list_steward_entry(arr[7]),
    );
  }

  @protected
  TimelineEntry dco_decode_timeline_entry(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 19)
      throw Exception('unexpected arr length: expect 19 but see ${arr.length}');
    return TimelineEntry(
      clientNonce: dco_decode_client_nonce(arr[0]),
      messageId: dco_decode_opt_box_autoadd_message_id(arr[1]),
      authorId: dco_decode_account_id(arr[2]),
      authorDisplayName: dco_decode_String(arr[3]),
      authorMentionToken: dco_decode_String(arr[4]),
      fromSelf: dco_decode_bool(arr[5]),
      body: dco_decode_String(arr[6]),
      segments: dco_decode_list_body_segment(arr[7]),
      threadRootId: dco_decode_opt_box_autoadd_message_id(arr[8]),
      reactions: dco_decode_list_reaction_summary(arr[9]),
      thread: dco_decode_opt_box_autoadd_thread_preview(arr[10]),
      delivery: dco_decode_delivery_state(arr[11]),
      rejection: dco_decode_opt_box_autoadd_message_rejection(arr[12]),
      attachments: dco_decode_list_attachment_view(arr[13]),
      pendingDrafts: dco_decode_list_draft_attachment_view(arr[14]),
      pinned: dco_decode_bool(arr[15]),
      shelved: dco_decode_bool(arr[16]),
      authorIsSteward: dco_decode_bool(arr[17]),
      stream: dco_decode_opt_box_autoadd_stream_state(arr[18]),
    );
  }

  @protected
  TimelineSnapshot dco_decode_timeline_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 10)
      throw Exception('unexpected arr length: expect 10 but see ${arr.length}');
    return TimelineSnapshot(
      roomId: dco_decode_opt_box_autoadd_room_id(arr[0]),
      entries: dco_decode_list_timeline_entry(arr[1]),
      syncing: dco_decode_bool(arr[2]),
      error: dco_decode_opt_box_autoadd_api_error(arr[3]),
      reactionPalette: dco_decode_list_String(arr[4]),
      drafts: dco_decode_list_draft_attachment_view(arr[5]),
      focusMessageId: dco_decode_opt_box_autoadd_message_id(arr[6]),
      canPin: dco_decode_bool(arr[7]),
      canUpload: dco_decode_bool(arr[8]),
      mentionableStewards: dco_decode_list_steward_entry(arr[9]),
    );
  }

  @protected
  ToolCallId dco_decode_tool_call_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ToolCallId(field0: dco_decode_Uuid(arr[0]));
  }

  @protected
  ToolDescriptor dco_decode_tool_descriptor(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return ToolDescriptor(
      name: dco_decode_String(arr[0]),
      description: dco_decode_String(arr[1]),
      inputSchema: dco_decode_String(arr[2]),
    );
  }

  @protected
  ToolErrorCode dco_decode_tool_error_code(dynamic raw) {
    return ToolErrorCode.values[raw as int];
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
  BigInt dco_decode_u_64(dynamic raw) {
    return dcoDecodeU64(raw);
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
  UpdateMemberRequest dco_decode_update_member_request(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return UpdateMemberRequest(role: dco_decode_role(arr[0]));
  }

  @protected
  UpdateShelf dco_decode_update_shelf(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return UpdateShelf(
      roomId: dco_decode_room_id(arr[0]),
      messageId: dco_decode_message_id(arr[1]),
      action: dco_decode_shelf_action(arr[2]),
      remindAtMs: dco_decode_opt_box_autoadd_u_64(arr[3]),
    );
  }

  @protected
  UploadState dco_decode_upload_state(dynamic raw) {
    return UploadState.values[raw as int];
  }

  @protected
  BigInt dco_decode_usize(dynamic raw) {
    return dcoDecodeU64(raw);
  }

  @protected
  VaultEntry dco_decode_vault_entry(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 8)
      throw Exception('unexpected arr length: expect 8 but see ${arr.length}');
    return VaultEntry(
      tab: dco_decode_vault_tab(arr[0]),
      roomId: dco_decode_room_id(arr[1]),
      messageId: dco_decode_message_id(arr[2]),
      authorDisplayName: dco_decode_String(arr[3]),
      preview: dco_decode_String(arr[4]),
      attachment: dco_decode_opt_box_autoadd_attachment_view(arr[5]),
      link: dco_decode_opt_String(arr[6]),
      pinnedAtMs: dco_decode_opt_box_autoadd_u_64(arr[7]),
    );
  }

  @protected
  VaultSnapshot dco_decode_vault_snapshot(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 7)
      throw Exception('unexpected arr length: expect 7 but see ${arr.length}');
    return VaultSnapshot(
      open: dco_decode_bool(arr[0]),
      roomId: dco_decode_opt_box_autoadd_room_id(arr[1]),
      tab: dco_decode_vault_tab(arr[2]),
      entries: dco_decode_list_vault_entry(arr[3]),
      loading: dco_decode_bool(arr[4]),
      hasMore: dco_decode_bool(arr[5]),
      error: dco_decode_opt_box_autoadd_api_error(arr[6]),
    );
  }

  @protected
  VaultTab dco_decode_vault_tab(dynamic raw) {
    return VaultTab.values[raw as int];
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
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer) {
    var inner = sse_decode_String(deserializer);
    return AnyhowException(inner);
  }

  @protected
  StreamWriter
  sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    SseDeserializer deserializer,
  ) {
    return StreamWriterImpl.frbInternalSseDecode(
      sse_decode_usize(deserializer),
      sse_decode_i_32(deserializer),
    );
  }

  @protected
  StreamWriter
  sse_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    SseDeserializer deserializer,
  ) {
    return StreamWriterImpl.frbInternalSseDecode(
      sse_decode_usize(deserializer),
      sse_decode_i_32(deserializer),
    );
  }

  @protected
  StreamWriter
  sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    SseDeserializer deserializer,
  ) {
    return StreamWriterImpl.frbInternalSseDecode(
      sse_decode_usize(deserializer),
      sse_decode_i_32(deserializer),
    );
  }

  @protected
  StreamWriter
  sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    SseDeserializer deserializer,
  ) {
    return StreamWriterImpl.frbInternalSseDecode(
      sse_decode_usize(deserializer),
      sse_decode_i_32(deserializer),
    );
  }

  @protected
  RustStreamSink<ClientSnapshot> sse_decode_StreamSink_client_snapshot_Sse(
    SseDeserializer deserializer,
  ) {
    throw UnimplementedError('Unreachable ()');
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
  AddMemberRequest sse_decode_add_member_request(SseDeserializer deserializer) {
    var var_email = sse_decode_String(deserializer);
    var var_role = sse_decode_role(deserializer);
    return AddMemberRequest(email: var_email, role: var_role);
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
  AttachmentId sse_decode_attachment_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return AttachmentId(field0: var_field0);
  }

  @protected
  AttachmentKind sse_decode_attachment_kind(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return AttachmentKind.values[inner];
  }

  @protected
  AttachmentView sse_decode_attachment_view(SseDeserializer deserializer) {
    var var_attachmentId = sse_decode_attachment_id(deserializer);
    var var_fileName = sse_decode_String(deserializer);
    var var_contentType = sse_decode_String(deserializer);
    var var_kind = sse_decode_attachment_kind(deserializer);
    var var_byteSize = sse_decode_u_64(deserializer);
    var var_url = sse_decode_opt_String(deserializer);
    return AttachmentView(
      attachmentId: var_attachmentId,
      fileName: var_fileName,
      contentType: var_contentType,
      kind: var_kind,
      byteSize: var_byteSize,
      url: var_url,
    );
  }

  @protected
  AuthPhase sse_decode_auth_phase(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return AuthPhase.values[inner];
  }

  @protected
  AuthToken sse_decode_auth_token(SseDeserializer deserializer) {
    var var_field0 = sse_decode_String(deserializer);
    return AuthToken(field0: var_field0);
  }

  @protected
  BodySegment sse_decode_body_segment(SseDeserializer deserializer) {
    var var_kind = sse_decode_segment_kind(deserializer);
    var var_text = sse_decode_String(deserializer);
    var var_accountId = sse_decode_opt_box_autoadd_account_id(deserializer);
    var var_url = sse_decode_opt_String(deserializer);
    return BodySegment(
      kind: var_kind,
      text: var_text,
      accountId: var_accountId,
      url: var_url,
    );
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
  AddMemberRequest sse_decode_box_autoadd_add_member_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_add_member_request(deserializer));
  }

  @protected
  ApiError sse_decode_box_autoadd_api_error(SseDeserializer deserializer) {
    return (sse_decode_api_error(deserializer));
  }

  @protected
  AttachmentId sse_decode_box_autoadd_attachment_id(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_attachment_id(deserializer));
  }

  @protected
  AttachmentView sse_decode_box_autoadd_attachment_view(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_attachment_view(deserializer));
  }

  @protected
  CampusId sse_decode_box_autoadd_campus_id(SseDeserializer deserializer) {
    return (sse_decode_campus_id(deserializer));
  }

  @protected
  CampusInvite sse_decode_box_autoadd_campus_invite(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_campus_invite(deserializer));
  }

  @protected
  ClientNonce sse_decode_box_autoadd_client_nonce(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_client_nonce(deserializer));
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
  CreateUploadRequest sse_decode_box_autoadd_create_upload_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_create_upload_request(deserializer));
  }

  @protected
  DraftId sse_decode_box_autoadd_draft_id(SseDeserializer deserializer) {
    return (sse_decode_draft_id(deserializer));
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
  InviteCode sse_decode_box_autoadd_invite_code(SseDeserializer deserializer) {
    return (sse_decode_invite_code(deserializer));
  }

  @protected
  IssuedStewardToken sse_decode_box_autoadd_issued_steward_token(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_issued_steward_token(deserializer));
  }

  @protected
  JoinCampusRequest sse_decode_box_autoadd_join_campus_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_join_campus_request(deserializer));
  }

  @protected
  LoungeId sse_decode_box_autoadd_lounge_id(SseDeserializer deserializer) {
    return (sse_decode_lounge_id(deserializer));
  }

  @protected
  LoungeTicket sse_decode_box_autoadd_lounge_ticket(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_lounge_ticket(deserializer));
  }

  @protected
  MessageId sse_decode_box_autoadd_message_id(SseDeserializer deserializer) {
    return (sse_decode_message_id(deserializer));
  }

  @protected
  MessageRejection sse_decode_box_autoadd_message_rejection(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_message_rejection(deserializer));
  }

  @protected
  Note sse_decode_box_autoadd_note(SseDeserializer deserializer) {
    return (sse_decode_note(deserializer));
  }

  @protected
  React sse_decode_box_autoadd_react(SseDeserializer deserializer) {
    return (sse_decode_react(deserializer));
  }

  @protected
  ReminderPreset sse_decode_box_autoadd_reminder_preset(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_reminder_preset(deserializer));
  }

  @protected
  ReplyPreview sse_decode_box_autoadd_reply_preview(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_reply_preview(deserializer));
  }

  @protected
  RoomId sse_decode_box_autoadd_room_id(SseDeserializer deserializer) {
    return (sse_decode_room_id(deserializer));
  }

  @protected
  SendMessage sse_decode_box_autoadd_send_message(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_send_message(deserializer));
  }

  @protected
  SessionId sse_decode_box_autoadd_session_id(SseDeserializer deserializer) {
    return (sse_decode_session_id(deserializer));
  }

  @protected
  SetTopicTitle sse_decode_box_autoadd_set_topic_title(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_set_topic_title(deserializer));
  }

  @protected
  ShelfEntry sse_decode_box_autoadd_shelf_entry(SseDeserializer deserializer) {
    return (sse_decode_shelf_entry(deserializer));
  }

  @protected
  ShelfItemId sse_decode_box_autoadd_shelf_item_id(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_shelf_item_id(deserializer));
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
  StartNoteRequest sse_decode_box_autoadd_start_note_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_start_note_request(deserializer));
  }

  @protected
  StewardTool sse_decode_box_autoadd_steward_tool(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_steward_tool(deserializer));
  }

  @protected
  StreamId sse_decode_box_autoadd_stream_id(SseDeserializer deserializer) {
    return (sse_decode_stream_id(deserializer));
  }

  @protected
  StreamState sse_decode_box_autoadd_stream_state(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_stream_state(deserializer));
  }

  @protected
  SubscriptionState sse_decode_box_autoadd_subscription_state(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_subscription_state(deserializer));
  }

  @protected
  ThreadPreview sse_decode_box_autoadd_thread_preview(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_thread_preview(deserializer));
  }

  @protected
  TimelineEntry sse_decode_box_autoadd_timeline_entry(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_timeline_entry(deserializer));
  }

  @protected
  ToolCallId sse_decode_box_autoadd_tool_call_id(SseDeserializer deserializer) {
    return (sse_decode_tool_call_id(deserializer));
  }

  @protected
  int sse_decode_box_autoadd_u_32(SseDeserializer deserializer) {
    return (sse_decode_u_32(deserializer));
  }

  @protected
  BigInt sse_decode_box_autoadd_u_64(SseDeserializer deserializer) {
    return (sse_decode_u_64(deserializer));
  }

  @protected
  UpdateMemberRequest sse_decode_box_autoadd_update_member_request(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_update_member_request(deserializer));
  }

  @protected
  UpdateShelf sse_decode_box_autoadd_update_shelf(
    SseDeserializer deserializer,
  ) {
    return (sse_decode_update_shelf(deserializer));
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
  CampusInvite sse_decode_campus_invite(SseDeserializer deserializer) {
    var var_campusId = sse_decode_campus_id(deserializer);
    var var_code = sse_decode_String(deserializer);
    var var_expiresAtMs = sse_decode_u_64(deserializer);
    return CampusInvite(
      campusId: var_campusId,
      code: var_code,
      expiresAtMs: var_expiresAtMs,
    );
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
  ClientNonce sse_decode_client_nonce(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return ClientNonce(field0: var_field0);
  }

  @protected
  ClientSnapshot sse_decode_client_snapshot(SseDeserializer deserializer) {
    var var_sequence = sse_decode_u_64(deserializer);
    var var_network = sse_decode_connection_snapshot(deserializer);
    var var_session = sse_decode_session_snapshot(deserializer);
    var var_timeline = sse_decode_timeline_snapshot(deserializer);
    var var_thread = sse_decode_thread_snapshot(deserializer);
    var var_inbox = sse_decode_inbox_snapshot(deserializer);
    var var_search = sse_decode_search_snapshot(deserializer);
    var var_vault = sse_decode_vault_snapshot(deserializer);
    var var_shelf = sse_decode_shelf_snapshot(deserializer);
    var var_lounges = sse_decode_lounge_snapshot(deserializer);
    var var_stewards = sse_decode_steward_snapshot(deserializer);
    return ClientSnapshot(
      sequence: var_sequence,
      network: var_network,
      session: var_session,
      timeline: var_timeline,
      thread: var_thread,
      inbox: var_inbox,
      search: var_search,
      vault: var_vault,
      shelf: var_shelf,
      lounges: var_lounges,
      stewards: var_stewards,
    );
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
  CreateUploadRequest sse_decode_create_upload_request(
    SseDeserializer deserializer,
  ) {
    var var_fileName = sse_decode_String(deserializer);
    var var_contentType = sse_decode_String(deserializer);
    var var_byteSize = sse_decode_u_64(deserializer);
    return CreateUploadRequest(
      fileName: var_fileName,
      contentType: var_contentType,
      byteSize: var_byteSize,
    );
  }

  @protected
  DeliveryState sse_decode_delivery_state(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return DeliveryState.values[inner];
  }

  @protected
  DevAccount sse_decode_dev_account(SseDeserializer deserializer) {
    var var_email = sse_decode_String(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    var var_role = sse_decode_role(deserializer);
    return DevAccount(
      email: var_email,
      displayName: var_displayName,
      role: var_role,
    );
  }

  @protected
  DraftAttachmentView sse_decode_draft_attachment_view(
    SseDeserializer deserializer,
  ) {
    var var_draftId = sse_decode_draft_id(deserializer);
    var var_roomId = sse_decode_room_id(deserializer);
    var var_fileName = sse_decode_String(deserializer);
    var var_kind = sse_decode_attachment_kind(deserializer);
    var var_byteSize = sse_decode_u_64(deserializer);
    var var_upload = sse_decode_upload_state(deserializer);
    return DraftAttachmentView(
      draftId: var_draftId,
      roomId: var_roomId,
      fileName: var_fileName,
      kind: var_kind,
      byteSize: var_byteSize,
      upload: var_upload,
    );
  }

  @protected
  DraftId sse_decode_draft_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return DraftId(field0: var_field0);
  }

  @protected
  HandshakeRejection sse_decode_handshake_rejection(
    SseDeserializer deserializer,
  ) {
    var inner = sse_decode_i_32(deserializer);
    return HandshakeRejection.values[inner];
  }

  @protected
  HighlightSpan sse_decode_highlight_span(SseDeserializer deserializer) {
    var var_text = sse_decode_String(deserializer);
    var var_matched = sse_decode_bool(deserializer);
    return HighlightSpan(text: var_text, matched: var_matched);
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    return deserializer.buffer.getInt32();
  }

  @protected
  InboxItem sse_decode_inbox_item(SseDeserializer deserializer) {
    var var_kind = sse_decode_inbox_item_kind(deserializer);
    var var_roomId = sse_decode_room_id(deserializer);
    var var_rootMessageId = sse_decode_message_id(deserializer);
    var var_topicTitle = sse_decode_opt_String(deserializer);
    var var_rootAuthorDisplayName = sse_decode_String(deserializer);
    var var_rootPreview = sse_decode_String(deserializer);
    var var_latestReply = sse_decode_opt_box_autoadd_reply_preview(
      deserializer,
    );
    var var_replyCount = sse_decode_u_32(deserializer);
    var var_unreadReplies = sse_decode_u_32(deserializer);
    var var_unreadMentions = sse_decode_u_32(deserializer);
    var var_subscription = sse_decode_subscription_state(deserializer);
    var var_activityMessageId = sse_decode_message_id(deserializer);
    return InboxItem(
      kind: var_kind,
      roomId: var_roomId,
      rootMessageId: var_rootMessageId,
      topicTitle: var_topicTitle,
      rootAuthorDisplayName: var_rootAuthorDisplayName,
      rootPreview: var_rootPreview,
      latestReply: var_latestReply,
      replyCount: var_replyCount,
      unreadReplies: var_unreadReplies,
      unreadMentions: var_unreadMentions,
      subscription: var_subscription,
      activityMessageId: var_activityMessageId,
    );
  }

  @protected
  InboxItemKind sse_decode_inbox_item_kind(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return InboxItemKind.values[inner];
  }

  @protected
  InboxSnapshot sse_decode_inbox_snapshot(SseDeserializer deserializer) {
    var var_open = sse_decode_bool(deserializer);
    var var_items = sse_decode_list_inbox_item(deserializer);
    var var_unreadThreads = sse_decode_u_32(deserializer);
    var var_selectedRootMessageId = sse_decode_opt_box_autoadd_message_id(
      deserializer,
    );
    var var_replying = sse_decode_bool(deserializer);
    return InboxSnapshot(
      open: var_open,
      items: var_items,
      unreadThreads: var_unreadThreads,
      selectedRootMessageId: var_selectedRootMessageId,
      replying: var_replying,
    );
  }

  @protected
  InboxStep sse_decode_inbox_step(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return InboxStep.values[inner];
  }

  @protected
  InputField sse_decode_input_field(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return InputField.values[inner];
  }

  @protected
  InviteCode sse_decode_invite_code(SseDeserializer deserializer) {
    var var_campusId = sse_decode_campus_id(deserializer);
    var var_secret = sse_decode_String(deserializer);
    return InviteCode(campusId: var_campusId, secret: var_secret);
  }

  @protected
  IssuedStewardToken sse_decode_issued_steward_token(
    SseDeserializer deserializer,
  ) {
    var var_stewardId = sse_decode_account_id(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    var var_token = sse_decode_auth_token(deserializer);
    return IssuedStewardToken(
      stewardId: var_stewardId,
      displayName: var_displayName,
      token: var_token,
    );
  }

  @protected
  JoinCampusRequest sse_decode_join_campus_request(
    SseDeserializer deserializer,
  ) {
    var var_code = sse_decode_String(deserializer);
    return JoinCampusRequest(code: var_code);
  }

  @protected
  List<String> sse_decode_list_String(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <String>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_String(deserializer));
    }
    return ans_;
  }

  @protected
  List<AccountId> sse_decode_list_account_id(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <AccountId>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_account_id(deserializer));
    }
    return ans_;
  }

  @protected
  List<AttachmentId> sse_decode_list_attachment_id(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <AttachmentId>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_attachment_id(deserializer));
    }
    return ans_;
  }

  @protected
  List<AttachmentView> sse_decode_list_attachment_view(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <AttachmentView>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_attachment_view(deserializer));
    }
    return ans_;
  }

  @protected
  List<BodySegment> sse_decode_list_body_segment(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <BodySegment>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_body_segment(deserializer));
    }
    return ans_;
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
  List<DevAccount> sse_decode_list_dev_account(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <DevAccount>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_dev_account(deserializer));
    }
    return ans_;
  }

  @protected
  List<DraftAttachmentView> sse_decode_list_draft_attachment_view(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <DraftAttachmentView>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_draft_attachment_view(deserializer));
    }
    return ans_;
  }

  @protected
  List<HighlightSpan> sse_decode_list_highlight_span(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <HighlightSpan>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_highlight_span(deserializer));
    }
    return ans_;
  }

  @protected
  List<InboxItem> sse_decode_list_inbox_item(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <InboxItem>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_inbox_item(deserializer));
    }
    return ans_;
  }

  @protected
  List<LoungeEntry> sse_decode_list_lounge_entry(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <LoungeEntry>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_lounge_entry(deserializer));
    }
    return ans_;
  }

  @protected
  List<LoungeParticipantView> sse_decode_list_lounge_participant_view(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <LoungeParticipantView>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_lounge_participant_view(deserializer));
    }
    return ans_;
  }

  @protected
  List<MemberEntry> sse_decode_list_member_entry(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <MemberEntry>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_member_entry(deserializer));
    }
    return ans_;
  }

  @protected
  List<MentionRef> sse_decode_list_mention_ref(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <MentionRef>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_mention_ref(deserializer));
    }
    return ans_;
  }

  @protected
  List<NoteEntry> sse_decode_list_note_entry(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <NoteEntry>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_note_entry(deserializer));
    }
    return ans_;
  }

  @protected
  List<NoteMember> sse_decode_list_note_member(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <NoteMember>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_note_member(deserializer));
    }
    return ans_;
  }

  @protected
  List<int> sse_decode_list_prim_u_8_loose(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  List<ReactionSummary> sse_decode_list_reaction_summary(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <ReactionSummary>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_reaction_summary(deserializer));
    }
    return ans_;
  }

  @protected
  List<ReminderPreset> sse_decode_list_reminder_preset(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <ReminderPreset>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_reminder_preset(deserializer));
    }
    return ans_;
  }

  @protected
  List<Role> sse_decode_list_role(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <Role>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_role(deserializer));
    }
    return ans_;
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
  List<RoomId> sse_decode_list_room_id(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <RoomId>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_room_id(deserializer));
    }
    return ans_;
  }

  @protected
  List<SearchHit> sse_decode_list_search_hit(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <SearchHit>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_search_hit(deserializer));
    }
    return ans_;
  }

  @protected
  List<ShelfEntry> sse_decode_list_shelf_entry(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <ShelfEntry>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_shelf_entry(deserializer));
    }
    return ans_;
  }

  @protected
  List<StewardEntry> sse_decode_list_steward_entry(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <StewardEntry>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_steward_entry(deserializer));
    }
    return ans_;
  }

  @protected
  List<TimelineEntry> sse_decode_list_timeline_entry(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <TimelineEntry>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_timeline_entry(deserializer));
    }
    return ans_;
  }

  @protected
  List<ToolDescriptor> sse_decode_list_tool_descriptor(
    SseDeserializer deserializer,
  ) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <ToolDescriptor>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_tool_descriptor(deserializer));
    }
    return ans_;
  }

  @protected
  List<VaultEntry> sse_decode_list_vault_entry(SseDeserializer deserializer) {
    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <VaultEntry>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_vault_entry(deserializer));
    }
    return ans_;
  }

  @protected
  LoungeEntry sse_decode_lounge_entry(SseDeserializer deserializer) {
    var var_loungeId = sse_decode_lounge_id(deserializer);
    var var_scope = sse_decode_lounge_scope(deserializer);
    var var_roomId = sse_decode_opt_box_autoadd_room_id(deserializer);
    var var_roomSlug = sse_decode_opt_String(deserializer);
    var var_participants = sse_decode_list_lounge_participant_view(
      deserializer,
    );
    var var_joined = sse_decode_bool(deserializer);
    return LoungeEntry(
      loungeId: var_loungeId,
      scope: var_scope,
      roomId: var_roomId,
      roomSlug: var_roomSlug,
      participants: var_participants,
      joined: var_joined,
    );
  }

  @protected
  LoungeId sse_decode_lounge_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return LoungeId(field0: var_field0);
  }

  @protected
  LoungeMediaState sse_decode_lounge_media_state(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return LoungeMediaState.values[inner];
  }

  @protected
  LoungeParticipantView sse_decode_lounge_participant_view(
    SseDeserializer deserializer,
  ) {
    var var_accountId = sse_decode_account_id(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    var var_fromSelf = sse_decode_bool(deserializer);
    var var_canSpeak = sse_decode_bool(deserializer);
    return LoungeParticipantView(
      accountId: var_accountId,
      displayName: var_displayName,
      fromSelf: var_fromSelf,
      canSpeak: var_canSpeak,
    );
  }

  @protected
  LoungeScope sse_decode_lounge_scope(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return LoungeScope.values[inner];
  }

  @protected
  LoungeSnapshot sse_decode_lounge_snapshot(SseDeserializer deserializer) {
    var var_lounges = sse_decode_list_lounge_entry(deserializer);
    var var_joinedLoungeId = sse_decode_opt_box_autoadd_lounge_id(deserializer);
    var var_joining = sse_decode_bool(deserializer);
    var var_ticket = sse_decode_opt_box_autoadd_lounge_ticket(deserializer);
    var var_media = sse_decode_lounge_media_state(deserializer);
    var var_canJoin = sse_decode_bool(deserializer);
    var var_canSpeak = sse_decode_bool(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    return LoungeSnapshot(
      lounges: var_lounges,
      joinedLoungeId: var_joinedLoungeId,
      joining: var_joining,
      ticket: var_ticket,
      media: var_media,
      canJoin: var_canJoin,
      canSpeak: var_canSpeak,
      error: var_error,
    );
  }

  @protected
  LoungeTicket sse_decode_lounge_ticket(SseDeserializer deserializer) {
    var var_loungeId = sse_decode_lounge_id(deserializer);
    var var_livekitUrl = sse_decode_String(deserializer);
    var var_roomName = sse_decode_String(deserializer);
    var var_token = sse_decode_String(deserializer);
    var var_expiresAtMs = sse_decode_u_64(deserializer);
    return LoungeTicket(
      loungeId: var_loungeId,
      livekitUrl: var_livekitUrl,
      roomName: var_roomName,
      token: var_token,
      expiresAtMs: var_expiresAtMs,
    );
  }

  @protected
  MemberEntry sse_decode_member_entry(SseDeserializer deserializer) {
    var var_accountId = sse_decode_account_id(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    var var_email = sse_decode_String(deserializer);
    var var_role = sse_decode_role(deserializer);
    var var_manageable = sse_decode_bool(deserializer);
    var var_isSelf = sse_decode_bool(deserializer);
    return MemberEntry(
      accountId: var_accountId,
      displayName: var_displayName,
      email: var_email,
      role: var_role,
      manageable: var_manageable,
      isSelf: var_isSelf,
    );
  }

  @protected
  MentionRef sse_decode_mention_ref(SseDeserializer deserializer) {
    var var_accountId = sse_decode_account_id(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    return MentionRef(accountId: var_accountId, displayName: var_displayName);
  }

  @protected
  MessageId sse_decode_message_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return MessageId(field0: var_field0);
  }

  @protected
  MessageRejection sse_decode_message_rejection(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return MessageRejection.values[inner];
  }

  @protected
  Note sse_decode_note(SseDeserializer deserializer) {
    var var_campusId = sse_decode_campus_id(deserializer);
    var var_roomId = sse_decode_room_id(deserializer);
    var var_members = sse_decode_list_note_member(deserializer);
    return Note(
      campusId: var_campusId,
      roomId: var_roomId,
      members: var_members,
    );
  }

  @protected
  NoteEntry sse_decode_note_entry(SseDeserializer deserializer) {
    var var_roomId = sse_decode_room_id(deserializer);
    var var_title = sse_decode_String(deserializer);
    var var_memberCount = sse_decode_u_32(deserializer);
    return NoteEntry(
      roomId: var_roomId,
      title: var_title,
      memberCount: var_memberCount,
    );
  }

  @protected
  NoteMember sse_decode_note_member(SseDeserializer deserializer) {
    var var_accountId = sse_decode_account_id(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    return NoteMember(accountId: var_accountId, displayName: var_displayName);
  }

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_String(deserializer));
    } else {
      return null;
    }
  }

  @protected
  AccountId? sse_decode_opt_box_autoadd_account_id(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_account_id(deserializer));
    } else {
      return null;
    }
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
  AttachmentView? sse_decode_opt_box_autoadd_attachment_view(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_attachment_view(deserializer));
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
  CampusInvite? sse_decode_opt_box_autoadd_campus_invite(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_campus_invite(deserializer));
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
  IssuedStewardToken? sse_decode_opt_box_autoadd_issued_steward_token(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_issued_steward_token(deserializer));
    } else {
      return null;
    }
  }

  @protected
  LoungeId? sse_decode_opt_box_autoadd_lounge_id(SseDeserializer deserializer) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_lounge_id(deserializer));
    } else {
      return null;
    }
  }

  @protected
  LoungeTicket? sse_decode_opt_box_autoadd_lounge_ticket(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_lounge_ticket(deserializer));
    } else {
      return null;
    }
  }

  @protected
  MessageId? sse_decode_opt_box_autoadd_message_id(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_message_id(deserializer));
    } else {
      return null;
    }
  }

  @protected
  MessageRejection? sse_decode_opt_box_autoadd_message_rejection(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_message_rejection(deserializer));
    } else {
      return null;
    }
  }

  @protected
  ReminderPreset? sse_decode_opt_box_autoadd_reminder_preset(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_reminder_preset(deserializer));
    } else {
      return null;
    }
  }

  @protected
  ReplyPreview? sse_decode_opt_box_autoadd_reply_preview(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_reply_preview(deserializer));
    } else {
      return null;
    }
  }

  @protected
  RoomId? sse_decode_opt_box_autoadd_room_id(SseDeserializer deserializer) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_room_id(deserializer));
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
  ShelfEntry? sse_decode_opt_box_autoadd_shelf_entry(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_shelf_entry(deserializer));
    } else {
      return null;
    }
  }

  @protected
  ShelfItemId? sse_decode_opt_box_autoadd_shelf_item_id(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_shelf_item_id(deserializer));
    } else {
      return null;
    }
  }

  @protected
  StewardTool? sse_decode_opt_box_autoadd_steward_tool(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_steward_tool(deserializer));
    } else {
      return null;
    }
  }

  @protected
  StreamState? sse_decode_opt_box_autoadd_stream_state(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_stream_state(deserializer));
    } else {
      return null;
    }
  }

  @protected
  SubscriptionState? sse_decode_opt_box_autoadd_subscription_state(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_subscription_state(deserializer));
    } else {
      return null;
    }
  }

  @protected
  ThreadPreview? sse_decode_opt_box_autoadd_thread_preview(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_thread_preview(deserializer));
    } else {
      return null;
    }
  }

  @protected
  TimelineEntry? sse_decode_opt_box_autoadd_timeline_entry(
    SseDeserializer deserializer,
  ) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_timeline_entry(deserializer));
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
  BigInt? sse_decode_opt_box_autoadd_u_64(SseDeserializer deserializer) {
    if (sse_decode_bool(deserializer)) {
      return (sse_decode_box_autoadd_u_64(deserializer));
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
  React sse_decode_react(SseDeserializer deserializer) {
    var var_roomId = sse_decode_room_id(deserializer);
    var var_messageId = sse_decode_message_id(deserializer);
    var var_emoji = sse_decode_String(deserializer);
    var var_action = sse_decode_reaction_action(deserializer);
    return React(
      roomId: var_roomId,
      messageId: var_messageId,
      emoji: var_emoji,
      action: var_action,
    );
  }

  @protected
  ReactionAction sse_decode_reaction_action(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ReactionAction.values[inner];
  }

  @protected
  ReactionSummary sse_decode_reaction_summary(SseDeserializer deserializer) {
    var var_emoji = sse_decode_String(deserializer);
    var var_count = sse_decode_u_32(deserializer);
    var var_reactedBySelf = sse_decode_bool(deserializer);
    return ReactionSummary(
      emoji: var_emoji,
      count: var_count,
      reactedBySelf: var_reactedBySelf,
    );
  }

  @protected
  (String, List<AttachmentId>) sse_decode_record_string_list_attachment_id(
    SseDeserializer deserializer,
  ) {
    var var_field0 = sse_decode_String(deserializer);
    var var_field1 = sse_decode_list_attachment_id(deserializer);
    return (var_field0, var_field1);
  }

  @protected
  ReminderPreset sse_decode_reminder_preset(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ReminderPreset.values[inner];
  }

  @protected
  ReplyPreview sse_decode_reply_preview(SseDeserializer deserializer) {
    var var_messageId = sse_decode_message_id(deserializer);
    var var_authorId = sse_decode_account_id(deserializer);
    var var_authorDisplayName = sse_decode_String(deserializer);
    var var_preview = sse_decode_String(deserializer);
    return ReplyPreview(
      messageId: var_messageId,
      authorId: var_authorId,
      authorDisplayName: var_authorDisplayName,
      preview: var_preview,
    );
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
  SearchHit sse_decode_search_hit(SseDeserializer deserializer) {
    var var_roomId = sse_decode_room_id(deserializer);
    var var_messageId = sse_decode_message_id(deserializer);
    var var_threadRootId = sse_decode_opt_box_autoadd_message_id(deserializer);
    var var_authorDisplayName = sse_decode_String(deserializer);
    var var_topicTitle = sse_decode_opt_String(deserializer);
    var var_snippet = sse_decode_list_highlight_span(deserializer);
    var var_score = sse_decode_u_32(deserializer);
    return SearchHit(
      roomId: var_roomId,
      messageId: var_messageId,
      threadRootId: var_threadRootId,
      authorDisplayName: var_authorDisplayName,
      topicTitle: var_topicTitle,
      snippet: var_snippet,
      score: var_score,
    );
  }

  @protected
  SearchSnapshot sse_decode_search_snapshot(SseDeserializer deserializer) {
    var var_open = sse_decode_bool(deserializer);
    var var_query = sse_decode_String(deserializer);
    var var_hits = sse_decode_list_search_hit(deserializer);
    var var_selectedIndex = sse_decode_opt_box_autoadd_u_32(deserializer);
    var var_searching = sse_decode_bool(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    return SearchSnapshot(
      open: var_open,
      query: var_query,
      hits: var_hits,
      selectedIndex: var_selectedIndex,
      searching: var_searching,
      error: var_error,
    );
  }

  @protected
  SegmentKind sse_decode_segment_kind(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return SegmentKind.values[inner];
  }

  @protected
  SendMessage sse_decode_send_message(SseDeserializer deserializer) {
    var var_roomId = sse_decode_room_id(deserializer);
    var var_clientNonce = sse_decode_client_nonce(deserializer);
    var var_body = sse_decode_String(deserializer);
    var var_threadRootId = sse_decode_opt_box_autoadd_message_id(deserializer);
    var var_attachmentIds = sse_decode_list_attachment_id(deserializer);
    return SendMessage(
      roomId: var_roomId,
      clientNonce: var_clientNonce,
      body: var_body,
      threadRootId: var_threadRootId,
      attachmentIds: var_attachmentIds,
    );
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
    var var_activeRoomId = sse_decode_opt_box_autoadd_room_id(deserializer);
    var var_invite = sse_decode_opt_box_autoadd_campus_invite(deserializer);
    var var_canInvite = sse_decode_bool(deserializer);
    var var_members = sse_decode_list_member_entry(deserializer);
    var var_assignableRoles = sse_decode_list_role(deserializer);
    var var_quickAccounts = sse_decode_list_dev_account(deserializer);
    var var_notes = sse_decode_list_note_entry(deserializer);
    var var_busy = sse_decode_bool(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    return SessionSnapshot(
      auth: var_auth,
      account: var_account,
      campuses: var_campuses,
      activeCampusId: var_activeCampusId,
      rooms: var_rooms,
      activeRoomId: var_activeRoomId,
      invite: var_invite,
      canInvite: var_canInvite,
      members: var_members,
      assignableRoles: var_assignableRoles,
      quickAccounts: var_quickAccounts,
      notes: var_notes,
      busy: var_busy,
      error: var_error,
    );
  }

  @protected
  SetTopicTitle sse_decode_set_topic_title(SseDeserializer deserializer) {
    var var_roomId = sse_decode_room_id(deserializer);
    var var_rootMessageId = sse_decode_message_id(deserializer);
    var var_topicTitle = sse_decode_String(deserializer);
    return SetTopicTitle(
      roomId: var_roomId,
      rootMessageId: var_rootMessageId,
      topicTitle: var_topicTitle,
    );
  }

  @protected
  ShelfAction sse_decode_shelf_action(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ShelfAction.values[inner];
  }

  @protected
  ShelfBucket sse_decode_shelf_bucket(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ShelfBucket.values[inner];
  }

  @protected
  ShelfEntry sse_decode_shelf_entry(SseDeserializer deserializer) {
    var var_shelfItemId = sse_decode_opt_box_autoadd_shelf_item_id(
      deserializer,
    );
    var var_roomId = sse_decode_room_id(deserializer);
    var var_messageId = sse_decode_message_id(deserializer);
    var var_authorDisplayName = sse_decode_String(deserializer);
    var var_preview = sse_decode_String(deserializer);
    var var_remindAtMs = sse_decode_opt_box_autoadd_u_64(deserializer);
    var var_doneAtMs = sse_decode_opt_box_autoadd_u_64(deserializer);
    var var_bucket = sse_decode_shelf_bucket(deserializer);
    var var_pending = sse_decode_bool(deserializer);
    return ShelfEntry(
      shelfItemId: var_shelfItemId,
      roomId: var_roomId,
      messageId: var_messageId,
      authorDisplayName: var_authorDisplayName,
      preview: var_preview,
      remindAtMs: var_remindAtMs,
      doneAtMs: var_doneAtMs,
      bucket: var_bucket,
      pending: var_pending,
    );
  }

  @protected
  ShelfItemId sse_decode_shelf_item_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return ShelfItemId(field0: var_field0);
  }

  @protected
  ShelfSnapshot sse_decode_shelf_snapshot(SseDeserializer deserializer) {
    var var_open = sse_decode_bool(deserializer);
    var var_due = sse_decode_list_shelf_entry(deserializer);
    var var_upcoming = sse_decode_list_shelf_entry(deserializer);
    var var_done = sse_decode_list_shelf_entry(deserializer);
    var var_dueCount = sse_decode_u_32(deserializer);
    var var_latestReminder = sse_decode_opt_box_autoadd_shelf_entry(
      deserializer,
    );
    var var_presets = sse_decode_list_reminder_preset(deserializer);
    var var_pickerAnchorMs = sse_decode_opt_box_autoadd_u_64(deserializer);
    var var_pickerLatestMs = sse_decode_opt_box_autoadd_u_64(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    return ShelfSnapshot(
      open: var_open,
      due: var_due,
      upcoming: var_upcoming,
      done: var_done,
      dueCount: var_dueCount,
      latestReminder: var_latestReminder,
      presets: var_presets,
      pickerAnchorMs: var_pickerAnchorMs,
      pickerLatestMs: var_pickerLatestMs,
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
  StartNoteRequest sse_decode_start_note_request(SseDeserializer deserializer) {
    var var_accountIds = sse_decode_list_account_id(deserializer);
    return StartNoteRequest(accountIds: var_accountIds);
  }

  @protected
  StewardEntry sse_decode_steward_entry(SseDeserializer deserializer) {
    var var_stewardId = sse_decode_account_id(deserializer);
    var var_displayName = sse_decode_String(deserializer);
    var var_mentionToken = sse_decode_String(deserializer);
    var var_roomIds = sse_decode_list_room_id(deserializer);
    var var_revoked = sse_decode_bool(deserializer);
    var var_manageable = sse_decode_bool(deserializer);
    return StewardEntry(
      stewardId: var_stewardId,
      displayName: var_displayName,
      mentionToken: var_mentionToken,
      roomIds: var_roomIds,
      revoked: var_revoked,
      manageable: var_manageable,
    );
  }

  @protected
  StewardSnapshot sse_decode_steward_snapshot(SseDeserializer deserializer) {
    var var_open = sse_decode_bool(deserializer);
    var var_stewards = sse_decode_list_steward_entry(deserializer);
    var var_canManage = sse_decode_bool(deserializer);
    var var_issued = sse_decode_opt_box_autoadd_issued_steward_token(
      deserializer,
    );
    var var_pending = sse_decode_bool(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    return StewardSnapshot(
      open: var_open,
      stewards: var_stewards,
      canManage: var_canManage,
      issued: var_issued,
      pending: var_pending,
      error: var_error,
    );
  }

  @protected
  StewardTool sse_decode_steward_tool(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return StewardTool.values[inner];
  }

  @protected
  StreamChunk sse_decode_stream_chunk(SseDeserializer deserializer) {
    var var_streamId = sse_decode_stream_id(deserializer);
    var var_roomId = sse_decode_room_id(deserializer);
    var var_threadRootId = sse_decode_opt_box_autoadd_message_id(deserializer);
    var var_phase = sse_decode_stream_phase(deserializer);
    var var_sequence = sse_decode_u_32(deserializer);
    var var_text = sse_decode_String(deserializer);
    return StreamChunk(
      streamId: var_streamId,
      roomId: var_roomId,
      threadRootId: var_threadRootId,
      phase: var_phase,
      sequence: var_sequence,
      text: var_text,
    );
  }

  @protected
  StreamId sse_decode_stream_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return StreamId(field0: var_field0);
  }

  @protected
  StreamPhase sse_decode_stream_phase(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return StreamPhase.values[inner];
  }

  @protected
  StreamState sse_decode_stream_state(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return StreamState.values[inner];
  }

  @protected
  SubscriptionState sse_decode_subscription_state(
    SseDeserializer deserializer,
  ) {
    var inner = sse_decode_i_32(deserializer);
    return SubscriptionState.values[inner];
  }

  @protected
  ThreadPreview sse_decode_thread_preview(SseDeserializer deserializer) {
    var var_topicTitle = sse_decode_opt_String(deserializer);
    var var_replyCount = sse_decode_u_32(deserializer);
    var var_latestReply = sse_decode_opt_box_autoadd_reply_preview(
      deserializer,
    );
    var var_showLatestReply = sse_decode_bool(deserializer);
    var var_subscription = sse_decode_opt_box_autoadd_subscription_state(
      deserializer,
    );
    var var_unreadReplies = sse_decode_u_32(deserializer);
    return ThreadPreview(
      topicTitle: var_topicTitle,
      replyCount: var_replyCount,
      latestReply: var_latestReply,
      showLatestReply: var_showLatestReply,
      subscription: var_subscription,
      unreadReplies: var_unreadReplies,
    );
  }

  @protected
  ThreadSnapshot sse_decode_thread_snapshot(SseDeserializer deserializer) {
    var var_roomId = sse_decode_opt_box_autoadd_room_id(deserializer);
    var var_rootMessageId = sse_decode_opt_box_autoadd_message_id(deserializer);
    var var_root = sse_decode_opt_box_autoadd_timeline_entry(deserializer);
    var var_replies = sse_decode_list_timeline_entry(deserializer);
    var var_topicTitle = sse_decode_opt_String(deserializer);
    var var_subscription = sse_decode_opt_box_autoadd_subscription_state(
      deserializer,
    );
    var var_syncing = sse_decode_bool(deserializer);
    var var_mentionableStewards = sse_decode_list_steward_entry(deserializer);
    return ThreadSnapshot(
      roomId: var_roomId,
      rootMessageId: var_rootMessageId,
      root: var_root,
      replies: var_replies,
      topicTitle: var_topicTitle,
      subscription: var_subscription,
      syncing: var_syncing,
      mentionableStewards: var_mentionableStewards,
    );
  }

  @protected
  TimelineEntry sse_decode_timeline_entry(SseDeserializer deserializer) {
    var var_clientNonce = sse_decode_client_nonce(deserializer);
    var var_messageId = sse_decode_opt_box_autoadd_message_id(deserializer);
    var var_authorId = sse_decode_account_id(deserializer);
    var var_authorDisplayName = sse_decode_String(deserializer);
    var var_authorMentionToken = sse_decode_String(deserializer);
    var var_fromSelf = sse_decode_bool(deserializer);
    var var_body = sse_decode_String(deserializer);
    var var_segments = sse_decode_list_body_segment(deserializer);
    var var_threadRootId = sse_decode_opt_box_autoadd_message_id(deserializer);
    var var_reactions = sse_decode_list_reaction_summary(deserializer);
    var var_thread = sse_decode_opt_box_autoadd_thread_preview(deserializer);
    var var_delivery = sse_decode_delivery_state(deserializer);
    var var_rejection = sse_decode_opt_box_autoadd_message_rejection(
      deserializer,
    );
    var var_attachments = sse_decode_list_attachment_view(deserializer);
    var var_pendingDrafts = sse_decode_list_draft_attachment_view(deserializer);
    var var_pinned = sse_decode_bool(deserializer);
    var var_shelved = sse_decode_bool(deserializer);
    var var_authorIsSteward = sse_decode_bool(deserializer);
    var var_stream = sse_decode_opt_box_autoadd_stream_state(deserializer);
    return TimelineEntry(
      clientNonce: var_clientNonce,
      messageId: var_messageId,
      authorId: var_authorId,
      authorDisplayName: var_authorDisplayName,
      authorMentionToken: var_authorMentionToken,
      fromSelf: var_fromSelf,
      body: var_body,
      segments: var_segments,
      threadRootId: var_threadRootId,
      reactions: var_reactions,
      thread: var_thread,
      delivery: var_delivery,
      rejection: var_rejection,
      attachments: var_attachments,
      pendingDrafts: var_pendingDrafts,
      pinned: var_pinned,
      shelved: var_shelved,
      authorIsSteward: var_authorIsSteward,
      stream: var_stream,
    );
  }

  @protected
  TimelineSnapshot sse_decode_timeline_snapshot(SseDeserializer deserializer) {
    var var_roomId = sse_decode_opt_box_autoadd_room_id(deserializer);
    var var_entries = sse_decode_list_timeline_entry(deserializer);
    var var_syncing = sse_decode_bool(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    var var_reactionPalette = sse_decode_list_String(deserializer);
    var var_drafts = sse_decode_list_draft_attachment_view(deserializer);
    var var_focusMessageId = sse_decode_opt_box_autoadd_message_id(
      deserializer,
    );
    var var_canPin = sse_decode_bool(deserializer);
    var var_canUpload = sse_decode_bool(deserializer);
    var var_mentionableStewards = sse_decode_list_steward_entry(deserializer);
    return TimelineSnapshot(
      roomId: var_roomId,
      entries: var_entries,
      syncing: var_syncing,
      error: var_error,
      reactionPalette: var_reactionPalette,
      drafts: var_drafts,
      focusMessageId: var_focusMessageId,
      canPin: var_canPin,
      canUpload: var_canUpload,
      mentionableStewards: var_mentionableStewards,
    );
  }

  @protected
  ToolCallId sse_decode_tool_call_id(SseDeserializer deserializer) {
    var var_field0 = sse_decode_Uuid(deserializer);
    return ToolCallId(field0: var_field0);
  }

  @protected
  ToolDescriptor sse_decode_tool_descriptor(SseDeserializer deserializer) {
    var var_name = sse_decode_String(deserializer);
    var var_description = sse_decode_String(deserializer);
    var var_inputSchema = sse_decode_String(deserializer);
    return ToolDescriptor(
      name: var_name,
      description: var_description,
      inputSchema: var_inputSchema,
    );
  }

  @protected
  ToolErrorCode sse_decode_tool_error_code(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return ToolErrorCode.values[inner];
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
  BigInt sse_decode_u_64(SseDeserializer deserializer) {
    return deserializer.buffer.getBigUint64();
  }

  @protected
  int sse_decode_u_8(SseDeserializer deserializer) {
    return deserializer.buffer.getUint8();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {}

  @protected
  UpdateMemberRequest sse_decode_update_member_request(
    SseDeserializer deserializer,
  ) {
    var var_role = sse_decode_role(deserializer);
    return UpdateMemberRequest(role: var_role);
  }

  @protected
  UpdateShelf sse_decode_update_shelf(SseDeserializer deserializer) {
    var var_roomId = sse_decode_room_id(deserializer);
    var var_messageId = sse_decode_message_id(deserializer);
    var var_action = sse_decode_shelf_action(deserializer);
    var var_remindAtMs = sse_decode_opt_box_autoadd_u_64(deserializer);
    return UpdateShelf(
      roomId: var_roomId,
      messageId: var_messageId,
      action: var_action,
      remindAtMs: var_remindAtMs,
    );
  }

  @protected
  UploadState sse_decode_upload_state(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return UploadState.values[inner];
  }

  @protected
  BigInt sse_decode_usize(SseDeserializer deserializer) {
    return deserializer.buffer.getBigUint64();
  }

  @protected
  VaultEntry sse_decode_vault_entry(SseDeserializer deserializer) {
    var var_tab = sse_decode_vault_tab(deserializer);
    var var_roomId = sse_decode_room_id(deserializer);
    var var_messageId = sse_decode_message_id(deserializer);
    var var_authorDisplayName = sse_decode_String(deserializer);
    var var_preview = sse_decode_String(deserializer);
    var var_attachment = sse_decode_opt_box_autoadd_attachment_view(
      deserializer,
    );
    var var_link = sse_decode_opt_String(deserializer);
    var var_pinnedAtMs = sse_decode_opt_box_autoadd_u_64(deserializer);
    return VaultEntry(
      tab: var_tab,
      roomId: var_roomId,
      messageId: var_messageId,
      authorDisplayName: var_authorDisplayName,
      preview: var_preview,
      attachment: var_attachment,
      link: var_link,
      pinnedAtMs: var_pinnedAtMs,
    );
  }

  @protected
  VaultSnapshot sse_decode_vault_snapshot(SseDeserializer deserializer) {
    var var_open = sse_decode_bool(deserializer);
    var var_roomId = sse_decode_opt_box_autoadd_room_id(deserializer);
    var var_tab = sse_decode_vault_tab(deserializer);
    var var_entries = sse_decode_list_vault_entry(deserializer);
    var var_loading = sse_decode_bool(deserializer);
    var var_hasMore = sse_decode_bool(deserializer);
    var var_error = sse_decode_opt_box_autoadd_api_error(deserializer);
    return VaultSnapshot(
      open: var_open,
      roomId: var_roomId,
      tab: var_tab,
      entries: var_entries,
      loading: var_loading,
      hasMore: var_hasMore,
      error: var_error,
    );
  }

  @protected
  VaultTab sse_decode_vault_tab(SseDeserializer deserializer) {
    var inner = sse_decode_i_32(deserializer);
    return VaultTab.values[inner];
  }

  @protected
  VersionMismatch sse_decode_version_mismatch(SseDeserializer deserializer) {
    var var_supported = sse_decode_protocol_version(deserializer);
    var var_received = sse_decode_protocol_version(deserializer);
    return VersionMismatch(supported: var_supported, received: var_received);
  }

  @protected
  void sse_encode_AnyhowException(
    AnyhowException self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.message, serializer);
  }

  @protected
  void
  sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    StreamWriter self,
    SseSerializer serializer,
  ) {
    sse_encode_usize(
      (self as StreamWriterImpl).frbInternalSseEncode(move: true),
      serializer,
    );
  }

  @protected
  void
  sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    StreamWriter self,
    SseSerializer serializer,
  ) {
    sse_encode_usize(
      (self as StreamWriterImpl).frbInternalSseEncode(move: false),
      serializer,
    );
  }

  @protected
  void
  sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    StreamWriter self,
    SseSerializer serializer,
  ) {
    sse_encode_usize(
      (self as StreamWriterImpl).frbInternalSseEncode(move: false),
      serializer,
    );
  }

  @protected
  void
  sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerStreamWriter(
    StreamWriter self,
    SseSerializer serializer,
  ) {
    sse_encode_usize(
      (self as StreamWriterImpl).frbInternalSseEncode(move: null),
      serializer,
    );
  }

  @protected
  void sse_encode_StreamSink_client_snapshot_Sse(
    RustStreamSink<ClientSnapshot> self,
    SseSerializer serializer,
  ) {
    sse_encode_String(
      self.setupAndSerialize(
        codec: SseCodec(
          decodeSuccessData: sse_decode_client_snapshot,
          decodeErrorData: sse_decode_AnyhowException,
        ),
      ),
      serializer,
    );
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
  void sse_encode_add_member_request(
    AddMemberRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.email, serializer);
    sse_encode_role(self.role, serializer);
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
  void sse_encode_attachment_id(AttachmentId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_attachment_kind(
    AttachmentKind self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_attachment_view(
    AttachmentView self,
    SseSerializer serializer,
  ) {
    sse_encode_attachment_id(self.attachmentId, serializer);
    sse_encode_String(self.fileName, serializer);
    sse_encode_String(self.contentType, serializer);
    sse_encode_attachment_kind(self.kind, serializer);
    sse_encode_u_64(self.byteSize, serializer);
    sse_encode_opt_String(self.url, serializer);
  }

  @protected
  void sse_encode_auth_phase(AuthPhase self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_auth_token(AuthToken self, SseSerializer serializer) {
    sse_encode_String(self.field0, serializer);
  }

  @protected
  void sse_encode_body_segment(BodySegment self, SseSerializer serializer) {
    sse_encode_segment_kind(self.kind, serializer);
    sse_encode_String(self.text, serializer);
    sse_encode_opt_box_autoadd_account_id(self.accountId, serializer);
    sse_encode_opt_String(self.url, serializer);
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
  void sse_encode_box_autoadd_add_member_request(
    AddMemberRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_add_member_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_api_error(
    ApiError self,
    SseSerializer serializer,
  ) {
    sse_encode_api_error(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_attachment_id(
    AttachmentId self,
    SseSerializer serializer,
  ) {
    sse_encode_attachment_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_attachment_view(
    AttachmentView self,
    SseSerializer serializer,
  ) {
    sse_encode_attachment_view(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_campus_id(
    CampusId self,
    SseSerializer serializer,
  ) {
    sse_encode_campus_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_campus_invite(
    CampusInvite self,
    SseSerializer serializer,
  ) {
    sse_encode_campus_invite(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_client_nonce(
    ClientNonce self,
    SseSerializer serializer,
  ) {
    sse_encode_client_nonce(self, serializer);
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
  void sse_encode_box_autoadd_create_upload_request(
    CreateUploadRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_create_upload_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_draft_id(DraftId self, SseSerializer serializer) {
    sse_encode_draft_id(self, serializer);
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
  void sse_encode_box_autoadd_invite_code(
    InviteCode self,
    SseSerializer serializer,
  ) {
    sse_encode_invite_code(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_issued_steward_token(
    IssuedStewardToken self,
    SseSerializer serializer,
  ) {
    sse_encode_issued_steward_token(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_join_campus_request(
    JoinCampusRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_join_campus_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_lounge_id(
    LoungeId self,
    SseSerializer serializer,
  ) {
    sse_encode_lounge_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_lounge_ticket(
    LoungeTicket self,
    SseSerializer serializer,
  ) {
    sse_encode_lounge_ticket(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_message_id(
    MessageId self,
    SseSerializer serializer,
  ) {
    sse_encode_message_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_message_rejection(
    MessageRejection self,
    SseSerializer serializer,
  ) {
    sse_encode_message_rejection(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_note(Note self, SseSerializer serializer) {
    sse_encode_note(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_react(React self, SseSerializer serializer) {
    sse_encode_react(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_reminder_preset(
    ReminderPreset self,
    SseSerializer serializer,
  ) {
    sse_encode_reminder_preset(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_reply_preview(
    ReplyPreview self,
    SseSerializer serializer,
  ) {
    sse_encode_reply_preview(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_room_id(RoomId self, SseSerializer serializer) {
    sse_encode_room_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_send_message(
    SendMessage self,
    SseSerializer serializer,
  ) {
    sse_encode_send_message(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_session_id(
    SessionId self,
    SseSerializer serializer,
  ) {
    sse_encode_session_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_set_topic_title(
    SetTopicTitle self,
    SseSerializer serializer,
  ) {
    sse_encode_set_topic_title(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_shelf_entry(
    ShelfEntry self,
    SseSerializer serializer,
  ) {
    sse_encode_shelf_entry(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_shelf_item_id(
    ShelfItemId self,
    SseSerializer serializer,
  ) {
    sse_encode_shelf_item_id(self, serializer);
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
  void sse_encode_box_autoadd_start_note_request(
    StartNoteRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_start_note_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_steward_tool(
    StewardTool self,
    SseSerializer serializer,
  ) {
    sse_encode_steward_tool(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_stream_id(
    StreamId self,
    SseSerializer serializer,
  ) {
    sse_encode_stream_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_stream_state(
    StreamState self,
    SseSerializer serializer,
  ) {
    sse_encode_stream_state(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_subscription_state(
    SubscriptionState self,
    SseSerializer serializer,
  ) {
    sse_encode_subscription_state(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_thread_preview(
    ThreadPreview self,
    SseSerializer serializer,
  ) {
    sse_encode_thread_preview(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_timeline_entry(
    TimelineEntry self,
    SseSerializer serializer,
  ) {
    sse_encode_timeline_entry(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_tool_call_id(
    ToolCallId self,
    SseSerializer serializer,
  ) {
    sse_encode_tool_call_id(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_u_32(int self, SseSerializer serializer) {
    sse_encode_u_32(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_u_64(BigInt self, SseSerializer serializer) {
    sse_encode_u_64(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_update_member_request(
    UpdateMemberRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_update_member_request(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_update_shelf(
    UpdateShelf self,
    SseSerializer serializer,
  ) {
    sse_encode_update_shelf(self, serializer);
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
  void sse_encode_campus_invite(CampusInvite self, SseSerializer serializer) {
    sse_encode_campus_id(self.campusId, serializer);
    sse_encode_String(self.code, serializer);
    sse_encode_u_64(self.expiresAtMs, serializer);
  }

  @protected
  void sse_encode_campus_summary(CampusSummary self, SseSerializer serializer) {
    sse_encode_campus_id(self.campusId, serializer);
    sse_encode_String(self.slug, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_role(self.role, serializer);
  }

  @protected
  void sse_encode_client_nonce(ClientNonce self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_client_snapshot(
    ClientSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_u_64(self.sequence, serializer);
    sse_encode_connection_snapshot(self.network, serializer);
    sse_encode_session_snapshot(self.session, serializer);
    sse_encode_timeline_snapshot(self.timeline, serializer);
    sse_encode_thread_snapshot(self.thread, serializer);
    sse_encode_inbox_snapshot(self.inbox, serializer);
    sse_encode_search_snapshot(self.search, serializer);
    sse_encode_vault_snapshot(self.vault, serializer);
    sse_encode_shelf_snapshot(self.shelf, serializer);
    sse_encode_lounge_snapshot(self.lounges, serializer);
    sse_encode_steward_snapshot(self.stewards, serializer);
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
  void sse_encode_create_upload_request(
    CreateUploadRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.fileName, serializer);
    sse_encode_String(self.contentType, serializer);
    sse_encode_u_64(self.byteSize, serializer);
  }

  @protected
  void sse_encode_delivery_state(DeliveryState self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_dev_account(DevAccount self, SseSerializer serializer) {
    sse_encode_String(self.email, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_role(self.role, serializer);
  }

  @protected
  void sse_encode_draft_attachment_view(
    DraftAttachmentView self,
    SseSerializer serializer,
  ) {
    sse_encode_draft_id(self.draftId, serializer);
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_String(self.fileName, serializer);
    sse_encode_attachment_kind(self.kind, serializer);
    sse_encode_u_64(self.byteSize, serializer);
    sse_encode_upload_state(self.upload, serializer);
  }

  @protected
  void sse_encode_draft_id(DraftId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_handshake_rejection(
    HandshakeRejection self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_highlight_span(HighlightSpan self, SseSerializer serializer) {
    sse_encode_String(self.text, serializer);
    sse_encode_bool(self.matched, serializer);
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    serializer.buffer.putInt32(self);
  }

  @protected
  void sse_encode_inbox_item(InboxItem self, SseSerializer serializer) {
    sse_encode_inbox_item_kind(self.kind, serializer);
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_message_id(self.rootMessageId, serializer);
    sse_encode_opt_String(self.topicTitle, serializer);
    sse_encode_String(self.rootAuthorDisplayName, serializer);
    sse_encode_String(self.rootPreview, serializer);
    sse_encode_opt_box_autoadd_reply_preview(self.latestReply, serializer);
    sse_encode_u_32(self.replyCount, serializer);
    sse_encode_u_32(self.unreadReplies, serializer);
    sse_encode_u_32(self.unreadMentions, serializer);
    sse_encode_subscription_state(self.subscription, serializer);
    sse_encode_message_id(self.activityMessageId, serializer);
  }

  @protected
  void sse_encode_inbox_item_kind(
    InboxItemKind self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_inbox_snapshot(InboxSnapshot self, SseSerializer serializer) {
    sse_encode_bool(self.open, serializer);
    sse_encode_list_inbox_item(self.items, serializer);
    sse_encode_u_32(self.unreadThreads, serializer);
    sse_encode_opt_box_autoadd_message_id(
      self.selectedRootMessageId,
      serializer,
    );
    sse_encode_bool(self.replying, serializer);
  }

  @protected
  void sse_encode_inbox_step(InboxStep self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_input_field(InputField self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_invite_code(InviteCode self, SseSerializer serializer) {
    sse_encode_campus_id(self.campusId, serializer);
    sse_encode_String(self.secret, serializer);
  }

  @protected
  void sse_encode_issued_steward_token(
    IssuedStewardToken self,
    SseSerializer serializer,
  ) {
    sse_encode_account_id(self.stewardId, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_auth_token(self.token, serializer);
  }

  @protected
  void sse_encode_join_campus_request(
    JoinCampusRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.code, serializer);
  }

  @protected
  void sse_encode_list_String(List<String> self, SseSerializer serializer) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_String(item, serializer);
    }
  }

  @protected
  void sse_encode_list_account_id(
    List<AccountId> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_account_id(item, serializer);
    }
  }

  @protected
  void sse_encode_list_attachment_id(
    List<AttachmentId> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_attachment_id(item, serializer);
    }
  }

  @protected
  void sse_encode_list_attachment_view(
    List<AttachmentView> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_attachment_view(item, serializer);
    }
  }

  @protected
  void sse_encode_list_body_segment(
    List<BodySegment> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_body_segment(item, serializer);
    }
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
  void sse_encode_list_dev_account(
    List<DevAccount> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_dev_account(item, serializer);
    }
  }

  @protected
  void sse_encode_list_draft_attachment_view(
    List<DraftAttachmentView> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_draft_attachment_view(item, serializer);
    }
  }

  @protected
  void sse_encode_list_highlight_span(
    List<HighlightSpan> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_highlight_span(item, serializer);
    }
  }

  @protected
  void sse_encode_list_inbox_item(
    List<InboxItem> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_inbox_item(item, serializer);
    }
  }

  @protected
  void sse_encode_list_lounge_entry(
    List<LoungeEntry> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_lounge_entry(item, serializer);
    }
  }

  @protected
  void sse_encode_list_lounge_participant_view(
    List<LoungeParticipantView> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_lounge_participant_view(item, serializer);
    }
  }

  @protected
  void sse_encode_list_member_entry(
    List<MemberEntry> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_member_entry(item, serializer);
    }
  }

  @protected
  void sse_encode_list_mention_ref(
    List<MentionRef> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_mention_ref(item, serializer);
    }
  }

  @protected
  void sse_encode_list_note_entry(
    List<NoteEntry> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_note_entry(item, serializer);
    }
  }

  @protected
  void sse_encode_list_note_member(
    List<NoteMember> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_note_member(item, serializer);
    }
  }

  @protected
  void sse_encode_list_prim_u_8_loose(
    List<int> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    serializer.buffer.putUint8List(
      self is Uint8List ? self : Uint8List.fromList(self),
    );
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
  void sse_encode_list_reaction_summary(
    List<ReactionSummary> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_reaction_summary(item, serializer);
    }
  }

  @protected
  void sse_encode_list_reminder_preset(
    List<ReminderPreset> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_reminder_preset(item, serializer);
    }
  }

  @protected
  void sse_encode_list_role(List<Role> self, SseSerializer serializer) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_role(item, serializer);
    }
  }

  @protected
  void sse_encode_list_room(List<Room> self, SseSerializer serializer) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_room(item, serializer);
    }
  }

  @protected
  void sse_encode_list_room_id(List<RoomId> self, SseSerializer serializer) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_room_id(item, serializer);
    }
  }

  @protected
  void sse_encode_list_search_hit(
    List<SearchHit> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_search_hit(item, serializer);
    }
  }

  @protected
  void sse_encode_list_shelf_entry(
    List<ShelfEntry> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_shelf_entry(item, serializer);
    }
  }

  @protected
  void sse_encode_list_steward_entry(
    List<StewardEntry> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_steward_entry(item, serializer);
    }
  }

  @protected
  void sse_encode_list_timeline_entry(
    List<TimelineEntry> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_timeline_entry(item, serializer);
    }
  }

  @protected
  void sse_encode_list_tool_descriptor(
    List<ToolDescriptor> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_tool_descriptor(item, serializer);
    }
  }

  @protected
  void sse_encode_list_vault_entry(
    List<VaultEntry> self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_vault_entry(item, serializer);
    }
  }

  @protected
  void sse_encode_lounge_entry(LoungeEntry self, SseSerializer serializer) {
    sse_encode_lounge_id(self.loungeId, serializer);
    sse_encode_lounge_scope(self.scope, serializer);
    sse_encode_opt_box_autoadd_room_id(self.roomId, serializer);
    sse_encode_opt_String(self.roomSlug, serializer);
    sse_encode_list_lounge_participant_view(self.participants, serializer);
    sse_encode_bool(self.joined, serializer);
  }

  @protected
  void sse_encode_lounge_id(LoungeId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_lounge_media_state(
    LoungeMediaState self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_lounge_participant_view(
    LoungeParticipantView self,
    SseSerializer serializer,
  ) {
    sse_encode_account_id(self.accountId, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_bool(self.fromSelf, serializer);
    sse_encode_bool(self.canSpeak, serializer);
  }

  @protected
  void sse_encode_lounge_scope(LoungeScope self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_lounge_snapshot(
    LoungeSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_list_lounge_entry(self.lounges, serializer);
    sse_encode_opt_box_autoadd_lounge_id(self.joinedLoungeId, serializer);
    sse_encode_bool(self.joining, serializer);
    sse_encode_opt_box_autoadd_lounge_ticket(self.ticket, serializer);
    sse_encode_lounge_media_state(self.media, serializer);
    sse_encode_bool(self.canJoin, serializer);
    sse_encode_bool(self.canSpeak, serializer);
    sse_encode_opt_box_autoadd_api_error(self.error, serializer);
  }

  @protected
  void sse_encode_lounge_ticket(LoungeTicket self, SseSerializer serializer) {
    sse_encode_lounge_id(self.loungeId, serializer);
    sse_encode_String(self.livekitUrl, serializer);
    sse_encode_String(self.roomName, serializer);
    sse_encode_String(self.token, serializer);
    sse_encode_u_64(self.expiresAtMs, serializer);
  }

  @protected
  void sse_encode_member_entry(MemberEntry self, SseSerializer serializer) {
    sse_encode_account_id(self.accountId, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_String(self.email, serializer);
    sse_encode_role(self.role, serializer);
    sse_encode_bool(self.manageable, serializer);
    sse_encode_bool(self.isSelf, serializer);
  }

  @protected
  void sse_encode_mention_ref(MentionRef self, SseSerializer serializer) {
    sse_encode_account_id(self.accountId, serializer);
    sse_encode_String(self.displayName, serializer);
  }

  @protected
  void sse_encode_message_id(MessageId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_message_rejection(
    MessageRejection self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_note(Note self, SseSerializer serializer) {
    sse_encode_campus_id(self.campusId, serializer);
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_list_note_member(self.members, serializer);
  }

  @protected
  void sse_encode_note_entry(NoteEntry self, SseSerializer serializer) {
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_String(self.title, serializer);
    sse_encode_u_32(self.memberCount, serializer);
  }

  @protected
  void sse_encode_note_member(NoteMember self, SseSerializer serializer) {
    sse_encode_account_id(self.accountId, serializer);
    sse_encode_String(self.displayName, serializer);
  }

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_String(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_account_id(
    AccountId? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_account_id(self, serializer);
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
  void sse_encode_opt_box_autoadd_attachment_view(
    AttachmentView? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_attachment_view(self, serializer);
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
  void sse_encode_opt_box_autoadd_campus_invite(
    CampusInvite? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_campus_invite(self, serializer);
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
  void sse_encode_opt_box_autoadd_issued_steward_token(
    IssuedStewardToken? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_issued_steward_token(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_lounge_id(
    LoungeId? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_lounge_id(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_lounge_ticket(
    LoungeTicket? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_lounge_ticket(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_message_id(
    MessageId? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_message_id(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_message_rejection(
    MessageRejection? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_message_rejection(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_reminder_preset(
    ReminderPreset? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_reminder_preset(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_reply_preview(
    ReplyPreview? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_reply_preview(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_room_id(
    RoomId? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_room_id(self, serializer);
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
  void sse_encode_opt_box_autoadd_shelf_entry(
    ShelfEntry? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_shelf_entry(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_shelf_item_id(
    ShelfItemId? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_shelf_item_id(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_steward_tool(
    StewardTool? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_steward_tool(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_stream_state(
    StreamState? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_stream_state(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_subscription_state(
    SubscriptionState? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_subscription_state(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_thread_preview(
    ThreadPreview? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_thread_preview(self, serializer);
    }
  }

  @protected
  void sse_encode_opt_box_autoadd_timeline_entry(
    TimelineEntry? self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_timeline_entry(self, serializer);
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
  void sse_encode_opt_box_autoadd_u_64(BigInt? self, SseSerializer serializer) {
    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_box_autoadd_u_64(self, serializer);
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
  void sse_encode_react(React self, SseSerializer serializer) {
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_message_id(self.messageId, serializer);
    sse_encode_String(self.emoji, serializer);
    sse_encode_reaction_action(self.action, serializer);
  }

  @protected
  void sse_encode_reaction_action(
    ReactionAction self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_reaction_summary(
    ReactionSummary self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.emoji, serializer);
    sse_encode_u_32(self.count, serializer);
    sse_encode_bool(self.reactedBySelf, serializer);
  }

  @protected
  void sse_encode_record_string_list_attachment_id(
    (String, List<AttachmentId>) self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.$1, serializer);
    sse_encode_list_attachment_id(self.$2, serializer);
  }

  @protected
  void sse_encode_reminder_preset(
    ReminderPreset self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_reply_preview(ReplyPreview self, SseSerializer serializer) {
    sse_encode_message_id(self.messageId, serializer);
    sse_encode_account_id(self.authorId, serializer);
    sse_encode_String(self.authorDisplayName, serializer);
    sse_encode_String(self.preview, serializer);
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
  void sse_encode_search_hit(SearchHit self, SseSerializer serializer) {
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_message_id(self.messageId, serializer);
    sse_encode_opt_box_autoadd_message_id(self.threadRootId, serializer);
    sse_encode_String(self.authorDisplayName, serializer);
    sse_encode_opt_String(self.topicTitle, serializer);
    sse_encode_list_highlight_span(self.snippet, serializer);
    sse_encode_u_32(self.score, serializer);
  }

  @protected
  void sse_encode_search_snapshot(
    SearchSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self.open, serializer);
    sse_encode_String(self.query, serializer);
    sse_encode_list_search_hit(self.hits, serializer);
    sse_encode_opt_box_autoadd_u_32(self.selectedIndex, serializer);
    sse_encode_bool(self.searching, serializer);
    sse_encode_opt_box_autoadd_api_error(self.error, serializer);
  }

  @protected
  void sse_encode_segment_kind(SegmentKind self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_send_message(SendMessage self, SseSerializer serializer) {
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_client_nonce(self.clientNonce, serializer);
    sse_encode_String(self.body, serializer);
    sse_encode_opt_box_autoadd_message_id(self.threadRootId, serializer);
    sse_encode_list_attachment_id(self.attachmentIds, serializer);
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
    sse_encode_opt_box_autoadd_room_id(self.activeRoomId, serializer);
    sse_encode_opt_box_autoadd_campus_invite(self.invite, serializer);
    sse_encode_bool(self.canInvite, serializer);
    sse_encode_list_member_entry(self.members, serializer);
    sse_encode_list_role(self.assignableRoles, serializer);
    sse_encode_list_dev_account(self.quickAccounts, serializer);
    sse_encode_list_note_entry(self.notes, serializer);
    sse_encode_bool(self.busy, serializer);
    sse_encode_opt_box_autoadd_api_error(self.error, serializer);
  }

  @protected
  void sse_encode_set_topic_title(
    SetTopicTitle self,
    SseSerializer serializer,
  ) {
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_message_id(self.rootMessageId, serializer);
    sse_encode_String(self.topicTitle, serializer);
  }

  @protected
  void sse_encode_shelf_action(ShelfAction self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_shelf_bucket(ShelfBucket self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_shelf_entry(ShelfEntry self, SseSerializer serializer) {
    sse_encode_opt_box_autoadd_shelf_item_id(self.shelfItemId, serializer);
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_message_id(self.messageId, serializer);
    sse_encode_String(self.authorDisplayName, serializer);
    sse_encode_String(self.preview, serializer);
    sse_encode_opt_box_autoadd_u_64(self.remindAtMs, serializer);
    sse_encode_opt_box_autoadd_u_64(self.doneAtMs, serializer);
    sse_encode_shelf_bucket(self.bucket, serializer);
    sse_encode_bool(self.pending, serializer);
  }

  @protected
  void sse_encode_shelf_item_id(ShelfItemId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_shelf_snapshot(ShelfSnapshot self, SseSerializer serializer) {
    sse_encode_bool(self.open, serializer);
    sse_encode_list_shelf_entry(self.due, serializer);
    sse_encode_list_shelf_entry(self.upcoming, serializer);
    sse_encode_list_shelf_entry(self.done, serializer);
    sse_encode_u_32(self.dueCount, serializer);
    sse_encode_opt_box_autoadd_shelf_entry(self.latestReminder, serializer);
    sse_encode_list_reminder_preset(self.presets, serializer);
    sse_encode_opt_box_autoadd_u_64(self.pickerAnchorMs, serializer);
    sse_encode_opt_box_autoadd_u_64(self.pickerLatestMs, serializer);
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
  void sse_encode_start_note_request(
    StartNoteRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_list_account_id(self.accountIds, serializer);
  }

  @protected
  void sse_encode_steward_entry(StewardEntry self, SseSerializer serializer) {
    sse_encode_account_id(self.stewardId, serializer);
    sse_encode_String(self.displayName, serializer);
    sse_encode_String(self.mentionToken, serializer);
    sse_encode_list_room_id(self.roomIds, serializer);
    sse_encode_bool(self.revoked, serializer);
    sse_encode_bool(self.manageable, serializer);
  }

  @protected
  void sse_encode_steward_snapshot(
    StewardSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_bool(self.open, serializer);
    sse_encode_list_steward_entry(self.stewards, serializer);
    sse_encode_bool(self.canManage, serializer);
    sse_encode_opt_box_autoadd_issued_steward_token(self.issued, serializer);
    sse_encode_bool(self.pending, serializer);
    sse_encode_opt_box_autoadd_api_error(self.error, serializer);
  }

  @protected
  void sse_encode_steward_tool(StewardTool self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_stream_chunk(StreamChunk self, SseSerializer serializer) {
    sse_encode_stream_id(self.streamId, serializer);
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_opt_box_autoadd_message_id(self.threadRootId, serializer);
    sse_encode_stream_phase(self.phase, serializer);
    sse_encode_u_32(self.sequence, serializer);
    sse_encode_String(self.text, serializer);
  }

  @protected
  void sse_encode_stream_id(StreamId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_stream_phase(StreamPhase self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_stream_state(StreamState self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_subscription_state(
    SubscriptionState self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_thread_preview(ThreadPreview self, SseSerializer serializer) {
    sse_encode_opt_String(self.topicTitle, serializer);
    sse_encode_u_32(self.replyCount, serializer);
    sse_encode_opt_box_autoadd_reply_preview(self.latestReply, serializer);
    sse_encode_bool(self.showLatestReply, serializer);
    sse_encode_opt_box_autoadd_subscription_state(
      self.subscription,
      serializer,
    );
    sse_encode_u_32(self.unreadReplies, serializer);
  }

  @protected
  void sse_encode_thread_snapshot(
    ThreadSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_opt_box_autoadd_room_id(self.roomId, serializer);
    sse_encode_opt_box_autoadd_message_id(self.rootMessageId, serializer);
    sse_encode_opt_box_autoadd_timeline_entry(self.root, serializer);
    sse_encode_list_timeline_entry(self.replies, serializer);
    sse_encode_opt_String(self.topicTitle, serializer);
    sse_encode_opt_box_autoadd_subscription_state(
      self.subscription,
      serializer,
    );
    sse_encode_bool(self.syncing, serializer);
    sse_encode_list_steward_entry(self.mentionableStewards, serializer);
  }

  @protected
  void sse_encode_timeline_entry(TimelineEntry self, SseSerializer serializer) {
    sse_encode_client_nonce(self.clientNonce, serializer);
    sse_encode_opt_box_autoadd_message_id(self.messageId, serializer);
    sse_encode_account_id(self.authorId, serializer);
    sse_encode_String(self.authorDisplayName, serializer);
    sse_encode_String(self.authorMentionToken, serializer);
    sse_encode_bool(self.fromSelf, serializer);
    sse_encode_String(self.body, serializer);
    sse_encode_list_body_segment(self.segments, serializer);
    sse_encode_opt_box_autoadd_message_id(self.threadRootId, serializer);
    sse_encode_list_reaction_summary(self.reactions, serializer);
    sse_encode_opt_box_autoadd_thread_preview(self.thread, serializer);
    sse_encode_delivery_state(self.delivery, serializer);
    sse_encode_opt_box_autoadd_message_rejection(self.rejection, serializer);
    sse_encode_list_attachment_view(self.attachments, serializer);
    sse_encode_list_draft_attachment_view(self.pendingDrafts, serializer);
    sse_encode_bool(self.pinned, serializer);
    sse_encode_bool(self.shelved, serializer);
    sse_encode_bool(self.authorIsSteward, serializer);
    sse_encode_opt_box_autoadd_stream_state(self.stream, serializer);
  }

  @protected
  void sse_encode_timeline_snapshot(
    TimelineSnapshot self,
    SseSerializer serializer,
  ) {
    sse_encode_opt_box_autoadd_room_id(self.roomId, serializer);
    sse_encode_list_timeline_entry(self.entries, serializer);
    sse_encode_bool(self.syncing, serializer);
    sse_encode_opt_box_autoadd_api_error(self.error, serializer);
    sse_encode_list_String(self.reactionPalette, serializer);
    sse_encode_list_draft_attachment_view(self.drafts, serializer);
    sse_encode_opt_box_autoadd_message_id(self.focusMessageId, serializer);
    sse_encode_bool(self.canPin, serializer);
    sse_encode_bool(self.canUpload, serializer);
    sse_encode_list_steward_entry(self.mentionableStewards, serializer);
  }

  @protected
  void sse_encode_tool_call_id(ToolCallId self, SseSerializer serializer) {
    sse_encode_Uuid(self.field0, serializer);
  }

  @protected
  void sse_encode_tool_descriptor(
    ToolDescriptor self,
    SseSerializer serializer,
  ) {
    sse_encode_String(self.name, serializer);
    sse_encode_String(self.description, serializer);
    sse_encode_String(self.inputSchema, serializer);
  }

  @protected
  void sse_encode_tool_error_code(
    ToolErrorCode self,
    SseSerializer serializer,
  ) {
    sse_encode_i_32(self.index, serializer);
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
  void sse_encode_u_64(BigInt self, SseSerializer serializer) {
    serializer.buffer.putBigUint64(self);
  }

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer) {
    serializer.buffer.putUint8(self);
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {}

  @protected
  void sse_encode_update_member_request(
    UpdateMemberRequest self,
    SseSerializer serializer,
  ) {
    sse_encode_role(self.role, serializer);
  }

  @protected
  void sse_encode_update_shelf(UpdateShelf self, SseSerializer serializer) {
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_message_id(self.messageId, serializer);
    sse_encode_shelf_action(self.action, serializer);
    sse_encode_opt_box_autoadd_u_64(self.remindAtMs, serializer);
  }

  @protected
  void sse_encode_upload_state(UploadState self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_usize(BigInt self, SseSerializer serializer) {
    serializer.buffer.putBigUint64(self);
  }

  @protected
  void sse_encode_vault_entry(VaultEntry self, SseSerializer serializer) {
    sse_encode_vault_tab(self.tab, serializer);
    sse_encode_room_id(self.roomId, serializer);
    sse_encode_message_id(self.messageId, serializer);
    sse_encode_String(self.authorDisplayName, serializer);
    sse_encode_String(self.preview, serializer);
    sse_encode_opt_box_autoadd_attachment_view(self.attachment, serializer);
    sse_encode_opt_String(self.link, serializer);
    sse_encode_opt_box_autoadd_u_64(self.pinnedAtMs, serializer);
  }

  @protected
  void sse_encode_vault_snapshot(VaultSnapshot self, SseSerializer serializer) {
    sse_encode_bool(self.open, serializer);
    sse_encode_opt_box_autoadd_room_id(self.roomId, serializer);
    sse_encode_vault_tab(self.tab, serializer);
    sse_encode_list_vault_entry(self.entries, serializer);
    sse_encode_bool(self.loading, serializer);
    sse_encode_bool(self.hasMore, serializer);
    sse_encode_opt_box_autoadd_api_error(self.error, serializer);
  }

  @protected
  void sse_encode_vault_tab(VaultTab self, SseSerializer serializer) {
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_version_mismatch(
    VersionMismatch self,
    SseSerializer serializer,
  ) {
    sse_encode_protocol_version(self.supported, serializer);
    sse_encode_protocol_version(self.received, serializer);
  }
}

@sealed
class StreamWriterImpl extends RustOpaque implements StreamWriter {
  StreamWriterImpl.frbInternalDcoDecode(List<dynamic> wire)
    : super.frbInternalDcoDecode(wire, _kStaticData);

  StreamWriterImpl.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
    : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_StreamWriter,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_StreamWriter,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_StreamWriterPtr,
  );

  Future<StreamChunk> abort() =>
      RustLib.instance.api.pulseProtocolStewardStreamWriterAbort(that: this);

  Future<StreamChunk> begin() =>
      RustLib.instance.api.pulseProtocolStewardStreamWriterBegin(that: this);

  Future<StreamChunk> delta({required String text}) => RustLib.instance.api
      .pulseProtocolStewardStreamWriterDelta(that: this, text: text);

  Future<StreamChunk> end() =>
      RustLib.instance.api.pulseProtocolStewardStreamWriterEnd(that: this);

  Future<int> nextSequence() => RustLib.instance.api
      .pulseProtocolStewardStreamWriterNextSequence(that: this);
}
