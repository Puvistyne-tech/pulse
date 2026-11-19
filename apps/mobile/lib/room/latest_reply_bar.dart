import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';

class LatestReplyBar extends StatefulWidget {
  const LatestReplyBar({
    required this.thread,
    required this.openThread,
    super.key,
  });

  final ThreadPreview thread;
  final VoidCallback openThread;

  @override
  State<LatestReplyBar> createState() => _LatestReplyBarState();
}

class _LatestReplyBarState extends State<LatestReplyBar> {
  bool _collapsed = false;

  @override
  Widget build(BuildContext context) {
    final latest = widget.thread.latestReply;
    if (!widget.thread.showLatestReply || latest == null) {
      return const SizedBox.shrink();
    }
    final tokens = PulseTokens.of(context);
    return Row(
      key: const Key('latestReplyBar'),
      children: [
        TextButton(
          onPressed: () => setState(() => _collapsed = !_collapsed),
          child: const Text('Latest reply'),
        ),
        if (!_collapsed) ...[
          Text(
            latest.authorDisplayName,
            style: const TextStyle(fontWeight: FontWeight.w700),
          ),
          const SizedBox(width: PulseSpacing.sm),
          Flexible(
            child: Text(
              latest.preview,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(color: tokens.textMuted),
            ),
          ),
          TextButton(
            onPressed: widget.openThread,
            child: const Text('Open thread'),
          ),
        ],
      ],
    );
  }
}
