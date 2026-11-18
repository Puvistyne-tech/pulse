import 'package:material_ui/material_ui.dart';

import '../session/error_notice.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import 'composer.dart';
import 'timeline_view.dart';

class RoomView extends StatelessWidget {
  const RoomView({
    required this.timeline,
    required this.sendMessage,
    required this.discardMessage,
    super.key,
  });

  final TimelineSnapshot timeline;
  final Future<void> Function(RoomId roomId, String body) sendMessage;
  final Future<void> Function(ClientNonce clientNonce) discardMessage;

  @override
  Widget build(BuildContext context) {
    final roomId = timeline.roomId;
    if (roomId == null) {
      return const SizedBox.shrink();
    }
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        TimelineView(timeline: timeline, discardMessage: discardMessage),
        ErrorNotice(error: timeline.error),
        Composer(roomId: roomId, sendMessage: sendMessage),
      ],
    );
  }
}
