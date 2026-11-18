import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

import '../client/pulse_intents.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

Map<ShortcutActivator, VoidCallback> inboxBindings(
  InboxSnapshot inbox,
  PulseIntents intents,
) {
  if (inbox.replying) {
    return {
      const SingleActivator(LogicalKeyboardKey.escape):
          intents.cancelInboxReply,
    };
  }
  return {
    const SingleActivator(LogicalKeyboardKey.keyJ): () =>
        intents.moveInboxSelection(InboxStep.next),
    const SingleActivator(LogicalKeyboardKey.keyK): () =>
        intents.moveInboxSelection(InboxStep.previous),
    const SingleActivator(LogicalKeyboardKey.keyE):
        intents.archiveInboxSelection,
    const SingleActivator(LogicalKeyboardKey.keyR): intents.startInboxReply,
  };
}
