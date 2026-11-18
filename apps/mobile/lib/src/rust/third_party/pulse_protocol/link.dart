import '../../frb_generated.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<List<String>> extractLinks({required String body}) =>
    RustLib.instance.api.pulseProtocolLinkExtractLinks(body: body);
