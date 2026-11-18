import 'package:material_ui/material_ui.dart';

import '../campus/campus_picker.dart';
import '../campus/create_campus_view.dart';
import '../campus/room_list_view.dart';
import '../client/pulse_intents.dart';
import '../inbox/inbox_view.dart';
import '../room/room_view.dart';
import '../room/thread_view.dart';
import '../session/auth_view.dart';
import '../session/error_notice.dart';
import '../snapshot/snapshot_view.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';

class HomeView extends StatelessWidget {
  const HomeView({required this.snapshot, required this.intents, super.key});

  final ClientSnapshot snapshot;
  final PulseIntents intents;

  @override
  Widget build(BuildContext context) {
    final session = snapshot.session;
    if (session.auth != AuthPhase.signedIn) {
      return AuthView(
        busy: session.busy,
        error: session.error,
        intents: intents,
      );
    }
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(session.account?.displayName ?? ''),
              Row(
                children: [
                  TextButton(
                    onPressed: intents.toggleInbox,
                    child: Text('Inbox ${snapshot.inbox.unreadThreads}'),
                  ),
                  TextButton(
                    onPressed: intents.signOut,
                    child: const Text('Sign out'),
                  ),
                ],
              ),
            ],
          ),
          ErrorNotice(error: session.error),
          CampusPicker(
            campuses: session.campuses,
            selectCampus: intents.selectCampus,
          ),
          CreateCampusView(
            busy: session.busy,
            createCampus: intents.createCampus,
          ),
          RoomListView(
            rooms: session.rooms,
            activeRoomId: session.activeRoomId,
            selectRoom: intents.selectRoom,
          ),
          if (snapshot.inbox.open)
            InboxView(inbox: snapshot.inbox, intents: intents),
          RoomView(timeline: snapshot.timeline, intents: intents),
          if (snapshot.thread.rootMessageId != null)
            ThreadView(
              thread: snapshot.thread,
              palette: snapshot.timeline.reactionPalette,
              intents: intents,
            ),
          SnapshotView(network: snapshot.network),
        ],
      ),
    );
  }
}
