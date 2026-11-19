import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/room/room_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/media.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/mention.dart'
    show BodySegment, SegmentKind;
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/messaging.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/reaction.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/shelf.dart';
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
  EntryCase(
    'failed unknown thread',
    failedEntry(MessageRejection.unknownThread),
    'You',
    'Thread not found',
    true,
  ),
];

class ReactCase {
  const ReactCase(this.name, this.key, this.emoji, this.action);

  final String name;
  final Key key;
  final String emoji;
  final ReactionAction action;
}

const reactCases = <ReactCase>[
  ReactCase(
    'chip reacted by self removes',
    ValueKey('reaction-👍'),
    '👍',
    ReactionAction.remove,
  ),
  ReactCase(
    'chip reacted by others adds',
    ValueKey('reaction-🎉'),
    '🎉',
    ReactionAction.add,
  ),
  ReactCase(
    'palette emoji reacted by self removes',
    ValueKey('palette-👍'),
    '👍',
    ReactionAction.remove,
  ),
  ReactCase(
    'palette emoji reacted by others adds',
    ValueKey('palette-🎉'),
    '🎉',
    ReactionAction.add,
  ),
  ReactCase(
    'palette emoji not yet used adds',
    ValueKey('palette-🙏'),
    '🙏',
    ReactionAction.add,
  ),
];

class BarCase {
  const BarCase(this.name, this.preview, this.visible);

  final String name;
  final ThreadPreview preview;
  final bool visible;
}

final barCases = <BarCase>[
  BarCase('shown when buried', threadPreviewOf(), true),
  BarCase(
    'hidden when not buried',
    threadPreviewOf(showLatestReply: false),
    false,
  ),
  BarCase(
    'hidden without a latest reply',
    threadPreviewOf(hasLatestReply: false),
    false,
  ),
];

Future<RecordedIntents> pumpRoom(
  WidgetTester tester,
  TimelineSnapshot timeline, {
  List<ReminderPreset> presets = const [],
  BigInt? pickerAnchorMs,
  BigInt? pickerLatestMs,
}) async {
  final recorded = RecordedIntents();
  await pumpView(
    tester,
    SingleChildScrollView(
      child: RoomView(
        timeline: timeline,
        intents: recorded.intents,
        presets: presets,
        pickerAnchorMs: pickerAnchorMs,
        pickerLatestMs: pickerLatestMs,
      ),
    ),
  );
  return recorded;
}

