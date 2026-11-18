import '../../frb_generated.dart';
import 'api.dart';
import 'auth.dart';
import 'campus.dart';
import 'frames.dart';
import 'ids.dart';
import 'lounge.dart';
import 'media.dart';
import 'mention.dart';
import 'messaging.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

import 'rbac.dart';
import 'shelf.dart';
import 'thread.dart';
import 'version.dart';

class AttachmentView {
  final AttachmentId attachmentId;
  final String fileName;
  final String contentType;
  final AttachmentKind kind;
  final BigInt byteSize;
  final String? url;

  const AttachmentView({
    required this.attachmentId,
    required this.fileName,
    required this.contentType,
    required this.kind,
    required this.byteSize,
    this.url,
  });

  @override
  int get hashCode =>
      attachmentId.hashCode ^
      fileName.hashCode ^
      contentType.hashCode ^
      kind.hashCode ^
      byteSize.hashCode ^
      url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AttachmentView &&
          runtimeType == other.runtimeType &&
          attachmentId == other.attachmentId &&
          fileName == other.fileName &&
          contentType == other.contentType &&
          kind == other.kind &&
          byteSize == other.byteSize &&
          url == other.url;
}

enum AuthPhase { signedOut, authenticating, signedIn }

class ClientSnapshot {
  final BigInt sequence;
  final ConnectionSnapshot network;
  final SessionSnapshot session;
  final TimelineSnapshot timeline;
  final ThreadSnapshot thread;
  final InboxSnapshot inbox;
  final SearchSnapshot search;
  final VaultSnapshot vault;
  final ShelfSnapshot shelf;
  final LoungeSnapshot lounges;
  final StewardSnapshot stewards;

  const ClientSnapshot({
    required this.sequence,
    required this.network,
    required this.session,
    required this.timeline,
    required this.thread,
    required this.inbox,
    required this.search,
    required this.vault,
    required this.shelf,
    required this.lounges,
    required this.stewards,
  });

  @override
  int get hashCode =>
      sequence.hashCode ^
      network.hashCode ^
      session.hashCode ^
      timeline.hashCode ^
      thread.hashCode ^
      inbox.hashCode ^
      search.hashCode ^
      vault.hashCode ^
      shelf.hashCode ^
      lounges.hashCode ^
      stewards.hashCode;

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
          inbox == other.inbox &&
          search == other.search &&
          vault == other.vault &&
          shelf == other.shelf &&
          lounges == other.lounges &&
          stewards == other.stewards;
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

class DraftAttachmentView {
  final DraftId draftId;
  final RoomId roomId;
  final String fileName;
  final AttachmentKind kind;
  final BigInt byteSize;
  final UploadState upload;

  const DraftAttachmentView({
    required this.draftId,
    required this.roomId,
    required this.fileName,
    required this.kind,
    required this.byteSize,
    required this.upload,
  });

  @override
  int get hashCode =>
      draftId.hashCode ^
      roomId.hashCode ^
      fileName.hashCode ^
      kind.hashCode ^
      byteSize.hashCode ^
      upload.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DraftAttachmentView &&
          runtimeType == other.runtimeType &&
          draftId == other.draftId &&
          roomId == other.roomId &&
          fileName == other.fileName &&
          kind == other.kind &&
          byteSize == other.byteSize &&
          upload == other.upload;
}

class HighlightSpan {
  final String text;
  final bool matched;

  const HighlightSpan({required this.text, required this.matched});

  @override
  int get hashCode => text.hashCode ^ matched.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is HighlightSpan &&
          runtimeType == other.runtimeType &&
          text == other.text &&
          matched == other.matched;
}

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

class IssuedStewardToken {
  final AccountId stewardId;
  final String displayName;
  final AuthToken token;

  const IssuedStewardToken({
    required this.stewardId,
    required this.displayName,
    required this.token,
  });

  @override
  int get hashCode =>
      stewardId.hashCode ^ displayName.hashCode ^ token.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is IssuedStewardToken &&
          runtimeType == other.runtimeType &&
          stewardId == other.stewardId &&
          displayName == other.displayName &&
          token == other.token;
}

class LoungeEntry {
  final LoungeId loungeId;
  final LoungeScope scope;
  final RoomId? roomId;
  final String? roomSlug;
  final List<LoungeParticipantView> participants;
  final bool joined;

