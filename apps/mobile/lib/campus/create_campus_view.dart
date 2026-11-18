import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/campus.dart';

class CreateCampusView extends StatefulWidget {
  const CreateCampusView({
    required this.busy,
    required this.createCampus,
    super.key,
  });

  final bool busy;
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
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        TextField(
          key: const Key('campusSlug'),
          controller: _slug,
          decoration: const InputDecoration(labelText: 'Campus slug'),
        ),
        TextField(
          key: const Key('campusName'),
          controller: _displayName,
          decoration: const InputDecoration(labelText: 'Campus name'),
        ),
        ElevatedButton(
          key: const Key('createCampus'),
          onPressed: widget.busy ? null : _submit,
          child: const Text('Create campus'),
        ),
      ],
    );
  }
}
