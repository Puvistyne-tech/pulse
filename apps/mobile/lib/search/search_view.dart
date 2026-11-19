import 'package:flutter/services.dart';
import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../session/error_notice.dart';
import '../shell/panel_card.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';

class SearchView extends StatefulWidget {
  const SearchView({required this.search, required this.intents, super.key});

  final SearchSnapshot search;
  final PulseIntents intents;

  @override
  State<SearchView> createState() => _SearchViewState();
}

class _SearchViewState extends State<SearchView> {
  late final TextEditingController _query = TextEditingController(
    text: widget.search.query,
  );

  @override
  void dispose() {
    _query.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final search = widget.search;
    final intents = widget.intents;
    return CallbackShortcuts(
      bindings: {
        const SingleActivator(LogicalKeyboardKey.arrowDown): () =>
            intents.moveSearchSelection(InboxStep.next),
        const SingleActivator(LogicalKeyboardKey.arrowUp): () =>
            intents.moveSearchSelection(InboxStep.previous),
        const SingleActivator(LogicalKeyboardKey.enter):
            intents.openSearchSelection,
        const SingleActivator(LogicalKeyboardKey.escape): intents.toggleSearch,
      },
      child: PanelCard(
        cardKey: const Key('searchView'),
        title: 'Search',
        closeLabel: 'Close search',
        onClose: intents.toggleSearch,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            TextField(
              key: const Key('searchQuery'),
              autofocus: true,
              controller: _query,
              decoration: const InputDecoration(
                labelText: 'Search messages',
                prefixIcon: Icon(Icons.search),
              ),
              onChanged: intents.search,
            ),
            if (search.searching)
              Padding(
                padding: const EdgeInsets.only(top: PulseSpacing.sm),
                child: Text(
                  'Searching',
                  style: TextStyle(color: tokens.textMuted),
                ),
              ),
            ErrorNotice(error: search.error),
            const SizedBox(height: PulseSpacing.sm),
            for (final (index, hit) in search.hits.indexed)
              Container(
                key: ValueKey(hit.messageId),
                margin: const EdgeInsets.only(bottom: PulseSpacing.xs),
                decoration: BoxDecoration(
                  color: index == search.selectedIndex
                      ? tokens.accentSoft
                      : null,
                  borderRadius: BorderRadius.circular(PulseRadius.control),
                ),
                child: InkWell(
                  borderRadius: BorderRadius.circular(PulseRadius.control),
                  onTap: () => intents.openSearchHit(hit.messageId),
                  child: Padding(
                    padding: const EdgeInsets.all(PulseSpacing.sm),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          [
                            if (hit.topicTitle != null) hit.topicTitle!,
                            hit.authorDisplayName,
                          ].join(' '),
                          style: TextStyle(
                            color: tokens.textMuted,
                            fontSize: 12,
                          ),
                        ),
                        Text.rich(
                          TextSpan(
                            children: [
                              for (final span in hit.snippet)
                                TextSpan(
                                  text: span.text,
                                  style: span.matched
                                      ? TextStyle(
                                          fontWeight: FontWeight.bold,
                                          color: tokens.accent,
                                        )
                                      : null,
                                ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}
