import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/media.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';

class AttachmentList extends StatelessWidget {
  const AttachmentList({required this.attachments, super.key});

  final List<AttachmentView> attachments;

  @override
  Widget build(BuildContext context) {
    if (attachments.isEmpty) {
      return const SizedBox.shrink();
    }
    final tokens = PulseTokens.of(context);
    return Padding(
      padding: const EdgeInsets.only(top: PulseSpacing.sm),
      child: Column(
        key: const Key('attachmentList'),
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (final attachment in attachments)
            attachment.url != null && attachment.kind == AttachmentKind.image
                ? ClipRRect(
                    borderRadius: BorderRadius.circular(PulseRadius.control),
                    child: Image.network(
                      attachment.url!,
                      key: ValueKey(attachment.attachmentId),
                      height: 160,
                      semanticLabel: attachment.fileName,
                    ),
                  )
                : Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: PulseSpacing.md,
                      vertical: PulseSpacing.sm,
                    ),
                    decoration: BoxDecoration(
                      color: tokens.surfaceMuted,
                      borderRadius: BorderRadius.circular(PulseRadius.control),
                      border: Border.all(color: tokens.border),
                    ),
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Icon(
                          Icons.insert_drive_file_outlined,
                          size: 16,
                          color: tokens.textMuted,
                        ),
                        const SizedBox(width: PulseSpacing.sm),
                        Text(
                          attachment.fileName,
                          key: ValueKey(attachment.attachmentId),
                        ),
                      ],
                    ),
                  ),
        ],
      ),
    );
  }
}
