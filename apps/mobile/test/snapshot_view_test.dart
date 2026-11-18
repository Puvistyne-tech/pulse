import 'package:flutter_test/flutter_test.dart';
import 'package:pulse_mobile/snapshot/snapshot_view.dart';

import 'fixtures.dart';

void main() {
  group('SnapshotView', () {
    for (final fixture in snapshotCases) {
      testWidgets('renders ${fixture.name} snapshot', (tester) async {
        await tester.pumpWidget(SnapshotView(snapshot: fixture.snapshot));
        expect(find.text(fixture.expected), findsOneWidget);
      });
    }
  });
}
