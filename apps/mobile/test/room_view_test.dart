import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/room/room_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/messaging.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

import 'fixtures.dart';

class EntryCase {
  const EntryCase(
    this.name,
    this.entry,
    this.author,
    this.status,
    this.discardable,
  );

  final String name;
  final TimelineEntry entry;
  final String author;
  final String? status;
  final bool discardable;
}

final entryCases = <EntryCase>[
  EntryCase('pending', pendingEntry, 'You', 'Sending', false),
  EntryCase('confirmed from a peer', confirmedEntry, 'Grace', null, false),
  EntryCase('confirmed from self', confirmedOwnEntry, 'You', null, false),
  EntryCase(
    'failed forbidden',
    failedEntry(MessageRejection.forbidden),
    'You',
    'Not allowed to post here',
    true,
  ),
  EntryCase(
    'failed invalid body',
    failedEntry(MessageRejection.invalidBody),
    'You',
    'Message not accepted',
    true,
  ),
  EntryCase(
    'failed unknown room',
    failedEntry(MessageRejection.unknownRoom),
    'You',
    'Room not found',
    true,
  ),
];

class Recorder {
  final sent = <(Object, String)>[];
  final discarded = <Object>[];

  Future<void> sendMessage(Object roomId, String body) async =>
      sent.add((roomId, body));

  Future<void> discardMessage(Object clientNonce) async =>
      discarded.add(clientNonce);
}

Future<Recorder> pumpRoom(
  WidgetTester tester,
  TimelineSnapshot timeline,
) async {
  final recorder = Recorder();
  await pumpView(
    tester,
    RoomView(
      timeline: timeline,
      sendMessage: (roomId, body) => recorder.sendMessage(roomId, body),
      discardMessage: (nonce) => recorder.discardMessage(nonce),
    ),
  );
  return recorder;
}

void main() {
  group('timeline entries', () {
    for (final fixture in entryCases) {
      testWidgets('${fixture.name} renders author, body and status', (
        tester,
      ) async {
        await pumpRoom(tester, timelineOf([fixture.entry]));
        expect(find.text(fixture.author), findsOneWidget);
        expect(find.text(fixture.entry.body), findsOneWidget);
        expect(
          find.text('Sending'),
          findsNWidgets(fixture.status == 'Sending' ? 1 : 0),
        );
        if (fixture.status != null) {
          expect(find.text(fixture.status!), findsOneWidget);
        }
        expect(
          find.text('Discard'),
          fixture.discardable ? findsOneWidget : findsNothing,
        );
      });
    }

    testWidgets('discarding reports the failed nonce', (tester) async {
      final recorder = await pumpRoom(
        tester,
        timelineOf([failedEntry(MessageRejection.forbidden)]),
      );
      await tester.tap(find.text('Discard'));
      expect(recorder.discarded, [nonceC]);
    });

    testWidgets('entries render in snapshot order', (tester) async {
      await pumpRoom(
        tester,
        timelineOf([confirmedEntry, confirmedOwnEntry, pendingEntry]),
      );
      final tops = [
        'welcome',
        'sent',
        'hello',
      ].map((body) => tester.getTopLeft(find.text(body)).dy).toList();
      expect(tops, orderedEquals([...tops]..sort()));
    });
  });

  group('syncing marker', () {
    for (final syncing in [true, false]) {
      testWidgets('syncing $syncing', (tester) async {
        await pumpRoom(tester, timelineOf([], syncing: syncing));
        expect(find.text('Syncing'), syncing ? findsOneWidget : findsNothing);
      });
    }
  });

  group('composer', () {
    for (final draft in ['hello', '  padded  ', '   ']) {
      testWidgets('sends "$draft" exactly as typed and clears', (tester) async {
        final recorder = await pumpRoom(tester, timelineOf([]));
        await tester.enterText(find.byKey(const Key('messageDraft')), draft);
        await tester.pump();
        await tester.tap(find.byKey(const Key('sendMessage')));
        await tester.pump();
        expect(recorder.sent, [(roomAId, draft)]);
        final field = tester.widget<TextField>(
          find.byKey(const Key('messageDraft')),
        );
        expect(field.controller?.text, '');
      });
    }

    for (final (name, draft, disabled) in [
      ('empty draft', '', true),
      ('typed draft', 'x', false),
      ('whitespace draft', ' ', false),
    ]) {
      testWidgets('$name disables send $disabled', (tester) async {
        await pumpRoom(tester, timelineOf([]));
        await tester.enterText(find.byKey(const Key('messageDraft')), draft);
        await tester.pump();
        final button = tester.widget<ElevatedButton>(
          find.byKey(const Key('sendMessage')),
        );
        expect(button.onPressed == null, disabled);
      });
    }
  });

  group('room view shell', () {
    testWidgets('renders nothing without an active room', (tester) async {
      await pumpRoom(tester, emptyTimeline);
      expect(find.byKey(const Key('messageDraft')), findsNothing);
      expect(find.text('Send'), findsNothing);
    });

    for (final (name, error, notice) in [
      ('no error', null, null),
      (
        'message body error',
        const ApiError(
          code: ApiErrorCode.invalidInput,
          field: InputField.messageBody,
        ),
        'Message is empty or too long',
      ),
    ]) {
      testWidgets('timeline $name', (tester) async {
        await pumpRoom(tester, timelineOf([], error: error));
        if (notice == null) {
          expect(find.text('Message is empty or too long'), findsNothing);
        } else {
          expect(find.text(notice), findsOneWidget);
        }
      });
    }
  });
}
