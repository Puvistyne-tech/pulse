import 'package:material_ui/material_ui.dart';

class PulseSpacing {
  const PulseSpacing._();

  static const double xs = 4;
  static const double sm = 8;
  static const double md = 12;
  static const double lg = 16;
  static const double xl = 24;
}

class PulseRadius {
  const PulseRadius._();

  static const double control = 8;
  static const double card = 12;
  static const double pill = 999;
}

class PulseTokens extends ThemeExtension<PulseTokens> {
  const PulseTokens({
    required this.background,
    required this.surface,
    required this.surfaceMuted,
    required this.border,
    required this.text,
    required this.textMuted,
    required this.accent,
    required this.accentHover,
    required this.accentSoft,
    required this.onAccent,
    required this.danger,
    required this.warning,
    required this.success,
    required this.sidebar,
    required this.sidebarText,
    required this.sidebarMuted,
    required this.rail,
  });

  final Color background;
  final Color surface;
  final Color surfaceMuted;
  final Color border;
  final Color text;
  final Color textMuted;
  final Color accent;
  final Color accentHover;
  final Color accentSoft;
  final Color onAccent;
  final Color danger;
  final Color warning;
  final Color success;
  final Color sidebar;
  final Color sidebarText;
  final Color sidebarMuted;
  final Color rail;

  static const light = PulseTokens(
    background: Color(0xFFFFFFFF),
    surface: Color(0xFFFFFFFF),
    surfaceMuted: Color(0xFFF6F7F9),
    border: Color(0xFFE4E7EC),
    text: Color(0xFF101828),
    textMuted: Color(0xFF667085),
    accent: Color(0xFF0F9F8F),
    accentHover: Color(0xFF0B8577),
    accentSoft: Color(0xFFE6F6F4),
    onAccent: Color(0xFFFFFFFF),
    danger: Color(0xFFD92D20),
    warning: Color(0xFFDC6803),
    success: Color(0xFF079455),
    sidebar: Color(0xFF10162A),
    sidebarText: Color(0xFFC8CFDC),
    sidebarMuted: Color(0xFF7D879C),
    rail: Color(0xFF0A0F1E),
  );

  static const dark = PulseTokens(
    background: Color(0xFF0E1220),
    surface: Color(0xFF141A2B),
    surfaceMuted: Color(0xFF1B2236),
    border: Color(0xFF263049),
    text: Color(0xFFE6E9F0),
    textMuted: Color(0xFF98A2B3),
    accent: Color(0xFF2BC4B2),
    accentHover: Color(0xFF4FD6C6),
    accentSoft: Color(0x242BC4B2),
    onAccent: Color(0xFF04201C),
    danger: Color(0xFFF97066),
    warning: Color(0xFFFDB022),
    success: Color(0xFF47CD89),
    sidebar: Color(0xFF0A0E1A),
    sidebarText: Color(0xFFC8CFDC),
    sidebarMuted: Color(0xFF7D879C),
    rail: Color(0xFF070A14),
  );

  static PulseTokens of(BuildContext context) {
    final theme = Theme.of(context);
    return theme.extension<PulseTokens>() ??
        (theme.brightness == Brightness.dark ? dark : light);
  }

  @override
  PulseTokens copyWith({
    Color? background,
    Color? surface,
    Color? surfaceMuted,
    Color? border,
    Color? text,
    Color? textMuted,
    Color? accent,
    Color? accentHover,
    Color? accentSoft,
    Color? onAccent,
    Color? danger,
    Color? warning,
    Color? success,
    Color? sidebar,
    Color? sidebarText,
    Color? sidebarMuted,
    Color? rail,
  }) => PulseTokens(
    background: background ?? this.background,
    surface: surface ?? this.surface,
    surfaceMuted: surfaceMuted ?? this.surfaceMuted,
    border: border ?? this.border,
    text: text ?? this.text,
    textMuted: textMuted ?? this.textMuted,
    accent: accent ?? this.accent,
    accentHover: accentHover ?? this.accentHover,
    accentSoft: accentSoft ?? this.accentSoft,
    onAccent: onAccent ?? this.onAccent,
    danger: danger ?? this.danger,
    warning: warning ?? this.warning,
    success: success ?? this.success,
    sidebar: sidebar ?? this.sidebar,
    sidebarText: sidebarText ?? this.sidebarText,
    sidebarMuted: sidebarMuted ?? this.sidebarMuted,
    rail: rail ?? this.rail,
  );

  @override
  PulseTokens lerp(ThemeExtension<PulseTokens>? other, double t) {
    if (other is! PulseTokens) {
      return this;
    }
    Color mix(Color from, Color to) => Color.lerp(from, to, t)!;
    return PulseTokens(
      background: mix(background, other.background),
      surface: mix(surface, other.surface),
      surfaceMuted: mix(surfaceMuted, other.surfaceMuted),
      border: mix(border, other.border),
      text: mix(text, other.text),
      textMuted: mix(textMuted, other.textMuted),
      accent: mix(accent, other.accent),
      accentHover: mix(accentHover, other.accentHover),
      accentSoft: mix(accentSoft, other.accentSoft),
      onAccent: mix(onAccent, other.onAccent),
      danger: mix(danger, other.danger),
      warning: mix(warning, other.warning),
      success: mix(success, other.success),
      sidebar: mix(sidebar, other.sidebar),
      sidebarText: mix(sidebarText, other.sidebarText),
      sidebarMuted: mix(sidebarMuted, other.sidebarMuted),
      rail: mix(rail, other.rail),
    );
  }
}

