import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';

class CampusPicker extends StatelessWidget {
  const CampusPicker({
    required this.campuses,
    required this.selectCampus,
    super.key,
  });

  final List<CampusSummary> campuses;
  final Future<void> Function(CampusId campusId) selectCampus;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        for (final campus in campuses)
          TextButton(
            onPressed: () => selectCampus(campus.campusId),
            child: Text(campus.displayName),
          ),
      ],
    );
  }
}
