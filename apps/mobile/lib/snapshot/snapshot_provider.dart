import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api/snapshot.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

final snapshotProvider = FutureProvider<ClientSnapshot>(
  (ref) => initialSnapshot(),
);
