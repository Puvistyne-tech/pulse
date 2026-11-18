import '../../frb_generated.dart';
import 'api.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> guessContentType({required String fileName}) =>
    RustLib.instance.api.pulseProtocolMediaGuessContentType(fileName: fileName);

enum AttachmentKind {
  image,
  video,
  audio,
  file;

  Future<void> asStr() =>
      RustLib.instance.api.pulseProtocolMediaAttachmentKindAsStr(that: this);

  static Future<AttachmentKind> fromContentType({
    required String contentType,
  }) => RustLib.instance.api.pulseProtocolMediaAttachmentKindFromContentType(
    contentType: contentType,
  );
}

class CreateUploadRequest {
  final String fileName;
  final String contentType;
  final BigInt byteSize;

  const CreateUploadRequest({
    required this.fileName,
    required this.contentType,
    required this.byteSize,
  });

  Future<void> validate() => RustLib.instance.api
      .pulseProtocolMediaCreateUploadRequestValidate(that: this);

  @override
  int get hashCode =>
      fileName.hashCode ^ contentType.hashCode ^ byteSize.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CreateUploadRequest &&
          runtimeType == other.runtimeType &&
          fileName == other.fileName &&
          contentType == other.contentType &&
          byteSize == other.byteSize;
}

enum VaultTab {
  media,
  files,
  links,
  pinned;

  static Future<VaultTab> default_() =>
      RustLib.instance.api.pulseProtocolMediaVaultTabDefault();
}
