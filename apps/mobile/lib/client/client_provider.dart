import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api/client.dart' as bridge;
import '../src/rust/third_party/pulse_protocol/auth.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import 'snapshot_order.dart';

final clientProvider = AsyncNotifierProvider<ClientNotifier, ClientSnapshot>(
  ClientNotifier.new,
);

class ClientNotifier extends AsyncNotifier<ClientSnapshot> {
  @override
  Future<ClientSnapshot> build() async {
    final subscription = bridge.watchSnapshots().listen(_apply);
    ref.onDispose(subscription.cancel);
    return newerSnapshot(state.value, await bridge.clientSnapshot());
  }

  Future<void> signUp(SignUpRequest request) =>
      _publish(bridge.signUp(request: request));

  Future<void> signIn(SignInRequest request) =>
      _publish(bridge.signIn(request: request));

  Future<void> signOut() => _publish(bridge.signOut());

  Future<void> refreshCampuses() => _publish(bridge.refreshCampuses());

  Future<void> createCampus(CreateCampusRequest request) =>
      _publish(bridge.createCampus(request: request));

  Future<void> selectCampus(CampusId campusId) =>
      _publish(bridge.selectCampus(campusId: campusId));

  Future<void> selectRoom(RoomId roomId) =>
      _publish(bridge.selectRoom(roomId: roomId));

  Future<void> sendMessage(RoomId roomId, String body) =>
      _publish(bridge.sendMessage(roomId: roomId, body: body));

  Future<void> discardMessage(ClientNonce clientNonce) =>
      _publish(bridge.discardMessage(clientNonce: clientNonce));

  void _apply(ClientSnapshot next) {
    state = AsyncData(newerSnapshot(state.value, next));
  }

  Future<void> _publish(Future<ClientSnapshot> pending) async {
    _apply(await pending);
  }
}
