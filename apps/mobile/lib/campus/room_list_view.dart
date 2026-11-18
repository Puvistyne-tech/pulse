import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/campus.dart';

class RoomListView extends StatelessWidget {
  const RoomListView({required this.rooms, super.key});

  final List<Room> rooms;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [for (final room in rooms) Text('#${room.slug}')],
    );
  }
}
