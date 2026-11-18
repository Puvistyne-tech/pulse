import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/snapshot.dart';

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
    return Row(
      key: const Key('latestReplyBar'),
      children: [
        TextButton(
          onPressed: () => setState(() => _collapsed = !_collapsed),
          child: const Text('Latest reply'),
        ),
        if (!_collapsed) ...[
          Text(latest.authorDisplayName),
          const SizedBox(width: 8),
          Flexible(
            child: Text(latest.preview, overflow: TextOverflow.ellipsis),
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
