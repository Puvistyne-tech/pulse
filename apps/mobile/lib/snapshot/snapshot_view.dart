import 'package:flutter/widgets.dart';

import '../src/rust/third_party/pulse_protocol/snapshot.dart';

String phaseLabel(ConnectionPhase phase) => switch (phase) {
  ConnectionPhase.disconnected => 'Disconnected',
  ConnectionPhase.connecting => 'Connecting',
  ConnectionPhase.handshaking => 'Handshaking',
  ConnectionPhase.connected => 'Connected',
  ConnectionPhase.backoff => 'Backoff',
};

class SnapshotView extends StatelessWidget {
  const SnapshotView({required this.snapshot, super.key});

  final ClientSnapshot snapshot;

  @override
  Widget build(BuildContext context) {
    final retryInMs = snapshot.retryInMs;
    final rejection = snapshot.rejection;
    return Directionality(
      textDirection: TextDirection.ltr,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(phaseLabel(snapshot.connection)),
          Text('Protocol version ${snapshot.protocolVersion.field0}'),
          Text('Attempt ${snapshot.attempt}'),
          if (retryInMs != null) Text('Retrying in $retryInMs ms'),
          if (rejection != null)
            Text(
              'Version rejected: supported ${rejection.supported.field0}, '
              'received ${rejection.received.field0}',
            ),
        ],
      ),
    );
  }
}
