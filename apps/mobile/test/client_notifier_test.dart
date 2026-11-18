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
];

void main() {
  late FakeRustApi api;
  late ProviderContainer container;

  setUp(() {
    api = FakeRustApi();
    RustLib.initMock(api: api);
    container = ProviderContainer();
  });

  tearDown(() {
    container.dispose();
    RustLib.dispose();
  });

  test('build publishes the bridge snapshot', () async {
    expect(await container.read(clientProvider.future), signedOut);
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
