import '../src/rust/third_party/pulse_protocol/api.dart';

const _invalidFieldMessages = {
  InputField.email: 'Enter a valid email address',
  InputField.displayName: 'Enter a valid display name',
  InputField.password: 'Enter a valid password',
  InputField.slug: 'Use lowercase letters, digits and hyphens for the slug',
  InputField.messageBody: 'Message is empty or too long',
  InputField.topicTitle:
      'Enter a shorter topic title without control characters',
  InputField.reactionEmoji: 'Choose a single emoji reaction',
  InputField.fileName: 'Choose a file with a valid name',
  InputField.contentType: 'That file type cannot be used',
  InputField.byteSize: 'That file is empty or too large',
  InputField.attachments: 'Check the attachments on that message',
  InputField.remindAt:
      'Choose a reminder time that is not too far in the past or future',
  InputField.rooms: 'Choose a valid set of rooms for that steward',
};

const _conflictFieldMessages = {
  InputField.email: 'That email is already registered',
  InputField.displayName: 'That display name is already taken',
  InputField.password: 'That password cannot be used',
  InputField.slug: 'That slug is already taken',
  InputField.messageBody: 'That message cannot be used',
  InputField.topicTitle: 'That topic title cannot be used',
  InputField.reactionEmoji: 'You have reacted with enough different emoji',
  InputField.fileName: 'That file name cannot be used',
  InputField.contentType: 'That file type cannot be used',
  InputField.byteSize: 'That file cannot be used',
  InputField.attachments: 'Those attachments cannot be used',
  InputField.remindAt: 'That reminder time cannot be used',
  InputField.rooms: 'Those rooms cannot be used',
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
