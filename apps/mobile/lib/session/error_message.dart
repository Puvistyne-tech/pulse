import '../src/rust/third_party/pulse_protocol/api.dart';

const _invalidFieldMessages = {
  InputField.email: 'Enter a valid email address',
  InputField.displayName: 'Enter a valid display name',
  InputField.password: 'Enter a valid password',
  InputField.slug: 'Use lowercase letters, digits and hyphens for the slug',
  InputField.messageBody: 'Write a message of up to 4000 characters',
};

const _conflictFieldMessages = {
  InputField.email: 'That email is already registered',
  InputField.displayName: 'That display name is already taken',
  InputField.password: 'That password cannot be used',
  InputField.slug: 'That slug is already taken',
  InputField.messageBody: 'That message was already sent',
};

const _codeMessages = {
  ApiErrorCode.invalidInput: 'Check the details you entered',
  ApiErrorCode.unauthenticated: 'Sign in again to continue',
  ApiErrorCode.forbidden: 'You are not allowed to do that',
  ApiErrorCode.notFound: 'That item could not be found',
  ApiErrorCode.conflict: 'That already exists',
  ApiErrorCode.unavailable: 'Pulse server unavailable',
  ApiErrorCode.internal: 'Something went wrong',
};

String errorMessage(ApiError error) {
  final field = error.field;
  if (field != null) {
    if (error.code == ApiErrorCode.invalidInput) {
      return _invalidFieldMessages[field]!;
    }
    if (error.code == ApiErrorCode.conflict) {
      return _conflictFieldMessages[field]!;
    }
  }
  return _codeMessages[error.code]!;
}
