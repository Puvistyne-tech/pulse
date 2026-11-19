import 'package:material_ui/material_ui.dart';

import '../shelf/shelf_menu.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/messaging.dart';
import '../src/rust/third_party/pulse_protocol/reaction.dart';
import '../src/rust/third_party/pulse_protocol/shelf.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_avatar.dart';
import '../theme/pulse_theme.dart';
import 'attachment_list.dart';
import 'latest_reply_bar.dart';
import 'message_body.dart';
import 'reactions_view.dart';

const _rejectionLabels = {
  MessageRejection.forbidden: 'Not allowed to post here',
  MessageRejection.invalidBody: 'Message not accepted',
  MessageRejection.unknownRoom: 'Room not found',
  MessageRejection.unknownThread: 'Thread not found',
  MessageRejection.invalidAttachment: 'Attachment not accepted',
};

const _streamLabels = {
  StreamState.streaming: 'Streaming',
  StreamState.finalizing: 'Finalizing',
};

class EntryActions {
  const EntryActions({
    required this.react,
    required this.discardMessage,
    required this.mention,
    this.openThread,
    this.setPin,
    this.updateShelf,
    this.canPin = false,
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
  final Future<void> Function(RoomId roomId, MessageId messageId, bool pinned)?
  setPin;
  final Future<void> Function(
    RoomId roomId,
    MessageId messageId,
    ShelfAction action,
    ReminderPreset? preset,
    BigInt? remindAtMs,
  )?
  updateShelf;
  final bool canPin;
}

class _ActionIcon extends StatelessWidget {
  const _ActionIcon({
    required this.icon,
    required this.tooltip,
    required this.onPressed,
  });

  final IconData icon;
  final String tooltip;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return IconButton(
      tooltip: tooltip,
      onPressed: onPressed,
      iconSize: 18,
      visualDensity: VisualDensity.compact,
      constraints: const BoxConstraints.tightFor(width: 32, height: 32),
      padding: EdgeInsets.zero,
      icon: Icon(icon),
    );
  }
}

class _Pill extends StatelessWidget {
  const _Pill({required this.label, required this.background, this.color});

  final String label;
  final Color background;
  final Color? color;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
      decoration: BoxDecoration(
        color: background,
        borderRadius: BorderRadius.circular(PulseRadius.pill),
      ),
      child: Text(
        label,
        style: TextStyle(
          color: color ?? tokens.accent,
          fontSize: 11,
          fontWeight: FontWeight.w700,
        ),
      ),
    );
  }
}

class MessageEntryView extends StatelessWidget {
  const MessageEntryView({
    required this.entry,
    required this.roomId,
    required this.palette,
    required this.actions,
    this.presets = const [],
    this.pickerAnchorMs,
    this.pickerLatestMs,
    this.focused = false,
    super.key,
  });

  final TimelineEntry entry;
  final RoomId roomId;
  final List<String> palette;
  final EntryActions actions;
  final List<ReminderPreset> presets;
  final BigInt? pickerAnchorMs;
  final BigInt? pickerLatestMs;
  final bool focused;

