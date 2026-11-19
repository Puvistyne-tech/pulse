import 'package:material_ui/material_ui.dart';

import '../session/error_notice.dart';
import '../shell/panel_card.dart';
import '../src/rust/third_party/pulse_protocol/api.dart';
import '../src/rust/third_party/pulse_protocol/campus.dart';
import '../theme/pulse_theme.dart';

class CreateCampusView extends StatefulWidget {
  const CreateCampusView({
    required this.busy,
    required this.createCampus,
    required this.onClose,
    this.error,
    super.key,
  });

  final bool busy;
  final ApiError? error;
  final VoidCallback onClose;
  final Future<void> Function(CreateCampusRequest request) createCampus;

  @override
  State<CreateCampusView> createState() => _CreateCampusViewState();
}

class _CreateCampusViewState extends State<CreateCampusView> {
  final _slug = TextEditingController();
  final _displayName = TextEditingController();

  @override
  void dispose() {
    _slug.dispose();
    _displayName.dispose();
    super.dispose();
  }

  void _submit() {
    widget.createCampus(
      CreateCampusRequest(slug: _slug.text, displayName: _displayName.text),
    );
  }

  @override
  Widget build(BuildContext context) {
    return PanelCard(
      cardKey: const Key('createCampusView'),
      title: 'New campus',
      closeLabel: 'Close new campus',
      onClose: widget.onClose,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          TextField(
            key: const Key('campusName'),
            controller: _displayName,
            decoration: const InputDecoration(labelText: 'Campus name'),
          ),
          const SizedBox(height: PulseSpacing.md),
          TextField(
            key: const Key('campusSlug'),
            controller: _slug,
            decoration: const InputDecoration(
              labelText: 'Campus slug',
              helperText:
                  'The slug must be unique across the whole server, not just '
                  'your own campuses.',
            ),
          ),
          ErrorNotice(error: widget.error),
          const SizedBox(height: PulseSpacing.lg),
          ElevatedButton(
            key: const Key('createCampus'),
            onPressed: widget.busy ? null : _submit,
            child: const Text('Create campus'),
          ),
        ],
      ),
    );
  }
}
