import 'package:flutter_test/flutter_test.dart';
import 'package:pulse_mobile/lounge/lounge_strip.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/lounge.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

import 'fixtures.dart';

class StripCase {
  const StripCase(this.name, this.lounges, this.present, this.absent);

  final String name;
  final LoungeSnapshot lounges;
  final List<String> present;
  final List<String> absent;
}

final twoLounges = loungesOf([
  loungeEntryOf(loungeAId, LoungeScope.campus),
  loungeEntryOf(loungeBId, LoungeScope.room, roomSlug: 'courtyard'),
]);

final stripCases = <StripCase>[
  StripCase('no lounges', emptyLounges, [], ['Lounges', 'Campus lounge']),
  StripCase(
    'an empty campus lounge',
    loungesOf([loungeEntryOf(loungeAId, LoungeScope.campus)]),
    ['Lounges', 'Campus lounge', 'Nobody here'],
    ['Join', 'Leave'],
  ),
  StripCase(
    'a room lounge with participants',
    loungesOf([
      loungeEntryOf(
        loungeAId,
        LoungeScope.room,
        roomSlug: 'commons',
        participants: [
          participantOf('Grace'),
          participantOf('Ada', fromSelf: true),
        ],
      ),
    ]),
    ['#commons lounge', 'Grace, You'],
    ['Nobody here', 'Join', 'Leave'],
  ),
  StripCase('two lounges', twoLounges, [
    'Campus lounge',
    '#courtyard lounge',
  ], []),
];

void main() {
  group('LoungeStrip', () {
    for (final fixture in stripCases) {
      testWidgets(fixture.name, (tester) async {
        await pumpView(tester, LoungeStrip(lounges: fixture.lounges));
        for (final text in fixture.present) {
          expect(find.text(text), findsWidgets);
        }
        for (final text in fixture.absent) {
          expect(find.text(text), findsNothing);
        }
      });
    }

    testWidgets('lounges render in snapshot order', (tester) async {
      await pumpView(tester, LoungeStrip(lounges: twoLounges));
      expect(
        tester.getTopLeft(find.text('Campus lounge')).dy,
        lessThan(tester.getTopLeft(find.text('#courtyard lounge')).dy),
      );
    });
  });
}
