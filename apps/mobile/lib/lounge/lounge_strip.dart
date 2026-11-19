import 'package:material_ui/material_ui.dart';

import '../shell/sidebar_section.dart';
import '../src/rust/third_party/pulse_protocol/lounge.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';

String _loungeLabel(LoungeEntry entry) => entry.scope == LoungeScope.campus
    ? 'Campus lounge'
    : '#${entry.roomSlug ?? ''} lounge';

class LoungeStrip extends StatelessWidget {
  const LoungeStrip({required this.lounges, super.key});

  final LoungeSnapshot lounges;

  @override
  Widget build(BuildContext context) {
    if (lounges.lounges.isEmpty) {
      return const SizedBox.shrink();
    }
    final tokens = PulseTokens.of(context);
    return Column(
      key: const Key('loungeStrip'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SidebarHeading(label: 'Lounges'),
        for (final entry in lounges.lounges)
          Padding(
            key: ValueKey(entry.loungeId),
            padding: const EdgeInsets.symmetric(
              horizontal: PulseSpacing.lg,
              vertical: PulseSpacing.xs,
            ),
            child: Row(
              children: [
                Icon(Icons.graphic_eq, size: 16, color: tokens.sidebarMuted),
                const SizedBox(width: PulseSpacing.sm),
                Text(
                  _loungeLabel(entry),
                  style: TextStyle(color: tokens.sidebarText),
                ),
                const SizedBox(width: PulseSpacing.sm),
                Flexible(
                  child: Text(
                    entry.participants.isEmpty
                        ? 'Nobody here'
                        : entry.participants
                              .map(
                                (participant) => participant.fromSelf
                                    ? 'You'
                                    : participant.displayName,
                              )
                              .join(', '),
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(color: tokens.sidebarMuted, fontSize: 12),
                  ),
                ),
              ],
            ),
          ),
      ],
    );
  }
}
