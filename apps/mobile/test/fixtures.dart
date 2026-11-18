import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/client/pulse_intents.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/auth.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/campus.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/frames.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/ids.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/mention.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/messaging.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/rbac.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/thread.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/version.dart';
import 'package:uuid/uuid.dart';

const _version = ProtocolVersion(field0: 3);

UuidValue _uuid(String suffix) =>
    UuidValue.fromString('0193a5c0-0000-7000-8000-0000000000$suffix');

final campusAId = CampusId(field0: _uuid('0a'));
final campusBId = CampusId(field0: _uuid('0b'));

final roomAId = RoomId(field0: _uuid('d1'));
final roomBId = RoomId(field0: _uuid('d2'));
final messageAId = MessageId(field0: _uuid('e1'));
final messageBId = MessageId(field0: _uuid('e2'));
final messageCId = MessageId(field0: _uuid('e3'));
final messageDId = MessageId(field0: _uuid('e4'));
final nonceA = ClientNonce(field0: _uuid('f1'));
final nonceB = ClientNonce(field0: _uuid('f2'));
final nonceC = ClientNonce(field0: _uuid('f3'));

final graceId = AccountId(field0: _uuid('c2'));

String mentionToken(AccountId accountId) => '<@${accountId.field0}>';

const reactionPalette = ['👍', '❤️', '🎉', '👀', '✅', '🙏'];

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
  Room(roomId: roomAId, campusId: campusAId, slug: 'commons'),
  Room(roomId: roomBId, campusId: campusAId, slug: 'courtyard'),
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

TimelineEntry _entry(
  DeliveryState delivery, {
  required ClientNonce clientNonce,
  MessageId? messageId,
  AccountId? authorId,
  String authorDisplayName = 'Ada',
  bool fromSelf = true,
  String body = 'hello',
  List<BodySegment>? segments,
  MessageId? threadRootId,
  List<ReactionSummary> reactions = const [],
  ThreadPreview? thread,
  MessageRejection? rejection,
}) {
  final author = authorId ?? account.accountId;
  return TimelineEntry(
    clientNonce: clientNonce,
    messageId: messageId,
    authorId: author,
    authorDisplayName: authorDisplayName,
    authorMentionToken: mentionToken(author),
    fromSelf: fromSelf,
    body: body,
    segments: segments ?? [BodySegment(kind: SegmentKind.text, text: body)],
    threadRootId: threadRootId,
    reactions: reactions,
    thread: thread,
    delivery: delivery,
    rejection: rejection,
  );
}

final pendingEntry = _entry(DeliveryState.pending, clientNonce: nonceA);

final confirmedEntry = _entry(
  DeliveryState.confirmed,
  clientNonce: nonceB,
  messageId: messageAId,
  authorId: graceId,
  authorDisplayName: 'Grace',
  fromSelf: false,
  body: 'welcome',
);

TimelineEntry _gracesEntry({
  required ClientNonce clientNonce,
  required MessageId messageId,
  String body = 'welcome',
  List<BodySegment>? segments,
  MessageId? threadRootId,
  List<ReactionSummary> reactions = const [],
  ThreadPreview? thread,
}) => _entry(
  DeliveryState.confirmed,
  clientNonce: clientNonce,
  messageId: messageId,
  authorId: graceId,
  authorDisplayName: 'Grace',
  fromSelf: false,
  body: body,
  segments: segments,
  threadRootId: threadRootId,
  reactions: reactions,
  thread: thread,
);

final mentionEntry = _gracesEntry(
  clientNonce: nonceC,
  messageId: messageCId,
  body: 'hi ${mentionToken(account.accountId)}',
  segments: [
    const BodySegment(kind: SegmentKind.text, text: 'hi '),
    BodySegment(
      kind: SegmentKind.mention,
      text: '@Ada',
      accountId: account.accountId,
    ),
  ],
);

final reactedEntry = _gracesEntry(
  clientNonce: nonceB,
  messageId: messageAId,
  reactions: const [
    ReactionSummary(emoji: '👍', count: 2, reactedBySelf: true),
    ReactionSummary(emoji: '🎉', count: 1, reactedBySelf: false),
  ],
);

ThreadPreview threadPreviewOf({
  String? topicTitle,
  int replyCount = 3,
  bool showLatestReply = true,
  bool hasLatestReply = true,
  SubscriptionState? subscription = SubscriptionState.following,
  int unreadReplies = 2,
}) => ThreadPreview(
  topicTitle: topicTitle,
  replyCount: replyCount,
  latestReply: hasLatestReply
      ? ReplyPreview(
          messageId: messageDId,
          authorId: graceId,
          authorDisplayName: 'Grace',
          preview: 'latest words',
        )
      : null,
  showLatestReply: showLatestReply,
  subscription: subscription,
  unreadReplies: unreadReplies,
);

TimelineEntry threadedEntry([ThreadPreview? preview]) => _gracesEntry(
  clientNonce: nonceB,
  messageId: messageAId,
  thread: preview ?? threadPreviewOf(),
);

final replyEntry = _gracesEntry(
  clientNonce: nonceC,
  messageId: messageDId,
  body: 'a reply',
  threadRootId: messageAId,
);

final confirmedOwnEntry = _entry(
  DeliveryState.confirmed,
  clientNonce: nonceA,
  messageId: messageBId,
  body: 'sent',
);

TimelineEntry failedEntry(MessageRejection rejection) => _entry(
  DeliveryState.failed,
  clientNonce: nonceC,
  body: 'denied',
  rejection: rejection,
);

