import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

const _uploadLabels = {
  UploadState.uploading: 'Uploading',
  UploadState.uploaded: 'Ready',
  UploadState.failed: 'Failed',
};

class Insertion {
  Insertion(this.text);

  final String text;
}

class Composer extends StatefulWidget {
  const Composer({
    required this.onSend,
    this.label = 'Message',
    this.insertion,
    this.drafts = const [],
    this.onAttach,
    this.onDiscardDraft,
    this.onRetryDraft,
    super.key,
  });

  final void Function(String body) onSend;
  final String label;
  final Insertion? insertion;
  final List<DraftAttachmentView> drafts;
  final VoidCallback? onAttach;
  final void Function(DraftId draftId)? onDiscardDraft;
  final void Function(DraftId draftId)? onRetryDraft;

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
    final onAttach = widget.onAttach;
    final onDiscardDraft = widget.onDiscardDraft;
    final onRetryDraft = widget.onRetryDraft;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        for (final draft in widget.drafts)
          Row(
            key: ValueKey(draft.draftId),
            children: [
              Expanded(
                child: Text('${draft.fileName} ${_uploadLabels[draft.upload]}'),
              ),
              if (draft.upload == UploadState.failed && onRetryDraft != null)
                TextButton(
                  onPressed: () => onRetryDraft(draft.draftId),
                  child: const Text('Retry'),
                ),
              if (onDiscardDraft != null)
                TextButton(
                  onPressed: () => onDiscardDraft(draft.draftId),
                  child: const Text('Remove'),
                ),
            ],
          ),
        Row(
          children: [
            if (onAttach != null)
              TextButton(
                key: const Key('attachFile'),
                onPressed: onAttach,
                child: const Text('Attach'),
              ),
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
        ),
      ],
    );
  }
}
