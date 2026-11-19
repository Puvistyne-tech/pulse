import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';

class ReminderBanner extends StatelessWidget {
  const ReminderBanner({required this.entry, required this.dismiss, super.key});

  final ShelfEntry entry;
  final Future<void> Function() dismiss;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Container(
      key: const Key('reminderBanner'),
      margin: const EdgeInsets.all(PulseSpacing.sm),
      padding: const EdgeInsets.symmetric(
        horizontal: PulseSpacing.md,
        vertical: PulseSpacing.xs,
      ),
      decoration: BoxDecoration(
        color: tokens.warning.withValues(alpha: 0.12),
        borderRadius: BorderRadius.circular(PulseRadius.control),
        border: Border.all(color: tokens.warning.withValues(alpha: 0.4)),
      ),
      child: Row(
        children: [
          Icon(Icons.notifications_active_outlined, color: tokens.warning),
          const SizedBox(width: PulseSpacing.sm),
          Expanded(child: Text('${entry.authorDisplayName}: ${entry.preview}')),
          TextButton(
            onPressed: dismiss,
            style: TextButton.styleFrom(foregroundColor: tokens.warning),
            child: const Text('Dismiss'),
          ),
        ],
      ),
    );
  }
}
