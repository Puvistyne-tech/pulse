import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/lounge.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

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
    return Column(
      key: const Key('loungeStrip'),
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Lounges', style: TextStyle(fontWeight: FontWeight.bold)),
        for (final entry in lounges.lounges)
          Row(
            key: ValueKey(entry.loungeId),
            children: [
              Text(_loungeLabel(entry)),
              const SizedBox(width: 8),
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
                ),
              ),
            ],
          ),
      ],
    );
  }
}
