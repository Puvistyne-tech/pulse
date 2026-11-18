import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/snapshot.dart';

class ReminderBanner extends StatelessWidget {
  const ReminderBanner({required this.entry, required this.dismiss, super.key});

  final ShelfEntry entry;
  final Future<void> Function() dismiss;

  @override
  Widget build(BuildContext context) {
    return Row(
      key: const Key('reminderBanner'),
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Expanded(child: Text('${entry.authorDisplayName}: ${entry.preview}')),
        TextButton(onPressed: dismiss, child: const Text('Dismiss')),
      ],
    );
  }
}
