import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/messaging.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

const _rejectionLabels = {
  MessageRejection.forbidden: 'Not allowed to post here',
  MessageRejection.invalidBody: 'Message not accepted',
  MessageRejection.unknownRoom: 'Room not found',
};

class TimelineView extends StatelessWidget {
  const TimelineView({
    required this.timeline,
    required this.discardMessage,
    super.key,
  });

  final TimelineSnapshot timeline;
  final Future<void> Function(ClientNonce clientNonce) discardMessage;

  Widget _status(TimelineEntry entry) {
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
              onPressed: () => discardMessage(entry.clientNonce),
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
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (timeline.syncing) const Text('Syncing'),
        for (final entry in timeline.entries)
          Row(
            key: ValueKey(entry.messageId ?? entry.clientNonce),
            children: [
              Text(entry.fromSelf ? 'You' : entry.authorDisplayName),
              const SizedBox(width: 8),
              Flexible(child: Text(entry.body)),
              const SizedBox(width: 8),
              _status(entry),
            ],
          ),
      ],
    );
  }
}
