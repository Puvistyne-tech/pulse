import '../src/rust/third_party/pulse_protocol/auth.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/media.dart';
import '../src/rust/third_party/pulse_protocol/reaction.dart';
import '../src/rust/third_party/pulse_protocol/shelf.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../src/rust/third_party/pulse_protocol/thread.dart';

class PulseIntents {
  const PulseIntents({
    required this.signUp,
    required this.signIn,
    required this.signOut,
    required this.createCampus,
    required this.selectCampus,
    required this.refreshCampuses,
    required this.selectRoom,
    required this.sendMessage,
    required this.discardMessage,
    required this.sendReply,
    required this.react,
    required this.setTopicTitle,
    required this.setThreadSubscription,
    required this.openThread,
    required this.closeThread,
    required this.toggleInbox,
    required this.moveInboxSelection,
    required this.archiveInboxSelection,
    required this.startInboxReply,
    required this.cancelInboxReply,
    required this.toggleSearch,
    required this.search,
    required this.moveSearchSelection,
    required this.openSearchSelection,
    required this.openSearchHit,
    required this.attachFile,
    required this.discardDraft,
    required this.retryDraft,
    required this.setPin,
    required this.openVault,
    required this.loadMoreVault,
    required this.closeVault,
    required this.toggleShelf,
    required this.updateShelf,
    required this.dismissReminder,
  });

  final Future<void> Function(SignUpRequest request) signUp;
  final Future<void> Function(SignInRequest request) signIn;
  final Future<void> Function() signOut;
  final Future<void> Function(CreateCampusRequest request) createCampus;
  final Future<void> Function(CampusId campusId) selectCampus;
  final Future<void> Function() refreshCampuses;
  final Future<void> Function(RoomId roomId) selectRoom;
  final Future<void> Function(RoomId roomId, String body) sendMessage;
  final Future<void> Function(ClientNonce clientNonce) discardMessage;
  final Future<void> Function(
    RoomId roomId,
    MessageId rootMessageId,
    String body,
  )
  sendReply;
  final Future<void> Function(
    RoomId roomId,
    MessageId messageId,
    String emoji,
    ReactionAction action,
  )
  react;
  final Future<void> Function(
    RoomId roomId,
    MessageId rootMessageId,
    String topicTitle,
  )
  setTopicTitle;
  final Future<void> Function(
    RoomId roomId,
    MessageId rootMessageId,
    SubscriptionState state,
  )
  setThreadSubscription;
  final Future<void> Function(RoomId roomId, MessageId rootMessageId)
  openThread;
  final Future<void> Function() closeThread;
  final Future<void> Function() toggleInbox;
  final Future<void> Function(InboxStep step) moveInboxSelection;
  final Future<void> Function() archiveInboxSelection;
  final Future<void> Function() startInboxReply;
  final Future<void> Function() cancelInboxReply;
  final Future<void> Function() toggleSearch;
  final Future<void> Function(String query) search;
  final Future<void> Function(InboxStep step) moveSearchSelection;
  final Future<void> Function() openSearchSelection;
  final Future<void> Function(MessageId messageId) openSearchHit;
  final Future<void> Function(RoomId roomId, String fileName, List<int> bytes)
  attachFile;
  final Future<void> Function(DraftId draftId) discardDraft;
  final Future<void> Function(DraftId draftId) retryDraft;
  final Future<void> Function(RoomId roomId, MessageId messageId, bool pinned)
  setPin;
  final Future<void> Function(RoomId roomId, VaultTab tab) openVault;
  final Future<void> Function() loadMoreVault;
  final Future<void> Function() closeVault;
  final Future<void> Function() toggleShelf;
  final Future<void> Function(
    RoomId roomId,
    MessageId messageId,
    ShelfAction action,
    ReminderPreset? preset,
    BigInt? remindAtMs,
  )
  updateShelf;
  final Future<void> Function() dismissReminder;
}
