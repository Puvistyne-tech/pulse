import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/inbox/inbox_shortcuts.dart';
import 'package:pulse_mobile/inbox/inbox_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/thread.dart';

import 'fixtures.dart';

final firstItem = inboxItemOf(messageBId, 'second root');
final secondItem = inboxItemOf(messageAId, 'first root');

Future<RecordedIntents> pumpInbox(
  WidgetTester tester,
  InboxSnapshot inbox,
) async {
  final recorded = RecordedIntents();
  await pumpView(
    tester,
    SingleChildScrollView(
      child: InboxView(inbox: inbox, intents: recorded.intents),
    ),
  );
  return recorded;
}

class BadgeCase {
  const BadgeCase(this.name, this.replies, this.mentions);

  final String name;
  final int replies;
  final int mentions;
}

const badgeCases = <BadgeCase>[
  BadgeCase('nothing unread', 0, 0),
  BadgeCase('replies only', 4, 0),
  BadgeCase('mentions only', 0, 2),
  BadgeCase('replies and mentions', 3, 1),
];

class KeyCase {
  const KeyCase(this.name, this.key, this.replying, this.expected);

  final String name;
  final LogicalKeyboardKey key;
  final bool replying;
  final List<Object> expected;
}

final keyCases = <KeyCase>[
  KeyCase('j selects the next thread', LogicalKeyboardKey.keyJ, false, [
    ('moveInboxSelection', InboxStep.next),
  ]),
  KeyCase('k selects the previous thread', LogicalKeyboardKey.keyK, false, [
    ('moveInboxSelection', InboxStep.previous),
  ]),
  KeyCase('e archives the selection', LogicalKeyboardKey.keyE, false, [
    'archiveInboxSelection',
  ]),
  KeyCase('r starts a reply', LogicalKeyboardKey.keyR, false, [
    'startInboxReply',
  ]),
  KeyCase(
    'escape does nothing before replying',
    LogicalKeyboardKey.escape,
    false,
    [],
  ),
  KeyCase('escape cancels a reply', LogicalKeyboardKey.escape, true, [
    'cancelInboxReply',
  ]),
  KeyCase('j is ignored while replying', LogicalKeyboardKey.keyJ, true, []),
  KeyCase('e is ignored while replying', LogicalKeyboardKey.keyE, true, []),
  KeyCase('r is ignored while replying', LogicalKeyboardKey.keyR, true, []),
  KeyCase('an unmapped key is ignored', LogicalKeyboardKey.keyX, false, []),
];

class BindingCase {
  const BindingCase(this.name, this.replying, this.keys);

  final String name;
  final bool replying;
  final Set<LogicalKeyboardKey> keys;
}

final bindingCases = <BindingCase>[
  BindingCase('browsing maps j k e r', false, {
    LogicalKeyboardKey.keyJ,
    LogicalKeyboardKey.keyK,
    LogicalKeyboardKey.keyE,
    LogicalKeyboardKey.keyR,
  }),
  BindingCase('replying maps only escape', true, {LogicalKeyboardKey.escape}),
];

