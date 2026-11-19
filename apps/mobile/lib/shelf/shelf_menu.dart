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
  BigInt? remindAtMs,
);

typedef _Choice = (ShelfAction, ReminderPreset?, bool);

class ShelfMenu extends StatelessWidget {
  const ShelfMenu({
    required this.presets,
    required this.shelved,
    required this.onUpdate,
    this.pickerAnchorMs,
    this.pickerLatestMs,
    super.key,
  });

  final List<ReminderPreset> presets;
  final bool shelved;
  final ShelfUpdate onUpdate;
  final BigInt? pickerAnchorMs;
  final BigInt? pickerLatestMs;

  Future<void> _pickCustomTime(
    BuildContext context,
    ShelfAction action,
    BigInt anchorMs,
    BigInt latestMs,
  ) async {
    final anchor = DateTime.fromMillisecondsSinceEpoch(anchorMs.toInt());
    final latest = DateTime.fromMillisecondsSinceEpoch(latestMs.toInt());
    final date = await showDatePicker(
      context: context,
      initialDate: anchor,
      firstDate: anchor,
      lastDate: latest,
    );
    if (date == null || !context.mounted) {
      return;
    }
    final time = await showTimePicker(
      context: context,
      initialTime: TimeOfDay.fromDateTime(anchor),
    );
    if (time == null) {
      return;
    }
    final chosen = DateTime(
      date.year,
      date.month,
      date.day,
      time.hour,
      time.minute,
    );
    await onUpdate(action, null, BigInt.from(chosen.millisecondsSinceEpoch));
  }

  @override
  Widget build(BuildContext context) {
    final scheduleAction = shelved ? ShelfAction.reschedule : ShelfAction.save;
    final anchorMs = pickerAnchorMs;
    final latestMs = pickerLatestMs;
    return PopupMenuButton<_Choice>(
      key: const Key('shelfMenu'),
      child: const Padding(padding: EdgeInsets.all(8), child: Text('Shelf')),
      onSelected: (choice) {
        if (choice.$3 && anchorMs != null && latestMs != null) {
          _pickCustomTime(context, choice.$1, anchorMs, latestMs);
        } else {
          onUpdate(choice.$1, choice.$2, null);
        }
      },
      itemBuilder: (context) => [
        if (!shelved)
          const PopupMenuItem(
            value: (ShelfAction.save, null, false),
            child: Text('Save to shelf'),
          ),
        for (final preset in presets)
          PopupMenuItem(
            value: (scheduleAction, preset, false),
            child: Text(presetLabels[preset]!),
          ),
        if (anchorMs != null && latestMs != null)
          PopupMenuItem(
            key: const Key('shelfCustomTime'),
            value: (scheduleAction, null, true),
            child: const Text('Pick a date and time'),
          ),
        if (shelved)
          const PopupMenuItem(
            value: (ShelfAction.remove, null, false),
            child: Text('Remove from shelf'),
          ),
      ],
    );
  }
}
