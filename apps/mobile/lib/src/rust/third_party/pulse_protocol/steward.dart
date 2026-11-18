import '../../frb_generated.dart';
import 'ids.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

Future<List<ToolDescriptor>> toolDescriptors() =>
    RustLib.instance.api.pulseProtocolStewardToolDescriptors();

abstract class StreamWriter implements RustOpaqueInterface {
  Future<StreamChunk> abort();

  Future<StreamChunk> begin();

  Future<StreamChunk> delta({required String text});

  Future<StreamChunk> end();

  static Future<StreamWriter> newInstance({
    required StreamId streamId,
    required RoomId roomId,
    MessageId? threadRootId,
  }) => RustLib.instance.api.pulseProtocolStewardStreamWriterNew(
    streamId: streamId,
    roomId: roomId,
    threadRootId: threadRootId,
  );

  Future<int> nextSequence();
}

enum StewardTool {
  listRooms,
  readRoomHistory,
  readThread,
  postMessage;

  Future<void> description() => RustLib.instance.api
      .pulseProtocolStewardStewardToolDescription(that: this);

  static Future<StewardTool?> fromName({required String name}) =>
      RustLib.instance.api.pulseProtocolStewardStewardToolFromName(name: name);

  Future<void> inputSchema() => RustLib.instance.api
      .pulseProtocolStewardStewardToolInputSchema(that: this);

  Future<void> name() =>
      RustLib.instance.api.pulseProtocolStewardStewardToolName(that: this);
}

class StreamChunk {
  final StreamId streamId;
  final RoomId roomId;
  final MessageId? threadRootId;
  final StreamPhase phase;
  final int sequence;
  final String text;

  const StreamChunk({
    required this.streamId,
    required this.roomId,
    this.threadRootId,
    required this.phase,
    required this.sequence,
    required this.text,
  });

  @override
  int get hashCode =>
      streamId.hashCode ^
      roomId.hashCode ^
      threadRootId.hashCode ^
      phase.hashCode ^
      sequence.hashCode ^
      text.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StreamChunk &&
          runtimeType == other.runtimeType &&
          streamId == other.streamId &&
          roomId == other.roomId &&
          threadRootId == other.threadRootId &&
          phase == other.phase &&
          sequence == other.sequence &&
          text == other.text;
}

enum StreamPhase { begin, delta, end, abort }

class ToolDescriptor {
  final String name;
  final String description;
  final String inputSchema;

  const ToolDescriptor({
    required this.name,
    required this.description,
    required this.inputSchema,
  });

  @override
  int get hashCode =>
      name.hashCode ^ description.hashCode ^ inputSchema.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ToolDescriptor &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          description == other.description &&
          inputSchema == other.inputSchema;
}

enum ToolErrorCode {
  forbidden,
  unknownTool,
  invalidArguments,
  unknownRoom,
  unknownThread,
  rejected;

  Future<void> asStr() =>
      RustLib.instance.api.pulseProtocolStewardToolErrorCodeAsStr(that: this);
}
