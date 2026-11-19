import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../room/composer.dart';
import '../shell/panel_card.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';
import 'inbox_shortcuts.dart';

class InboxView extends StatelessWidget {
  const InboxView({required this.inbox, required this.intents, super.key});

  final InboxSnapshot inbox;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    return CallbackShortcuts(
      bindings: inboxBindings(inbox, intents),
      child: Focus(
        autofocus: true,
        child: PanelCard(
          cardKey: const Key('inboxView'),
          title: 'Inbox ${inbox.unreadThreads}',
          closeLabel: 'Close inbox',
          onClose: intents.toggleInbox,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              for (final item in inbox.items)
                _InboxRow(
                  key: ValueKey(item.rootMessageId),
                  item: item,
                  selected: item.rootMessageId == inbox.selectedRootMessageId,
                  replying: inbox.replying,
                  intents: intents,
                ),
            ],
          ),
        ),
      ),
    );
  }
}

class _InboxRow extends StatelessWidget {
  const _InboxRow({
    required this.item,
    required this.selected,
    required this.replying,
    required this.intents,
    super.key,
  });

  final InboxItem item;
  final bool selected;
  final bool replying;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final latest = item.latestReply;
    return Container(
      margin: const EdgeInsets.only(bottom: PulseSpacing.sm),
      decoration: BoxDecoration(
        color: selected ? tokens.accentSoft : tokens.surface,
        borderRadius: BorderRadius.circular(PulseRadius.card),
        border: Border.all(color: selected ? tokens.accent : tokens.border),
      ),
      padding: const EdgeInsets.all(PulseSpacing.md),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Expanded(
                child: Text(
                  item.topicTitle ?? item.rootPreview,
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
              ),
              if (item.unreadMentions > 0)
                Text(
                  '@',
                  style: TextStyle(
                    color: tokens.accent,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              if (item.unreadReplies > 0) ...[
                const SizedBox(width: PulseSpacing.xs),
                Text(
                  '${item.unreadReplies}',
                  style: TextStyle(
                    color: tokens.accent,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
              const SizedBox(width: PulseSpacing.sm),
              Text(
                item.subscription.name,
                style: TextStyle(color: tokens.textMuted, fontSize: 12),
              ),
            ],
          ),
          const SizedBox(height: PulseSpacing.xs),
          Text(
            '${item.rootAuthorDisplayName}: ${item.rootPreview}',
            style: TextStyle(color: tokens.textMuted),
          ),
          if (latest != null)
            Text(
              '${latest.authorDisplayName}: ${latest.preview}',
              style: TextStyle(color: tokens.textMuted),
            ),
          if (selected)
            Row(
              children: [
                TextButton(
                  onPressed: intents.archiveInboxSelection,
                  child: const Text('Archive'),
                ),
                TextButton(
                  onPressed: intents.startInboxReply,
                  child: const Text('Reply'),
                ),
              ],
            ),
          if (selected && replying)
            Composer(
              label: 'Inbox reply',
              onSend: (body) =>
                  intents.sendReply(item.roomId, item.rootMessageId, body),
            ),
        ],
      ),
    );
  }
}
