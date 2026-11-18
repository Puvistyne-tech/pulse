import 'package:material_ui/material_ui.dart';

class Insertion {
  Insertion(this.text);

  final String text;
}

class Composer extends StatefulWidget {
  const Composer({
    required this.onSend,
    this.label = 'Message',
    this.insertion,
    super.key,
  });

  final void Function(String body) onSend;
  final String label;
  final Insertion? insertion;

  @override
  State<Composer> createState() => _ComposerState();
}

class _ComposerState extends State<Composer> {
  final _draft = TextEditingController();

  @override
  void didUpdateWidget(Composer oldWidget) {
    super.didUpdateWidget(oldWidget);
    final insertion = widget.insertion;
    if (insertion != null && !identical(insertion, oldWidget.insertion)) {
      final prefix = _draft.text.isEmpty ? '' : '${_draft.text} ';
      _draft.text = '$prefix${insertion.text} ';
    }
  }

  @override
  void dispose() {
    _draft.dispose();
    super.dispose();
  }

  void _submit() {
    widget.onSend(_draft.text);
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
            decoration: InputDecoration(labelText: widget.label),
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
