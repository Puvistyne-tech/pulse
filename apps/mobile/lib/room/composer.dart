import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/ids.dart';

class Composer extends StatefulWidget {
  const Composer({required this.roomId, required this.sendMessage, super.key});

  final RoomId roomId;
  final Future<void> Function(RoomId roomId, String body) sendMessage;

  @override
  State<Composer> createState() => _ComposerState();
}

class _ComposerState extends State<Composer> {
  final _draft = TextEditingController();

  @override
  void dispose() {
    _draft.dispose();
    super.dispose();
  }

  void _submit() {
    widget.sendMessage(widget.roomId, _draft.text);
    _draft.clear();
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: TextField(
            key: const Key('messageDraft'),
            controller: _draft,
            decoration: const InputDecoration(labelText: 'Message'),
          ),
        ),
        ListenableBuilder(
          listenable: _draft,
          builder: (context, child) => ElevatedButton(
            key: const Key('sendMessage'),
            onPressed: _draft.text.isEmpty ? null : _submit,
            child: const Text('Send'),
          ),
        ),
      ],
    );
  }
}
