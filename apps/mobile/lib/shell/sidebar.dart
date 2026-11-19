import 'package:material_ui/material_ui.dart';

import '../campus/campus_picker.dart';
import '../campus/room_list_view.dart';
import '../client/pulse_intents.dart';
import '../lounge/lounge_strip.dart';
import '../snapshot/snapshot_view.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_avatar.dart';
import '../theme/pulse_theme.dart';
import 'sidebar_section.dart';

class Sidebar extends StatelessWidget {
  const Sidebar({
    required this.snapshot,
    required this.intents,
    required this.onNewCampus,
    this.afterNavigate,
    super.key,
  });

  final ClientSnapshot snapshot;
  final PulseIntents intents;
  final VoidCallback onNewCampus;
  final VoidCallback? afterNavigate;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final session = snapshot.session;
    final displayName = session.account?.displayName ?? '';
    return Material(
      key: const Key('sidebar'),
      color: tokens.sidebar,
      child: SafeArea(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Padding(
              padding: const EdgeInsets.all(PulseSpacing.lg),
              child: Row(
                children: [
                  Icon(Icons.bolt, color: tokens.accent, size: 24),
                  const SizedBox(width: PulseSpacing.xs),
                  Text(
                    'Pulse',
                    style: TextStyle(
                      color: tokens.sidebarText,
                      fontSize: 20,
                      fontWeight: FontWeight.w800,
                    ),
                  ),
                ],
              ),
            ),
            Expanded(
              child: ListView(
                padding: EdgeInsets.zero,
                children: [
                  const SidebarHeading(label: 'Campuses'),
                  CampusPicker(
                    campuses: session.campuses,
                    activeCampusId: session.activeCampusId,
                    selectCampus: (campusId) {
                      afterNavigate?.call();
                      return intents.selectCampus(campusId);
                    },
                  ),
                  Padding(
                    padding: const EdgeInsets.symmetric(
                      horizontal: PulseSpacing.sm,
                      vertical: PulseSpacing.xs,
                    ),
                    child: OutlinedButton.icon(
                      key: const Key('newCampus'),
                      onPressed: onNewCampus,
                      style: OutlinedButton.styleFrom(
                        foregroundColor: tokens.sidebarText,
                        side: BorderSide(color: tokens.sidebarMuted),
                        alignment: Alignment.centerLeft,
                      ),
                      icon: const Icon(Icons.add, size: 18),
                      label: const Text('New campus'),
                    ),
                  ),
                  const SidebarHeading(label: 'Rooms'),
                  RoomListView(
                    rooms: session.rooms,
                    activeRoomId: session.activeRoomId,
                    selectRoom: (roomId) {
                      afterNavigate?.call();
                      return intents.selectRoom(roomId);
                    },
                  ),
                  LoungeStrip(lounges: snapshot.lounges),
                ],
              ),
            ),
            DefaultTextStyle.merge(
              style: TextStyle(color: tokens.sidebarMuted, fontSize: 11),
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: PulseSpacing.lg,
                  vertical: PulseSpacing.sm,
                ),
                child: SnapshotView(network: snapshot.network),
              ),
            ),
            Container(
              padding: const EdgeInsets.all(PulseSpacing.md),
              decoration: BoxDecoration(
                color: tokens.rail,
                border: Border(top: BorderSide(color: tokens.sidebarMuted)),
              ),
              child: Row(
                children: [
                  PulseAvatar(name: displayName, size: 32),
                  const SizedBox(width: PulseSpacing.md),
                  Expanded(
                    child: Text(
                      displayName,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        color: tokens.sidebarText,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ),
                  TextButton(
                    onPressed: intents.signOut,
                    style: TextButton.styleFrom(
                      foregroundColor: tokens.sidebarText,
                    ),
                    child: const Text('Sign out'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