  const LoungeEntry({
    required this.loungeId,
    required this.scope,
    this.roomId,
    this.roomSlug,
    required this.participants,
    required this.joined,
  });

  @override
  int get hashCode =>
      loungeId.hashCode ^
      scope.hashCode ^
      roomId.hashCode ^
      roomSlug.hashCode ^
      participants.hashCode ^
      joined.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoungeEntry &&
          runtimeType == other.runtimeType &&
          loungeId == other.loungeId &&
          scope == other.scope &&
          roomId == other.roomId &&
          roomSlug == other.roomSlug &&
          participants == other.participants &&
          joined == other.joined;
}

enum LoungeMediaState {
  idle,
  connecting,
  connected,
  failed;

  static Future<LoungeMediaState> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotLoungeMediaStateDefault();
}

class LoungeParticipantView {
  final AccountId accountId;
  final String displayName;
  final bool fromSelf;
  final bool canSpeak;

  const LoungeParticipantView({
    required this.accountId,
    required this.displayName,
    required this.fromSelf,
    required this.canSpeak,
  });

  @override
  int get hashCode =>
      accountId.hashCode ^
      displayName.hashCode ^
      fromSelf.hashCode ^
      canSpeak.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoungeParticipantView &&
          runtimeType == other.runtimeType &&
          accountId == other.accountId &&
          displayName == other.displayName &&
          fromSelf == other.fromSelf &&
          canSpeak == other.canSpeak;
}

class LoungeSnapshot {
  final List<LoungeEntry> lounges;
  final LoungeId? joinedLoungeId;
  final bool joining;
  final LoungeTicket? ticket;
  final LoungeMediaState media;
  final bool canJoin;
  final bool canSpeak;
  final ApiError? error;

  const LoungeSnapshot({
    required this.lounges,
    this.joinedLoungeId,
    required this.joining,
    this.ticket,
    required this.media,
    required this.canJoin,
    required this.canSpeak,
    this.error,
  });

  static Future<LoungeSnapshot> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotLoungeSnapshotDefault();

  @override
  int get hashCode =>
      lounges.hashCode ^
      joinedLoungeId.hashCode ^
      joining.hashCode ^
      ticket.hashCode ^
      media.hashCode ^
      canJoin.hashCode ^
      canSpeak.hashCode ^
      error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoungeSnapshot &&
          runtimeType == other.runtimeType &&
          lounges == other.lounges &&
          joinedLoungeId == other.joinedLoungeId &&
          joining == other.joining &&
          ticket == other.ticket &&
          media == other.media &&
          canJoin == other.canJoin &&
          canSpeak == other.canSpeak &&
          error == other.error;
}

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

class SearchHit {
  final RoomId roomId;
  final MessageId messageId;
  final MessageId? threadRootId;
  final String authorDisplayName;
  final String? topicTitle;
  final List<HighlightSpan> snippet;
  final int score;

  const SearchHit({
    required this.roomId,
    required this.messageId,
    this.threadRootId,
    required this.authorDisplayName,
    this.topicTitle,
    required this.snippet,
    required this.score,
  });

  @override
  int get hashCode =>
      roomId.hashCode ^
      messageId.hashCode ^
      threadRootId.hashCode ^
      authorDisplayName.hashCode ^
      topicTitle.hashCode ^
      snippet.hashCode ^
      score.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchHit &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          messageId == other.messageId &&
          threadRootId == other.threadRootId &&
          authorDisplayName == other.authorDisplayName &&
          topicTitle == other.topicTitle &&
          snippet == other.snippet &&
          score == other.score;
}

class SearchSnapshot {
  final bool open;
  final String query;
  final List<SearchHit> hits;
  final int? selectedIndex;
  final bool searching;
  final ApiError? error;

  const SearchSnapshot({
    required this.open,
    required this.query,
    required this.hits,
    this.selectedIndex,
    required this.searching,
    this.error,
  });

  static Future<SearchSnapshot> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotSearchSnapshotDefault();

  @override
  int get hashCode =>
      open.hashCode ^
      query.hashCode ^
      hits.hashCode ^
      selectedIndex.hashCode ^
      searching.hashCode ^
      error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchSnapshot &&
          runtimeType == other.runtimeType &&
          open == other.open &&
          query == other.query &&
          hits == other.hits &&
          selectedIndex == other.selectedIndex &&
          searching == other.searching &&
          error == other.error;
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

class ShelfEntry {
  final ShelfItemId? shelfItemId;
  final RoomId roomId;
  final MessageId messageId;
  final String authorDisplayName;
  final String preview;
  final BigInt? remindAtMs;
  final BigInt? doneAtMs;
  final ShelfBucket bucket;
  final bool pending;

