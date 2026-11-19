import 'package:material_ui/material_ui.dart';

import '../theme/pulse_theme.dart';

class SidebarHeading extends StatelessWidget {
  const SidebarHeading({required this.label, this.trailing, super.key});

  final String label;
  final Widget? trailing;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Padding(
      padding: const EdgeInsets.fromLTRB(
        PulseSpacing.lg,
        PulseSpacing.lg,
        PulseSpacing.sm,
        PulseSpacing.xs,
      ),
      child: Row(
        children: [
          Expanded(
            child: Text(
              label,
              style: TextStyle(
                color: tokens.sidebarMuted,
                fontSize: 12,
                fontWeight: FontWeight.w700,
              ),
            ),
          ),
          ?trailing,
        ],
      ),
    );
  }
}

class SidebarRow extends StatelessWidget {
  const SidebarRow({
    required this.label,
    required this.onTap,
    required this.active,
    this.leading,
    this.fill,
    super.key,
  });

  final String label;
  final VoidCallback onTap;
  final bool active;
  final Widget? leading;
  final Color? fill;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    final background = active ? (fill ?? tokens.accent) : null;
    final foreground = active && fill == null
        ? tokens.onAccent
        : tokens.sidebarText;
    return Padding(
      padding: const EdgeInsets.symmetric(
        horizontal: PulseSpacing.sm,
        vertical: 1,
      ),
      child: Material(
        color: background ?? const Color(0x00000000),
        borderRadius: BorderRadius.circular(PulseRadius.control),
        child: InkWell(
          borderRadius: BorderRadius.circular(PulseRadius.control),
          onTap: onTap,
          child: Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: PulseSpacing.sm,
              vertical: PulseSpacing.sm,
            ),
            child: Row(
              children: [
                if (leading != null) ...[
                  leading!,
                  const SizedBox(width: PulseSpacing.md),
                ],
                Expanded(
                  child: Text(
                    label,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      color: foreground,
                      fontWeight: active ? FontWeight.bold : FontWeight.w500,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