OutlineInputBorder _outline(Color color, {double width = 1}) =>
    OutlineInputBorder(
      borderRadius: BorderRadius.circular(PulseRadius.control),
      borderSide: BorderSide(color: color, width: width),
    );

ThemeData _build(PulseTokens tokens, Brightness brightness) {
  final scheme = ColorScheme(
    brightness: brightness,
    primary: tokens.accent,
    onPrimary: tokens.onAccent,
    primaryContainer: tokens.accentSoft,
    onPrimaryContainer: tokens.accent,
    secondary: tokens.accent,
    onSecondary: tokens.onAccent,
    error: tokens.danger,
    onError: tokens.onAccent,
    surface: tokens.surface,
    onSurface: tokens.text,
    onSurfaceVariant: tokens.textMuted,
    outline: tokens.border,
    outlineVariant: tokens.border,
    surfaceContainerHighest: tokens.surfaceMuted,
    surfaceTint: const Color(0x00000000),
  );
  final shape = RoundedRectangleBorder(
    borderRadius: BorderRadius.circular(PulseRadius.control),
  );
  final base = ThemeData(
    useMaterial3: true,
    brightness: brightness,
    colorScheme: scheme,
  );
  return base.copyWith(
    scaffoldBackgroundColor: tokens.background,
    canvasColor: tokens.background,
    dividerColor: tokens.border,
    textTheme: base.textTheme.apply(
      bodyColor: tokens.text,
      displayColor: tokens.text,
    ),
    extensions: [tokens],
    appBarTheme: AppBarThemeData(
      backgroundColor: tokens.background,
      foregroundColor: tokens.text,
      elevation: 0,
      scrolledUnderElevation: 0,
      surfaceTintColor: const Color(0x00000000),
      shape: Border(bottom: BorderSide(color: tokens.border)),
      titleTextStyle: base.textTheme.titleMedium?.copyWith(
        color: tokens.text,
        fontWeight: FontWeight.w700,
      ),
    ),
    dividerTheme: DividerThemeData(
      color: tokens.border,
      space: 1,
      thickness: 1,
    ),
    drawerTheme: DrawerThemeData(
      backgroundColor: tokens.sidebar,
      surfaceTintColor: const Color(0x00000000),
      shape: const RoundedRectangleBorder(),
    ),
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: tokens.surfaceMuted,
      isDense: true,
      contentPadding: const EdgeInsets.symmetric(
        horizontal: PulseSpacing.md,
        vertical: PulseSpacing.md,
      ),
      border: _outline(tokens.border),
      enabledBorder: _outline(tokens.border),
      focusedBorder: _outline(tokens.accent, width: 1.5),
      errorBorder: _outline(tokens.danger),
      focusedErrorBorder: _outline(tokens.danger, width: 1.5),
      labelStyle: TextStyle(color: tokens.textMuted),
      helperStyle: TextStyle(color: tokens.textMuted),
      helperMaxLines: 3,
      hintStyle: TextStyle(color: tokens.textMuted),
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        backgroundColor: tokens.accent,
        foregroundColor: tokens.onAccent,
        disabledBackgroundColor: tokens.surfaceMuted,
        disabledForegroundColor: tokens.textMuted,
        elevation: 0,
        minimumSize: const Size(64, 44),
        shape: shape,
        textStyle: const TextStyle(fontWeight: FontWeight.w600),
      ),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        backgroundColor: tokens.accent,
        foregroundColor: tokens.onAccent,
        shape: shape,
      ),
    ),
    textButtonTheme: TextButtonThemeData(
      style: TextButton.styleFrom(
        foregroundColor: tokens.accent,
        shape: shape,
        visualDensity: VisualDensity.compact,
      ),
    ),
    iconButtonTheme: IconButtonThemeData(
      style: IconButton.styleFrom(
        foregroundColor: tokens.textMuted,
        shape: shape,
      ),
    ),
    chipTheme: ChipThemeData(
      backgroundColor: tokens.surfaceMuted,
      selectedColor: tokens.accentSoft,
      side: BorderSide(color: tokens.border),
      shape: const StadiumBorder(),
      showCheckmark: false,
      labelStyle: TextStyle(color: tokens.text, fontSize: 13),
      padding: EdgeInsets.zero,
    ),
    badgeTheme: BadgeThemeData(
      backgroundColor: tokens.accent,
      textColor: tokens.onAccent,
    ),
    popupMenuTheme: PopupMenuThemeData(
      color: tokens.surface,
      surfaceTintColor: const Color(0x00000000),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(PulseRadius.card),
        side: BorderSide(color: tokens.border),
      ),
    ),
    tooltipTheme: TooltipThemeData(
      decoration: BoxDecoration(
        color: tokens.sidebar,
        borderRadius: BorderRadius.circular(PulseRadius.control),
      ),
      textStyle: TextStyle(color: tokens.sidebarText, fontSize: 12),
    ),
  );
}

ThemeData pulseLightTheme() => _build(PulseTokens.light, Brightness.light);

ThemeData pulseDarkTheme() => _build(PulseTokens.dark, Brightness.dark);
