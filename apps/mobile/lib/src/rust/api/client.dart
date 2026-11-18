import '../frb_generated.dart';
import '../third_party/pulse_protocol/api.dart';
import '../third_party/pulse_protocol/auth.dart';
import '../third_party/pulse_protocol/campus.dart';
import '../third_party/pulse_protocol/frames.dart';
import '../third_party/pulse_protocol/ids.dart';
import '../third_party/pulse_protocol/messaging.dart';
import '../third_party/pulse_protocol/rbac.dart';
import '../third_party/pulse_protocol/snapshot.dart';
import '../third_party/pulse_protocol/version.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<ClientSnapshot> clientSnapshot() =>
    RustLib.instance.api.crateApiClientClientSnapshot();

Future<ClientSnapshot> signUp({required SignUpRequest request}) =>
    RustLib.instance.api.crateApiClientSignUp(request: request);

Future<ClientSnapshot> signIn({required SignInRequest request}) =>
    RustLib.instance.api.crateApiClientSignIn(request: request);

Future<ClientSnapshot> signOut() =>
    RustLib.instance.api.crateApiClientSignOut();

Future<ClientSnapshot> refreshCampuses() =>
    RustLib.instance.api.crateApiClientRefreshCampuses();

Future<ClientSnapshot> createCampus({required CreateCampusRequest request}) =>
    RustLib.instance.api.crateApiClientCreateCampus(request: request);

Future<ClientSnapshot> selectCampus({required CampusId campusId}) =>
    RustLib.instance.api.crateApiClientSelectCampus(campusId: campusId);

Future<ClientSnapshot> selectRoom({required RoomId roomId}) =>
    RustLib.instance.api.crateApiClientSelectRoom(roomId: roomId);

Future<ClientSnapshot> sendMessage({
  required RoomId roomId,
  required String body,
}) =>
    RustLib.instance.api.crateApiClientSendMessage(roomId: roomId, body: body);

Future<ClientSnapshot> discardMessage({required ClientNonce clientNonce}) =>
    RustLib.instance.api.crateApiClientDiscardMessage(clientNonce: clientNonce);

Stream<ClientSnapshot> watchSnapshots() =>
    RustLib.instance.api.crateApiClientWatchSnapshots();
