import '../src/rust/third_party/pulse_protocol/auth.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';

class PulseIntents {
  const PulseIntents({
    required this.signUp,
    required this.signIn,
    required this.signOut,
    required this.createCampus,
    required this.selectCampus,
    required this.refreshCampuses,
  });

  final Future<void> Function(SignUpRequest request) signUp;
  final Future<void> Function(SignInRequest request) signIn;
  final Future<void> Function() signOut;
  final Future<void> Function(CreateCampusRequest request) createCampus;
  final Future<void> Function(CampusId campusId) selectCampus;
  final Future<void> Function() refreshCampuses;
}
