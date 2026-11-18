import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../room/composer.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
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
        child: Column(
          key: const Key('inboxView'),
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text('Inbox ${inbox.unreadThreads}'),
                TextButton(
                  onPressed: intents.toggleInbox,
                  child: const Text('Close inbox'),
                ),
              ],
            ),
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
    final latest = item.latestReply;
    return Container(
      color: selected ? const Color(0x11000000) : null,
      padding: const EdgeInsets.all(8),
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
              if (item.unreadMentions > 0) const Text('@'),
              if (item.unreadReplies > 0) Text('${item.unreadReplies}'),
              const SizedBox(width: 8),
              Text(item.subscription.name),
            ],
          ),
          Text('${item.rootAuthorDisplayName}: ${item.rootPreview}'),
          if (latest != null)
            Text('${latest.authorDisplayName}: ${latest.preview}'),
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
