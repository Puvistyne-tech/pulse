import '../frb_generated.dart';
import '../third_party/pulse_protocol/api.dart';
import '../third_party/pulse_protocol/auth.dart';
import '../third_party/pulse_protocol/campus.dart';
import '../third_party/pulse_protocol/frames.dart';
import '../third_party/pulse_protocol/ids.dart';
import '../third_party/pulse_protocol/media.dart';
import '../third_party/pulse_protocol/mention.dart';
import '../third_party/pulse_protocol/messaging.dart';
import '../third_party/pulse_protocol/rbac.dart';
import '../third_party/pulse_protocol/reaction.dart';
import '../third_party/pulse_protocol/shelf.dart';
import '../third_party/pulse_protocol/snapshot.dart';
import '../third_party/pulse_protocol/thread.dart';
import '../third_party/pulse_protocol/version.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<void> initStorage({required String dataDir}) =>
    RustLib.instance.api.crateApiClientInitStorage(dataDir: dataDir);

Future<ClientSnapshot> clientSnapshot() =>
    RustLib.instance.api.crateApiClientClientSnapshot();

Future<ClientSnapshot> signUp({required SignUpRequest request}) =>
    RustLib.instance.api.crateApiClientSignUp(request: request);

Future<ClientSnapshot> signIn({required SignInRequest request}) =>
    RustLib.instance.api.crateApiClientSignIn(request: request);

Future<ClientSnapshot> signOut() =>
    RustLib.instance.api.crateApiClientSignOut();

Future<ClientSnapshot> refreshCampuses() =>
    RustLib.instance.api.crateApiClientRefreshCampuses();

Future<ClientSnapshot> createCampus({required CreateCampusRequest request}) =>
    RustLib.instance.api.crateApiClientCreateCampus(request: request);

Future<ClientSnapshot> selectCampus({required CampusId campusId}) =>
    RustLib.instance.api.crateApiClientSelectCampus(campusId: campusId);

Future<ClientSnapshot> selectRoom({required RoomId roomId}) =>
    RustLib.instance.api.crateApiClientSelectRoom(roomId: roomId);

Future<ClientSnapshot> sendMessage({
  required RoomId roomId,
  required String body,
}) =>
    RustLib.instance.api.crateApiClientSendMessage(roomId: roomId, body: body);

Future<ClientSnapshot> discardMessage({required ClientNonce clientNonce}) =>
    RustLib.instance.api.crateApiClientDiscardMessage(clientNonce: clientNonce);

Future<ClientSnapshot> sendReply({
  required RoomId roomId,
  required MessageId rootMessageId,
  required String body,
}) => RustLib.instance.api.crateApiClientSendReply(
  roomId: roomId,
  rootMessageId: rootMessageId,
  body: body,
);

Future<ClientSnapshot> react({
  required RoomId roomId,
  required MessageId messageId,
  required String emoji,
  required ReactionAction action,
}) => RustLib.instance.api.crateApiClientReact(
  roomId: roomId,
  messageId: messageId,
  emoji: emoji,
  action: action,
);

Future<ClientSnapshot> setTopicTitle({
  required RoomId roomId,
  required MessageId rootMessageId,
  required String topicTitle,
}) => RustLib.instance.api.crateApiClientSetTopicTitle(
  roomId: roomId,
  rootMessageId: rootMessageId,
  topicTitle: topicTitle,
);

Future<ClientSnapshot> setThreadSubscription({
  required RoomId roomId,
  required MessageId rootMessageId,
  required SubscriptionState state,
}) => RustLib.instance.api.crateApiClientSetThreadSubscription(
  roomId: roomId,
  rootMessageId: rootMessageId,
  state: state,
);

Future<ClientSnapshot> openThread({
  required RoomId roomId,
  required MessageId rootMessageId,
}) => RustLib.instance.api.crateApiClientOpenThread(
  roomId: roomId,
  rootMessageId: rootMessageId,
);

Future<ClientSnapshot> closeThread() =>
    RustLib.instance.api.crateApiClientCloseThread();

Future<ClientSnapshot> toggleInbox() =>
    RustLib.instance.api.crateApiClientToggleInbox();

Future<ClientSnapshot> moveInboxSelection({required InboxStep step}) =>
    RustLib.instance.api.crateApiClientMoveInboxSelection(step: step);

Future<ClientSnapshot> archiveInboxSelection() =>
    RustLib.instance.api.crateApiClientArchiveInboxSelection();

Future<ClientSnapshot> startInboxReply() =>
    RustLib.instance.api.crateApiClientStartInboxReply();

Future<ClientSnapshot> cancelInboxReply() =>
    RustLib.instance.api.crateApiClientCancelInboxReply();

Future<ClientSnapshot> toggleSearch() =>
    RustLib.instance.api.crateApiClientToggleSearch();

Future<ClientSnapshot> search({required String query}) =>
    RustLib.instance.api.crateApiClientSearch(query: query);

Future<ClientSnapshot> moveSearchSelection({required InboxStep step}) =>
    RustLib.instance.api.crateApiClientMoveSearchSelection(step: step);

Future<ClientSnapshot> openSearchSelection() =>
    RustLib.instance.api.crateApiClientOpenSearchSelection();

Future<ClientSnapshot> openSearchHit({required MessageId messageId}) =>
    RustLib.instance.api.crateApiClientOpenSearchHit(messageId: messageId);

Future<ClientSnapshot> attachFile({
  required RoomId roomId,
  required String fileName,
  required List<int> bytes,
}) => RustLib.instance.api.crateApiClientAttachFile(
  roomId: roomId,
  fileName: fileName,
  bytes: bytes,
);

Future<ClientSnapshot> discardDraft({required DraftId draftId}) =>
    RustLib.instance.api.crateApiClientDiscardDraft(draftId: draftId);

Future<ClientSnapshot> retryDraft({required DraftId draftId}) =>
    RustLib.instance.api.crateApiClientRetryDraft(draftId: draftId);

Future<ClientSnapshot> setPin({
  required RoomId roomId,
  required MessageId messageId,
  required bool pinned,
}) => RustLib.instance.api.crateApiClientSetPin(
  roomId: roomId,
  messageId: messageId,
  pinned: pinned,
);

Future<ClientSnapshot> openVault({
  required RoomId roomId,
  required VaultTab tab,
}) => RustLib.instance.api.crateApiClientOpenVault(roomId: roomId, tab: tab);

Future<ClientSnapshot> loadMoreVault() =>
    RustLib.instance.api.crateApiClientLoadMoreVault();

Future<ClientSnapshot> closeVault() =>
    RustLib.instance.api.crateApiClientCloseVault();

Future<ClientSnapshot> toggleShelf() =>
    RustLib.instance.api.crateApiClientToggleShelf();

Future<ClientSnapshot> updateShelf({
  required RoomId roomId,
  required MessageId messageId,
  required ShelfAction action,
  ReminderPreset? preset,
  BigInt? remindAtMs,
}) => RustLib.instance.api.crateApiClientUpdateShelf(
  roomId: roomId,
  messageId: messageId,
  action: action,
  preset: preset,
  remindAtMs: remindAtMs,
);

Future<ClientSnapshot> dismissReminder() =>
    RustLib.instance.api.crateApiClientDismissReminder();

Stream<ClientSnapshot> watchSnapshots() =>
    RustLib.instance.api.crateApiClientWatchSnapshots();
