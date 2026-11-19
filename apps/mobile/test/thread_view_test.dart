import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/room/thread_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/thread.dart';

import 'fixtures.dart';

Future<RecordedIntents> pumpThread(
  WidgetTester tester,
  ThreadSnapshot thread, {
  List<String> palette = reactionPalette,
}) async {
  final recorded = RecordedIntents();
  await pumpView(
    tester,
    SingleChildScrollView(
      child: ThreadView(
        thread: thread,
        palette: palette,
        intents: recorded.intents,
      ),
    ),
  );
  return recorded;
}

String titleText(WidgetTester tester) => tester
    .widget<TextField>(find.byKey(const Key('topicTitleDraft')))
    .controller!
    .text;

const subscriptionLabels = {
  SubscriptionState.following: 'Following',
  SubscriptionState.muted: 'Muted',
  SubscriptionState.archived: 'Archived',
};

void main() {
  group('visibility', () {
    for (final (name, thread) in [
      ('an empty snapshot', emptyThread),
      ('an unanchored thread', threadOf(anchored: false)),
    ]) {
      testWidgets('renders nothing for $name', (tester) async {
        await pumpThread(tester, thread);
        expect(find.byKey(const Key('threadView')), findsNothing);
        expect(tooltip('Close thread'), findsNothing);
      });
    }

    testWidgets('renders the panel for an anchored thread', (tester) async {
      await pumpThread(tester, threadOf());
      expect(find.byKey(const Key('threadView')), findsOneWidget);
    });
  });

  group('entries', () {
    testWidgets('lists the root then replies in snapshot order', (
      tester,
    ) async {
      await pumpThread(tester, threadOf(replies: [replyEntry, pendingEntry]));
      final tops = [
        'welcome',
        'a reply',
        'hello',
      ].map((text) => tester.getTopLeft(find.text(text)).dy).toList();
      expect(tops, orderedEquals([...tops]..sort()));
    });

    testWidgets('renders replies alone while the root is not loaded', (
      tester,
    ) async {
      await pumpThread(tester, threadOf(withRoot: false));
      expect(find.text('welcome'), findsNothing);
      expect(find.text('a reply'), findsOneWidget);
    });

    testWidgets('shows reactions with the supplied palette', (tester) async {
      await pumpThread(
        tester,
        threadOf(root: reactedEntry, replies: []),
        palette: ['🙏'],
      );
      expect(find.text('👍 2'), findsOneWidget);
      expect(find.byKey(const ValueKey('palette-🙏')), findsOneWidget);
      expect(find.byKey(const ValueKey('palette-👍')), findsNothing);
    });

    for (final syncing in [true, false]) {
      testWidgets('syncing $syncing', (tester) async {
        await pumpThread(tester, threadOf(syncing: syncing));
        expect(find.text('Syncing'), syncing ? findsOneWidget : findsNothing);
      });
    }
  });

  group('topic title', () {
    for (final (name, title, value) in [
      ('a stored title', 'Plans', 'Plans'),
      ('no title', null, ''),
    ]) {
      testWidgets('input starts with $name', (tester) async {
        await pumpThread(tester, threadOf(topicTitle: title));
        expect(titleText(tester), value);
      });
    }

    for (final (name, draft) in [
      ('new title', 'Agenda'),
      ('padded title', '  Agenda  '),
      ('cleared title', ''),
    ]) {
      testWidgets('saving a $name sends the draft exactly as typed', (
        tester,
      ) async {
        final recorded = await pumpThread(
          tester,
          threadOf(topicTitle: 'Plans'),
        );
        await tester.enterText(find.byKey(const Key('topicTitleDraft')), draft);
        await tester.tap(find.text('Save title'));
        expect(recorded.calls, [('setTopicTitle', roomAId, messageAId, draft)]);
      });
    }

    testWidgets('a snapshot title replaces the draft', (tester) async {
      final recorded = RecordedIntents();
      Widget build(ThreadSnapshot thread) => SingleChildScrollView(
        child: ThreadView(
          thread: thread,
          palette: reactionPalette,
          intents: recorded.intents,
        ),
      );
      await pumpView(tester, build(threadOf(topicTitle: 'Plans')));
      await tester.enterText(
        find.byKey(const Key('topicTitleDraft')),
        'typing',
      );
      await pumpView(tester, build(threadOf(topicTitle: 'Agenda')));
      expect(titleText(tester), 'Agenda');
    });
  });

  group('subscription', () {
    for (final entry in subscriptionLabels.entries) {
      testWidgets('${entry.value} requests ${entry.key.name}', (tester) async {
        final recorded = await pumpThread(tester, threadOf(subscription: null));
        await tester.tap(
          find.byKey(ValueKey('subscription-${entry.key.name}')),
        );
        expect(recorded.calls, [
          ('setThreadSubscription', roomAId, messageAId, entry.key),
        ]);
      });

      testWidgets('${entry.value} is emphasised only when current', (
        tester,
      ) async {
        await pumpThread(tester, threadOf(subscription: entry.key));
        for (final other in subscriptionLabels.keys) {
          final button = tester.widget<TextButton>(
            find.byKey(ValueKey('subscription-${other.name}')),
          );
          final bold =
              button.style?.textStyle?.resolve({})?.fontWeight ==
              FontWeight.bold;
          expect(bold, other == entry.key);
        }
      });
    }
  });

  group('actions', () {
    testWidgets('close button closes the thread', (tester) async {
      final recorded = await pumpThread(tester, threadOf());
      await tester.tap(tooltip('Close thread'));
      expect(recorded.calls, ['closeThread']);
    });

    for (final (name, draft) in [
      ('plain', 'thanks'),
      ('padded', '  thanks  '),
    ]) {
      testWidgets('reply composer sends a $name draft to the root', (
        tester,
      ) async {
        final recorded = await pumpThread(tester, threadOf());
        await tester.enterText(find.byKey(const Key('messageDraft')), draft);
        await tester.pump();
        await tester.tap(find.byKey(const Key('sendMessage')));
        await tester.pump();
        expect(recorded.calls, [('sendReply', roomAId, messageAId, draft)]);
        final field = tester.widget<TextField>(
          find.byKey(const Key('messageDraft')),
        );
        expect(field.controller?.text, '');
      });
    }

    testWidgets('mention inserts the author token into the reply draft', (
      tester,
    ) async {
      await pumpThread(tester, threadOf(replies: []));
      await tester.tap(tooltip('Mention'));
      await tester.pump();
      final field = tester.widget<TextField>(
        find.byKey(const Key('messageDraft')),
      );
      expect(field.controller?.text, '${mentionToken(graceId)} ');
    });

    testWidgets('replies cannot open nested threads', (tester) async {
      await pumpThread(tester, threadOf(withRoot: false));
      expect(find.text('Reply in thread'), findsNothing);
    });
  });
}
