import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pulse_mobile/client/client_provider.dart';
import 'package:pulse_mobile/src/rust/frb_generated.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/auth.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/campus.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/ids.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

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
