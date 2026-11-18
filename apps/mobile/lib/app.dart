import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:material_ui/material_ui.dart';

import 'client/client_provider.dart';
import 'client/pulse_intents.dart';
import 'home/home_view.dart';

class PulseApp extends ConsumerWidget {
  const PulseApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final snapshot = ref.watch(clientProvider);
    final notifier = ref.read(clientProvider.notifier);
    final intents = PulseIntents(
      signUp: notifier.signUp,
      signIn: notifier.signIn,
      signOut: notifier.signOut,
      createCampus: notifier.createCampus,
      selectCampus: notifier.selectCampus,
      refreshCampuses: notifier.refreshCampuses,
    );
    return MaterialApp(
      title: 'Pulse',
      home: Scaffold(
        body: SafeArea(
          child: snapshot.when(
            data: (value) => HomeView(snapshot: value, intents: intents),
            loading: () => const Center(child: Text('Loading Pulse')),
            error: (error, stackTrace) =>
                const Center(child: Text('Pulse core unavailable')),
          ),
        ),
      ),
    );
  }
}
