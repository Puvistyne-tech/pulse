import 'package:flutter_test/flutter_test.dart';
import 'package:material_ui/material_ui.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/api.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/media.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/snapshot.dart';
import 'package:pulse_mobile/vault/vault_view.dart';

import 'fixtures.dart';

Future<RecordedIntents> pumpVault(
  WidgetTester tester,
  VaultSnapshot vault,
) async {
  final recorded = RecordedIntents();
  await pumpView(
    tester,
    SingleChildScrollView(
      child: VaultView(vault: vault, intents: recorded.intents),
    ),
  );
  return recorded;
}

const tabLabels = <VaultTab, String>{
  VaultTab.media: 'Media',
  VaultTab.files: 'Files',
  VaultTab.links: 'Links',
  VaultTab.pinned: 'Pinned',
};

bool isBold(WidgetTester tester, String label) =>
    tester.widget<Text>(find.text(label)).style?.fontWeight == FontWeight.bold;

void main() {
  group('tabs', () {
    testWidgets('the four tabs render in order', (tester) async {
      await pumpVault(tester, vaultOf([]));
      final lefts = tabLabels.values
          .map((label) => tester.getTopLeft(find.text(label)).dx)
          .toList();
      expect(lefts, orderedEquals([...lefts]..sort()));
    });

    for (final current in VaultTab.values) {
      testWidgets('only ${tabLabels[current]} is bold when current', (
        tester,
      ) async {
        await pumpVault(tester, vaultOf([], tab: current));
        expect(
          [for (final label in tabLabels.values) isBold(tester, label)],
          [for (final tab in tabLabels.keys) tab == current],
        );
      });
    }

    for (final entry in tabLabels.entries) {
      testWidgets('tapping ${entry.value} opens that tab in the room', (
        tester,
      ) async {
        final recorded = await pumpVault(tester, vaultOf([]));
        await tester.tap(find.text(entry.value));
        expect(recorded.calls, [('openVault', roomAId, entry.key)]);
      });
    }

    testWidgets('tabs are disabled without a room', (tester) async {
      final recorded = await pumpVault(tester, vaultOf([], withRoom: false));
      for (final tab in tabLabels.keys) {
        final button = tester.widget<TextButton>(find.byKey(ValueKey(tab)));
        expect(button.onPressed, isNull);
      }
      expect(recorded.calls, isEmpty);
    });
  });

  group('entries', () {
    testWidgets('entries render in snapshot order with their author', (
      tester,
    ) async {
      await pumpVault(
        tester,
        vaultOf([
          vaultEntryOf(
            VaultTab.links,
            messageBId,
            link: 'https://example.org/b',
          ),
          vaultEntryOf(
            VaultTab.links,
            messageAId,
            link: 'https://example.org/a',
          ),
        ]),
      );
      final b = tester.getTopLeft(find.text('https://example.org/b')).dy;
      final a = tester.getTopLeft(find.text('https://example.org/a')).dy;
      expect(b, lessThan(a));
      expect(find.text('Grace'), findsNWidgets(2));
    });

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
      ('an image without a ticket', AttachmentKind.image, null, 'named'),
      ('audio without a ticket', AttachmentKind.audio, null, 'named'),
    ]) {
      testWidgets('$name is listed by file name', (tester) async {
        await pumpVault(
          tester,
          vaultOf([
            vaultEntryOf(
              VaultTab.media,
              messageAId,
              attachment: attachmentViewOf(kind, fileName: 'named', url: url),
            ),
          ]),
        );
        expect(find.text(shown), findsOneWidget);
        expect(find.byType(Image), findsNothing);
      });
    }

    testWidgets('an entry without attachment or link shows its preview', (
      tester,
    ) async {
      await pumpVault(
        tester,
        vaultOf([
          vaultEntryOf(VaultTab.pinned, messageCId, preview: 'keep this'),
        ]),
      );
      expect(find.text('keep this'), findsOneWidget);
    });

    testWidgets('a link entry shows the link instead of the preview', (
      tester,
    ) async {
      await pumpVault(
        tester,
        vaultOf([
          vaultEntryOf(
            VaultTab.links,
            messageCId,
            link: 'https://example.org/c',
            preview: 'hidden preview',
          ),
        ]),
      );
      expect(find.text('https://example.org/c'), findsOneWidget);
      expect(find.text('hidden preview'), findsNothing);
    });
  });

  group('status and paging', () {
    for (final (name, loading, hasMore) in [
      ('loading', true, false),
      ('idle', false, false),
      ('more pages', false, true),
      ('loading with more pages', true, true),
    ]) {
      testWidgets(name, (tester) async {
        await pumpVault(
          tester,
          vaultOf([], loading: loading, hasMore: hasMore),
        );
        expect(find.text('Loading'), loading ? findsOneWidget : findsNothing);
        expect(find.text('Load more'), hasMore ? findsOneWidget : findsNothing);
        if (hasMore) {
          final button = tester.widget<TextButton>(
            find.widgetWithText(TextButton, 'Load more'),
          );
          expect(button.onPressed == null, loading);
        }
      });
    }

    testWidgets('load more asks for the next page', (tester) async {
      final recorded = await pumpVault(tester, vaultOf([], hasMore: true));
      await tester.tap(find.text('Load more'));
      expect(recorded.calls, ['loadMoreVault']);
    });

    for (final (name, error, notice) in [
      ('no error', null, null),
      (
        'forbidden',
        const ApiError(code: ApiErrorCode.forbidden),
        'You are not allowed to do that',
      ),
      (
        'unknown room',
        const ApiError(code: ApiErrorCode.notFound),
        'That item could not be found',
      ),
    ]) {
      testWidgets('error $name', (tester) async {
        await pumpVault(tester, vaultOf([], error: error));
        for (final text in [
          'You are not allowed to do that',
          'That item could not be found',
        ]) {
          expect(
            find.text(text),
            text == notice ? findsOneWidget : findsNothing,
          );
        }
      });
    }

    testWidgets('the close button closes the vault', (tester) async {
      final recorded = await pumpVault(tester, vaultOf([]));
      await tester.tap(tooltip('Close vault'));
      expect(recorded.calls, ['closeVault']);
    });
  });
}
