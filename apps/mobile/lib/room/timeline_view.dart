import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/messaging.dart';
import '../src/rust/third_party/pulse_protocol/reaction.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import 'latest_reply_bar.dart';
import 'message_body.dart';
import 'reactions_view.dart';

const _rejectionLabels = {
  MessageRejection.forbidden: 'Not allowed to post here',
  MessageRejection.invalidBody: 'Message not accepted',
  MessageRejection.unknownRoom: 'Room not found',
  MessageRejection.unknownThread: 'Thread not found',
};

class EntryActions {
  const EntryActions({
    required this.react,
    required this.discardMessage,
    required this.mention,
    this.openThread,
  });

  final Future<void> Function(
    RoomId roomId,
    MessageId messageId,
    String emoji,
    ReactionAction action,
  )
  react;
  final Future<void> Function(ClientNonce clientNonce) discardMessage;
  final void Function(String token) mention;
  final Future<void> Function(RoomId roomId, MessageId rootMessageId)?
  openThread;
}

class MessageEntryView extends StatelessWidget {
  const MessageEntryView({
    required this.entry,
    required this.roomId,
    required this.palette,
    required this.actions,
    super.key,
  });

  final TimelineEntry entry;
  final RoomId roomId;
  final List<String> palette;
  final EntryActions actions;

  Widget _status() {
    switch (entry.delivery) {
      case DeliveryState.pending:
        return const Text('Sending');
      case DeliveryState.failed:
        final rejection = entry.rejection;
        return Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(rejection == null ? 'Failed' : _rejectionLabels[rejection]!),
            TextButton(
              onPressed: () => actions.discardMessage(entry.clientNonce),
              child: const Text('Discard'),
            ),
          ],
        );
      case DeliveryState.confirmed:
        return const SizedBox.shrink();
    }
  }

  @override
  Widget build(BuildContext context) {
    final messageId = entry.messageId;
    final thread = entry.thread;
    final openThread = actions.openThread;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text(entry.fromSelf ? 'You' : entry.authorDisplayName),
            const SizedBox(width: 8),
            Flexible(child: MessageBody(segments: entry.segments)),
            const SizedBox(width: 8),
            _status(),
          ],
        ),
        if (thread != null)
          Row(
            children: [
              if (thread.topicTitle != null) Text(thread.topicTitle!),
              const SizedBox(width: 8),
              Text('Replies ${thread.replyCount}'),
              if (thread.subscription != null) ...[
                const SizedBox(width: 8),
                Text(thread.subscription!.name),
              ],
              if (thread.unreadReplies > 0) ...[
                const SizedBox(width: 8),
                Text('${thread.unreadReplies}'),
              ],
            ],
          ),
        if (thread != null && messageId != null && openThread != null)
          LatestReplyBar(
            thread: thread,
            openThread: () => openThread(roomId, messageId),
          ),
        if (messageId != null)
          ReactionsView(
            roomId: roomId,
            messageId: messageId,
            reactions: entry.reactions,
            palette: palette,
            react: actions.react,
          ),
        Row(
          children: [
            if (messageId != null &&
                openThread != null &&
                entry.threadRootId == null)
              TextButton(
                onPressed: () => openThread(roomId, messageId),
                child: const Text('Reply in thread'),
              ),
            if (!entry.fromSelf)
              TextButton(
                onPressed: () => actions.mention(entry.authorMentionToken),
                child: const Text('Mention'),
              ),
          ],
        ),
      ],
    );
  }
}

class TimelineView extends StatelessWidget {
  const TimelineView({
    required this.timeline,
    required this.roomId,
    required this.actions,
    super.key,
  });

  final TimelineSnapshot timeline;
  final RoomId roomId;
  final EntryActions actions;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (timeline.syncing) const Text('Syncing'),
        for (final entry in timeline.entries)
          MessageEntryView(
            key: ValueKey(entry.messageId ?? entry.clientNonce),
            entry: entry,
            roomId: roomId,
            palette: timeline.reactionPalette,
            actions: actions,
          ),
      ],
    );
  }
}
