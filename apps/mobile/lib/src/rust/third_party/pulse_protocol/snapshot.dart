import '../../frb_generated.dart';
import 'api.dart';
import 'auth.dart';
import 'campus.dart';
import 'frames.dart';
import 'ids.dart';
import 'mention.dart';
import 'messaging.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'rbac.dart';
import 'thread.dart';
import 'version.dart';

enum AuthPhase { signedOut, authenticating, signedIn }

class ClientSnapshot {
  final BigInt sequence;
  final ConnectionSnapshot network;
  final SessionSnapshot session;
  final TimelineSnapshot timeline;
  final ThreadSnapshot thread;
  final InboxSnapshot inbox;

  const ClientSnapshot({
    required this.sequence,
    required this.network,
    required this.session,
    required this.timeline,
    required this.thread,
    required this.inbox,
  });

  @override
  int get hashCode =>
      sequence.hashCode ^
      network.hashCode ^
      session.hashCode ^
      timeline.hashCode ^
      thread.hashCode ^
      inbox.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ClientSnapshot &&
          runtimeType == other.runtimeType &&
          sequence == other.sequence &&
          network == other.network &&
          session == other.session &&
          timeline == other.timeline &&
          thread == other.thread &&
          inbox == other.inbox;
}

enum ConnectionPhase {
  disconnected,
  connecting,
  handshaking,
  connected,
  backoff,
}

class ConnectionSnapshot {
  final ProtocolVersion protocolVersion;
  final ConnectionPhase connection;
  final int? retryInMs;
  final CampusId? campusId;
  final SessionId? sessionId;
  final int attempt;
  final VersionMismatch? rejection;
  final HandshakeRejection? handshakeRejection;

  const ConnectionSnapshot({
    required this.protocolVersion,
    required this.connection,
    this.retryInMs,
    this.campusId,
    this.sessionId,
    required this.attempt,
    this.rejection,
    this.handshakeRejection,
  });

  @override
  int get hashCode =>
      protocolVersion.hashCode ^
      connection.hashCode ^
      retryInMs.hashCode ^
      campusId.hashCode ^
      sessionId.hashCode ^
      attempt.hashCode ^
      rejection.hashCode ^
      handshakeRejection.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConnectionSnapshot &&
          runtimeType == other.runtimeType &&
          protocolVersion == other.protocolVersion &&
          connection == other.connection &&
          retryInMs == other.retryInMs &&
          campusId == other.campusId &&
          sessionId == other.sessionId &&
          attempt == other.attempt &&
          rejection == other.rejection &&
          handshakeRejection == other.handshakeRejection;
}

enum DeliveryState { pending, confirmed, failed }

class InboxItem {
  final InboxItemKind kind;
  final RoomId roomId;
  final MessageId rootMessageId;
  final String? topicTitle;
  final String rootAuthorDisplayName;
  final String rootPreview;
  final ReplyPreview? latestReply;
  final int replyCount;
  final int unreadReplies;
  final int unreadMentions;
  final SubscriptionState subscription;
  final MessageId activityMessageId;

  const InboxItem({
    required this.kind,
    required this.roomId,
    required this.rootMessageId,
    this.topicTitle,
    required this.rootAuthorDisplayName,
    required this.rootPreview,
    this.latestReply,
    required this.replyCount,
    required this.unreadReplies,
    required this.unreadMentions,
    required this.subscription,
    required this.activityMessageId,
  });

  @override
  int get hashCode =>
      kind.hashCode ^
      roomId.hashCode ^
      rootMessageId.hashCode ^
      topicTitle.hashCode ^
      rootAuthorDisplayName.hashCode ^
      rootPreview.hashCode ^
      latestReply.hashCode ^
      replyCount.hashCode ^
      unreadReplies.hashCode ^
      unreadMentions.hashCode ^
      subscription.hashCode ^
      activityMessageId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is InboxItem &&
          runtimeType == other.runtimeType &&
          kind == other.kind &&
          roomId == other.roomId &&
          rootMessageId == other.rootMessageId &&
          topicTitle == other.topicTitle &&
          rootAuthorDisplayName == other.rootAuthorDisplayName &&
          rootPreview == other.rootPreview &&
          latestReply == other.latestReply &&
          replyCount == other.replyCount &&
          unreadReplies == other.unreadReplies &&
          unreadMentions == other.unreadMentions &&
          subscription == other.subscription &&
          activityMessageId == other.activityMessageId;
}

