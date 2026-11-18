import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/media.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

class AttachmentList extends StatelessWidget {
  const AttachmentList({required this.attachments, super.key});

  final List<AttachmentView> attachments;

  @override
  Widget build(BuildContext context) {
    if (attachments.isEmpty) {
      return const SizedBox.shrink();
    }
    return Column(
      key: const Key('attachmentList'),
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        for (final attachment in attachments)
          attachment.url != null && attachment.kind == AttachmentKind.image
              ? Image.network(
                  attachment.url!,
                  key: ValueKey(attachment.attachmentId),
                  height: 160,
                  semanticLabel: attachment.fileName,
                )
              : Text(
                  attachment.fileName,
                  key: ValueKey(attachment.attachmentId),
                ),
      ],
    );
  }
}
