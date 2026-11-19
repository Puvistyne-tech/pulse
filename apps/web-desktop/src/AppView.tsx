import { useEffect, useRef, useState, type ReactNode } from "react";
import { Bookmark, Bot, Hash, Inbox, LogIn, LogOut, Monitor, Moon, Plus, Search, Sun, Users } from "lucide-react";
import type { ClientSnapshot } from "@pulse/protocol";
import { CampusPicker } from "./campus/CampusPicker";
import { CreateCampusForm } from "./campus/CreateCampusForm";
import { InvitePanel } from "./campus/InvitePanel";
import { JoinCampusForm } from "./campus/JoinCampusForm";
import { MembersPanel } from "./campus/MembersPanel";
import { RoomList } from "./campus/RoomList";
import type { PulseIntents } from "./client/usePulseClient";
import { LoungePanel } from "./lounge/LoungePanel";
import { InboxView } from "./inbox/InboxView";
import { useInboxKeys } from "./inbox/useInboxKeys";
import { SearchPalette } from "./search/SearchPalette";
import { useSearchKeys } from "./search/useSearchKeys";
import { StewardsPanel } from "./steward/StewardsPanel";
import { ReminderNotice } from "./shelf/ReminderNotice";
import { ShelfView } from "./shelf/ShelfView";
import { VaultPanel } from "./vault/VaultPanel";
import { RoomView } from "./room/RoomView";
import { ThreadPanel } from "./room/ThreadPanel";
import { AuthForm } from "./session/AuthForm";
import { ErrorNotice } from "./session/ErrorNotice";
import { SnapshotView } from "./snapshot/SnapshotView";
import { Avatar } from "./ui/Avatar";
import { Dialog } from "./ui/Dialog";
import { PulseMark } from "./ui/PulseMark";
import { ui } from "./ui/styles";
import { useTheme, type ThemeChoice } from "./ui/useTheme";

interface AppViewProps {
  snapshot: ClientSnapshot;
  intents: PulseIntents;
}

const nextTheme: Record<ThemeChoice, ThemeChoice> = {
  system: "light",
  light: "dark",
  dark: "system",
};

const themeIcons: Record<ThemeChoice, ReactNode> = {
  system: <Monitor size={15} aria-hidden="true" />,
  light: <Sun size={15} aria-hidden="true" />,
  dark: <Moon size={15} aria-hidden="true" />,
};

const navButton =
  "flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-sm text-side-ink transition-colors hover:bg-side-hover hover:text-white aria-pressed:bg-side-hover aria-pressed:text-white";

const countBadge = "ml-auto rounded-full bg-accent px-1.5 text-[11px] font-semibold leading-5 text-on-accent";

type CampusDialog = "create" | "join" | null;

function useCampusDialog(campusCount: number): [CampusDialog, (dialog: CampusDialog) => void] {
  const [dialog, setDialog] = useState<CampusDialog>(null);
  const seen = useRef(campusCount);

  useEffect(() => {
    if (campusCount > seen.current) {
      setDialog(null);
    }
    seen.current = campusCount;
  }, [campusCount]);

  return [dialog, setDialog];
}

