import 'package:flutter/widgets.dart';

import '../src/rust/third_party/pulse_protocol/frames.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

String phaseLabel(ConnectionPhase phase) => switch (phase) {
  ConnectionPhase.disconnected => 'Disconnected',
  ConnectionPhase.connecting => 'Connecting',
  ConnectionPhase.handshaking => 'Handshaking',
  ConnectionPhase.connected => 'Connected',
  ConnectionPhase.backoff => 'Backoff',
};

String handshakeRejectionLabel(HandshakeRejection rejection) =>
    switch (rejection) {
      HandshakeRejection.unauthenticated =>
        'Handshake rejected: unauthenticated',
      HandshakeRejection.campusUnavailable =>
        'Handshake rejected: campus unavailable',
    };

class SnapshotView extends StatelessWidget {
  const SnapshotView({required this.network, super.key});

  final ConnectionSnapshot network;

  @override
  Widget build(BuildContext context) {
    final retryInMs = network.retryInMs;
    final rejection = network.rejection;
    final handshakeRejection = network.handshakeRejection;
    return Directionality(
      textDirection: TextDirection.ltr,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(phaseLabel(network.connection)),
          Text('Protocol version ${network.protocolVersion.field0}'),
          Text('Attempt ${network.attempt}'),
          if (retryInMs != null) Text('Retrying in $retryInMs ms'),
          if (rejection != null)
            Text(
              'Version rejected: supported ${rejection.supported.field0}, '
              'received ${rejection.received.field0}',
            ),
          if (handshakeRejection != null)
            Text(handshakeRejectionLabel(handshakeRejection)),
        ],
      ),
    );
  }
}
