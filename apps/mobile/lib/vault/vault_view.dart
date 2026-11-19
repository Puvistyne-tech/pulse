import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../room/attachment_list.dart';
import '../session/error_notice.dart';
import '../shell/panel_card.dart';
import '../theme/pulse_theme.dart';
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
    final tokens = PulseTokens.of(context);
    return PanelCard(
      cardKey: const Key('vaultView'),
      title: 'Vault',
      closeLabel: 'Close vault',
      onClose: intents.closeVault,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Wrap(
            spacing: PulseSpacing.xs,
            children: [
              for (final entry in _tabLabels.entries)
                TextButton(
                  key: ValueKey(entry.key),
                  style: entry.key == vault.tab
                      ? TextButton.styleFrom(backgroundColor: tokens.accentSoft)
                      : TextButton.styleFrom(foregroundColor: tokens.textMuted),
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
          if (vault.loading)
            Text('Loading', style: TextStyle(color: tokens.textMuted)),
          for (final (index, entry) in vault.entries.indexed)
            Container(
              key: ValueKey('${entry.messageId}-$index'),
              margin: const EdgeInsets.only(top: PulseSpacing.sm),
              padding: const EdgeInsets.all(PulseSpacing.md),
              decoration: BoxDecoration(
                color: tokens.surfaceMuted,
                borderRadius: BorderRadius.circular(PulseRadius.control),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    entry.authorDisplayName,
                    style: const TextStyle(fontWeight: FontWeight.bold),
                  ),
                  if (entry.attachment != null)
                    AttachmentList(attachments: [entry.attachment!]),
                  if (entry.link != null)
                    Text(entry.link!, style: TextStyle(color: tokens.accent)),
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
      ),
    );
  }
}
