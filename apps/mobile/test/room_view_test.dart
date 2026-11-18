import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/room/room_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/messaging.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/reaction.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

import 'fixtures.dart';

class EntryCase {
  const EntryCase(
    this.name,
    this.entry,
    this.author,
    this.status,
    this.discardable,
  );

  final String name;
  final TimelineEntry entry;
  final String author;
  final String? status;
  final bool discardable;
}

final entryCases = <EntryCase>[
  EntryCase('pending', pendingEntry, 'You', 'Sending', false),
  EntryCase('confirmed from a peer', confirmedEntry, 'Grace', null, false),
  EntryCase('confirmed from self', confirmedOwnEntry, 'You', null, false),
  EntryCase(
    'failed forbidden',
    failedEntry(MessageRejection.forbidden),
    'You',
    'Not allowed to post here',
    true,
  ),
  EntryCase(
    'failed invalid body',
    failedEntry(MessageRejection.invalidBody),
    'You',
    'Message not accepted',
    true,
  ),
  EntryCase(
    'failed unknown room',
    failedEntry(MessageRejection.unknownRoom),
    'You',
    'Room not found',
    true,
  ),
  EntryCase(
    'failed unknown thread',
    failedEntry(MessageRejection.unknownThread),
    'You',
    'Thread not found',
    true,
  ),
];

class ReactCase {
  const ReactCase(this.name, this.key, this.emoji, this.action);

  final String name;
  final Key key;
  final String emoji;
  final ReactionAction action;
}

const reactCases = <ReactCase>[
  ReactCase(
    'chip reacted by self removes',
    ValueKey('reaction-👍'),
    '👍',
    ReactionAction.remove,
  ),
  ReactCase(
    'chip reacted by others adds',
    ValueKey('reaction-🎉'),
    '🎉',
    ReactionAction.add,
  ),
  ReactCase(
    'palette emoji reacted by self removes',
    ValueKey('palette-👍'),
    '👍',
    ReactionAction.remove,
  ),
  ReactCase(
    'palette emoji reacted by others adds',
    ValueKey('palette-🎉'),
    '🎉',
    ReactionAction.add,
  ),
  ReactCase(
    'palette emoji not yet used adds',
    ValueKey('palette-🙏'),
    '🙏',
    ReactionAction.add,
  ),
];

class BarCase {
  const BarCase(this.name, this.preview, this.visible);

  final String name;
  final ThreadPreview preview;
  final bool visible;
}

final barCases = <BarCase>[
  BarCase('shown when buried', threadPreviewOf(), true),
  BarCase(
    'hidden when not buried',
    threadPreviewOf(showLatestReply: false),
    false,
  ),
  BarCase(
    'hidden without a latest reply',
    threadPreviewOf(hasLatestReply: false),
    false,
  ),
];

Future<RecordedIntents> pumpRoom(
  WidgetTester tester,
  TimelineSnapshot timeline,
) async {
  final recorded = RecordedIntents();
  await pumpView(
    tester,
    SingleChildScrollView(
      child: RoomView(timeline: timeline, intents: recorded.intents),
    ),
  );
  return recorded;
}

String draftText(WidgetTester tester) => tester
    .widget<TextField>(find.byKey(const Key('messageDraft')))
    .controller!
    .text;

