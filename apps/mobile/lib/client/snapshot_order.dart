import '../src/rust/third_party/pulse_protocol/snapshot.dart';

ClientSnapshot newerSnapshot(ClientSnapshot? current, ClientSnapshot next) {
  if (current != null && current.sequence > next.sequence) {
    return current;
  }
  return next;
}
