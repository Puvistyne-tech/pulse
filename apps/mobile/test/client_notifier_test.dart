import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pulse_mobile/client/client_provider.dart';
import 'package:pulse_mobile/src/rust/frb_generated.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/auth.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/campus.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/ids.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/media.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/reaction.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/shelf.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/thread.dart';

import 'fixtures.dart';

class FakeRustApi extends Fake implements RustLibApi {
  final calls = <Object>[];
  final pushes = StreamController<ClientSnapshot>.broadcast();

  @override
  Stream<ClientSnapshot> crateApiClientWatchSnapshots() => pushes.stream;

  @override
  Future<ClientSnapshot> crateApiClientSelectRoom({
    required RoomId roomId,
  }) async {
    calls.add(roomId);
    return snapshotAt(signedInWithRooms, 5);
  }

  @override
  Future<ClientSnapshot> crateApiClientSendMessage({
    required RoomId roomId,
    required String body,
  }) async {
    calls.add((roomId, body));
    return snapshotAt(signedInWithMessages, 6);
  }

  @override
  Future<ClientSnapshot> crateApiClientDiscardMessage({
    required ClientNonce clientNonce,
  }) async {
    calls.add(clientNonce);
    return snapshotAt(signedInWithRooms, 7);
  }

  @override
  Future<ClientSnapshot> crateApiClientSendReply({
    required RoomId roomId,
    required MessageId rootMessageId,
    required String body,
  }) async {
    calls.add(('sendReply', roomId, rootMessageId, body));
    return snapshotAt(signedInWithMessages, 8);
  }

  @override
  Future<ClientSnapshot> crateApiClientReact({
    required RoomId roomId,
    required MessageId messageId,
    required String emoji,
    required ReactionAction action,
  }) async {
    calls.add(('react', roomId, messageId, emoji, action));
    return snapshotAt(signedInWithMessages, 9);
  }

  @override
  Future<ClientSnapshot> crateApiClientSetTopicTitle({
    required RoomId roomId,
    required MessageId rootMessageId,
    required String topicTitle,
  }) async {
    calls.add(('setTopicTitle', roomId, rootMessageId, topicTitle));
    return snapshotAt(signedInWithMessages, 10);
  }

  @override
  Future<ClientSnapshot> crateApiClientSetThreadSubscription({
    required RoomId roomId,
    required MessageId rootMessageId,
    required SubscriptionState state,
  }) async {
    calls.add(('setThreadSubscription', roomId, rootMessageId, state));
    return snapshotAt(signedInWithMessages, 11);
  }

  @override
  Future<ClientSnapshot> crateApiClientOpenThread({
    required RoomId roomId,
    required MessageId rootMessageId,
  }) async {
    calls.add(('openThread', roomId, rootMessageId));
    return snapshotAt(signedInWithMessages, 12);
  }

  @override
  Future<ClientSnapshot> crateApiClientCloseThread() async {
    calls.add('closeThread');
    return snapshotAt(signedInWithMessages, 13);
  }

  @override
  Future<ClientSnapshot> crateApiClientToggleInbox() async {
    calls.add('toggleInbox');
    return snapshotAt(signedInWithMessages, 14);
  }

  @override
  Future<ClientSnapshot> crateApiClientMoveInboxSelection({
    required InboxStep step,
  }) async {
    calls.add(('moveInboxSelection', step));
    return snapshotAt(signedInWithMessages, 15);
  }

  @override
  Future<ClientSnapshot> crateApiClientArchiveInboxSelection() async {
    calls.add('archiveInboxSelection');
    return snapshotAt(signedInWithMessages, 16);
  }

  @override
  Future<ClientSnapshot> crateApiClientStartInboxReply() async {
    calls.add('startInboxReply');
    return snapshotAt(signedInWithMessages, 17);
  }

  @override
  Future<ClientSnapshot> crateApiClientCancelInboxReply() async {
    calls.add('cancelInboxReply');
    return snapshotAt(signedInWithMessages, 18);
  }