TimelineSnapshot timelineOf(
  List<TimelineEntry> entries, {
  RoomId? roomId,
  bool syncing = false,
  ApiError? error,
  List<String> palette = reactionPalette,
}) => TimelineSnapshot(
  roomId: roomId ?? roomAId,
  entries: entries,
  syncing: syncing,
  error: error,
  reactionPalette: palette,
);

const emptyTimeline = TimelineSnapshot(
  entries: [],
  syncing: false,
  reactionPalette: reactionPalette,
);

const emptyThread = ThreadSnapshot(replies: [], syncing: false);

const emptyInbox = InboxSnapshot(
  open: false,
  items: [],
  unreadThreads: 0,
  replying: false,
);

ThreadSnapshot threadOf({
  TimelineEntry? root,
  bool withRoot = true,
  List<TimelineEntry>? replies,
  String? topicTitle,
  SubscriptionState? subscription = SubscriptionState.following,
  bool syncing = false,
  bool anchored = true,
}) => ThreadSnapshot(
  roomId: anchored ? roomAId : null,
  rootMessageId: anchored ? messageAId : null,
  root: withRoot ? (root ?? confirmedEntry) : null,
  replies: replies ?? [replyEntry],
  topicTitle: topicTitle,
  subscription: subscription,
  syncing: syncing,
);

InboxItem inboxItemOf(
  MessageId root,
  String preview, {
  String? topicTitle,
  ReplyPreview? latestReply,
  int unreadReplies = 0,
  int unreadMentions = 0,
  SubscriptionState subscription = SubscriptionState.following,
}) => InboxItem(
  kind: InboxItemKind.thread,
  roomId: roomAId,
  rootMessageId: root,
  topicTitle: topicTitle,
  rootAuthorDisplayName: 'Grace',
  rootPreview: preview,
  latestReply: latestReply,
  replyCount: 0,
  unreadReplies: unreadReplies,
  unreadMentions: unreadMentions,
  subscription: subscription,
  activityMessageId: root,
);

InboxSnapshot inboxOf(
  List<InboxItem> items, {
  bool open = true,
  bool replying = false,
  int? unreadThreads,
  MessageId? selected,
  bool selectFirst = true,
}) => InboxSnapshot(
  open: open,
  items: items,
  unreadThreads:
      unreadThreads ??
      items
          .where((item) => item.unreadReplies > 0 || item.unreadMentions > 0)
          .length,
  selectedRootMessageId:
      selected ??
      (selectFirst && items.isNotEmpty ? items.first.rootMessageId : null),
  replying: replying,
);

ClientSnapshot withExtras(
  ClientSnapshot snapshot, {
  ThreadSnapshot? thread,
  InboxSnapshot? inbox,
}) => ClientSnapshot(
  sequence: snapshot.sequence,
  network: snapshot.network,
  session: snapshot.session,
  timeline: snapshot.timeline,
  thread: thread ?? snapshot.thread,
  inbox: inbox ?? snapshot.inbox,
);

ClientSnapshot snapshotAt(ClientSnapshot snapshot, int sequence) =>
    ClientSnapshot(
      sequence: BigInt.from(sequence),
      network: snapshot.network,
      session: snapshot.session,
      timeline: snapshot.timeline,
      thread: snapshot.thread,
      inbox: snapshot.inbox,
    );

ClientSnapshot _snapshot({
  AuthPhase auth = AuthPhase.signedOut,
  AccountProfile? profile,
  List<CampusSummary> campuses = const [],
  CampusId? activeCampusId,
  List<Room> roomList = const [],
  RoomId? activeRoomId,
  TimelineSnapshot timeline = emptyTimeline,
  int sequence = 1,
  bool busy = false,
  ApiError? error,
}) => ClientSnapshot(
  sequence: BigInt.from(sequence),
  timeline: timeline,
  thread: emptyThread,
  inbox: emptyInbox,
  network: network(),
  session: SessionSnapshot(
    auth: auth,
    account: profile,
    campuses: campuses,
    activeCampusId: activeCampusId,
    rooms: roomList,
    activeRoomId: activeRoomId,
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
  activeRoomId: roomAId,
  timeline: timelineOf([]),
);

final signedInWithMessages = _snapshot(
  auth: AuthPhase.signedIn,
  profile: account,
  campuses: [campusA, campusB],
  activeCampusId: campusAId,
  roomList: rooms,
  activeRoomId: roomAId,
  timeline: timelineOf([confirmedEntry, pendingEntry], syncing: true),
  sequence: 2,
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
    'Version rejected: supported 3, received 99',
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
      selectRoom: (roomId) async => calls.add(roomId),
      sendMessage: (roomId, body) async => calls.add((roomId, body)),
      discardMessage: (clientNonce) async => calls.add(clientNonce),
      sendReply: (roomId, rootMessageId, body) async =>
          calls.add(('sendReply', roomId, rootMessageId, body)),
      react: (roomId, messageId, emoji, action) async =>
          calls.add(('react', roomId, messageId, emoji, action)),
      setTopicTitle: (roomId, rootMessageId, topicTitle) async =>
          calls.add(('setTopicTitle', roomId, rootMessageId, topicTitle)),
      setThreadSubscription: (roomId, rootMessageId, state) async =>
          calls.add(('setThreadSubscription', roomId, rootMessageId, state)),
      openThread: (roomId, rootMessageId) async =>
          calls.add(('openThread', roomId, rootMessageId)),
      closeThread: () async => calls.add('closeThread'),
      toggleInbox: () async => calls.add('toggleInbox'),
      moveInboxSelection: (step) async =>
          calls.add(('moveInboxSelection', step)),
      archiveInboxSelection: () async => calls.add('archiveInboxSelection'),
      startInboxReply: () async => calls.add('startInboxReply'),
      cancelInboxReply: () async => calls.add('cancelInboxReply'),
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
