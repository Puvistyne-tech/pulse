import 'package:file_picker/file_picker.dart';
import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../session/error_notice.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/media.dart';
import '../src/rust/third_party/pulse_protocol/shelf.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import 'composer.dart';
import 'timeline_view.dart';

class RoomView extends StatefulWidget {
  const RoomView({
    required this.timeline,
    required this.intents,
    this.presets = const [],
    this.pickerAnchorMs,
    this.pickerLatestMs,
    super.key,
  });

  final TimelineSnapshot timeline;
  final PulseIntents intents;
  final List<ReminderPreset> presets;
  final BigInt? pickerAnchorMs;
  final BigInt? pickerLatestMs;

  @override
  State<RoomView> createState() => _RoomViewState();
}

class _RoomViewState extends State<RoomView> {
  Insertion? _insertion;

  Future<void> _pickFile(RoomId roomId) async {
    final result = await FilePicker.platform.pickFiles(withData: true);
    final file = result?.files.firstOrNull;
    final bytes = file?.bytes;
    if (file != null && bytes != null) {
      await widget.intents.attachFile(roomId, file.name, bytes);
    }
  }

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
        Align(
          alignment: Alignment.centerRight,
          child: TextButton(
            key: const Key('openVault'),
            onPressed: () => intents.openVault(roomId, VaultTab.media),
            child: const Text('Vault'),
          ),
        ),
        TimelineView(
          timeline: widget.timeline,
          roomId: roomId,
          presets: widget.presets,
          pickerAnchorMs: widget.pickerAnchorMs,
          pickerLatestMs: widget.pickerLatestMs,
          actions: EntryActions(
            react: intents.react,
            discardMessage: intents.discardMessage,
            openThread: intents.openThread,
            setPin: intents.setPin,
            updateShelf: intents.updateShelf,
            canPin: widget.timeline.canPin,
            mention: (token) => setState(() => _insertion = Insertion(token)),
          ),
        ),
        ErrorNotice(error: widget.timeline.error),
        Composer(
          insertion: _insertion,
          drafts: widget.timeline.drafts,
          onAttach: widget.timeline.canUpload ? () => _pickFile(roomId) : null,
          onDiscardDraft: intents.discardDraft,
          onRetryDraft: intents.retryDraft,
          onSend: (body) => intents.sendMessage(roomId, body),
        ),
      ],
    );
  }
}
