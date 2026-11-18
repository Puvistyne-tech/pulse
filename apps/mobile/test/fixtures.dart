import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/client/pulse_intents.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/auth.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/campus.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/frames.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/ids.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/rbac.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/version.dart';
import 'package:uuid/uuid.dart';

const _version = ProtocolVersion(field0: 2);

UuidValue _uuid(String suffix) =>
    UuidValue.fromString('0193a5c0-0000-7000-8000-0000000000$suffix');

final campusAId = CampusId(field0: _uuid('0a'));
final campusBId = CampusId(field0: _uuid('0b'));

final account = AccountProfile(
  accountId: AccountId(field0: _uuid('c1')),
  email: 'ada@example.test',
  displayName: 'Ada',
);

final campusA = CampusSummary(
  campusId: campusAId,
  slug: 'alpha',
  displayName: 'Alpha Campus',
  role: Role.owner,
);

final campusB = CampusSummary(
  campusId: campusBId,
  slug: 'beta',
  displayName: 'Beta Campus',
  role: Role.member,
);

final rooms = <Room>[
  Room(
    roomId: RoomId(field0: _uuid('d1')),
    campusId: campusAId,
    slug: 'commons',
  ),
  Room(
    roomId: RoomId(field0: _uuid('d2')),
    campusId: campusAId,
    slug: 'courtyard',
  ),
];

ConnectionSnapshot network({
  ConnectionPhase connection = ConnectionPhase.disconnected,
  int? retryInMs,
  int attempt = 0,
  VersionMismatch? rejection,
  HandshakeRejection? handshakeRejection,
}) => ConnectionSnapshot(
  protocolVersion: _version,
  connection: connection,
  retryInMs: retryInMs,
  attempt: attempt,
  rejection: rejection,
  handshakeRejection: handshakeRejection,
);

ClientSnapshot _snapshot({
  AuthPhase auth = AuthPhase.signedOut,
  AccountProfile? profile,
  List<CampusSummary> campuses = const [],
  CampusId? activeCampusId,
  List<Room> roomList = const [],
  bool busy = false,
  ApiError? error,
}) => ClientSnapshot(
  network: network(),
  session: SessionSnapshot(
    auth: auth,
    account: profile,
    campuses: campuses,
    activeCampusId: activeCampusId,
    rooms: roomList,
    busy: busy,
    error: error,
  ),
);

final signedOut = _snapshot();

final authenticating = _snapshot(auth: AuthPhase.authenticating, busy: true);

final signedOutWithError = _snapshot(
  error: const ApiError(
    code: ApiErrorCode.invalidInput,
    field: InputField.email,
  ),
);

final signedInNoCampus = _snapshot(auth: AuthPhase.signedIn, profile: account);

final signedInWithRooms = _snapshot(
  auth: AuthPhase.signedIn,
  profile: account,
  campuses: [campusA, campusB],
  activeCampusId: campusAId,
  roomList: rooms,
);

class NetworkCase {
  const NetworkCase(this.name, this.network, this.expected);

  final String name;
  final ConnectionSnapshot network;
  final String expected;
}

final networkCases = <NetworkCase>[
  NetworkCase('disconnected', network(), 'Disconnected'),
  NetworkCase(
    'connecting',
    network(connection: ConnectionPhase.connecting),
    'Connecting',
  ),
  NetworkCase(
    'handshaking',
    network(connection: ConnectionPhase.handshaking),
    'Handshaking',
  ),
  NetworkCase(
    'connected',
    network(connection: ConnectionPhase.connected),
    'Connected',
  ),
  NetworkCase(
    'backoff',
    network(connection: ConnectionPhase.backoff, retryInMs: 1500, attempt: 2),
    'Retrying in 1500 ms',
  ),
  NetworkCase(
    'version rejected',
    network(
      rejection: const VersionMismatch(
        supported: _version,
        received: ProtocolVersion(field0: 99),
      ),
    ),
    'Version rejected: supported 2, received 99',
  ),
  NetworkCase(
    'handshake rejected',
    network(handshakeRejection: HandshakeRejection.campusUnavailable),
    'Handshake rejected: campus unavailable',
  ),
];

class RecordedIntents {
  RecordedIntents() {
    intents = PulseIntents(
      signUp: (request) async => calls.add(request),
      signIn: (request) async => calls.add(request),
      signOut: () async => calls.add('signOut'),
      createCampus: (request) async => calls.add(request),
      selectCampus: (campusId) async => calls.add(campusId),
      refreshCampuses: () async => calls.add('refreshCampuses'),
    );
  }

  final calls = <Object>[];
  late final PulseIntents intents;
}

Future<void> pumpView(WidgetTester tester, Widget child) =>
    tester.pumpWidget(MaterialApp(home: Scaffold(body: child)));

const signInRequest = SignInRequest(
  email: 'ada@example.test',
  password: 'correct-horse-battery',
);

const signUpRequest = SignUpRequest(
  email: 'ada@example.test',
  displayName: 'Ada',
  password: 'correct-horse-battery',
);

const createCampusRequest = CreateCampusRequest(
  slug: 'gamma',
  displayName: 'Gamma Campus',
);
