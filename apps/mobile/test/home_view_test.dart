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
  });
}
