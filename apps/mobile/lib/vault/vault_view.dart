import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../room/attachment_list.dart';
import '../session/error_notice.dart';
import '../src/rust/third_party/pulse_protocol/media.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

const _tabLabels = {
  VaultTab.media: 'Media',
  VaultTab.files: 'Files',
  VaultTab.links: 'Links',
  VaultTab.pinned: 'Pinned',
};

class VaultView extends StatelessWidget {
  const VaultView({required this.vault, required this.intents, super.key});

  final VaultSnapshot vault;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    final roomId = vault.roomId;
    return Column(
      key: const Key('vaultView'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            const Text('Vault'),
            TextButton(
              onPressed: intents.closeVault,
              child: const Text('Close vault'),
            ),
          ],
        ),
        Row(
          children: [
            for (final entry in _tabLabels.entries)
              TextButton(
                key: ValueKey(entry.key),
                onPressed: roomId == null
                    ? null
                    : () => intents.openVault(roomId, entry.key),
                child: Text(
                  entry.value,
                  style: entry.key == vault.tab
                      ? const TextStyle(fontWeight: FontWeight.bold)
                      : null,
                ),
              ),
          ],
        ),
        ErrorNotice(error: vault.error),
        if (vault.loading) const Text('Loading'),
        for (final (index, entry) in vault.entries.indexed)
          Padding(
            key: ValueKey('${entry.messageId}-$index'),
            padding: const EdgeInsets.symmetric(vertical: 4),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(entry.authorDisplayName),
                if (entry.attachment != null)
                  AttachmentList(attachments: [entry.attachment!]),
                if (entry.link != null) Text(entry.link!),
                if (entry.attachment == null && entry.link == null)
                  Text(entry.preview),
              ],
            ),
          ),
        if (vault.hasMore)
          TextButton(
            onPressed: vault.loading ? null : intents.loadMoreVault,
            child: const Text('Load more'),
          ),
      ],
    );
  }
}
