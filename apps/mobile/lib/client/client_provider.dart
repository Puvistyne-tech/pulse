import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api/client.dart' as bridge;
import '../src/rust/third_party/pulse_protocol/auth.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/media.dart';
import '../src/rust/third_party/pulse_protocol/reaction.dart';
import '../src/rust/third_party/pulse_protocol/shelf.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../src/rust/third_party/pulse_protocol/thread.dart';
import 'snapshot_order.dart';

final clientProvider = AsyncNotifierProvider<ClientNotifier, ClientSnapshot>(
  ClientNotifier.new,
);

class ClientNotifier extends AsyncNotifier<ClientSnapshot> {
  @override
  Future<ClientSnapshot> build() async {
    final subscription = bridge.watchSnapshots().listen(_apply);
    ref.onDispose(subscription.cancel);
    return newerSnapshot(state.value, await bridge.clientSnapshot());
  }

  Future<void> signUp(SignUpRequest request) =>
      _publish(bridge.signUp(request: request));

  Future<void> signIn(SignInRequest request) =>
      _publish(bridge.signIn(request: request));

  Future<void> signOut() => _publish(bridge.signOut());

  Future<void> refreshCampuses() => _publish(bridge.refreshCampuses());

  Future<void> createCampus(CreateCampusRequest request) =>
      _publish(bridge.createCampus(request: request));

  Future<void> selectCampus(CampusId campusId) =>
      _publish(bridge.selectCampus(campusId: campusId));

  Future<void> selectRoom(RoomId roomId) =>
      _publish(bridge.selectRoom(roomId: roomId));

  Future<void> sendMessage(RoomId roomId, String body) =>
      _publish(bridge.sendMessage(roomId: roomId, body: body));

  Future<void> discardMessage(ClientNonce clientNonce) =>
      _publish(bridge.discardMessage(clientNonce: clientNonce));

  Future<void> sendReply(RoomId roomId, MessageId rootMessageId, String body) =>
      _publish(
        bridge.sendReply(
          roomId: roomId,
          rootMessageId: rootMessageId,
          body: body,
        ),
      );

  Future<void> react(
    RoomId roomId,
    MessageId messageId,
    String emoji,
    ReactionAction action,
  ) => _publish(
    bridge.react(
      roomId: roomId,
      messageId: messageId,
      emoji: emoji,
      action: action,
    ),
  );

  Future<void> setTopicTitle(
    RoomId roomId,
    MessageId rootMessageId,
    String topicTitle,
  ) => _publish(
    bridge.setTopicTitle(
      roomId: roomId,
      rootMessageId: rootMessageId,
      topicTitle: topicTitle,
    ),
  );

  Future<void> setThreadSubscription(
    RoomId roomId,
    MessageId rootMessageId,
    SubscriptionState state,
  ) => _publish(
    bridge.setThreadSubscription(
      roomId: roomId,
      rootMessageId: rootMessageId,
      state: state,
    ),
  );

  Future<void> openThread(RoomId roomId, MessageId rootMessageId) =>
      _publish(bridge.openThread(roomId: roomId, rootMessageId: rootMessageId));

  Future<void> closeThread() => _publish(bridge.closeThread());

  Future<void> toggleInbox() => _publish(bridge.toggleInbox());

  Future<void> moveInboxSelection(InboxStep step) =>
      _publish(bridge.moveInboxSelection(step: step));

  Future<void> archiveInboxSelection() =>
      _publish(bridge.archiveInboxSelection());

  Future<void> startInboxReply() => _publish(bridge.startInboxReply());

  Future<void> cancelInboxReply() => _publish(bridge.cancelInboxReply());

  Future<void> toggleSearch() => _publish(bridge.toggleSearch());

  Future<void> search(String query) => _publish(bridge.search(query: query));

  Future<void> moveSearchSelection(InboxStep step) =>
      _publish(bridge.moveSearchSelection(step: step));

  Future<void> openSearchSelection() => _publish(bridge.openSearchSelection());

  Future<void> openSearchHit(MessageId messageId) =>
      _publish(bridge.openSearchHit(messageId: messageId));

  Future<void> attachFile(RoomId roomId, String fileName, List<int> bytes) =>
      _publish(
        bridge.attachFile(roomId: roomId, fileName: fileName, bytes: bytes),
      );

  Future<void> discardDraft(DraftId draftId) =>
      _publish(bridge.discardDraft(draftId: draftId));

  Future<void> retryDraft(DraftId draftId) =>
      _publish(bridge.retryDraft(draftId: draftId));

  Future<void> setPin(RoomId roomId, MessageId messageId, bool pinned) =>
      _publish(
        bridge.setPin(roomId: roomId, messageId: messageId, pinned: pinned),
      );

  Future<void> openVault(RoomId roomId, VaultTab tab) =>
      _publish(bridge.openVault(roomId: roomId, tab: tab));

  Future<void> loadMoreVault() => _publish(bridge.loadMoreVault());

  Future<void> closeVault() => _publish(bridge.closeVault());

  Future<void> toggleShelf() => _publish(bridge.toggleShelf());

  Future<void> updateShelf(
    RoomId roomId,
    MessageId messageId,
    ShelfAction action,
    ReminderPreset? preset,
    BigInt? remindAtMs,
  ) => _publish(
    bridge.updateShelf(
      roomId: roomId,
      messageId: messageId,
      action: action,
      preset: preset,
      remindAtMs: remindAtMs,
    ),
  );

  Future<void> dismissReminder() => _publish(bridge.dismissReminder());

  void _apply(ClientSnapshot next) {
    state = AsyncData(newerSnapshot(state.value, next));
  }

  Future<void> _publish(Future<ClientSnapshot> pending) async {
    _apply(await pending);
  }
}
