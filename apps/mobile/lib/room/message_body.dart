import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/mention.dart';

class MessageBody extends StatelessWidget {
  const MessageBody({required this.segments, super.key});

  final List<BodySegment> segments;

  TextStyle? _style(SegmentKind kind) {
    switch (kind) {
      case SegmentKind.mention:
        return const TextStyle(fontWeight: FontWeight.bold);
      case SegmentKind.link:
        return const TextStyle(decoration: TextDecoration.underline);
      case SegmentKind.text:
        return null;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Text.rich(
      TextSpan(
        children: [
          for (final segment in segments)
            TextSpan(text: segment.text, style: _style(segment.kind)),
        ],
      ),
    );
  }
}
