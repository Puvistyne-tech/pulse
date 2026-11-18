import 'package:flutter/widgets.dart';

import '../src/rust/third_party/pulse_protocol/api.dart';
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
    return Text(errorMessage(current));
  }
}
