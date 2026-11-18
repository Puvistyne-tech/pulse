import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../session/error_notice.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import 'composer.dart';
import 'timeline_view.dart';

class RoomView extends StatefulWidget {
  const RoomView({required this.timeline, required this.intents, super.key});

  final TimelineSnapshot timeline;
  final PulseIntents intents;

  @override
  State<RoomView> createState() => _RoomViewState();
}

class _RoomViewState extends State<RoomView> {
  Insertion? _insertion;

  @override
  Widget build(BuildContext context) {
    final roomId = widget.timeline.roomId;
    if (roomId == null) {
      return const SizedBox.shrink();
    }
    final intents = widget.intents;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        TimelineView(
          timeline: widget.timeline,
          roomId: roomId,
          actions: EntryActions(
            react: intents.react,
            discardMessage: intents.discardMessage,
            openThread: intents.openThread,
            mention: (token) => setState(() => _insertion = Insertion(token)),
          ),
        ),
        ErrorNotice(error: widget.timeline.error),
        Composer(
          insertion: _insertion,
          onSend: (body) => intents.sendMessage(roomId, body),
        ),
      ],
    );
  }
}
