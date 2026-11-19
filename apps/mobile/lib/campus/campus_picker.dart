import 'package:material_ui/material_ui.dart';

import '../shell/sidebar_section.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../src/rust/third_party/pulse_protocol/ids.dart';
import '../theme/pulse_avatar.dart';
import '../theme/pulse_theme.dart';

class CampusPicker extends StatelessWidget {
  const CampusPicker({
    required this.campuses,
    required this.activeCampusId,
    required this.selectCampus,
    super.key,
  });

  final List<CampusSummary> campuses;
  final CampusId? activeCampusId;
  final Future<void> Function(CampusId campusId) selectCampus;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        for (final campus in campuses)
          SidebarRow(
            active: campus.campusId == activeCampusId,
            onTap: () => selectCampus(campus.campusId),
            fill: tokens.rail,
            leading: PulseAvatar(
              name: campus.displayName,
              size: 28,
              square: true,
              highlighted: campus.campusId == activeCampusId,
            ),
            label: campus.displayName,
          ),
      ],
    );
  }
}
