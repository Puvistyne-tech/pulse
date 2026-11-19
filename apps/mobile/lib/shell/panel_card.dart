import 'package:material_ui/material_ui.dart';

import '../theme/pulse_theme.dart';

class PanelCard extends StatelessWidget {
  const PanelCard({
    required this.title,
    required this.closeLabel,
    required this.onClose,
    required this.child,
    this.footer,
    this.cardKey,
    super.key,
  });

  final String title;
  final String closeLabel;
  final VoidCallback onClose;
  final Widget child;
  final Widget? footer;
  final Key? cardKey;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Column(
      key: cardKey,
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(
            PulseSpacing.lg,
            PulseSpacing.sm,
            PulseSpacing.sm,
            PulseSpacing.sm,
          ),
          child: Row(
            children: [
              Expanded(
                child: Text(
                  title,
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    color: tokens.text,
                    fontWeight: FontWeight.w700,
                  ),
                ),
              ),
              IconButton(
                tooltip: closeLabel,
                onPressed: onClose,
                icon: const Icon(Icons.close, size: 20),
              ),
            ],
          ),
        ),
        Divider(color: tokens.border),
        Flexible(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(PulseSpacing.lg),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [child],
            ),
          ),
        ),
        if (footer != null) ...[Divider(color: tokens.border), footer!],
      ],
    );
  }
}

class PanelSurface extends StatelessWidget {
  const PanelSurface({required this.child, this.radius, super.key});

  final Widget child;
  final BorderRadius? radius;

  @override
  Widget build(BuildContext context) {
    final tokens = PulseTokens.of(context);
    return Material(
      color: tokens.surface,
      shape: RoundedRectangleBorder(
        borderRadius:
            radius ?? const BorderRadius.all(Radius.circular(PulseRadius.card)),
        side: BorderSide(color: tokens.border),
      ),
      clipBehavior: Clip.antiAlias,
      child: child,
    );
  }
}

class ModalOverlay extends StatelessWidget {
  const ModalOverlay({
    required this.onDismiss,
    required this.child,
    required this.dismissLabel,
    this.centered = false,
    super.key,
  });

  final VoidCallback onDismiss;
  final String dismissLabel;
  final Widget child;
  final bool centered;

  @override
  Widget build(BuildContext context) {
    final height = MediaQuery.sizeOf(context).height;
    return Stack(
      children: [
        Positioned.fill(
          child: Semantics(
            label: dismissLabel,
            button: true,
            child: GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: onDismiss,
              child: const ColoredBox(color: Color(0x80101828)),
            ),
          ),
        ),
        Align(
          alignment: centered ? Alignment.center : Alignment.bottomCenter,
          child: ConstrainedBox(
            constraints: BoxConstraints(
              maxWidth: centered ? 440 : double.infinity,
              maxHeight: height * 0.85,
            ),
            child: PanelSurface(
              radius: centered
                  ? null
                  : const BorderRadius.vertical(
                      top: Radius.circular(PulseRadius.card),
                    ),
              child: child,
            ),
          ),
        ),
      ],
    );
  }
}
