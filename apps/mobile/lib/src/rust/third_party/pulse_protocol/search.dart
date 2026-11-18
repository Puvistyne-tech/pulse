import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<List<String>> searchTerms({required String text}) =>
    RustLib.instance.api.pulseProtocolSearchSearchTerms(text: text);
