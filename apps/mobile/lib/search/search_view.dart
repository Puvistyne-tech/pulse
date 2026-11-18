import 'package:flutter/services.dart';
import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../session/error_notice.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

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
      child: Column(
        key: const Key('searchView'),
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          TextField(
            key: const Key('searchQuery'),
            autofocus: true,
            controller: _query,
            decoration: const InputDecoration(labelText: 'Search messages'),
            onChanged: intents.search,
          ),
          if (search.searching) const Text('Searching'),
          ErrorNotice(error: search.error),
          for (final (index, hit) in search.hits.indexed)
            Container(
              key: ValueKey(hit.messageId),
              color: index == search.selectedIndex
                  ? const Color(0x11000000)
                  : null,
              child: InkWell(
                onTap: () => intents.openSearchHit(hit.messageId),
                child: Padding(
                  padding: const EdgeInsets.all(8),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        [
                          if (hit.topicTitle != null) hit.topicTitle!,
                          hit.authorDisplayName,
                        ].join(' '),
                      ),
                      Text.rich(
                        TextSpan(
                          children: [
                            for (final span in hit.snippet)
                              TextSpan(
                                text: span.text,
                                style: span.matched
                                    ? const TextStyle(
                                        fontWeight: FontWeight.bold,
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
          TextButton(
            onPressed: intents.toggleSearch,
            child: const Text('Close search'),
          ),
        ],
      ),
    );
  }
}
