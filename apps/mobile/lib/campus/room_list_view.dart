import 'package:material_ui/material_ui.dart';

import '../shell/sidebar_section.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';

class RoomListView extends StatelessWidget {
  const RoomListView({
    required this.rooms,
    required this.activeRoomId,
    required this.selectRoom,
    super.key,
  });

  final List<Room> rooms;
  final RoomId? activeRoomId;
  final Future<void> Function(RoomId roomId) selectRoom;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        for (final room in rooms)
          SidebarRow(
            active: room.roomId == activeRoomId,
            onTap: () => selectRoom(room.roomId),
            label: '#${room.slug}',
          ),
      ],
    );
  }
}
