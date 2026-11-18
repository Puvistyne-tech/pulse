import '../frb_generated.dart';
import '../third_party/pulse_protocol/frames.dart';
import '../third_party/pulse_protocol/ids.dart';
import '../third_party/pulse_protocol/snapshot.dart';
import '../third_party/pulse_protocol/version.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<ClientSnapshot> initialSnapshot() =>
    RustLib.instance.api.crateApiSnapshotInitialSnapshot();
