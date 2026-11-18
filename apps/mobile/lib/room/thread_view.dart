import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../src/rust/third_party/pulse_protocol/thread.dart';
import 'composer.dart';
import 'timeline_view.dart';

const _subscriptionLabels = {
  SubscriptionState.following: 'Following',
  SubscriptionState.muted: 'Muted',
  SubscriptionState.archived: 'Archived',
};

class ThreadView extends StatefulWidget {
  const ThreadView({
    required this.thread,
    required this.palette,
    required this.intents,
    super.key,
  });

  final ThreadSnapshot thread;
  final List<String> palette;
  final PulseIntents intents;

  @override
  State<ThreadView> createState() => _ThreadViewState();
}

class _ThreadViewState extends State<ThreadView> {
  late final TextEditingController _title;
  Insertion? _insertion;

  @override
  void initState() {
    super.initState();
    _title = TextEditingController(text: widget.thread.topicTitle ?? '');
  }

  @override
  void didUpdateWidget(ThreadView oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.thread.topicTitle != widget.thread.topicTitle ||
        oldWidget.thread.rootMessageId != widget.thread.rootMessageId) {
      _title.text = widget.thread.topicTitle ?? '';
    }
  }

  @override
  void dispose() {
    _title.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final thread = widget.thread;
    final roomId = thread.roomId;
    final rootId = thread.rootMessageId;
    if (roomId == null || rootId == null) {
      return const SizedBox.shrink();
    }
    final intents = widget.intents;
    final root = thread.root;
    final entries = [?root, ...thread.replies];
    final actions = EntryActions(
      react: intents.react,
      discardMessage: intents.discardMessage,
      mention: (token) => setState(() => _insertion = Insertion(token)),
    );
    return Column(
      key: const Key('threadView'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            const Text('Thread'),
            TextButton(
              onPressed: intents.closeThread,
              child: const Text('Close thread'),
            ),
          ],
        ),
        Row(
          children: [
            Expanded(
              child: TextField(
                key: const Key('topicTitleDraft'),
                controller: _title,
                decoration: const InputDecoration(labelText: 'Topic title'),
              ),
            ),
            TextButton(
              onPressed: () =>
                  intents.setTopicTitle(roomId, rootId, _title.text),
              child: const Text('Save title'),
            ),
          ],
        ),
        Row(
          children: [
            for (final entry in _subscriptionLabels.entries)
              TextButton(
                key: ValueKey('subscription-${entry.key.name}'),
                style: thread.subscription == entry.key
                    ? TextButton.styleFrom(
                        textStyle: const TextStyle(fontWeight: FontWeight.bold),
                      )
                    : null,
                onPressed: () =>
                    intents.setThreadSubscription(roomId, rootId, entry.key),
                child: Text(entry.value),
              ),
          ],
        ),
        if (thread.syncing) const Text('Syncing'),
        for (final entry in entries)
          MessageEntryView(
            key: ValueKey(entry.messageId ?? entry.clientNonce),
            entry: entry,
            roomId: roomId,
            palette: widget.palette,
            actions: actions,
          ),
        Composer(
          label: 'Reply',
          insertion: _insertion,
          onSend: (body) => intents.sendReply(roomId, rootId, body),
        ),
      ],
    );
  }
}
