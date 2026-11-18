import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/mention.dart';

class MessageBody extends StatelessWidget {
  const MessageBody({required this.segments, super.key});

  final List<BodySegment> segments;

  @override
  Widget build(BuildContext context) {
    return Text.rich(
      TextSpan(
        children: [
          for (final segment in segments)
            TextSpan(
              text: segment.text,
              style: segment.kind == SegmentKind.mention
                  ? const TextStyle(fontWeight: FontWeight.bold)
                  : null,
            ),
        ],
      ),
    );
  }
}
