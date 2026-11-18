import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:material_ui/material_ui.dart';

import 'snapshot/snapshot_provider.dart';
import 'snapshot/snapshot_view.dart';

class PulseApp extends ConsumerWidget {
  const PulseApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final snapshot = ref.watch(snapshotProvider);
    return MaterialApp(
      title: 'Pulse',
      home: Scaffold(
        body: Center(
          child: snapshot.when(
            data: (value) => SnapshotView(snapshot: value),
            loading: () => const Text('Loading Pulse'),
            error: (error, stackTrace) => const Text('Pulse core unavailable'),
          ),
        ),
      ),
    );
  }
}