enum InboxItemKind { thread }

class InboxSnapshot {
  final bool open;
  final List<InboxItem> items;
  final int unreadThreads;
  final MessageId? selectedRootMessageId;
  final bool replying;

  const InboxSnapshot({
    required this.open,
    required this.items,
    required this.unreadThreads,
    this.selectedRootMessageId,
    required this.replying,
  });

  static Future<InboxSnapshot> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotInboxSnapshotDefault();

  @override
  int get hashCode =>
      open.hashCode ^
      items.hashCode ^
      unreadThreads.hashCode ^
      selectedRootMessageId.hashCode ^
      replying.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is InboxSnapshot &&
          runtimeType == other.runtimeType &&
          open == other.open &&
          items == other.items &&
          unreadThreads == other.unreadThreads &&
          selectedRootMessageId == other.selectedRootMessageId &&
          replying == other.replying;
}

enum InboxStep { next, previous }

class ReactionSummary {
  final String emoji;
  final int count;
  final bool reactedBySelf;

  const ReactionSummary({
    required this.emoji,
    required this.count,
    required this.reactedBySelf,
  });

  @override
  int get hashCode => emoji.hashCode ^ count.hashCode ^ reactedBySelf.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReactionSummary &&
          runtimeType == other.runtimeType &&
          emoji == other.emoji &&
          count == other.count &&
          reactedBySelf == other.reactedBySelf;
}

class ReplyPreview {
  final MessageId messageId;
  final AccountId authorId;
  final String authorDisplayName;
  final String preview;

  const ReplyPreview({
    required this.messageId,
    required this.authorId,
    required this.authorDisplayName,
    required this.preview,
  });

  @override
  int get hashCode =>
      messageId.hashCode ^
      authorId.hashCode ^
      authorDisplayName.hashCode ^
      preview.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReplyPreview &&
          runtimeType == other.runtimeType &&
          messageId == other.messageId &&
          authorId == other.authorId &&
          authorDisplayName == other.authorDisplayName &&
          preview == other.preview;
}

class SessionSnapshot {
  final AuthPhase auth;
  final AccountProfile? account;
  final List<CampusSummary> campuses;
  final CampusId? activeCampusId;
  final List<Room> rooms;
  final RoomId? activeRoomId;
  final bool busy;
  final ApiError? error;

  const SessionSnapshot({
    required this.auth,
    this.account,
    required this.campuses,
    this.activeCampusId,
    required this.rooms,
    this.activeRoomId,
    required this.busy,
    this.error,
  });

  @override
  int get hashCode =>
      auth.hashCode ^
      account.hashCode ^
      campuses.hashCode ^
      activeCampusId.hashCode ^
      rooms.hashCode ^
      activeRoomId.hashCode ^
      busy.hashCode ^
      error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SessionSnapshot &&
          runtimeType == other.runtimeType &&
          auth == other.auth &&
          account == other.account &&
          campuses == other.campuses &&
          activeCampusId == other.activeCampusId &&
          rooms == other.rooms &&
          activeRoomId == other.activeRoomId &&
          busy == other.busy &&
          error == other.error;
}

class ThreadPreview {
  final String? topicTitle;
  final int replyCount;
  final ReplyPreview? latestReply;
  final bool showLatestReply;
  final SubscriptionState? subscription;
  final int unreadReplies;

  const ThreadPreview({
    this.topicTitle,
    required this.replyCount,
    this.latestReply,
    required this.showLatestReply,
    this.subscription,
    required this.unreadReplies,
  });

