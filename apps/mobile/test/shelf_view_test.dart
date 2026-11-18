import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/shelf/reminder_banner.dart';
import 'package:pulse_mobile/shelf/shelf_menu.dart';
import 'package:pulse_mobile/shelf/shelf_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/shelf.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';

import 'fixtures.dart';

Future<RecordedIntents> pumpShelf(
  WidgetTester tester,
  ShelfSnapshot shelf,
) async {
  final recorded = RecordedIntents();
  await pumpView(
    tester,
    SingleChildScrollView(
      child: ShelfView(shelf: shelf, intents: recorded.intents),
    ),
  );
  return recorded;
}

final fullShelf = shelfOf(
  due: [
    shelfEntryOf(ShelfBucket.due, messageBId),
    shelfEntryOf(ShelfBucket.due, messageAId),
  ],
  upcoming: [shelfEntryOf(ShelfBucket.upcoming, messageCId)],
  done: [shelfEntryOf(ShelfBucket.done, messageDId)],
);

class MenuCase {
  const MenuCase(this.name, this.shelved, this.presets, this.labels);

  final String name;
  final bool shelved;
  final List<ReminderPreset> presets;
  final List<String> labels;
}

final menuCases = <MenuCase>[
  MenuCase('unshelved with every preset', false, allPresets, [
    'Save to shelf',
    ...allPresets.map((preset) => presetLabels[preset]!),
  ]),
  MenuCase('shelved with every preset', true, allPresets, [
    ...allPresets.map((preset) => presetLabels[preset]!),
    'Remove from shelf',
  ]),
  MenuCase('unshelved without presets', false, const [], ['Save to shelf']),
  MenuCase('shelved without presets', true, const [], ['Remove from shelf']),
  MenuCase(
    'a subset of presets',
    false,
    const [ReminderPreset.inOneHour, ReminderPreset.tomorrow],
    ['Save to shelf', 'In one hour', 'Tomorrow'],
  ),
];

class ChoiceCase {
  const ChoiceCase(this.name, this.shelved, this.label, this.expected);

  final String name;
  final bool shelved;
  final String label;
  final (ShelfAction, ReminderPreset?) expected;
}

const choiceCases = <ChoiceCase>[
  ChoiceCase('saving an unshelved message', false, 'Save to shelf', (
    ShelfAction.save,
    null,
  )),
  ChoiceCase('removing', true, 'Remove from shelf', (ShelfAction.remove, null)),
  ChoiceCase('a preset on an unshelved message', false, 'In one hour', (
    ShelfAction.save,
    ReminderPreset.inOneHour,
  )),
  ChoiceCase('a preset on a shelved message', true, 'Next week', (
    ShelfAction.reschedule,
    ReminderPreset.nextWeek,
  )),
];

Future<void> pumpMenu(
  WidgetTester tester, {
  required bool shelved,
  required List<ReminderPreset> presets,
  required List<Object> calls,
}) => pumpView(
  tester,
  Center(
    child: ShelfMenu(
      presets: presets,
      shelved: shelved,
      onUpdate: (action, preset) async => calls.add((action, preset)),
    ),
  ),
);

