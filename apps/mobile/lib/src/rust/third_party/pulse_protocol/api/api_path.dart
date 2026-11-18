import '../../../frb_generated.dart';
import '../ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<String> campusRooms({required CampusId campusId}) =>
    RustLib.instance.api.pulseProtocolApiApiPathCampusRooms(campusId: campusId);

Future<String> campusRoomUploads({
  required CampusId campusId,
  required RoomId roomId,
}) => RustLib.instance.api.pulseProtocolApiApiPathCampusRoomUploads(
  campusId: campusId,
  roomId: roomId,
);

Future<String> campusDownloads({required CampusId campusId}) => RustLib
    .instance
    .api
    .pulseProtocolApiApiPathCampusDownloads(campusId: campusId);
