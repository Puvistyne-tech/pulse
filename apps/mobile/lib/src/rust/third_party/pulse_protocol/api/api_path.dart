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

Future<String> campusInvites({required CampusId campusId}) => RustLib
    .instance
    .api
    .pulseProtocolApiApiPathCampusInvites(campusId: campusId);

Future<String> campusMembers({required CampusId campusId}) => RustLib
    .instance
    .api
    .pulseProtocolApiApiPathCampusMembers(campusId: campusId);

Future<String> campusMember({
  required CampusId campusId,
  required AccountId accountId,
}) => RustLib.instance.api.pulseProtocolApiApiPathCampusMember(
  campusId: campusId,
  accountId: accountId,
);

Future<String> campusDownloads({required CampusId campusId}) => RustLib
    .instance
    .api
    .pulseProtocolApiApiPathCampusDownloads(campusId: campusId);
