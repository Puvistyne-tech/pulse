import 'package:material_ui/material_ui.dart';

import '../src/rust/third_party/pulse_protocol/api.dart';
import '../theme/pulse_theme.dart';
import 'error_message.dart';

class ErrorNotice extends StatelessWidget {
  const ErrorNotice({required this.error, super.key});

  final ApiError? error;

  @override
  Widget build(BuildContext context) {
    final current = error;
    if (current == null) {
      return const SizedBox.shrink();
    }
    final tokens = PulseTokens.of(context);
    return Container(
      key: const Key('errorNotice'),
      margin: const EdgeInsets.symmetric(vertical: PulseSpacing.sm),
      padding: const EdgeInsets.all(PulseSpacing.md),
      decoration: BoxDecoration(
        color: tokens.danger.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(PulseRadius.control),
        border: Border.all(color: tokens.danger.withValues(alpha: 0.35)),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Icon(Icons.error_outline, size: 18, color: tokens.danger),
          const SizedBox(width: PulseSpacing.sm),
          Expanded(
            child: Text(
              errorMessage(current),
              style: TextStyle(color: tokens.danger, fontSize: 14),
            ),
          ),
        ],
      ),
    );
  }
}