String draftText(WidgetTester tester) => tester
    .widget<TextField>(find.byKey(const Key('messageDraft')))
    .controller!
    .text;

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
      final recorded = await pumpRoom(
        tester,
        timelineOf([failedEntry(MessageRejection.forbidden)]),
      );
      await tester.tap(find.text('Discard'));
      expect(recorded.calls, [nonceC]);
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

  group('steward entries', () {
    for (final (name, entry, badge, state) in [
      ('a human author', gracesEntryWith(), false, null),
      ('a steward author', gracesEntryWith(authorIsSteward: true), true, null),
      (
        'a streaming steward',
        gracesEntryWith(authorIsSteward: true, stream: StreamState.streaming),
        true,
        'Streaming',
      ),
      (
        'a finalizing steward',
        gracesEntryWith(authorIsSteward: true, stream: StreamState.finalizing),
        true,
        'Finalizing',
      ),
    ]) {
      testWidgets('$name shows the badge and stream state it has', (
        tester,
      ) async {
        await pumpRoom(tester, timelineOf([entry]));
        expect(find.text('Steward'), badge ? findsOneWidget : findsNothing);
        for (final label in ['Streaming', 'Finalizing']) {
          expect(
            find.text(label),
            label == state ? findsOneWidget : findsNothing,
          );
        }
      });
    }
  });

  group('body segments', () {
    testWidgets('mentions render emphasised and plain text does not', (
      tester,
    ) async {
      await pumpRoom(tester, timelineOf([mentionEntry]));
      expect(find.textContaining('@Ada', findRichText: true), findsOneWidget);
      final rich = tester.widget<RichText>(
        find.descendant(
          of: find.byType(Flexible),
          matching: find.byType(RichText),
        ),
      );
      expect(boldSpans(rich), [('hi ', false), ('@Ada', true)]);
    });

    testWidgets('the raw mention token is never shown', (tester) async {
      await pumpRoom(tester, timelineOf([mentionEntry]));
      expect(find.textContaining('<@', findRichText: true), findsNothing);
    });
  });

  group('reactions', () {
    testWidgets('chips show emoji and counts', (tester) async {
      await pumpRoom(tester, timelineOf([reactedEntry]));
      expect(find.text('👍 2'), findsOneWidget);
      expect(find.text('🎉 1'), findsOneWidget);
      final mine = tester.widget<FilterChip>(
        find.byKey(const ValueKey('reaction-👍')),
      );
      final theirs = tester.widget<FilterChip>(
        find.byKey(const ValueKey('reaction-🎉')),
      );
      expect(mine.selected, isTrue);
      expect(theirs.selected, isFalse);
    });

    for (final fixture in reactCases) {
      testWidgets(fixture.name, (tester) async {
        final recorded = await pumpRoom(tester, timelineOf([reactedEntry]));
        await tester.tap(find.byKey(fixture.key));
        expect(recorded.calls, [
          ('react', roomAId, messageAId, fixture.emoji, fixture.action),
        ]);
      });
    }

    for (final (name, palette) in [
      ('the protocol palette', reactionPalette),
      ('a custom palette', ['🙏']),
      ('an empty palette', <String>[]),
    ]) {
      testWidgets('palette renders $name from the snapshot', (tester) async {
        await pumpRoom(tester, timelineOf([confirmedEntry], palette: palette));
        final keys = tester
            .widgetList<TextButton>(find.byType(TextButton))
            .map((button) => button.key)
            .whereType<ValueKey<String>>()
            .where((key) => key.value.startsWith('palette-'))
            .map((key) => key.value.substring('palette-'.length))
            .toList();
        expect(keys, palette);
      });
    }

    testWidgets('pending entries offer no reactions', (tester) async {
      await pumpRoom(tester, timelineOf([pendingEntry]));
      expect(find.byKey(const ValueKey('palette-👍')), findsNothing);
    });
  });

  group('thread preview', () {
    for (final fixture in barCases) {
      testWidgets('latest reply bar is ${fixture.name}', (tester) async {
        await pumpRoom(tester, timelineOf([threadedEntry(fixture.preview)]));
        expect(
          find.byKey(const Key('latestReplyBar')),
          fixture.visible ? findsOneWidget : findsNothing,
        );
        expect(
          find.text('latest words'),
          fixture.visible ? findsOneWidget : findsNothing,
        );
      });
    }

    testWidgets('collapsing hides the reply and expanding restores it', (
      tester,
    ) async {
      await pumpRoom(tester, timelineOf([threadedEntry()]));
      expect(find.text('latest words'), findsOneWidget);
      await tester.tap(find.text('Latest reply'));
      await tester.pump();
      expect(find.text('latest words'), findsNothing);
      await tester.tap(find.text('Latest reply'));
      await tester.pump();
      expect(find.text('latest words'), findsOneWidget);
    });

    for (final label in ['Open thread', 'Reply in thread']) {
      testWidgets('$label opens the thread of the entry', (tester) async {
        final recorded = await pumpRoom(tester, timelineOf([threadedEntry()]));
        await tester.tap(find.text(label));
        expect(recorded.calls, [('openThread', roomAId, messageAId)]);
      });
    }

    for (final (name, preview, texts) in [
      ('counts and unread', threadPreviewOf(), ['Replies 3', 'following', '2']),
      (
        'topic title',
        threadPreviewOf(topicTitle: 'Plans', unreadReplies: 0),
        ['Plans', 'Replies 3'],
      ),
    ]) {
      testWidgets('summary shows $name', (tester) async {
        await pumpRoom(tester, timelineOf([threadedEntry(preview)]));
        for (final text in texts) {
          expect(find.text(text), findsWidgets);
        }
      });
    }

    testWidgets('entries without a thread show no summary', (tester) async {
      await pumpRoom(tester, timelineOf([confirmedEntry]));
      expect(find.textContaining('Replies'), findsNothing);
    });
  });

  group('mentions', () {
    testWidgets('mention appends the author token to the draft', (
      tester,
    ) async {
      await pumpRoom(tester, timelineOf([confirmedEntry, mentionEntry]));
      final buttons = find.text('Mention');
      await tester.tap(buttons.first);
      await tester.pump();
      expect(draftText(tester), '${mentionToken(graceId)} ');
      await tester.enterText(find.byKey(const Key('messageDraft')), 'hey');
      await tester.tap(buttons.last);
      await tester.pump();
      expect(draftText(tester), 'hey ${mentionToken(graceId)} ');
    });

    testWidgets('own entries have no mention button', (tester) async {
      await pumpRoom(tester, timelineOf([confirmedOwnEntry, pendingEntry]));
      expect(find.text('Mention'), findsNothing);
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
        final recorded = await pumpRoom(tester, timelineOf([]));
        await tester.enterText(find.byKey(const Key('messageDraft')), draft);
        await tester.pump();
        await tester.tap(find.byKey(const Key('sendMessage')));
        await tester.pump();
        expect(recorded.calls, [(roomAId, draft)]);
        expect(draftText(tester), '');
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
      (
        'reaction emoji error',
        const ApiError(
          code: ApiErrorCode.invalidInput,
          field: InputField.reactionEmoji,
        ),
        'Choose a single emoji reaction',
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

  group('vault button', () {
    testWidgets('opens the media tab of the active room', (tester) async {
      final recorded = await pumpRoom(tester, timelineOf([]));
      await tester.tap(find.byKey(const Key('openVault')));
      expect(recorded.calls, [('openVault', roomAId, VaultTab.media)]);
    });
  });

  group('attachments', () {
    for (final (name, kind, url, shown) in [
      (
        'a file with a ticket',
        AttachmentKind.file,
        'https://files.example.test/f',
        'named',
      ),
      (
        'a video with a ticket',
        AttachmentKind.video,
        'https://files.example.test/v',
        'named',
      ),
      (
        'audio with a ticket',
        AttachmentKind.audio,
        'https://files.example.test/a',
        'named',
      ),
      ('an image without a ticket', AttachmentKind.image, null, 'named'),
      ('a file without a ticket', AttachmentKind.file, null, 'named'),
    ]) {
      testWidgets('$name is listed by file name', (tester) async {
        await pumpRoom(
          tester,
          timelineOf([
            gracesEntryWith(
              attachments: [attachmentViewOf(kind, fileName: shown, url: url)],
            ),
          ]),
        );
        expect(find.byKey(const Key('attachmentList')), findsOneWidget);
        expect(find.text(shown), findsOneWidget);
        expect(find.byType(Image), findsNothing);
      });
    }

    testWidgets('attachments render in message order', (tester) async {
      await pumpRoom(
        tester,
        timelineOf([
          gracesEntryWith(
            attachments: [
              attachmentViewOf(
                AttachmentKind.file,
                attachmentId: attachmentAId,
                fileName: 'first',
                url: null,
              ),
              attachmentViewOf(
                AttachmentKind.file,
                attachmentId: attachmentBId,
                fileName: 'second',
                url: null,
              ),
            ],
          ),
        ]),
      );
      expect(
        tester.getTopLeft(find.text('first')).dy,
        lessThan(tester.getTopLeft(find.text('second')).dy),
      );
    });

    testWidgets('entries without attachments render no list', (tester) async {
      await pumpRoom(tester, timelineOf([confirmedEntry]));
      expect(find.byKey(const Key('attachmentList')), findsNothing);
    });
  });

  group('links', () {
    for (final (name, kind, underlined) in [
      ('link segments are underlined', SegmentKind.link, true),
      ('text segments are plain', SegmentKind.text, false),
      ('mention segments are not underlined', SegmentKind.mention, false),
    ]) {
      testWidgets(name, (tester) async {
        await pumpRoom(
          tester,
          timelineOf([
            gracesEntryWith(
              segments: [
                BodySegment(
                  kind: kind,
                  text: 'segment-text',
                  url: kind == SegmentKind.link
                      ? 'https://example.org/a'
                      : null,
                ),
              ],
            ),
          ]),
        );
        final rich = tester.widget<RichText>(
          find.descendant(
            of: find.byType(Flexible),
            matching: find.byType(RichText),
          ),
        );
        expect(
          richSpans(rich).single.style?.decoration == TextDecoration.underline,
          underlined,
        );
      });
    }
  });

  group('drafts', () {
    for (final (upload, label, retry) in [
      (UploadState.uploading, 'Uploading', false),
      (UploadState.uploaded, 'Ready', false),
      (UploadState.failed, 'Failed', true),
    ]) {
      testWidgets('a ${upload.name} draft reads $label', (tester) async {
        await pumpRoom(tester, timelineOf([], drafts: [draftOf(upload)]));
        expect(find.text('draft.png $label'), findsOneWidget);
        expect(find.text('Retry'), retry ? findsOneWidget : findsNothing);
        expect(find.text('Remove'), findsOneWidget);
      });
    }

    testWidgets('retry and remove call their intents with the draft', (
      tester,
    ) async {
      final recorded = await pumpRoom(
        tester,
        timelineOf(
          [],
          drafts: [
            draftOf(UploadState.failed),
            draftOf(UploadState.uploaded, draftId: draftBId, fileName: 'b.png'),
          ],
        ),
      );
      await tester.tap(find.text('Retry'));
      await tester.tap(find.text('Remove').last);
      expect(recorded.calls, [
        ('retryDraft', draftAId),
        ('discardDraft', draftBId),
      ]);
    });

    testWidgets('no drafts render no chips', (tester) async {
      await pumpRoom(tester, timelineOf([]));
      expect(find.text('Remove'), findsNothing);
    });

    for (final (canUpload, visible) in [(true, true), (false, false)]) {
      testWidgets('canUpload $canUpload shows the attach button $visible', (
        tester,
      ) async {
        await pumpRoom(tester, timelineOf([], canUpload: canUpload));
        expect(
          find.byKey(const Key('attachFile')),
          visible ? findsOneWidget : findsNothing,
        );
      });
    }

    testWidgets('a draft without text still enables send', (tester) async {
      await pumpRoom(
        tester,
        timelineOf([], drafts: [draftOf(UploadState.uploaded)]),
      );
      final button = tester.widget<ElevatedButton>(
        find.byKey(const Key('sendMessage')),
      );
      expect(button.onPressed, isNotNull);
    });
  });

  group('rejection labels', () {
    testWidgets('a failed attachment send names the attachment', (
      tester,
    ) async {
      await pumpRoom(
        tester,
        timelineOf([failedEntry(MessageRejection.invalidAttachment)]),
      );
      expect(find.text('Attachment not accepted'), findsOneWidget);
    });
  });

  group('pinning', () {
    for (final (name, canPin, entry, label) in [
      ('allowed on a confirmed entry', true, gracesEntryWith(), 'Pin'),
      (
        'allowed on a pinned entry',
        true,
        gracesEntryWith(pinned: true),
        'Unpin',
      ),
      ('hidden without permission', false, gracesEntryWith(), null),
      ('hidden for a pending entry', true, pendingEntry, null),
    ]) {
      testWidgets('pin button $name', (tester) async {
        await pumpRoom(tester, timelineOf([entry], canPin: canPin));
        expect(
          find.text('Pin'),
          label == 'Pin' ? findsOneWidget : findsNothing,
        );
        expect(
          find.text('Unpin'),
          label == 'Unpin' ? findsOneWidget : findsNothing,
        );
      });
    }

    for (final (pinned, next) in [(false, true), (true, false)]) {
      testWidgets('a pinned=$pinned entry sends pinned=$next', (tester) async {
        final recorded = await pumpRoom(
          tester,
          timelineOf([gracesEntryWith(pinned: pinned)], canPin: true),
        );
        await tester.tap(find.text(pinned ? 'Unpin' : 'Pin'));
        expect(recorded.calls, [('setPin', roomAId, messageAId, next)]);
      });
    }
  });

  group('shelf menu on entries', () {
    for (final (name, shelved, label, absent) in [
      ('unshelved offers saving', false, 'Save to shelf', 'Remove from shelf'),
      ('shelved offers removal', true, 'Remove from shelf', 'Save to shelf'),
    ]) {
      testWidgets('a $name entry', (tester) async {
        await pumpRoom(
          tester,
          timelineOf([gracesEntryWith(shelved: shelved)]),
          presets: allPresets,
        );
        await tester.tap(find.text('Shelf'));
        await tester.pumpAndSettle();
        expect(find.text(label), findsOneWidget);
        expect(find.text(absent), findsNothing);
      });
    }

    for (final (name, label, action, preset) in [
      ('save', 'Save to shelf', ShelfAction.save, null),
      ('a preset', 'Tomorrow', ShelfAction.save, ReminderPreset.tomorrow),
    ]) {
      testWidgets('choosing $name updates the shelf for that message', (
        tester,
      ) async {
        final recorded = await pumpRoom(
          tester,
          timelineOf([gracesEntryWith()]),
          presets: allPresets,
        );
        await tester.tap(find.text('Shelf'));
        await tester.pumpAndSettle();
        await tester.tap(find.text(label));
        await tester.pumpAndSettle();
        expect(recorded.calls, [
          ('updateShelf', roomAId, messageAId, action, preset, null),
        ]);
      });
    }

    testWidgets('the entry menu passes remindAtMs built from the anchor', (
      tester,
    ) async {
      final recorded = await pumpRoom(
        tester,
        timelineOf([gracesEntryWith()]),
        presets: allPresets,
        pickerAnchorMs: pickerAnchor,
        pickerLatestMs: pickerLatest,
      );
      await tester.tap(find.text('Shelf'));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const Key('shelfCustomTime')));
      await tester.pumpAndSettle();
      await tester.tap(find.text('OK'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('OK'));
      await tester.pumpAndSettle();
      expect(recorded.calls, [
        (
          'updateShelf',
          roomAId,
          messageAId,
          ShelfAction.save,
          null,
          pickerAnchor,
        ),
      ]);
    });

    testWidgets('pending entries have no shelf menu', (tester) async {
      await pumpRoom(tester, timelineOf([pendingEntry]), presets: allPresets);
      expect(find.byKey(const Key('shelfMenu')), findsNothing);
    });
  });

  group('focus', () {
    testWidgets('the focused entry is scrolled into view', (tester) async {
      final entries = [
        for (var index = 0; index < 30; index++)
          gracesEntryWith(
            messageId: index == 29 ? messageBId : messageIdAt(index),
            body: 'entry $index',
          ),
      ];
      await pumpRoom(tester, timelineOf(entries, focusMessageId: messageBId));
      await tester.pumpAndSettle();
      final bottom = tester.getBottomLeft(find.text('entry 29')).dy;
      expect(
        bottom,
        lessThan(
          tester.view.physicalSize.height / tester.view.devicePixelRatio,
        ),
      );
    });

    testWidgets('without a focus the list stays at the top', (tester) async {
      final entries = [
        for (var index = 0; index < 30; index++)
          gracesEntryWith(
            messageId: index == 29 ? messageBId : messageIdAt(index),
            body: 'entry $index',
          ),
      ];
      await pumpRoom(tester, timelineOf(entries));
      await tester.pumpAndSettle();
      final bottom = tester.getBottomLeft(find.text('entry 29')).dy;
      expect(
        bottom,
        greaterThan(
          tester.view.physicalSize.height / tester.view.devicePixelRatio,
        ),
      );
    });
  });
}