  @override
  Future<ClientSnapshot> crateApiClientToggleSearch() async {
    calls.add('toggleSearch');
    return snapshotAt(signedInWithMessages, 19);
  }

  @override
  Future<ClientSnapshot> crateApiClientSearch({required String query}) async {
    calls.add(('search', query));
    return snapshotAt(signedInWithMessages, 20);
  }

  @override
  Future<ClientSnapshot> crateApiClientMoveSearchSelection({
    required InboxStep step,
  }) async {
    calls.add(('moveSearchSelection', step));
    return snapshotAt(signedInWithMessages, 21);
  }

  @override
  Future<ClientSnapshot> crateApiClientOpenSearchSelection() async {
    calls.add('openSearchSelection');
    return snapshotAt(signedInWithMessages, 22);
  }

  @override
  Future<ClientSnapshot> crateApiClientOpenSearchHit({
    required MessageId messageId,
  }) async {
    calls.add(('openSearchHit', messageId));
    return snapshotAt(signedInWithMessages, 23);
  }

  @override
  Future<ClientSnapshot> crateApiClientAttachFile({
    required RoomId roomId,
    required String fileName,
    required List<int> bytes,
  }) async {
    calls.add(('attachFile', roomId, fileName, bytes));
    return snapshotAt(signedInWithMessages, 24);
  }

  @override
  Future<ClientSnapshot> crateApiClientDiscardDraft({
    required DraftId draftId,
  }) async {
    calls.add(('discardDraft', draftId));
    return snapshotAt(signedInWithMessages, 25);
  }

  @override
  Future<ClientSnapshot> crateApiClientRetryDraft({
    required DraftId draftId,
  }) async {
    calls.add(('retryDraft', draftId));
    return snapshotAt(signedInWithMessages, 26);
  }

  @override
  Future<ClientSnapshot> crateApiClientSetPin({
    required RoomId roomId,
    required MessageId messageId,
    required bool pinned,
  }) async {
    calls.add(('setPin', roomId, messageId, pinned));
    return snapshotAt(signedInWithMessages, 27);
  }

  @override
  Future<ClientSnapshot> crateApiClientOpenVault({
    required RoomId roomId,
    required VaultTab tab,
  }) async {
    calls.add(('openVault', roomId, tab));
    return snapshotAt(signedInWithMessages, 28);
  }

  @override
  Future<ClientSnapshot> crateApiClientLoadMoreVault() async {
    calls.add('loadMoreVault');
    return snapshotAt(signedInWithMessages, 29);
  }

  @override
  Future<ClientSnapshot> crateApiClientCloseVault() async {
    calls.add('closeVault');
    return snapshotAt(signedInWithMessages, 30);
  }

  @override
  Future<ClientSnapshot> crateApiClientToggleShelf() async {
    calls.add('toggleShelf');
    return snapshotAt(signedInWithMessages, 31);
  }

  @override
  Future<ClientSnapshot> crateApiClientUpdateShelf({
    required RoomId roomId,
    required MessageId messageId,
    required ShelfAction action,
    ReminderPreset? preset,
    BigInt? remindAtMs,
  }) async {
    calls.add(('updateShelf', roomId, messageId, action, preset, remindAtMs));
    return snapshotAt(signedInWithMessages, 32);
  }

  @override
  Future<ClientSnapshot> crateApiClientDismissReminder() async {
    calls.add('dismissReminder');
    return snapshotAt(signedInWithMessages, 33);
  }

  @override
  Future<ClientSnapshot> crateApiClientClientSnapshot() async => signedOut;

  @override
  Future<ClientSnapshot> crateApiClientSignUp({
    required SignUpRequest request,
  }) async {
    calls.add(request);
    return authenticating;
  }

  @override
  Future<ClientSnapshot> crateApiClientSignIn({
    required SignInRequest request,
  }) async {
    calls.add(request);
    return signedInNoCampus;
  }

  @override
  Future<ClientSnapshot> crateApiClientSignOut() async {
    calls.add('signOut');
    return signedOutWithError;
  }

  @override
  Future<ClientSnapshot> crateApiClientRefreshCampuses() async {
    calls.add('refreshCampuses');
    return signedInNoCampus;
  }

