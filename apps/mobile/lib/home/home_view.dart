import 'package:material_ui/material_ui.dart';

import '../campus/create_campus_view.dart';
import '../client/pulse_intents.dart';
import '../inbox/inbox_view.dart';
import '../room/room_view.dart';
import '../room/thread_view.dart';
import '../search/search_view.dart';
import '../session/auth_view.dart';
import '../session/error_notice.dart';
import '../shell/panel_card.dart';
import '../shell/sidebar.dart';
import '../shelf/reminder_banner.dart';
import '../shelf/shelf_view.dart';
import '../src/rust/third_party/pulse_protocol/snapshot.dart';
import '../theme/pulse_theme.dart';
import '../vault/vault_view.dart';

const _wideBreakpoint = 900.0;
const _sidebarWidth = 280.0;
const _sidePanelWidth = 400.0;

class _OpenPanel {
  const _OpenPanel(this.key, this.dismissLabel, this.close, this.child);

  final Key key;
  final String dismissLabel;
  final VoidCallback close;
  final Widget child;
}

class HomeView extends StatefulWidget {
  const HomeView({required this.snapshot, required this.intents, super.key});

  final ClientSnapshot snapshot;
  final PulseIntents intents;

  @override
  State<HomeView> createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  var _creatingCampus = false;

  @override
  void didUpdateWidget(HomeView oldWidget) {
    super.didUpdateWidget(oldWidget);
    final grew =
        widget.snapshot.session.campuses.length >
        oldWidget.snapshot.session.campuses.length;
    if (grew && _creatingCampus) {
      _creatingCampus = false;
    }
  }

  String _title() {
    final session = widget.snapshot.session;
    for (final room in session.rooms) {
      if (room.roomId == session.activeRoomId) {
        return '# ${room.slug}';
      }
    }
    for (final campus in session.campuses) {
      if (campus.campusId == session.activeCampusId) {
        return campus.displayName;
      }
    }
    return 'Pulse';
  }

  List<_OpenPanel> _openPanels() {
    final snapshot = widget.snapshot;
    final intents = widget.intents;
    return [
      if (snapshot.inbox.open)
        _OpenPanel(
          const ValueKey('panel-inbox'),
          'Dismiss inbox',
          intents.toggleInbox,
          InboxView(inbox: snapshot.inbox, intents: intents),
        ),
      if (snapshot.search.open)
        _OpenPanel(
          const ValueKey('panel-search'),
          'Dismiss search',
          intents.toggleSearch,
          SearchView(search: snapshot.search, intents: intents),
        ),
      if (snapshot.shelf.open)
        _OpenPanel(
          const ValueKey('panel-shelf'),
          'Dismiss shelf',
          intents.toggleShelf,
          ShelfView(shelf: snapshot.shelf, intents: intents),
        ),
      if (snapshot.vault.open)
        _OpenPanel(
          const ValueKey('panel-vault'),
          'Dismiss vault',
          intents.closeVault,
          VaultView(vault: snapshot.vault, intents: intents),
        ),
      if (snapshot.thread.rootMessageId != null)
        _OpenPanel(
          const ValueKey('panel-thread'),
          'Dismiss thread',
          intents.closeThread,
          ThreadView(
            thread: snapshot.thread,
            palette: snapshot.timeline.reactionPalette,
            intents: intents,
          ),
        ),
    ];
  }

  Widget _barAction({
    required IconData icon,
    required String tooltip,
    required int count,
    required VoidCallback onPressed,
  }) {
    return IconButton(
      tooltip: tooltip,
      onPressed: onPressed,
      icon: Badge(
        isLabelVisible: count > 0,
        label: Text('$count'),
        child: Icon(icon),
      ),
    );
  }

