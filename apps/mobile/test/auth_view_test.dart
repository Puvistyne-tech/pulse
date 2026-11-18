import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pulse_mobile/session/auth_view.dart';
import 'package:pulse_mobile/src/rust/third_party/pulse_protocol/auth.dart';

import 'fixtures.dart';

class AuthCase {
  const AuthCase(this.name, this.signUp, this.fields, this.expected);

  final String name;
  final bool signUp;
  final Map<String, String> fields;
  final Object expected;
}

final authCases = <AuthCase>[
  AuthCase('sign in exactly as typed', false, {
    'email': 'Ada@Example.test',
    'password': ' p4ss word ',
  }, const SignInRequest(email: 'Ada@Example.test', password: ' p4ss word ')),
  AuthCase(
    'sign up exactly as typed',
    true,
    {
      'email': 'ada@example.test',
      'displayName': '  Ada ',
      'password': 'correct-horse-battery',
    },
    const SignUpRequest(
      email: 'ada@example.test',
      displayName: '  Ada ',
      password: 'correct-horse-battery',
    ),
  ),
];

void main() {
  group('AuthView', () {
    for (final fixture in authCases) {
      testWidgets(fixture.name, (tester) async {
        final recorded = RecordedIntents();
        await pumpView(
          tester,
          AuthView(busy: false, error: null, intents: recorded.intents),
        );
        if (fixture.signUp) {
          await tester.tap(find.byKey(const Key('toggleMode')));
          await tester.pump();
        }
        for (final entry in fixture.fields.entries) {
          await tester.enterText(find.byKey(Key(entry.key)), entry.value);
        }
        await tester.tap(find.byKey(const Key('submit')));
        expect(recorded.calls, [fixture.expected]);
      });
    }

    for (final busy in [true, false]) {
      testWidgets('busy $busy toggles submit', (tester) async {
        final recorded = RecordedIntents();
        await pumpView(
          tester,
          AuthView(busy: busy, error: null, intents: recorded.intents),
        );
        await tester.tap(find.byKey(const Key('submit')));
        expect(recorded.calls.length, busy ? 0 : 1);
      });
    }
  });
}
