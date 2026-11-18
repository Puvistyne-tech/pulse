import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:path_provider/path_provider.dart';

import 'app.dart';
import 'src/rust/api/client.dart' as bridge;
import 'src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  final directory = await getApplicationSupportDirectory();
  await bridge.initStorage(dataDir: directory.path);
  runApp(const ProviderScope(child: PulseApp()));
}
