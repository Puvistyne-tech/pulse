import '../../../frb_generated.dart';
import '../ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<String> campusRooms({required CampusId campusId}) =>
    RustLib.instance.api.pulseProtocolApiApiPathCampusRooms(campusId: campusId);