void main() {
  group('sections', () {
    testWidgets('populated sections render in order', (tester) async {
      await pumpShelf(tester, fullShelf);
      final tops = [
        'Due',
        'Upcoming',
        'Done',
      ].map((label) => tester.getTopLeft(find.text(label)).dy).toList();
      expect(tops, orderedEquals([...tops]..sort()));
    });

    for (final (label, previews) in [
      ('Due', ['shelved e2', 'shelved e1']),
      ('Upcoming', ['shelved e3']),
      ('Done', ['shelved e4']),
    ]) {
      testWidgets('$label lists its entries in snapshot order', (tester) async {
        await pumpShelf(tester, fullShelf);
        for (final preview in previews) {
          expect(find.text('Grace: $preview'), findsOneWidget);
        }
        final tops = [
          for (final preview in previews)
            tester.getTopLeft(find.text('Grace: $preview')).dy,
        ];
        expect(tops, orderedEquals([...tops]..sort()));
      });
    }

    testWidgets('empty sections are omitted', (tester) async {
      await pumpShelf(tester, shelfOf());
      for (final label in ['Due', 'Upcoming', 'Done']) {
        expect(find.text(label), findsNothing);
      }
    });

    testWidgets('the header shows the due count', (tester) async {
      await pumpShelf(tester, fullShelf);
      expect(find.text('Shelf 2'), findsOneWidget);
    });

    for (final (name, error, notice) in [
      ('no error', null, null),
      (
        'unavailable',
        const ApiError(code: ApiErrorCode.unavailable),
        'Pulse server unavailable',
      ),
      (
        'invalid reminder time',
        const ApiError(
          code: ApiErrorCode.invalidInput,
          field: InputField.remindAt,
        ),
        'Choose a reminder time that is not too far in the past or future',
      ),
    ]) {
      testWidgets('error $name', (tester) async {
        await pumpShelf(tester, shelfOf(error: error));
        for (final text in [
          'Pulse server unavailable',
          'Choose a reminder time that is not too far in the past or future',
        ]) {
          expect(
            find.text(text),
            text == notice ? findsOneWidget : findsNothing,
          );
        }
      });
    }

    testWidgets('the close button toggles the shelf', (tester) async {
      final recorded = await pumpShelf(tester, fullShelf);
      await tester.tap(find.text('Close shelf'));
      expect(recorded.calls, ['toggleShelf']);
    });
  });

  group('entry actions', () {
    for (final (label, message, button, action) in [
      ('Due', messageBId, 'Complete', ShelfAction.complete),
      ('Upcoming', messageCId, 'Complete', ShelfAction.complete),
      ('Done', messageDId, 'Reopen', ShelfAction.reopen),
    ]) {
      testWidgets('$label offers $button', (tester) async {
        final recorded = await pumpShelf(tester, fullShelf);
        final row = find.byKey(ValueKey(message));
        await tester.tap(find.descendant(of: row, matching: find.text(button)));
        expect(recorded.calls, [
          ('updateShelf', roomAId, message, action, null, null),
        ]);
        final other = button == 'Complete' ? 'Reopen' : 'Complete';
        expect(
          find.descendant(of: row, matching: find.text(other)),
          findsNothing,
        );
      });
    }

    testWidgets('a pending entry cannot be completed', (tester) async {
      final recorded = await pumpShelf(
        tester,
        shelfOf(
          upcoming: [
            shelfEntryOf(ShelfBucket.upcoming, messageAId, pending: true),
          ],
        ),
      );
      final button = tester.widget<TextButton>(
        find.widgetWithText(TextButton, 'Complete'),
      );
      expect(button.onPressed, isNull);
      expect(recorded.calls, isEmpty);
    });
  });

  group('shelf menu', () {
    for (final fixture in menuCases) {
      testWidgets(fixture.name, (tester) async {
        final calls = <Object>[];
        await pumpMenu(
          tester,
          shelved: fixture.shelved,
          presets: fixture.presets,
          calls: calls,
        );
        await tester.tap(find.text('Shelf'));
        await tester.pumpAndSettle();
        final items = tester
            .widgetList<PopupMenuItem<(ShelfAction, ReminderPreset?)>>(
              find.byType(PopupMenuItem<(ShelfAction, ReminderPreset?)>),
            )
            .map((item) => (item.child as Text).data)
            .toList();
        expect(items, fixture.labels);
      });
    }

    for (final fixture in choiceCases) {
      testWidgets('choosing ${fixture.name} reports it', (tester) async {
        final calls = <Object>[];
        await pumpMenu(
          tester,
          shelved: fixture.shelved,
          presets: allPresets,
          calls: calls,
        );
        await tester.tap(find.text('Shelf'));
        await tester.pumpAndSettle();
        await tester.tap(find.text(fixture.label));
        await tester.pumpAndSettle();
        expect(calls, [fixture.expected]);
      });
    }

    testWidgets('entries in the shelf view reschedule their own message', (
      tester,
    ) async {
      final recorded = await pumpShelf(
        tester,
        shelfOf(upcoming: [shelfEntryOf(ShelfBucket.upcoming, messageCId)]),
      );
      await tester.tap(find.text('Shelf').last);
      await tester.pumpAndSettle();
      await tester.tap(find.text('In three hours'));
      await tester.pumpAndSettle();
      expect(recorded.calls, [
        (
          'updateShelf',
          roomAId,
          messageCId,
          ShelfAction.reschedule,
          ReminderPreset.inThreeHours,
          null,
        ),
      ]);
    });
  });

  group('reminder banner', () {
    testWidgets('shows the author and preview and dismisses', (tester) async {
      var dismissed = 0;
      await pumpView(
        tester,
        ReminderBanner(
          entry: shelfEntryOf(
            ShelfBucket.due,
            messageAId,
            preview: 'call back',
          ),
          dismiss: () async => dismissed += 1,
        ),
      );
      expect(find.text('Grace: call back'), findsOneWidget);
      await tester.tap(find.text('Dismiss'));
      expect(dismissed, 1);
    });
  });
}
