import 'package:material_ui/material_ui.dart';

import '../client/pulse_intents.dart';
import '../src/rust/third_party/pulse_protocol/api.dart';
import '../src/rust/third_party/pulse_protocol/auth.dart';
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
    final submitLabel = _signingUp ? 'Create account' : 'Sign in';
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('Pulse'),
          TextField(
            key: const Key('email'),
            controller: _email,
            keyboardType: TextInputType.emailAddress,
            decoration: const InputDecoration(labelText: 'Email'),
          ),
          if (_signingUp)
            TextField(
              key: const Key('displayName'),
              controller: _displayName,
              decoration: const InputDecoration(labelText: 'Display name'),
            ),
          TextField(
            key: const Key('password'),
            controller: _password,
            obscureText: true,
            decoration: const InputDecoration(labelText: 'Password'),
          ),
          ErrorNotice(error: widget.error),
          ElevatedButton(
            key: const Key('submit'),
            onPressed: widget.busy ? null : _submit,
            child: Text(submitLabel),
          ),
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
    );
  }
}
