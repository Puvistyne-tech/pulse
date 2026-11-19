import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/reaction.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';

class ReactionsView extends StatelessWidget {
  const ReactionsView({
    required this.roomId,
    required this.messageId,
    required this.reactions,
    required this.palette,
    required this.react,
    super.key,
  });

  final RoomId roomId;
  final MessageId messageId;
  final List<ReactionSummary> reactions;
  final List<String> palette;
  final Future<void> Function(
    RoomId roomId,
    MessageId messageId,
    String emoji,
    ReactionAction action,
  )
  react;

  void _toggle(String emoji) {
    final reactedBySelf = reactions.any(
      (reaction) => reaction.emoji == emoji && reaction.reactedBySelf,
    );
    react(
      roomId,
      messageId,
      emoji,
      reactedBySelf ? ReactionAction.remove : ReactionAction.add,
    );
  }

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Padding(
      padding: const EdgeInsets.only(top: PulseSpacing.sm),
      child: Wrap(
        spacing: PulseSpacing.xs,
        runSpacing: PulseSpacing.xs,
        crossAxisAlignment: WrapCrossAlignment.center,
        children: [
          for (final reaction in reactions)
            FilterChip(
              key: ValueKey('reaction-${reaction.emoji}'),
              selected: reaction.reactedBySelf,
              side: BorderSide(
                color: reaction.reactedBySelf ? tokens.accent : tokens.border,
              ),
              label: Text('${reaction.emoji} ${reaction.count}'),
              onSelected: (_) => _toggle(reaction.emoji),
            ),
          for (final emoji in palette)
            TextButton(
              key: ValueKey('palette-$emoji'),
              onPressed: () => _toggle(emoji),
              style: TextButton.styleFrom(
                foregroundColor: tokens.textMuted,
                minimumSize: const Size(32, 28),
                padding: const EdgeInsets.symmetric(horizontal: 6),
                textStyle: const TextStyle(fontSize: 14),
              ),
              child: Text(emoji),
            ),
        ],
      ),
    );
  }
}