void main() {
  group('timeline entries', () {
    for (final fixture in entryCases) {
      testWidgets('${fixture.name} renders author, body and status', (
        tester,
      ) async {
        await pumpRoom(tester, timelineOf([fixture.entry]));
        expect(find.text(fixture.author), findsOneWidget);
        expect(find.text(fixture.entry.body), findsOneWidget);
        expect(
          find.text('Sending'),
          findsNWidgets(fixture.status == 'Sending' ? 1 : 0),
        );
        if (fixture.status != null) {
          expect(find.text(fixture.status!), findsOneWidget);
        }
        expect(
          find.text('Discard'),
          fixture.discardable ? findsOneWidget : findsNothing,
        );
      });
    }

    testWidgets('discarding reports the failed nonce', (tester) async {
      final recorded = await pumpRoom(
        tester,
        timelineOf([failedEntry(MessageRejection.forbidden)]),
      );
      await tester.tap(find.text('Discard'));
      expect(recorded.calls, [nonceC]);
    });

    testWidgets('entries render in snapshot order', (tester) async {
      await pumpRoom(
        tester,
        timelineOf([confirmedEntry, confirmedOwnEntry, pendingEntry]),
      );
      final tops = [
        'welcome',
        'sent',
        'hello',
      ].map((body) => tester.getTopLeft(find.text(body)).dy).toList();
      expect(tops, orderedEquals([...tops]..sort()));
    });
  });

  group('body segments', () {
    testWidgets('mentions render emphasised and plain text does not', (
      tester,
    ) async {
      await pumpRoom(tester, timelineOf([mentionEntry]));
      expect(find.textContaining('@Ada', findRichText: true), findsOneWidget);
      final rich = tester.widget<RichText>(
        find.descendant(
          of: find.byType(Flexible),
          matching: find.byType(RichText),
        ),
      );
      final spans = <TextSpan>[];
      rich.text.visitChildren((span) {
        if (span is TextSpan && span.text != null) {
          spans.add(span);
        }
        return true;
      });
      expect(spans.map((span) => span.text), ['hi ', '@Ada']);
      expect(spans.map((span) => span.style?.fontWeight == FontWeight.bold), [
        false,
        true,
      ]);
    });

    testWidgets('the raw mention token is never shown', (tester) async {
      await pumpRoom(tester, timelineOf([mentionEntry]));
      expect(find.textContaining('<@', findRichText: true), findsNothing);
    });
  });

  group('reactions', () {
    testWidgets('chips show emoji and counts', (tester) async {
      await pumpRoom(tester, timelineOf([reactedEntry]));
      expect(find.text('👍 2'), findsOneWidget);
      expect(find.text('🎉 1'), findsOneWidget);
      final mine = tester.widget<FilterChip>(
        find.byKey(const ValueKey('reaction-👍')),
      );
      final theirs = tester.widget<FilterChip>(
        find.byKey(const ValueKey('reaction-🎉')),
      );
      expect(mine.selected, isTrue);
      expect(theirs.selected, isFalse);
    });

    for (final fixture in reactCases) {
      testWidgets(fixture.name, (tester) async {
        final recorded = await pumpRoom(tester, timelineOf([reactedEntry]));
        await tester.tap(find.byKey(fixture.key));
        expect(recorded.calls, [
          ('react', roomAId, messageAId, fixture.emoji, fixture.action),
        ]);
      });
    }

    for (final (name, palette) in [
      ('the protocol palette', reactionPalette),
      ('a custom palette', ['🙏']),
      ('an empty palette', <String>[]),
    ]) {
      testWidgets('palette renders $name from the snapshot', (tester) async {
        await pumpRoom(tester, timelineOf([confirmedEntry], palette: palette));
        final keys = tester
            .widgetList<TextButton>(find.byType(TextButton))
            .map((button) => button.key)
            .whereType<ValueKey<String>>()
            .where((key) => key.value.startsWith('palette-'))
            .map((key) => key.value.substring('palette-'.length))
            .toList();
        expect(keys, palette);
      });
    }

    testWidgets('pending entries offer no reactions', (tester) async {
      await pumpRoom(tester, timelineOf([pendingEntry]));
      expect(find.byKey(const ValueKey('palette-👍')), findsNothing);
    });
  });

  group('thread preview', () {
    for (final fixture in barCases) {
      testWidgets('latest reply bar is ${fixture.name}', (tester) async {
        await pumpRoom(tester, timelineOf([threadedEntry(fixture.preview)]));
        expect(
          find.byKey(const Key('latestReplyBar')),
          fixture.visible ? findsOneWidget : findsNothing,
        );
        expect(
          find.text('latest words'),
          fixture.visible ? findsOneWidget : findsNothing,
        );
      });
    }

    testWidgets('collapsing hides the reply and expanding restores it', (
      tester,
    ) async {
      await pumpRoom(tester, timelineOf([threadedEntry()]));
      expect(find.text('latest words'), findsOneWidget);
      await tester.tap(find.text('Latest reply'));
      await tester.pump();
      expect(find.text('latest words'), findsNothing);
      await tester.tap(find.text('Latest reply'));
      await tester.pump();
      expect(find.text('latest words'), findsOneWidget);
    });

    for (final label in ['Open thread', 'Reply in thread']) {
      testWidgets('$label opens the thread of the entry', (tester) async {
        final recorded = await pumpRoom(tester, timelineOf([threadedEntry()]));
        await tester.tap(find.text(label));
        expect(recorded.calls, [('openThread', roomAId, messageAId)]);
      });
    }

    for (final (name, preview, texts) in [
      ('counts and unread', threadPreviewOf(), ['Replies 3', 'following', '2']),
      (
        'topic title',
        threadPreviewOf(topicTitle: 'Plans', unreadReplies: 0),
        ['Plans', 'Replies 3'],
      ),
    ]) {
      testWidgets('summary shows $name', (tester) async {
        await pumpRoom(tester, timelineOf([threadedEntry(preview)]));
        for (final text in texts) {
          expect(find.text(text), findsWidgets);
        }
      });
    }

    testWidgets('entries without a thread show no summary', (tester) async {
      await pumpRoom(tester, timelineOf([confirmedEntry]));
      expect(find.textContaining('Replies'), findsNothing);
    });
  });

  group('mentions', () {
    testWidgets('mention appends the author token to the draft', (
      tester,
    ) async {
      await pumpRoom(tester, timelineOf([confirmedEntry, mentionEntry]));
      final buttons = find.text('Mention');
      await tester.tap(buttons.first);
      await tester.pump();
      expect(draftText(tester), '${mentionToken(graceId)} ');
      await tester.enterText(find.byKey(const Key('messageDraft')), 'hey');
      await tester.tap(buttons.last);
      await tester.pump();
      expect(draftText(tester), 'hey ${mentionToken(graceId)} ');
    });

    testWidgets('own entries have no mention button', (tester) async {
      await pumpRoom(tester, timelineOf([confirmedOwnEntry, pendingEntry]));
      expect(find.text('Mention'), findsNothing);
    });
  });

  group('syncing marker', () {
    for (final syncing in [true, false]) {
      testWidgets('syncing $syncing', (tester) async {
        await pumpRoom(tester, timelineOf([], syncing: syncing));
        expect(find.text('Syncing'), syncing ? findsOneWidget : findsNothing);
      });
    }
  });

  group('composer', () {
    for (final draft in ['hello', '  padded  ', '   ']) {
      testWidgets('sends "$draft" exactly as typed and clears', (tester) async {
        final recorded = await pumpRoom(tester, timelineOf([]));
        await tester.enterText(find.byKey(const Key('messageDraft')), draft);
        await tester.pump();
        await tester.tap(find.byKey(const Key('sendMessage')));
        await tester.pump();
        expect(recorded.calls, [(roomAId, draft)]);
        expect(draftText(tester), '');
      });
    }

    for (final (name, draft, disabled) in [
      ('empty draft', '', true),
      ('typed draft', 'x', false),
      ('whitespace draft', ' ', false),
    ]) {
      testWidgets('$name disables send $disabled', (tester) async {
        await pumpRoom(tester, timelineOf([]));
        await tester.enterText(find.byKey(const Key('messageDraft')), draft);
        await tester.pump();
        final button = tester.widget<ElevatedButton>(
          find.byKey(const Key('sendMessage')),
        );
        expect(button.onPressed == null, disabled);
      });
    }
  });

  group('room view shell', () {
    testWidgets('renders nothing without an active room', (tester) async {
      await pumpRoom(tester, emptyTimeline);
      expect(find.byKey(const Key('messageDraft')), findsNothing);
      expect(find.text('Send'), findsNothing);
    });

    for (final (name, error, notice) in [
      ('no error', null, null),
      (
        'message body error',
        const ApiError(
          code: ApiErrorCode.invalidInput,
          field: InputField.messageBody,
        ),
        'Message is empty or too long',
      ),
      (
        'reaction emoji error',
        const ApiError(
          code: ApiErrorCode.invalidInput,
          field: InputField.reactionEmoji,
        ),
        'Choose a single emoji reaction',
      ),
    ]) {
      testWidgets('timeline $name', (tester) async {
        await pumpRoom(tester, timelineOf([], error: error));
        if (notice == null) {
          expect(find.text('Message is empty or too long'), findsNothing);
        } else {
          expect(find.text(notice), findsOneWidget);
        }
      });
    }
  });
}
