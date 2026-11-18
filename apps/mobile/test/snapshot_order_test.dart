import 'package:flutter_test/flutter_test.dart';
import 'package:pulse_mobile/client/snapshot_order.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

import 'fixtures.dart';

class OrderCase {
  const OrderCase(this.name, this.current, this.next, this.keepsNext);

  final String name;
  final ClientSnapshot? current;
  final ClientSnapshot next;
  final bool keepsNext;
}

final orderCases = <OrderCase>[
  OrderCase(
    'no current snapshot',
    null,
    snapshotAt(signedInWithRooms, 3),
    true,
  ),
  OrderCase(
    'older current',
    snapshotAt(signedOut, 2),
    snapshotAt(signedInWithRooms, 3),
    true,
  ),
  OrderCase(
    'equal sequence',
    snapshotAt(signedOut, 3),
    snapshotAt(signedInWithRooms, 3),
    true,
  ),
  OrderCase(
    'newer current',
    snapshotAt(signedOut, 4),
    snapshotAt(signedInWithRooms, 3),
    false,
  ),
  OrderCase(
    'zero current',
    snapshotAt(signedOut, 0),
    snapshotAt(signedInWithRooms, 1),
    true,
  ),
  OrderCase(
    'sequence beyond 32 bits',
    ClientSnapshot(
      sequence: BigInt.parse('4294967296'),
      network: signedOut.network,
      session: signedOut.session,
      timeline: signedOut.timeline,
    ),
    snapshotAt(signedInWithRooms, 5),
    false,
  ),
];

void main() {
  group('newerSnapshot', () {
    for (final fixture in orderCases) {
      test(fixture.name, () {
        final result = newerSnapshot(fixture.current, fixture.next);
        expect(
          identical(result, fixture.keepsNext ? fixture.next : fixture.current),
          isTrue,
        );
      });
    }

    test('a stale sequence never regresses a folded stream', () {
      final stream = [2, 5, 3, 5, 4, 6].map((n) => snapshotAt(signedOut, n));
      final folded = stream.fold<ClientSnapshot?>(null, newerSnapshot);
      expect(folded?.sequence, BigInt.from(6));
    });
  });
}
