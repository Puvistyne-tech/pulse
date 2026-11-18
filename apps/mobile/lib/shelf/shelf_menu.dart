import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/shelf.dart';

const presetLabels = {
  ReminderPreset.inTwentyMinutes: 'In twenty minutes',
  ReminderPreset.inOneHour: 'In one hour',
  ReminderPreset.inThreeHours: 'In three hours',
  ReminderPreset.tomorrow: 'Tomorrow',
  ReminderPreset.nextWeek: 'Next week',
};

typedef ShelfUpdate = Future<void> Function(
  ShelfAction action,
  ReminderPreset? preset,
);

class ShelfMenu extends StatelessWidget {
  const ShelfMenu({
    required this.presets,
    required this.shelved,
    required this.onUpdate,
    super.key,
  });

  final List<ReminderPreset> presets;
  final bool shelved;
  final ShelfUpdate onUpdate;

  @override
  Widget build(BuildContext context) {
    final scheduleAction = shelved ? ShelfAction.reschedule : ShelfAction.save;
    return PopupMenuButton<(ShelfAction, ReminderPreset?)>(
      key: const Key('shelfMenu'),
      child: const Padding(padding: EdgeInsets.all(8), child: Text('Shelf')),
      onSelected: (choice) => onUpdate(choice.$1, choice.$2),
      itemBuilder: (context) => [
        if (!shelved)
          const PopupMenuItem(
            value: (ShelfAction.save, null),
            child: Text('Save to shelf'),
          ),
        for (final preset in presets)
          PopupMenuItem(
            value: (scheduleAction, preset),
            child: Text(presetLabels[preset]!),
          ),
        if (shelved)
          const PopupMenuItem(
            value: (ShelfAction.remove, null),
            child: Text('Remove from shelf'),
          ),
      ],
    );
  }
}
