import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/home/home_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

import 'fixtures.dart';

class HomeCase {
  const HomeCase(this.name, this.snapshot, this.present, this.absent);

  final String name;
  final ClientSnapshot snapshot;
  final List<String> present;
  final List<String> absent;
}

final homeCases = <HomeCase>[
  HomeCase('signed out', signedOut, ['Sign in'], ['Sign out', 'Create campus']),
  HomeCase(
    'signed out with error',
    signedOutWithError,
    ['Enter a valid email address'],
    ['Sign out'],
  ),
  HomeCase(
    'signed in without campus',
    signedInNoCampus,
    ['Ada', 'Sign out', 'Create campus'],
    ['#commons', '#courtyard'],
  ),
  HomeCase('signed in with rooms', signedInWithRooms, [
    'Ada',
    'Alpha Campus',
    'Beta Campus',
    '#commons',
    '#courtyard',
  ], []),
  HomeCase(
    'signed in with messages',
    signedInWithMessages,
    ['Grace', 'welcome', 'You', 'hello', 'Sending', 'Syncing', 'Send'],
    ['Discard'],
  ),
];

class PanelCase {
  const PanelCase(this.name, this.snapshot, this.key, this.visible);

  final String name;
  final ClientSnapshot snapshot;
  final Key key;
  final bool visible;
}

final openInboxSnapshot = withExtras(
  signedInWithRooms,
  inbox: inboxOf([inboxItemOf(messageAId, 'first root', unreadReplies: 2)]),
);

final panelCases = <PanelCase>[
  PanelCase(
    'closed inbox hides the inbox',
    signedInWithRooms,
    const Key('inboxView'),
    false,
  ),
  PanelCase(
    'open inbox shows the inbox',
    openInboxSnapshot,
    const Key('inboxView'),
    true,
  ),
  PanelCase(
    'no open thread hides the thread panel',
    signedInWithRooms,
    const Key('threadView'),
    false,
  ),
  PanelCase(
    'open thread shows the thread panel',
    withExtras(signedInWithRooms, thread: threadOf()),
    const Key('threadView'),
    true,
  ),
];

class RoomCase {
  const RoomCase(this.label, this.roomId);

  final String label;
  final Object roomId;
}

final roomCases = <RoomCase>[
  RoomCase('#commons', roomAId),
  RoomCase('#courtyard', roomBId),
];

void main() {
  group('HomeView', () {
    for (final fixture in homeCases) {
      testWidgets(fixture.name, (tester) async {
        await pumpView(
          tester,
          HomeView(
            snapshot: fixture.snapshot,
            intents: RecordedIntents().intents,
          ),
        );
        for (final text in fixture.present) {
          expect(find.text(text), findsWidgets);
        }
        for (final text in fixture.absent) {
          expect(find.text(text), findsNothing);
        }
      });
    }

    testWidgets('authenticating shows the auth form with submit disabled', (
      tester,
    ) async {
      await pumpView(
        tester,
        HomeView(snapshot: authenticating, intents: RecordedIntents().intents),
      );
      expect(find.text('Sign in'), findsWidgets);
      final submit = tester.widget<ElevatedButton>(
        find.byKey(const Key('submit')),
      );
      expect(submit.onPressed, isNull);
    });

    testWidgets('rooms render in snapshot order', (tester) async {
      await pumpView(
        tester,
        HomeView(
          snapshot: signedInWithRooms,
          intents: RecordedIntents().intents,
        ),
      );
      final commons = tester.getTopLeft(find.text('#commons'));
      final courtyard = tester.getTopLeft(find.text('#courtyard'));
      expect(commons.dy, lessThan(courtyard.dy));
    });

    testWidgets('selecting a campus forwards its campus id', (tester) async {
      final recorded = RecordedIntents();
      await pumpView(
        tester,
        HomeView(snapshot: signedInWithRooms, intents: recorded.intents),
      );
      await tester.tap(find.text('Beta Campus'));
      expect(recorded.calls, [campusBId]);
    });

    for (final fixture in roomCases) {
      testWidgets('selecting ${fixture.label} forwards its room id', (
        tester,
      ) async {
        final recorded = RecordedIntents();
        await pumpView(
          tester,
          HomeView(snapshot: signedInWithRooms, intents: recorded.intents),
        );
        await tester.tap(find.text(fixture.label));
        expect(recorded.calls, [fixture.roomId]);
      });
    }

    for (final (label, bold) in [('#commons', true), ('#courtyard', false)]) {
      testWidgets('active room marking for $label', (tester) async {
        await pumpView(
          tester,
          HomeView(
            snapshot: signedInWithRooms,
            intents: RecordedIntents().intents,
          ),
        );
        final text = tester.widget<Text>(find.text(label));
        expect(text.style?.fontWeight == FontWeight.bold, bold);
      });
    }

    for (final fixture in panelCases) {
      testWidgets(fixture.name, (tester) async {
        await pumpView(
          tester,
          HomeView(
            snapshot: fixture.snapshot,
            intents: RecordedIntents().intents,
          ),
        );
        expect(
          find.byKey(fixture.key),
          fixture.visible ? findsOneWidget : findsNothing,
        );
      });
    }

    for (final (name, unread) in [
      ('no unread threads', 0),
      ('three unread', 3),
    ]) {
      testWidgets('the inbox button shows $name', (tester) async {
        await pumpView(
          tester,
          HomeView(
            snapshot: withExtras(
              signedInWithRooms,
              inbox: inboxOf([], open: false, unreadThreads: unread),
            ),
            intents: RecordedIntents().intents,
          ),
        );
        expect(find.text('Inbox $unread'), findsOneWidget);
      });
    }

    testWidgets('the inbox button toggles the inbox', (tester) async {
      final recorded = RecordedIntents();
      await pumpView(
        tester,
        HomeView(snapshot: signedInWithRooms, intents: recorded.intents),
      );
      await tester.tap(find.text('Inbox 0'));
      expect(recorded.calls, ['toggleInbox']);
    });

    testWidgets('sending forwards the active room id and draft', (
      tester,
    ) async {
      final recorded = RecordedIntents();
      await pumpView(
        tester,
        HomeView(snapshot: signedInWithRooms, intents: recorded.intents),
      );
      await tester.enterText(find.byKey(const Key('messageDraft')), 'hi');
      await tester.pump();
      await tester.tap(find.byKey(const Key('sendMessage')));
      expect(recorded.calls, [(roomAId, 'hi')]);
    });
  });
}
