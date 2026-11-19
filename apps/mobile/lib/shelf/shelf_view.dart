import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../session/error_notice.dart';
import '../src/rust/third_party/pulse_protocol/shelf.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../shell/panel_card.dart';
import '../theme/pulse_theme.dart';
import 'shelf_menu.dart';

class ShelfView extends StatelessWidget {
  const ShelfView({required this.shelf, required this.intents, super.key});

  final ShelfSnapshot shelf;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    return PanelCard(
      cardKey: const Key('shelfView'),
      title: 'Shelf ${shelf.dueCount}',
      closeLabel: 'Close shelf',
      onClose: intents.toggleShelf,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          ErrorNotice(error: shelf.error),
          _Section(
            label: 'Due',
            entries: shelf.due,
            presets: shelf.presets,
            pickerAnchorMs: shelf.pickerAnchorMs,
            pickerLatestMs: shelf.pickerLatestMs,
            intents: intents,
          ),
          _Section(
            label: 'Upcoming',
            entries: shelf.upcoming,
            presets: shelf.presets,
            pickerAnchorMs: shelf.pickerAnchorMs,
            pickerLatestMs: shelf.pickerLatestMs,
            intents: intents,
          ),
          _Section(
            label: 'Done',
            entries: shelf.done,
            presets: shelf.presets,
            pickerAnchorMs: shelf.pickerAnchorMs,
            pickerLatestMs: shelf.pickerLatestMs,
            intents: intents,
          ),
        ],
      ),
    );
  }
}

class _Section extends StatelessWidget {
  const _Section({
    required this.label,
    required this.entries,
    required this.presets,
    required this.pickerAnchorMs,
    required this.pickerLatestMs,
    required this.intents,
  });

  final String label;
  final List<ShelfEntry> entries;
  final List<ReminderPreset> presets;
  final BigInt? pickerAnchorMs;
  final BigInt? pickerLatestMs;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    if (entries.isEmpty) {
      return const SizedBox.shrink();
    }
    final tokens = PulseTokens.of(context);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.only(
            top: PulseSpacing.sm,
            bottom: PulseSpacing.xs,
          ),
          child: Text(
            label,
            style: TextStyle(
              color: tokens.textMuted,
              fontSize: 12,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
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
                pickerAnchorMs: pickerAnchorMs,
                pickerLatestMs: pickerLatestMs,
                onUpdate: (action, preset, remindAtMs) => intents.updateShelf(
                  entry.roomId,
                  entry.messageId,
                  action,
                  preset,
                  remindAtMs,
                ),
              ),
            ],
          ),
      ],
    );
  }
}