  const ShelfEntry({
    this.shelfItemId,
    required this.roomId,
    required this.messageId,
    required this.authorDisplayName,
    required this.preview,
    this.remindAtMs,
    this.doneAtMs,
    required this.bucket,
    required this.pending,
  });

  @override
  int get hashCode =>
      shelfItemId.hashCode ^
      roomId.hashCode ^
      messageId.hashCode ^
      authorDisplayName.hashCode ^
      preview.hashCode ^
      remindAtMs.hashCode ^
      doneAtMs.hashCode ^
      bucket.hashCode ^
      pending.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ShelfEntry &&
          runtimeType == other.runtimeType &&
          shelfItemId == other.shelfItemId &&
          roomId == other.roomId &&
          messageId == other.messageId &&
          authorDisplayName == other.authorDisplayName &&
          preview == other.preview &&
          remindAtMs == other.remindAtMs &&
          doneAtMs == other.doneAtMs &&
          bucket == other.bucket &&
          pending == other.pending;
}

class ShelfSnapshot {
  final bool open;
  final List<ShelfEntry> due;
  final List<ShelfEntry> upcoming;
  final List<ShelfEntry> done;
  final int dueCount;
  final ShelfEntry? latestReminder;
  final List<ReminderPreset> presets;
  final BigInt? pickerAnchorMs;
  final BigInt? pickerLatestMs;
  final ApiError? error;

  const ShelfSnapshot({
    required this.open,
    required this.due,
    required this.upcoming,
    required this.done,
    required this.dueCount,
    this.latestReminder,
    required this.presets,
    this.pickerAnchorMs,
    this.pickerLatestMs,
    this.error,
  });

  static Future<ShelfSnapshot> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotShelfSnapshotDefault();

  @override
  int get hashCode =>
      open.hashCode ^
      due.hashCode ^
      upcoming.hashCode ^
      done.hashCode ^
      dueCount.hashCode ^
      latestReminder.hashCode ^
      presets.hashCode ^
      pickerAnchorMs.hashCode ^
      pickerLatestMs.hashCode ^
      error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ShelfSnapshot &&
          runtimeType == other.runtimeType &&
          open == other.open &&
          due == other.due &&
          upcoming == other.upcoming &&
          done == other.done &&
          dueCount == other.dueCount &&
          latestReminder == other.latestReminder &&
          presets == other.presets &&
          pickerAnchorMs == other.pickerAnchorMs &&
          pickerLatestMs == other.pickerLatestMs &&
          error == other.error;
}

class StewardEntry {
  final AccountId stewardId;
  final String displayName;
  final String mentionToken;
  final List<RoomId> roomIds;
  final bool revoked;

  const StewardEntry({
    required this.stewardId,
    required this.displayName,
    required this.mentionToken,
    required this.roomIds,
    required this.revoked,
  });

  @override
  int get hashCode =>
      stewardId.hashCode ^
      displayName.hashCode ^
      mentionToken.hashCode ^
      roomIds.hashCode ^
      revoked.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StewardEntry &&
          runtimeType == other.runtimeType &&
          stewardId == other.stewardId &&
          displayName == other.displayName &&
          mentionToken == other.mentionToken &&
          roomIds == other.roomIds &&
          revoked == other.revoked;
}

class StewardSnapshot {
  final bool open;
  final List<StewardEntry> stewards;
  final bool canManage;
  final IssuedStewardToken? issued;
  final bool pending;
  final ApiError? error;

  const StewardSnapshot({
    required this.open,
    required this.stewards,
    required this.canManage,
    this.issued,
    required this.pending,
    this.error,
  });

  static Future<StewardSnapshot> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotStewardSnapshotDefault();