  @override
  Future<ClientSnapshot> crateApiClientCreateCampus({
    required CreateCampusRequest request,
  }) async {
    calls.add(request);
    return signedInWithRooms;
  }

  @override
  Future<ClientSnapshot> crateApiClientSelectCampus({
    required CampusId campusId,
  }) async {
    calls.add(campusId);
    return signedInWithRooms;
  }
}

class IntentCase {
  const IntentCase(this.name, this.run, this.expectedCall, this.expectedState);

  final String name;
  final Future<void> Function(ClientNotifier notifier) run;
  final Object expectedCall;
  final ClientSnapshot expectedState;
}

final intentCases = <IntentCase>[
  IntentCase(
    'signUp',
    (notifier) => notifier.signUp(signUpRequest),
    signUpRequest,
    authenticating,
  ),
  IntentCase(
    'signIn',
    (notifier) => notifier.signIn(signInRequest),
    signInRequest,
    signedInNoCampus,
  ),
  IntentCase(
    'signOut',
    (notifier) => notifier.signOut(),
    'signOut',
    signedOutWithError,
  ),
  IntentCase(
    'refreshCampuses',
    (notifier) => notifier.refreshCampuses(),
    'refreshCampuses',
    signedInNoCampus,
  ),
  IntentCase(
    'createCampus',
    (notifier) => notifier.createCampus(createCampusRequest),
    createCampusRequest,
    signedInWithRooms,
  ),
  IntentCase(
    'selectCampus',
    (notifier) => notifier.selectCampus(campusBId),
    campusBId,
    signedInWithRooms,
  ),
  IntentCase(
    'selectRoom',
    (notifier) => notifier.selectRoom(roomBId),
    roomBId,
    snapshotAt(signedInWithRooms, 5),
  ),
  IntentCase(
    'sendMessage',
    (notifier) => notifier.sendMessage(roomAId, '  hello  '),
    (roomAId, '  hello  '),
    snapshotAt(signedInWithMessages, 6),
  ),
  IntentCase(
    'discardMessage',
    (notifier) => notifier.discardMessage(nonceC),
    nonceC,
    snapshotAt(signedInWithRooms, 7),
  ),
  IntentCase(
    'sendReply',
    (notifier) => notifier.sendReply(roomAId, messageAId, 'reply'),
    ('sendReply', roomAId, messageAId, 'reply'),
    snapshotAt(signedInWithMessages, 8),
  ),
  IntentCase(
    'react add',
    (notifier) => notifier.react(roomAId, messageAId, '👍', ReactionAction.add),
    ('react', roomAId, messageAId, '👍', ReactionAction.add),
    snapshotAt(signedInWithMessages, 9),
  ),
  IntentCase(
    'react remove',
    (notifier) =>
        notifier.react(roomAId, messageAId, '🎉', ReactionAction.remove),
    ('react', roomAId, messageAId, '🎉', ReactionAction.remove),
    snapshotAt(signedInWithMessages, 9),
  ),
  IntentCase(
    'setTopicTitle',
    (notifier) => notifier.setTopicTitle(roomAId, messageAId, 'Plans'),
    ('setTopicTitle', roomAId, messageAId, 'Plans'),
    snapshotAt(signedInWithMessages, 10),
  ),
  IntentCase(
    'setThreadSubscription',
    (notifier) => notifier.setThreadSubscription(
      roomAId,
      messageAId,
      SubscriptionState.muted,
    ),
    ('setThreadSubscription', roomAId, messageAId, SubscriptionState.muted),
    snapshotAt(signedInWithMessages, 11),
  ),
  IntentCase(
    'openThread',
    (notifier) => notifier.openThread(roomAId, messageAId),
    ('openThread', roomAId, messageAId),
    snapshotAt(signedInWithMessages, 12),
  ),
  IntentCase(
    'closeThread',
    (notifier) => notifier.closeThread(),
    'closeThread',
    snapshotAt(signedInWithMessages, 13),
  ),
  IntentCase(
    'toggleInbox',
    (notifier) => notifier.toggleInbox(),
    'toggleInbox',
    snapshotAt(signedInWithMessages, 14),
  ),
  IntentCase(
    'moveInboxSelection next',
    (notifier) => notifier.moveInboxSelection(InboxStep.next),
    ('moveInboxSelection', InboxStep.next),
    snapshotAt(signedInWithMessages, 15),
  ),
  IntentCase(
    'moveInboxSelection previous',
    (notifier) => notifier.moveInboxSelection(InboxStep.previous),
    ('moveInboxSelection', InboxStep.previous),
    snapshotAt(signedInWithMessages, 15),
  ),
  IntentCase(
    'archiveInboxSelection',
    (notifier) => notifier.archiveInboxSelection(),
    'archiveInboxSelection',
    snapshotAt(signedInWithMessages, 16),
  ),
  IntentCase(
    'startInboxReply',
    (notifier) => notifier.startInboxReply(),
    'startInboxReply',
    snapshotAt(signedInWithMessages, 17),
  ),
  IntentCase(
    'cancelInboxReply',
    (notifier) => notifier.cancelInboxReply(),
    'cancelInboxReply',
    snapshotAt(signedInWithMessages, 18),
  ),
  IntentCase(
    'toggleSearch',
    (notifier) => notifier.toggleSearch(),
    'toggleSearch',
    snapshotAt(signedInWithMessages, 19),
  ),
  IntentCase('search', (notifier) => notifier.search('alpha beta'), (
    'search',
    'alpha beta',
  ), snapshotAt(signedInWithMessages, 20)),
  IntentCase(
    'moveSearchSelection next',
    (notifier) => notifier.moveSearchSelection(InboxStep.next),
    ('moveSearchSelection', InboxStep.next),
    snapshotAt(signedInWithMessages, 21),
  ),
  IntentCase(
    'moveSearchSelection previous',
    (notifier) => notifier.moveSearchSelection(InboxStep.previous),
    ('moveSearchSelection', InboxStep.previous),
    snapshotAt(signedInWithMessages, 21),
  ),
  IntentCase(
    'openSearchSelection',
    (notifier) => notifier.openSearchSelection(),
    'openSearchSelection',
    snapshotAt(signedInWithMessages, 22),
  ),
  IntentCase(
    'openSearchHit',
    (notifier) => notifier.openSearchHit(messageAId),
    ('openSearchHit', messageAId),
    snapshotAt(signedInWithMessages, 23),
  ),
  IntentCase(
    'attachFile',
    (notifier) => notifier.attachFile(roomAId, 'plan.png', const [1, 2, 3]),
    ('attachFile', roomAId, 'plan.png', const [1, 2, 3]),
    snapshotAt(signedInWithMessages, 24),
  ),
  IntentCase('discardDraft', (notifier) => notifier.discardDraft(draftAId), (
    'discardDraft',
    draftAId,
  ), snapshotAt(signedInWithMessages, 25)),
  IntentCase('retryDraft', (notifier) => notifier.retryDraft(draftAId), (
    'retryDraft',
    draftAId,
  ), snapshotAt(signedInWithMessages, 26)),
  IntentCase(
    'setPin pins',
    (notifier) => notifier.setPin(roomAId, messageAId, true),
    ('setPin', roomAId, messageAId, true),
    snapshotAt(signedInWithMessages, 27),
  ),
  IntentCase(
    'setPin unpins',
    (notifier) => notifier.setPin(roomAId, messageAId, false),
    ('setPin', roomAId, messageAId, false),
    snapshotAt(signedInWithMessages, 27),
  ),
  for (final tab in VaultTab.values)
    IntentCase(
      'openVault ${tab.name}',
      (notifier) => notifier.openVault(roomAId, tab),
      ('openVault', roomAId, tab),
      snapshotAt(signedInWithMessages, 28),
    ),
  IntentCase(
    'loadMoreVault',
    (notifier) => notifier.loadMoreVault(),
    'loadMoreVault',
    snapshotAt(signedInWithMessages, 29),
  ),
  IntentCase(
    'closeVault',
    (notifier) => notifier.closeVault(),
    'closeVault',
    snapshotAt(signedInWithMessages, 30),
  ),
  IntentCase(
    'toggleShelf',
    (notifier) => notifier.toggleShelf(),
    'toggleShelf',
    snapshotAt(signedInWithMessages, 31),
  ),
  IntentCase(
    'updateShelf with a preset',
    (notifier) => notifier.updateShelf(
      roomAId,
      messageAId,
      ShelfAction.reschedule,
      ReminderPreset.inOneHour,
      null,
    ),
    (
      'updateShelf',
      roomAId,
      messageAId,
      ShelfAction.reschedule,
      ReminderPreset.inOneHour,
      null,
    ),
    snapshotAt(signedInWithMessages, 32),
  ),
  IntentCase(
    'updateShelf with a custom time',
    (notifier) => notifier.updateShelf(
      roomAId,
      messageAId,
      ShelfAction.save,
      null,
      BigInt.from(1234),
    ),
    (
      'updateShelf',
      roomAId,
      messageAId,
      ShelfAction.save,
      null,
      BigInt.from(1234),
    ),
    snapshotAt(signedInWithMessages, 32),
  ),
  IntentCase(
    'dismissReminder',
    (notifier) => notifier.dismissReminder(),
    'dismissReminder',
    snapshotAt(signedInWithMessages, 33),
  ),
];

