import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../session/error_notice.dart';
import '../src/rust/third_party/pulse_protocol/shelf.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import 'shelf_menu.dart';

class ShelfView extends StatelessWidget {
  const ShelfView({required this.shelf, required this.intents, super.key});

  final ShelfSnapshot shelf;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    return Column(
      key: const Key('shelfView'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text('Shelf ${shelf.dueCount}'),
            TextButton(
              onPressed: intents.toggleShelf,
              child: const Text('Close shelf'),
            ),
          ],
        ),
        ErrorNotice(error: shelf.error),
        _Section(
          label: 'Due',
          entries: shelf.due,
          presets: shelf.presets,
          intents: intents,
        ),
        _Section(
          label: 'Upcoming',
          entries: shelf.upcoming,
          presets: shelf.presets,
          intents: intents,
        ),
        _Section(
          label: 'Done',
          entries: shelf.done,
          presets: shelf.presets,
          intents: intents,
        ),
      ],
    );
  }
}

class _Section extends StatelessWidget {
  const _Section({
    required this.label,
    required this.entries,
    required this.presets,
    required this.intents,
  });

  final String label;
  final List<ShelfEntry> entries;
  final List<ReminderPreset> presets;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    if (entries.isEmpty) {
      return const SizedBox.shrink();
    }
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: const TextStyle(fontWeight: FontWeight.bold)),
        for (final entry in entries)
          Row(
            key: ValueKey(entry.messageId),
            children: [
              Expanded(
                child: Text('${entry.authorDisplayName}: ${entry.preview}'),
              ),
              if (entry.bucket == ShelfBucket.done)
                TextButton(
                  onPressed: entry.pending
                      ? null
                      : () => intents.updateShelf(
                          entry.roomId,
                          entry.messageId,
                          ShelfAction.reopen,
                          null,
                          null,
                        ),
                  child: const Text('Reopen'),
                )
              else
                TextButton(
                  onPressed: entry.pending
                      ? null
                      : () => intents.updateShelf(
                          entry.roomId,
                          entry.messageId,
                          ShelfAction.complete,
                          null,
                          null,
                        ),
                  child: const Text('Complete'),
                ),
              ShelfMenu(
                presets: presets,
                shelved: true,
                onUpdate: (action, preset) => intents.updateShelf(
                  entry.roomId,
                  entry.messageId,
                  action,
                  preset,
                  null,
                ),
              ),
            ],
          ),
      ],
    );
  }
}