  @override
  int get hashCode =>
      open.hashCode ^
      stewards.hashCode ^
      canManage.hashCode ^
      issued.hashCode ^
      pending.hashCode ^
      error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StewardSnapshot &&
          runtimeType == other.runtimeType &&
          open == other.open &&
          stewards == other.stewards &&
          canManage == other.canManage &&
          issued == other.issued &&
          pending == other.pending &&
          error == other.error;
}

enum StreamState { streaming, finalizing }

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
  final List<AttachmentView> attachments;
  final List<DraftAttachmentView> pendingDrafts;
  final bool pinned;
  final bool shelved;
  final bool authorIsSteward;
  final StreamState? stream;

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
    required this.attachments,
    required this.pendingDrafts,
    required this.pinned,
    required this.shelved,
    required this.authorIsSteward,
    this.stream,
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
      rejection.hashCode ^
      attachments.hashCode ^
      pendingDrafts.hashCode ^
      pinned.hashCode ^
      shelved.hashCode ^
      authorIsSteward.hashCode ^
      stream.hashCode;

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
          rejection == other.rejection &&
          attachments == other.attachments &&
          pendingDrafts == other.pendingDrafts &&
          pinned == other.pinned &&
          shelved == other.shelved &&
          authorIsSteward == other.authorIsSteward &&
          stream == other.stream;
}

class TimelineSnapshot {
  final RoomId? roomId;
  final List<TimelineEntry> entries;
  final bool syncing;
  final ApiError? error;
  final List<String> reactionPalette;
  final List<DraftAttachmentView> drafts;
  final MessageId? focusMessageId;
  final bool canPin;
  final bool canUpload;

  const TimelineSnapshot({
    this.roomId,
    required this.entries,
    required this.syncing,
    this.error,
    required this.reactionPalette,
    required this.drafts,
    this.focusMessageId,
    required this.canPin,
    required this.canUpload,
  });

  @override
  int get hashCode =>
      roomId.hashCode ^
      entries.hashCode ^
      syncing.hashCode ^
      error.hashCode ^
      reactionPalette.hashCode ^
      drafts.hashCode ^
      focusMessageId.hashCode ^
      canPin.hashCode ^
      canUpload.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TimelineSnapshot &&
          runtimeType == other.runtimeType &&
          roomId == other.roomId &&
          entries == other.entries &&
          syncing == other.syncing &&
          error == other.error &&
          reactionPalette == other.reactionPalette &&
          drafts == other.drafts &&
          focusMessageId == other.focusMessageId &&
          canPin == other.canPin &&
          canUpload == other.canUpload;
}

enum UploadState { uploading, uploaded, failed }

class VaultEntry {
  final VaultTab tab;
  final RoomId roomId;
  final MessageId messageId;
  final String authorDisplayName;
  final String preview;
  final AttachmentView? attachment;
  final String? link;
  final BigInt? pinnedAtMs;

  const VaultEntry({
    required this.tab,
    required this.roomId,
    required this.messageId,
    required this.authorDisplayName,
    required this.preview,
    this.attachment,
    this.link,
    this.pinnedAtMs,
  });

  @override
  int get hashCode =>
      tab.hashCode ^
      roomId.hashCode ^
      messageId.hashCode ^
      authorDisplayName.hashCode ^
      preview.hashCode ^
      attachment.hashCode ^
      link.hashCode ^
      pinnedAtMs.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is VaultEntry &&
          runtimeType == other.runtimeType &&
          tab == other.tab &&
          roomId == other.roomId &&
          messageId == other.messageId &&
          authorDisplayName == other.authorDisplayName &&
          preview == other.preview &&
          attachment == other.attachment &&
          link == other.link &&
          pinnedAtMs == other.pinnedAtMs;
}

class VaultSnapshot {
  final bool open;
  final RoomId? roomId;
  final VaultTab tab;
  final List<VaultEntry> entries;
  final bool loading;
  final bool hasMore;
  final ApiError? error;

  const VaultSnapshot({
    required this.open,
    this.roomId,
    required this.tab,
    required this.entries,
    required this.loading,
    required this.hasMore,
    this.error,
  });

  static Future<VaultSnapshot> default_() =>
      RustLib.instance.api.pulseProtocolSnapshotVaultSnapshotDefault();

  @override
  int get hashCode =>
      open.hashCode ^
      roomId.hashCode ^
      tab.hashCode ^
      entries.hashCode ^
      loading.hashCode ^
      hasMore.hashCode ^
      error.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is VaultSnapshot &&
          runtimeType == other.runtimeType &&
          open == other.open &&
          roomId == other.roomId &&
          tab == other.tab &&
          entries == other.entries &&
          loading == other.loading &&
          hasMore == other.hasMore &&
          error == other.error;
}