class PushCase {
  const PushCase(this.name, this.pushes, this.expected);

  final String name;
  final List<ClientSnapshot> pushes;
  final ClientSnapshot expected;
}

final pushCases = <PushCase>[
  PushCase('newer push is applied', [
    snapshotAt(signedInNoCampus, 5),
  ], snapshotAt(signedInNoCampus, 5)),
  PushCase('equal sequence push is applied', [
    signedInNoCampus,
  ], signedInNoCampus),
  PushCase('stale push is dropped', [
    snapshotAt(signedInNoCampus, 0),
  ], signedOut),
  PushCase('out of order pushes settle on the newest', [
    snapshotAt(signedInNoCampus, 6),
    snapshotAt(signedInWithRooms, 4),
    snapshotAt(signedInWithMessages, 5),
  ], snapshotAt(signedInNoCampus, 6)),
  PushCase('pushes in order keep the last', [
    snapshotAt(signedInNoCampus, 2),
    snapshotAt(signedInWithRooms, 3),
    snapshotAt(signedInWithMessages, 4),
  ], snapshotAt(signedInWithMessages, 4)),
];

void main() {
  late FakeRustApi api;
  late ProviderContainer container;

  setUp(() {
    api = FakeRustApi();
    RustLib.initMock(api: api);
    container = ProviderContainer();
  });

  tearDown(() async {
    container.dispose();
    await api.pushes.close();
    RustLib.dispose();
  });

  test('build publishes the bridge snapshot', () async {
    expect(await container.read(clientProvider.future), signedOut);
  });

  group('pushed snapshots fold through newerSnapshot', () {
    for (final fixture in pushCases) {
      test(fixture.name, () async {
        await container.read(clientProvider.future);
        fixture.pushes.forEach(api.pushes.add);
        await pumpEventQueue();
        expect(container.read(clientProvider).value, fixture.expected);
      });
    }
  });

  test('a command reply older than a push never regresses state', () async {
    await container.read(clientProvider.future);
    api.pushes.add(snapshotAt(signedInWithMessages, 9));
    await pumpEventQueue();
    await container.read(clientProvider.notifier).signIn(signInRequest);
    expect(
      container.read(clientProvider).value,
      snapshotAt(signedInWithMessages, 9),
    );
  });

  group('intents publish the snapshot returned by the bridge', () {
    for (final intent in intentCases) {
      test(intent.name, () async {
        await container.read(clientProvider.future);
        await intent.run(container.read(clientProvider.notifier));
        expect(api.calls, [intent.expectedCall]);
        expect(container.read(clientProvider).value, intent.expectedState);
      });
    }
  });
}
