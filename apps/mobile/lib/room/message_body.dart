import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/mention.dart';
import '../theme/pulse_theme.dart';

class MessageBody extends StatelessWidget {
  const MessageBody({required this.segments, super.key});

  final List<BodySegment> segments;

  TextStyle? _style(SegmentKind kind, PulseTokens tokens) {
    switch (kind) {
      case SegmentKind.mention:
        return TextStyle(fontWeight: FontWeight.bold, color: tokens.accent);
      case SegmentKind.link:
        return TextStyle(
          decoration: TextDecoration.underline,
          color: tokens.accent,
        );
      case SegmentKind.text:
        return null;
    }
  }

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Text.rich(
      TextSpan(
        children: [
          for (final segment in segments)
            TextSpan(text: segment.text, style: _style(segment.kind, tokens)),
        ],
      ),
    );
  }
}
