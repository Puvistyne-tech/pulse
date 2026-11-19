import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';

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

  Widget _draftRow(
    BuildContext context,
    DraftAttachmentView draft,
    PulseTokens tokens,
  ) {
    final onDiscardDraft = widget.onDiscardDraft;
    final onRetryDraft = widget.onRetryDraft;
    final failed = draft.upload == UploadState.failed;
    return Container(
      key: ValueKey(draft.draftId),
      margin: const EdgeInsets.only(bottom: PulseSpacing.xs),
      padding: const EdgeInsets.only(left: PulseSpacing.md),
      decoration: BoxDecoration(
        color: failed
            ? tokens.danger.withValues(alpha: 0.1)
            : tokens.surfaceMuted,
        borderRadius: BorderRadius.circular(PulseRadius.control),
      ),
      child: Row(
        children: [
          Icon(
            Icons.attach_file,
            size: 16,
            color: failed ? tokens.danger : tokens.textMuted,
          ),
          const SizedBox(width: PulseSpacing.xs),
          Expanded(
            child: Text(
              '${draft.fileName} ${_uploadLabels[draft.upload]}',
              style: TextStyle(
                color: failed ? tokens.danger : tokens.textMuted,
                fontSize: 13,
              ),
            ),
          ),
          if (failed && onRetryDraft != null)
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
    );
  }

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final onAttach = widget.onAttach;
    const noBorder = InputBorder.none;
    return Padding(
      padding: const EdgeInsets.all(PulseSpacing.md),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          for (final draft in widget.drafts) _draftRow(context, draft, tokens),
          Container(
            padding: const EdgeInsets.symmetric(
              horizontal: PulseSpacing.xs,
              vertical: PulseSpacing.xs,
            ),
            decoration: BoxDecoration(
              color: tokens.surface,
              borderRadius: BorderRadius.circular(PulseRadius.card),
              border: Border.all(color: tokens.border),
            ),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                if (onAttach != null)
                  IconButton(
                    key: const Key('attachFile'),
                    tooltip: 'Attach',
                    onPressed: onAttach,
                    icon: const Icon(Icons.attach_file),
                  ),
                Expanded(
                  child: TextField(
                    key: const Key('messageDraft'),
                    controller: _draft,
                    minLines: 1,
                    maxLines: 6,
                    keyboardType: TextInputType.multiline,
                    textInputAction: TextInputAction.newline,
                    decoration: InputDecoration(
                      hintText: widget.label,
                      filled: false,
                      border: noBorder,
                      enabledBorder: noBorder,
                      focusedBorder: noBorder,
                    ),
                  ),
                ),
                ListenableBuilder(
                  listenable: _draft,
                  builder: (context, child) => IconButton.filled(
                    key: const Key('sendMessage'),
                    tooltip: 'Send',
                    onPressed: _draft.text.isEmpty && widget.drafts.isEmpty
                        ? null
                        : _submit,
                    style: IconButton.styleFrom(
                      backgroundColor: tokens.accent,
                      foregroundColor: tokens.onAccent,
                      disabledBackgroundColor: tokens.surfaceMuted,
                      disabledForegroundColor: tokens.textMuted,
                    ),
                    icon: const Icon(Icons.arrow_upward_rounded),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
