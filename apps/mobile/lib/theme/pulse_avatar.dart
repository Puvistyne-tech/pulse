import 'package:material_ui/material_ui.dart';

import 'pulse_theme.dart';

const _avatarPalette = <Color>[
  Color(0xFF5B6CC9),
  Color(0xFF2F8F83),
  Color(0xFFB7791F),
  Color(0xFFC2527A),
  Color(0xFF2B80B9),
  Color(0xFF8B5CC6),
  Color(0xFF5E8C31),
  Color(0xFFC46A2D),
];

String initialsOf(String name) {
  final words = name
      .trim()
      .split(RegExp(r'\s+'))
      .where((word) => word.isNotEmpty)
      .toList();
  if (words.isEmpty) {
    return '?';
  }
  final letters = words.length == 1
      ? words.first.characters.first
      : words.first.characters.first + words.last.characters.first;
  return letters.toUpperCase();
}

Color avatarColorFor(String name) {
  final hash = name.codeUnits.fold<int>(
    7,
    (value, unit) => (value * 31 + unit) & 0x7fffffff,
  );
  return _avatarPalette[hash % _avatarPalette.length];
}

class PulseAvatar extends StatelessWidget {
  const PulseAvatar({
    required this.name,
    this.size = 36,
    this.square = false,
    this.highlighted = false,
    super.key,
  });

  final String name;
  final double size;
  final bool square;
  final bool highlighted;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final radius = BorderRadius.circular(
      square ? PulseRadius.control : size / 2,
    );
    return Container(
      width: size,
      height: size,
      alignment: Alignment.center,
      decoration: BoxDecoration(
        color: avatarColorFor(name),
        borderRadius: radius,
        border: highlighted ? Border.all(color: tokens.accent, width: 2) : null,
      ),
      child: Text(
        initialsOf(name),
        style: TextStyle(
          color: const Color(0xFFFFFFFF),
          fontSize: size * 0.38,
          fontWeight: FontWeight.w700,
        ),
      ),
    );
  }
}
