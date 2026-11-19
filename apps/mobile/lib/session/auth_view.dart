import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../shell/panel_card.dart';
import '../src/rust/third_party/pulse_protocol/api.dart';
import '../src/rust/third_party/pulse_protocol/auth.dart';
import '../theme/pulse_theme.dart';
import 'error_notice.dart';

class AuthView extends StatefulWidget {
  const AuthView({
    required this.busy,
    required this.error,
    required this.intents,
    super.key,
  });

  final bool busy;
  final ApiError? error;
  final PulseIntents intents;

  @override
  State<AuthView> createState() => _AuthViewState();
}

class _AuthViewState extends State<AuthView> {
  final _email = TextEditingController();
  final _displayName = TextEditingController();
  final _password = TextEditingController();
  var _signingUp = false;

  @override
  void dispose() {
    _email.dispose();
    _displayName.dispose();
    _password.dispose();
    super.dispose();
  }

  void _submit() {
    if (_signingUp) {
      widget.intents.signUp(
        SignUpRequest(
          email: _email.text,
          displayName: _displayName.text,
          password: _password.text,
        ),
      );
    } else {
      widget.intents.signIn(
        SignInRequest(email: _email.text, password: _password.text),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final submitLabel = _signingUp ? 'Create account' : 'Sign in';
    return Center(
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(PulseSpacing.lg),
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: PanelSurface(
            child: Padding(
              padding: const EdgeInsets.all(PulseSpacing.xl),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.bolt, color: tokens.accent, size: 28),
                      const SizedBox(width: PulseSpacing.xs),
                      Text(
                        'Pulse',
                        style: Theme.of(context).textTheme.headlineSmall
                            ?.copyWith(
                              color: tokens.text,
                              fontWeight: FontWeight.w800,
                            ),
                      ),
                    ],
                  ),
                  const SizedBox(height: PulseSpacing.xl),
                  TextField(
                    key: const Key('email'),
                    controller: _email,
                    keyboardType: TextInputType.emailAddress,
                    decoration: const InputDecoration(labelText: 'Email'),
                  ),
                  if (_signingUp) ...[
                    const SizedBox(height: PulseSpacing.md),
                    TextField(
                      key: const Key('displayName'),
                      controller: _displayName,
                      decoration: const InputDecoration(
                        labelText: 'Display name',
                      ),
                    ),
                  ],
                  const SizedBox(height: PulseSpacing.md),
                  TextField(
                    key: const Key('password'),
                    controller: _password,
                    obscureText: true,
                    decoration: const InputDecoration(labelText: 'Password'),
                  ),
                  ErrorNotice(error: widget.error),
                  const SizedBox(height: PulseSpacing.lg),
                  ElevatedButton(
                    key: const Key('submit'),
                    onPressed: widget.busy ? null : _submit,
                    child: Text(submitLabel),
                  ),
                  const SizedBox(height: PulseSpacing.sm),
                  TextButton(
                    key: const Key('toggleMode'),
                    onPressed: () => setState(() => _signingUp = !_signingUp),
                    child: Text(
                      _signingUp
                          ? 'Have an account? Sign in'
                          : 'New here? Create account',
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
