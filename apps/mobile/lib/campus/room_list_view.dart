import 'package:material_ui/material_ui.dart';

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
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        for (final room in rooms)
          TextButton(
            onPressed: () => selectRoom(room.roomId),
            child: Text(
              '#${room.slug}',
              style: room.roomId == activeRoomId
                  ? const TextStyle(fontWeight: FontWeight.bold)
                  : null,
            ),
          ),
      ],
    );
  }
}