void main() {
  group('rows', () {
    testWidgets('render in snapshot order', (tester) async {
      await pumpInbox(tester, inboxOf([firstItem, secondItem]));
      final tops = [
        'second root',
        'first root',
      ].map((text) => tester.getTopLeft(find.text(text)).dy).toList();
      expect(tops, orderedEquals([...tops]..sort()));
    });

    for (final (name, title, heading) in [
      ('topic title', 'Plans', 'Plans'),
      ('root preview when untitled', null, 'untitled root'),
    ]) {
      testWidgets('heading uses the $name', (tester) async {
        await pumpInbox(
          tester,
          inboxOf([
            inboxItemOf(messageAId, 'untitled root', topicTitle: title),
          ]),
        );
        expect(find.text(heading), findsOneWidget);
      });
    }

    testWidgets('shows the root author and latest reply', (tester) async {
      await pumpInbox(
        tester,
        inboxOf([
          inboxItemOf(
            messageAId,
            'first root',
            latestReply: ReplyPreview(
              messageId: messageBId,
              authorId: graceId,
              authorDisplayName: 'Linus',
              preview: 'see you there',
            ),
          ),
        ]),
      );
      expect(find.text('Grace: first root'), findsOneWidget);
      expect(find.text('Linus: see you there'), findsOneWidget);
    });

    for (final fixture in badgeCases) {
      testWidgets('${fixture.name} shows badges accordingly', (tester) async {
        await pumpInbox(
          tester,
          inboxOf([
            inboxItemOf(
              messageAId,
              'first root',
              unreadReplies: fixture.replies,
              unreadMentions: fixture.mentions,
            ),
          ]),
        );
        expect(
          find.text('${fixture.replies}'),
          fixture.replies > 0 ? findsOneWidget : findsNothing,
        );
        expect(
          find.text('@'),
          fixture.mentions > 0 ? findsOneWidget : findsNothing,
        );
      });
    }

    for (final state in SubscriptionState.values) {
      testWidgets('shows the ${state.name} state', (tester) async {
        await pumpInbox(
          tester,
          inboxOf([inboxItemOf(messageAId, 'first root', subscription: state)]),
        );
        expect(find.text(state.name), findsOneWidget);
      });
    }
  });

  group('header', () {
    for (final count in [0, 1, 7]) {
      testWidgets('shows $count unread threads from the snapshot', (
        tester,
      ) async {
        await pumpInbox(tester, inboxOf([firstItem], unreadThreads: count));
        expect(find.text('Inbox $count'), findsOneWidget);
      });
    }

    testWidgets('close button toggles the inbox', (tester) async {
      final recorded = await pumpInbox(tester, inboxOf([]));
      await tester.tap(tooltip('Close inbox'));
      expect(recorded.calls, ['toggleInbox']);
    });

    testWidgets('an empty inbox renders no rows', (tester) async {
      await pumpInbox(tester, inboxOf([]));
      expect(find.text('Archive'), findsNothing);
    });
  });

  group('selection', () {
    testWidgets('only the selected row offers archive and reply', (
      tester,
    ) async {
      await pumpInbox(
        tester,
        inboxOf([firstItem, secondItem], selected: messageAId),
      );
      expect(find.text('Archive'), findsOneWidget);
      expect(find.text('Reply'), findsOneWidget);
    });

    testWidgets('without a selection no row offers actions', (tester) async {
      await pumpInbox(
        tester,
        inboxOf([firstItem, secondItem], selectFirst: false),
      );
      expect(find.text('Archive'), findsNothing);
      expect(find.text('Reply'), findsNothing);
    });

    testWidgets('archive and reply buttons call their intents', (tester) async {
      final recorded = await pumpInbox(tester, inboxOf([firstItem]));
      await tester.tap(find.text('Archive'));
      await tester.tap(find.text('Reply'));
      expect(recorded.calls, ['archiveInboxSelection', 'startInboxReply']);
    });
  });

  group('inline reply', () {
    for (final (name, replying, selectFirst, composers) in [
      ('not replying', false, true, 0),
      ('replying with a selection', true, true, 1),
      ('replying without a selection', true, false, 0),
    ]) {
      testWidgets('$name shows $composers composers', (tester) async {
        await pumpInbox(
          tester,
          inboxOf(
            [firstItem, secondItem],
            replying: replying,
            selectFirst: selectFirst,
          ),
        );
        expect(find.byKey(const Key('messageDraft')), findsNWidgets(composers));
      });
    }

    testWidgets('sends the reply to the selected thread', (tester) async {
      final recorded = await pumpInbox(
        tester,
        inboxOf([firstItem, secondItem], replying: true, selected: messageAId),
      );
      await tester.enterText(find.byKey(const Key('messageDraft')), 'on it');
      await tester.pump();
      await tester.tap(find.byKey(const Key('sendMessage')));
      expect(recorded.calls, [('sendReply', roomAId, messageAId, 'on it')]);
    });

    testWidgets('the composer is labelled as an inbox reply', (tester) async {
      await pumpInbox(tester, inboxOf([firstItem], replying: true));
      expect(find.text('Inbox reply'), findsOneWidget);
    });
  });

  group('shortcuts', () {
    for (final fixture in bindingCases) {
      test(fixture.name, () {
        final bindings = inboxBindings(
          inboxOf([firstItem], replying: fixture.replying),
          RecordedIntents().intents,
        );
        final keys = bindings.keys
            .map((activator) => (activator as SingleActivator).trigger)
            .toSet();
        expect(keys, fixture.keys);
      });
    }

    for (final fixture in keyCases) {
      testWidgets(fixture.name, (tester) async {
        final recorded = await pumpInbox(
          tester,
          inboxOf([firstItem, secondItem], replying: fixture.replying),
        );
        await tester.pump();
        await tester.sendKeyEvent(fixture.key);
        await tester.pump();
        expect(recorded.calls, fixture.expected);
      });
    }
  });
}
