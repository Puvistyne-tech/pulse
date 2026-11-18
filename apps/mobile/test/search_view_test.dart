import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/search/search_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/ids.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

import 'fixtures.dart';

Future<RecordedIntents> pumpSearch(
  WidgetTester tester,
  SearchSnapshot search,
) async {
  final recorded = RecordedIntents();
  await pumpView(
    tester,
    SingleChildScrollView(
      child: SearchView(search: search, intents: recorded.intents),
    ),
  );
  return recorded;
}

final threeHits = searchOf([
  searchHitOf(
    messageAId,
    snippet: snippetOf([('first ', false), ('alpha', true)]),
  ),
  searchHitOf(messageBId, topicTitle: 'Plans', author: 'Linus'),
  searchHitOf(
    messageCId,
    snippet: snippetOf([('alpha', true), (' last', false)]),
  ),
], selectedIndex: 1);

class KeyCase {
  const KeyCase(this.name, this.key, this.expected);

  final String name;
  final LogicalKeyboardKey key;
  final Object expected;
}

const keyCases = <KeyCase>[
  KeyCase('arrow down moves next', LogicalKeyboardKey.arrowDown, (
    'moveSearchSelection',
    InboxStep.next,
  )),
  KeyCase('arrow up moves previous', LogicalKeyboardKey.arrowUp, (
    'moveSearchSelection',
    InboxStep.previous,
  )),
  KeyCase(
    'enter opens the selection',
    LogicalKeyboardKey.enter,
    'openSearchSelection',
  ),
  KeyCase(
    'escape closes the palette',
    LogicalKeyboardKey.escape,
    'toggleSearch',
  ),
];

void main() {
  group('results', () {
    testWidgets('hits render in snapshot order', (tester) async {
      await pumpSearch(tester, threeHits);
      final tops = ['first alpha', 'Plans Linus', 'alpha last']
          .map(
            (text) => tester
                .getTopLeft(find.textContaining(text, findRichText: true))
                .dy,
          )
          .toList();
      expect(tops, orderedEquals([...tops]..sort()));
      expect(tops.toSet(), hasLength(3));
    });

    for (final (name, search, selected) in [
      ('the snapshot selection', threeHits, 1),
      ('the first hit', searchOf(threeHits.hits), 0),
      ('no selection', searchOf(threeHits.hits, selectFirst: false), -1),
    ]) {
      testWidgets('$name is the highlighted row', (tester) async {
        await pumpSearch(tester, search);
        final colors = [
          for (final hit in search.hits)
            tester.widget<Container>(find.byKey(ValueKey(hit.messageId))).color,
        ];
        expect(colors.map((color) => color != null), [
          for (var index = 0; index < 3; index++) index == selected,
        ]);
      });
    }

    testWidgets('matched spans are bold and others are not', (tester) async {
      await pumpSearch(tester, searchOf([threeHits.hits.first]));
      final rich = tester.widget<RichText>(
        find
            .descendant(
              of: find.byKey(ValueKey(messageAId)),
              matching: find.byType(RichText),
            )
            .last,
      );
      final spans = <TextSpan>[];
      rich.text.visitChildren((span) {
        if (span is TextSpan && span.text != null) {
          spans.add(span);
        }
        return true;
      });
      expect(spans.map((span) => span.text), ['first ', 'alpha']);
      expect(spans.map((span) => span.style?.fontWeight == FontWeight.bold), [
        false,
        true,
      ]);
    });

    for (final (name, search, text) in [
      ('a topic title is shown with the author', threeHits, 'Plans Linus'),
      (
        'the author alone without a title',
        searchOf([searchHitOf(messageAId)]),
        'Grace',
      ),
    ]) {
      testWidgets(name, (tester) async {
        await pumpSearch(tester, search);
        expect(find.text(text), findsOneWidget);
      });
    }

    testWidgets('no hits render no rows', (tester) async {
      await pumpSearch(tester, searchOf([]));
      expect(
        find.byWidgetPredicate((widget) => widget.key is ValueKey<MessageId>),
        findsNothing,
      );
    });
  });

  group('status', () {
    for (final (name, searching) in [('searching', true), ('idle', false)]) {
      testWidgets('$name progress marker', (tester) async {
        await pumpSearch(tester, searchOf([], searching: searching));
        expect(
          find.text('Searching'),
          searching ? findsOneWidget : findsNothing,
        );
      });
    }

    for (final (name, error, notice) in [
      ('no error', null, null),
      (
        'unavailable',
        const ApiError(code: ApiErrorCode.unavailable),
        'Pulse server unavailable',
      ),
    ]) {
      testWidgets('error $name', (tester) async {
        await pumpSearch(tester, searchOf([], error: error));
        if (notice == null) {
          expect(find.text('Pulse server unavailable'), findsNothing);
        } else {
          expect(find.text(notice), findsOneWidget);
        }
      });
    }

    testWidgets('the query field starts with the snapshot query', (
      tester,
    ) async {
      await pumpSearch(tester, searchOf([], query: 'seed'));
      final field = tester.widget<TextField>(
        find.byKey(const Key('searchQuery')),
      );
      expect(field.controller!.text, 'seed');
    });
  });

  group('intents', () {
    for (final query in ['a', 'alpha beta']) {
      testWidgets('typing "$query" searches for it', (tester) async {
        final recorded = await pumpSearch(tester, searchOf([]));
        await tester.enterText(find.byKey(const Key('searchQuery')), query);
        expect(recorded.calls, [('search', query)]);
      });
    }

    for (final hit in threeHits.hits) {
      testWidgets(
        'tapping ${hit.authorDisplayName} ${hit.messageId.field0} opens it',
        (tester) async {
          final recorded = await pumpSearch(tester, threeHits);
          await tester.tap(find.byKey(ValueKey(hit.messageId)));
          expect(recorded.calls, [('openSearchHit', hit.messageId)]);
        },
      );
    }

    testWidgets('the close button toggles the palette', (tester) async {
      final recorded = await pumpSearch(tester, threeHits);
      await tester.tap(find.text('Close search'));
      expect(recorded.calls, ['toggleSearch']);
    });

    for (final fixture in keyCases) {
      testWidgets(fixture.name, (tester) async {
        final recorded = await pumpSearch(tester, threeHits);
        await tester.pump();
        await tester.sendKeyEvent(fixture.key);
        await tester.pump();
        expect(recorded.calls, [fixture.expected]);
      });
    }
  });
}