  @override
  int get hashCode =>
      topicTitle.hashCode ^
      replyCount.hashCode ^
      latestReply.hashCode ^
      showLatestReply.hashCode ^
      subscription.hashCode ^
      unreadReplies.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ThreadPreview &&
          runtimeType == other.runtimeType &&
          topicTitle == other.topicTitle &&
          replyCount == other.replyCount &&
          latestReply == other.latestReply &&
          showLatestReply == other.showLatestReply &&
          subscription == other.subscription &&
          unreadReplies == other.unreadReplies;
}

class ThreadSnapshot {
  final RoomId? roomId;
  final MessageId? rootMessageId;
  final TimelineEntry? root;
  final List<TimelineEntry> replies;
  final String? topicTitle;
  final SubscriptionState? subscription;
  final bool syncing;

  const ThreadSnapshot({
    this.roomId,
    this.rootMessageId,
    this.root,
    required this.replies,
    this.topicTitle,
    this.subscription,
    required this.syncing,
  });

  static Future<ThreadSnapshot> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotThreadSnapshotDefault();

  @override
  int get hashCode =>
      roomId.hashCode ^
      rootMessageId.hashCode ^
      root.hashCode ^
      replies.hashCode ^
      topicTitle.hashCode ^
      subscription.hashCode ^
      syncing.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ThreadSnapshot &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          rootMessageId == other.rootMessageId &&
          root == other.root &&
          replies == other.replies &&
          topicTitle == other.topicTitle &&
          subscription == other.subscription &&
          syncing == other.syncing;
}

class TimelineEntry {
  final ClientNonce clientNonce;
  final MessageId? messageId;
  final AccountId authorId;
  final String authorDisplayName;
  final String authorMentionToken;
  final bool fromSelf;
  final String body;
  final List<BodySegment> segments;
  final MessageId? threadRootId;
  final List<ReactionSummary> reactions;
  final ThreadPreview? thread;
  final DeliveryState delivery;
  final MessageRejection? rejection;

  const TimelineEntry({
    required this.clientNonce,
    this.messageId,
    required this.authorId,
    required this.authorDisplayName,
    required this.authorMentionToken,
    required this.fromSelf,
    required this.body,
    required this.segments,
    this.threadRootId,
    required this.reactions,
    this.thread,
    required this.delivery,
    this.rejection,
  });

  @override
  int get hashCode =>
      clientNonce.hashCode ^
      messageId.hashCode ^
      authorId.hashCode ^
      authorDisplayName.hashCode ^
      authorMentionToken.hashCode ^
      fromSelf.hashCode ^
      body.hashCode ^
      segments.hashCode ^
      threadRootId.hashCode ^
      reactions.hashCode ^
      thread.hashCode ^
      delivery.hashCode ^
      rejection.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TimelineEntry &&
          runtimeType == other.runtimeType &&
          clientNonce == other.clientNonce &&
          messageId == other.messageId &&
          authorId == other.authorId &&
          authorDisplayName == other.authorDisplayName &&
          authorMentionToken == other.authorMentionToken &&
          fromSelf == other.fromSelf &&
          body == other.body &&
          segments == other.segments &&
          threadRootId == other.threadRootId &&
          reactions == other.reactions &&
          thread == other.thread &&
          delivery == other.delivery &&
          rejection == other.rejection;
}

class TimelineSnapshot {
  final RoomId? roomId;
  final List<TimelineEntry> entries;
  final bool syncing;
  final ApiError? error;
  final List<String> reactionPalette;

  const TimelineSnapshot({
    this.roomId,
    required this.entries,
    required this.syncing,
    this.error,
    required this.reactionPalette,
  });

  @override
  int get hashCode =>
      roomId.hashCode ^
      entries.hashCode ^
      syncing.hashCode ^
      error.hashCode ^
      reactionPalette.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TimelineSnapshot &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          entries == other.entries &&
          syncing == other.syncing &&
          error == other.error &&
          reactionPalette == other.reactionPalette;
}
