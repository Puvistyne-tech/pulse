import '../../frb_generated.dart';
import 'ids.dart';
import 'messaging.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<String> mentionToken({required AccountId accountId}) =>
    RustLib.instance.api.pulseProtocolMentionMentionToken(accountId: accountId);

Future<List<AccountId>> extractMentions({required String body}) =>
    RustLib.instance.api.pulseProtocolMentionExtractMentions(body: body);

Future<List<BodySegment>> bodySegments({
  required String body,
  required List<MentionRef> mentions,
}) => RustLib.instance.api.pulseProtocolMentionBodySegments(
  body: body,
  mentions: mentions,
);

class BodySegment {
  final SegmentKind kind;
  final String text;
  final AccountId? accountId;
  final String? url;

  const BodySegment({
    required this.kind,
    required this.text,
    this.accountId,
    this.url,
  });

  @override
  int get hashCode =>
      kind.hashCode ^ text.hashCode ^ accountId.hashCode ^ url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BodySegment &&
          runtimeType == other.runtimeType &&
          kind == other.kind &&
          text == other.text &&
          accountId == other.accountId &&
          url == other.url;
}

enum SegmentKind { text, mention, link }