  Widget _status(PulseTokens tokens) {
    switch (entry.delivery) {
      case DeliveryState.pending:
        return Text(
          'Sending',
          style: TextStyle(color: tokens.textMuted, fontSize: 12),
        );
      case DeliveryState.failed:
        final rejection = entry.rejection;
        return Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.error_outline, size: 14, color: tokens.danger),
            const SizedBox(width: PulseSpacing.xs),
            Text(
              rejection == null ? 'Failed' : _rejectionLabels[rejection]!,
              style: TextStyle(color: tokens.danger, fontSize: 12),
            ),
            TextButton(
              onPressed: () => actions.discardMessage(entry.clientNonce),
              style: TextButton.styleFrom(foregroundColor: tokens.danger),
              child: const Text('Discard'),
            ),
          ],
        );
      case DeliveryState.confirmed:
        return const SizedBox.shrink();
    }
  }

  Widget _threadSummary(
    PulseTokens tokens,
    ThreadPreview thread,
    VoidCallback? open,
  ) {
    return Padding(
      padding: const EdgeInsets.only(top: PulseSpacing.sm),
      child: Material(
        color: tokens.surfaceMuted,
        borderRadius: BorderRadius.circular(PulseRadius.control),
        child: InkWell(
          borderRadius: BorderRadius.circular(PulseRadius.control),
          onTap: open,
          child: Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: PulseSpacing.md,
              vertical: PulseSpacing.sm,
            ),
            child: Wrap(
              spacing: PulseSpacing.sm,
              runSpacing: PulseSpacing.xs,
              crossAxisAlignment: WrapCrossAlignment.center,
              children: [
                Icon(Icons.forum_outlined, size: 16, color: tokens.accent),
                if (thread.topicTitle != null)
                  Text(
                    thread.topicTitle!,
                    style: const TextStyle(fontWeight: FontWeight.w700),
                  ),
                Text(
                  'Replies ${thread.replyCount}',
                  style: TextStyle(
                    color: tokens.accent,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                if (thread.subscription != null)
                  Text(
                    thread.subscription!.name,
                    style: TextStyle(color: tokens.textMuted, fontSize: 12),
                  ),
                if (thread.unreadReplies > 0)
                  _Pill(
                    label: '${thread.unreadReplies}',
                    background: tokens.accent,
                    color: tokens.onAccent,
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final messageId = entry.messageId;
    final thread = entry.thread;
    final openThread = actions.openThread;
    final setPin = actions.setPin;
    final updateShelf = actions.updateShelf;
    final pending = entry.delivery == DeliveryState.pending;
    final failed = entry.delivery == DeliveryState.failed;
    final bodyColor = pending
        ? tokens.textMuted
        : failed
        ? tokens.danger
        : tokens.text;
    return _FocusScroll(
      focused: focused,
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: PulseSpacing.sm),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            PulseAvatar(name: entry.authorDisplayName),
            const SizedBox(width: PulseSpacing.md),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Wrap(
                    spacing: PulseSpacing.sm,
                    crossAxisAlignment: WrapCrossAlignment.center,
                    children: [
                      Text(
                        entry.fromSelf ? 'You' : entry.authorDisplayName,
                        style: const TextStyle(fontWeight: FontWeight.w700),
                      ),
                      if (entry.authorIsSteward)
                        _Pill(label: 'Steward', background: tokens.accentSoft),
                      if (entry.stream != null)
                        Text(
                          _streamLabels[entry.stream]!,
                          style: TextStyle(
                            color: tokens.textMuted,
                            fontSize: 12,
                          ),
                        ),
                      _status(tokens),
                    ],
                  ),
                  const SizedBox(height: PulseSpacing.xs),
                  DefaultTextStyle.merge(
                    style: TextStyle(color: bodyColor, height: 1.4),
                    child: Row(
                      children: [
                        Flexible(child: MessageBody(segments: entry.segments)),
                      ],
                    ),
                  ),
                  AttachmentList(attachments: entry.attachments),
                  if (thread != null)
                    _threadSummary(
                      tokens,
                      thread,
                      messageId != null && openThread != null
                          ? () => openThread(roomId, messageId)
                          : null,
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
                        _ActionIcon(
                          icon: Icons.forum_outlined,
                          tooltip: 'Reply in thread',
                          onPressed: () => openThread(roomId, messageId),
                        ),
                      if (messageId != null && actions.canPin && setPin != null)
                        _ActionIcon(
                          icon: entry.pinned
                              ? Icons.push_pin
                              : Icons.push_pin_outlined,
                          tooltip: entry.pinned ? 'Unpin' : 'Pin',
                          onPressed: () =>
                              setPin(roomId, messageId, !entry.pinned),
                        ),
                      if (messageId != null && updateShelf != null)
                        ShelfMenu(
                          presets: presets,
                          shelved: entry.shelved,
                          pickerAnchorMs: pickerAnchorMs,
                          pickerLatestMs: pickerLatestMs,
                          onUpdate: (action, preset, remindAtMs) => updateShelf(
                            roomId,
                            messageId,
                            action,
                            preset,
                            remindAtMs,
                          ),
                        ),
                      if (!entry.fromSelf)
                        _ActionIcon(
                          icon: Icons.alternate_email,
                          tooltip: 'Mention',
                          onPressed: () =>
                              actions.mention(entry.authorMentionToken),
                        ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _FocusScroll extends StatefulWidget {
  const _FocusScroll({required this.focused, required this.child});

  final bool focused;
  final Widget child;

  @override
  State<_FocusScroll> createState() => _FocusScrollState();
}

class _FocusScrollState extends State<_FocusScroll> {
  @override
  void initState() {
    super.initState();
    _revealWhenFocused();
  }

  @override
  void didUpdateWidget(_FocusScroll oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.focused && !oldWidget.focused) {
      _revealWhenFocused();
    }
  }

  void _revealWhenFocused() {
    if (!widget.focused) {
      return;
    }
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        Scrollable.ensureVisible(context);
      }
    });
  }

  @override
  Widget build(BuildContext context) => widget.child;
}

class TimelineView extends StatelessWidget {
  const TimelineView({
    required this.timeline,
    required this.roomId,
    required this.actions,
    this.presets = const [],
    this.pickerAnchorMs,
    this.pickerLatestMs,
    super.key,
  });

  final TimelineSnapshot timeline;
  final RoomId roomId;
  final EntryActions actions;
  final List<ReminderPreset> presets;
  final BigInt? pickerAnchorMs;
  final BigInt? pickerLatestMs;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        if (timeline.syncing)
          Padding(
            padding: const EdgeInsets.symmetric(vertical: PulseSpacing.sm),
            child: Text('Syncing', style: TextStyle(color: tokens.textMuted)),
          ),
        for (final entry in timeline.entries)
          MessageEntryView(
            key: ValueKey(entry.messageId ?? entry.clientNonce),
            entry: entry,
            roomId: roomId,
            palette: timeline.reactionPalette,
            actions: actions,
            presets: presets,
            pickerAnchorMs: pickerAnchorMs,
            pickerLatestMs: pickerLatestMs,
            focused:
                entry.messageId != null &&
                entry.messageId == timeline.focusMessageId,
          ),
      ],
    );
  }
}