export function AppView({ snapshot, intents }: AppViewProps) {
  const {
    session,
    network,
    timeline,
    thread,
    inbox,
    search,
    vault,
    shelf,
    lounges,
    stewards,
  } = snapshot;
  useInboxKeys(inbox, intents);
  useSearchKeys(search, intents);
  const [theme, setTheme] = useTheme();
  const [campusDialog, setCampusDialog] = useCampusDialog(session.campuses.length);
  const [peopleOpen, setPeopleOpen] = useState(false);

  if (session.auth !== "signed_in") {
    return (
      <AuthForm
        busy={session.busy}
        error={session.error}
        quickAccounts={session.quick_accounts}
        intents={intents}
      />
    );
  }

  const activeCampus = session.campuses.find(
    (campus) => campus.campus_id === session.active_campus_id,
  );
  const activeRoom = session.rooms.find((room) => room.room_id === session.active_room_id);
  const displayName = session.account?.display_name ?? "";
  const threadOpen = thread.root_message_id !== null;
  const sidePanelOpen = threadOpen || inbox.open || shelf.open || vault.open || stewards.open;

  return (
    <div className="flex h-full overflow-hidden bg-canvas text-ink">
      <aside className="flex w-[68px] shrink-0 flex-col items-center gap-3 bg-rail py-3">
        <PulseMark size={36} />
        <div className="h-px w-8 bg-white/10" />
        <div className="min-h-0 flex-1 overflow-y-auto">
          <CampusPicker
            campuses={session.campuses}
            activeCampusId={session.active_campus_id}
            selectCampus={intents.selectCampus}
          />
        </div>
        <button
          className="flex h-10 w-10 items-center justify-center rounded-xl border border-dashed border-white/20 text-side-subtle transition hover:border-accent hover:text-accent"
          type="button"
          aria-label="New campus"
          title="New campus"
          onClick={() => setCampusDialog("create")}
        >
          <Plus size={18} aria-hidden="true" />
        </button>
        <button
          className="flex h-10 w-10 items-center justify-center rounded-xl border border-dashed border-white/20 text-side-subtle transition hover:border-accent hover:text-accent"
          type="button"
          aria-label="Join campus"
          title="Join campus"
          onClick={() => setCampusDialog("join")}
        >
          <LogIn size={18} aria-hidden="true" />
        </button>
      </aside>

      <aside className="flex w-64 shrink-0 flex-col bg-side text-side-ink">
        <header className="flex h-14 shrink-0 items-center border-b border-white/10 px-4">
          <div className="min-w-0">
            <p className="truncate text-[15px] font-semibold text-white">
              {activeCampus?.display_name ?? "Pulse"}
            </p>
            {activeCampus && (
              <p className="truncate text-[11px] text-side-subtle">
                {activeCampus.slug} · {activeCampus.role}
              </p>
            )}
          </div>
        </header>
        <div className="min-h-0 flex-1 space-y-5 overflow-y-auto px-2 py-3">
          <nav className="space-y-0.5">
            <button className={navButton} type="button" aria-pressed={search.open} onClick={() => void intents.toggleSearch()}>
              <Search size={16} aria-hidden="true" />
              Search
              <kbd className="ml-auto rounded border border-white/15 px-1 font-sans text-[10px] text-side-subtle">⌘K</kbd>
            </button>
            <button className={navButton} type="button" aria-pressed={inbox.open} onClick={() => void intents.toggleInbox()}>
              <Inbox size={16} aria-hidden="true" />
              Inbox
              {inbox.unread_threads > 0 && <span className={countBadge}>{inbox.unread_threads}</span>}
            </button>
            <button className={navButton} type="button" aria-pressed={shelf.open} onClick={() => void intents.toggleShelf()}>
              <Bookmark size={16} aria-hidden="true" />
              Shelf
              <span
                aria-label="Due reminders"
                className={shelf.due_count > 0 ? countBadge : "ml-auto text-[11px] text-side-subtle"}
              >
                {shelf.due_count}
              </span>
            </button>
            <button className={navButton} type="button" aria-pressed={stewards.open} onClick={() => void intents.toggleStewards()}>
              <Bot size={16} aria-hidden="true" />
              Stewards
            </button>
            {activeCampus && (
              <button
                className={navButton}
                type="button"
                aria-pressed={peopleOpen}
                onClick={() => {
                  setPeopleOpen(true);
                  void intents.loadMembers();
                }}
              >
                <Users size={16} aria-hidden="true" />
                People
              </button>
            )}
          </nav>
          {session.rooms.length > 0 && (
            <div className="space-y-1">
              <h2 className="px-2.5 text-[11px] font-semibold uppercase tracking-wider text-side-subtle">Rooms</h2>
              <RoomList
                rooms={session.rooms}
                activeRoomId={session.active_room_id}
                selectRoom={intents.selectRoom}
              />
            </div>
          )}
          <LoungePanel lounges={lounges} intents={intents} />
        </div>
        <footer className="shrink-0 space-y-2 border-t border-white/10 p-3">
          <div className="flex items-center gap-2.5">
            <Avatar name={displayName} />
            <div className="min-w-0 flex-1">
              <p className="truncate text-sm font-semibold text-white">{displayName}</p>
              <SnapshotView network={network} />
            </div>
          </div>
          <div className="flex items-center gap-1">
            <button
              className="inline-flex h-8 w-8 items-center justify-center rounded-md text-side-subtle hover:bg-side-hover hover:text-white"
              type="button"
              aria-label={`Theme ${theme}`}
              title={`Theme: ${theme}`}
              onClick={() => setTheme(nextTheme[theme])}
            >
              {themeIcons[theme]}
            </button>
            <button
              className="ml-auto inline-flex items-center gap-1.5 rounded-md px-2 py-1.5 text-xs font-medium text-side-subtle hover:bg-side-hover hover:text-white"
              type="button"
              onClick={() => void intents.signOut()}
            >
              <LogOut size={14} aria-hidden="true" />
              Sign out
            </button>
          </div>
        </footer>
      </aside>

      <main className="flex min-w-0 flex-1 flex-col">
        {campusDialog === null && !peopleOpen && session.error !== null && (
          <div className="shrink-0 px-5 pt-3">
            <ErrorNotice error={session.error} />
          </div>
        )}
        {session.campuses.length === 0 ? (
          <div className="flex flex-1 flex-col items-center justify-center gap-4 px-6 text-center">
            <PulseMark size={52} />
            <div className="space-y-1">
              <h1 className="text-xl font-semibold text-ink">Welcome to Pulse, {displayName}</h1>
              <p className="text-sm text-subtle">Create a campus, or join one with an invite code.</p>
            </div>
            <div className="flex flex-wrap justify-center gap-2">
              <button className={ui.primaryButton} type="button" onClick={() => setCampusDialog("create")}>
                <Plus size={16} aria-hidden="true" />
                Create campus
              </button>
              <button className={ui.secondaryButton} type="button" onClick={() => setCampusDialog("join")}>
                <LogIn size={16} aria-hidden="true" />
                Join with invite code
              </button>
            </div>
          </div>
        ) : timeline.room_id === null ? (
          <div className="flex flex-1 flex-col items-center justify-center gap-2 text-center text-subtle">
            <Hash size={32} aria-hidden="true" className="text-accent" />
            <p className="text-sm">Pick a room from the sidebar.</p>
          </div>
        ) : (
          <RoomView
            timeline={timeline}
            intents={intents}
            presets={shelf.presets}
            customTime={shelf.picker_anchor_ms !== null}
            roomSlug={activeRoom?.slug ?? null}
          />
        )}
      </main>

      {sidePanelOpen && (
        <div className="fixed inset-y-0 right-0 z-20 flex w-[min(400px,100vw)] flex-col divide-y divide-line border-l border-line bg-surface shadow-panel xl:static xl:shadow-none">
          {threadOpen && (
            <ThreadPanel
              thread={thread}
              palette={timeline.reaction_palette}
              intents={intents}
            />
          )}
          {inbox.open && <InboxView inbox={inbox} intents={intents} />}
          {shelf.open && <ShelfView shelf={shelf} intents={intents} />}
          {vault.open && <VaultPanel vault={vault} intents={intents} />}
          {stewards.open && (
            <StewardsPanel
              stewards={stewards}
              rooms={session.rooms}
              intents={intents}
            />
          )}
        </div>
      )}

      {search.open && <SearchPalette search={search} intents={intents} />}
      {shelf.latest_reminder !== null && (
        <ReminderNotice
          entry={shelf.latest_reminder}
          dismiss={intents.dismissReminder}
        />
      )}
      {campusDialog === "create" && (
        <Dialog
          title="Create campus"
          description="A campus is a home for your team, with its own rooms, lounges and stewards."
          closeLabel="Close"
          onClose={() => setCampusDialog(null)}
        >
          <ErrorNotice error={session.error} />
          <CreateCampusForm busy={session.busy} createCampus={intents.createCampus} />
        </Dialog>
      )}
      {campusDialog === "join" && (
        <Dialog
          title="Join campus"
          description="Paste the invite code someone shared with you."
          closeLabel="Close"
          onClose={() => setCampusDialog(null)}
        >
          <ErrorNotice error={session.error} />
          <JoinCampusForm busy={session.busy} joinCampus={intents.joinCampus} />
        </Dialog>
      )}
      {peopleOpen && activeCampus && (
        <Dialog
          title="People"
          description={`Everyone in ${activeCampus.display_name}. Other campuses never see them.`}
          closeLabel="Close"
          onClose={() => setPeopleOpen(false)}
        >
          <ErrorNotice error={session.error} />
          <MembersPanel
            members={session.members}
            assignableRoles={session.assignable_roles}
            busy={session.busy}
            addMember={intents.addMember}
            updateMember={intents.updateMember}
            removeMember={intents.removeMember}
          />
          {session.can_invite && (
            <section className="space-y-2 border-t border-line pt-4">
              <h3 className={ui.sectionTitle}>Invite code</h3>
              <InvitePanel invite={session.invite} busy={session.busy} createInvite={intents.createInvite} />
            </section>
          )}
        </Dialog>
      )}
    </div>
  );
}
