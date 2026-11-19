import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:material_ui/material_ui.dart';

import 'client/client_provider.dart';
import 'client/pulse_intents.dart';
import 'home/home_view.dart';
import 'theme/pulse_theme.dart';

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
      selectRoom: notifier.selectRoom,
      sendMessage: notifier.sendMessage,
      discardMessage: notifier.discardMessage,
      sendReply: notifier.sendReply,
      react: notifier.react,
      setTopicTitle: notifier.setTopicTitle,
      setThreadSubscription: notifier.setThreadSubscription,
      openThread: notifier.openThread,
      closeThread: notifier.closeThread,
      toggleInbox: notifier.toggleInbox,
      moveInboxSelection: notifier.moveInboxSelection,
      archiveInboxSelection: notifier.archiveInboxSelection,
      startInboxReply: notifier.startInboxReply,
      cancelInboxReply: notifier.cancelInboxReply,
      toggleSearch: notifier.toggleSearch,
      search: notifier.search,
      moveSearchSelection: notifier.moveSearchSelection,
      openSearchSelection: notifier.openSearchSelection,
      openSearchHit: notifier.openSearchHit,
      attachFile: notifier.attachFile,
      discardDraft: notifier.discardDraft,
      retryDraft: notifier.retryDraft,
      setPin: notifier.setPin,
      openVault: notifier.openVault,
      loadMoreVault: notifier.loadMoreVault,
      closeVault: notifier.closeVault,
      toggleShelf: notifier.toggleShelf,
      updateShelf: notifier.updateShelf,
      dismissReminder: notifier.dismissReminder,
    );
    return MaterialApp(
      title: 'Pulse',
      theme: pulseLightTheme(),
      darkTheme: pulseDarkTheme(),
      themeMode: ThemeMode.system,
      home: snapshot.when(
        data: (value) => HomeView(snapshot: value, intents: intents),
        loading: () =>
            const Scaffold(body: Center(child: Text('Loading Pulse'))),
        error: (error, stackTrace) =>
            const Scaffold(body: Center(child: Text('Pulse core unavailable'))),
      ),
    );
  }
}
