import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api/client.dart' as bridge;
import '../src/rust/third_party/pulse_protocol/auth.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

final clientProvider = AsyncNotifierProvider<ClientNotifier, ClientSnapshot>(
  ClientNotifier.new,
);

class ClientNotifier extends AsyncNotifier<ClientSnapshot> {
  @override
  Future<ClientSnapshot> build() => bridge.clientSnapshot();

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

  Future<void> _publish(Future<ClientSnapshot> pending) async {
    state = AsyncData(await pending);
  }
}