  Widget _content() {
    final snapshot = widget.snapshot;
    final intents = widget.intents;
    final tokens = PulseTokens.of(context);
    final latestReminder = snapshot.shelf.latestReminder;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        if (latestReminder != null)
          ReminderBanner(
            entry: latestReminder,
            dismiss: intents.dismissReminder,
          ),
        if (!_creatingCampus)
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: PulseSpacing.md),
            child: ErrorNotice(error: snapshot.session.error),
          ),
        Expanded(
          child: snapshot.timeline.roomId == null
              ? Center(
                  child: Text(
                    'Choose a Room to start',
                    style: TextStyle(color: tokens.textMuted),
                  ),
                )
              : RoomView(
                  timeline: snapshot.timeline,
                  intents: intents,
                  presets: snapshot.shelf.presets,
                  pickerAnchorMs: snapshot.shelf.pickerAnchorMs,
                  pickerLatestMs: snapshot.shelf.pickerLatestMs,
                ),
        ),
      ],
    );
  }

  Widget _createCampusOverlay(bool wide) {
    final session = widget.snapshot.session;
    return ModalOverlay(
      centered: wide,
      dismissLabel: 'Dismiss new campus',
      onDismiss: () => setState(() => _creatingCampus = false),
      child: CreateCampusView(
        busy: session.busy,
        error: session.error,
        createCampus: widget.intents.createCampus,
        onClose: () => setState(() => _creatingCampus = false),
      ),
    );
  }

  Widget _sidePanels(List<_OpenPanel> panels) {
    final tokens = PulseTokens.of(context);
    return SizedBox(
      key: const Key('sidePanels'),
      width: _sidePanelWidth,
      child: DecoratedBox(
        decoration: BoxDecoration(
          color: tokens.surface,
          border: Border(left: BorderSide(color: tokens.border)),
        ),
        child: Material(
          color: tokens.surface,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              for (final panel in panels)
                Flexible(key: panel.key, child: panel.child),
            ],
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final snapshot = widget.snapshot;
    final intents = widget.intents;
    final session = snapshot.session;
    if (session.auth != AuthPhase.signedIn) {
      return Scaffold(
        body: SafeArea(
          child: AuthView(
            busy: session.busy,
            error: session.error,
            intents: intents,
          ),
        ),
      );
    }
    final panels = _openPanels();
    return LayoutBuilder(
      builder: (context, constraints) {
        final wide = constraints.maxWidth >= _wideBreakpoint;
        final main = Scaffold(
          appBar: AppBar(
            title: Text(_title()),
            actions: [
              IconButton(
                tooltip: 'Open search',
                onPressed: intents.toggleSearch,
                icon: const Icon(Icons.search),
              ),
              _barAction(
                icon: Icons.inbox_outlined,
                tooltip: 'Open inbox',
                count: snapshot.inbox.unreadThreads,
                onPressed: intents.toggleInbox,
              ),
              _barAction(
                icon: Icons.bookmark_border,
                tooltip: 'Open shelf',
                count: snapshot.shelf.dueCount,
                onPressed: intents.toggleShelf,
              ),
            ],
          ),
          drawer: wide
              ? null
              : Drawer(
                  child: Builder(
                    builder: (drawerContext) => Sidebar(
                      snapshot: snapshot,
                      intents: intents,
                      afterNavigate: () => Navigator.of(drawerContext).pop(),
                      onNewCampus: () {
                        Navigator.of(drawerContext).pop();
                        setState(() => _creatingCampus = true);
                      },
                    ),
                  ),
                ),
          body: Stack(
            children: [
              _content(),
              if (!wide)
                for (final panel in panels)
                  ModalOverlay(
                    key: panel.key,
                    dismissLabel: panel.dismissLabel,
                    onDismiss: panel.close,
                    child: panel.child,
                  ),
              if (_creatingCampus) _createCampusOverlay(wide),
            ],
          ),
        );
        if (!wide) {
          return main;
        }
        return Scaffold(
          body: Row(
            children: [
              SizedBox(
                width: _sidebarWidth,
                child: Sidebar(
                  snapshot: snapshot,
                  intents: intents,
                  onNewCampus: () => setState(() => _creatingCampus = true),
                ),
              ),
              Expanded(child: main),
              if (panels.isNotEmpty) _sidePanels(panels),
            ],
          ),
        );
      },
    );
  }
}
