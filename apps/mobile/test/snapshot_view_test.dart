import 'package:flutter_test/flutter_test.dart';
import 'package:pulse_mobile/snapshot/snapshot_view.dart';

import 'fixtures.dart';

void main() {
  group('SnapshotView', () {
    for (final fixture in networkCases) {
      testWidgets('renders ${fixture.name} network', (tester) async {
        await tester.pumpWidget(SnapshotView(network: fixture.network));
        expect(find.text(fixture.expected), findsOneWidget);
      });
    }
  });
}
