import type { ClientSnapshot } from "@pulse/protocol";
import { CampusPicker } from "./campus/CampusPicker";
import { CreateCampusForm } from "./campus/CreateCampusForm";
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

interface AppViewProps {
  snapshot: ClientSnapshot;
  intents: PulseIntents;
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

  if (session.auth !== "signed_in") {
    return (
      <AuthForm busy={session.busy} error={session.error} intents={intents} />
    );
  }

  return (
    <main className="mx-auto max-w-md space-y-4 p-6 text-slate-900">
      <header className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold">
          {session.account?.display_name}
        </h1>
        <button
          className="text-sm underline"
          type="button"
          onClick={() => void intents.signOut()}
        >
          Sign out
        </button>
      </header>
      <nav className="flex gap-3 text-sm">
        <button
          className="underline"
          type="button"
          onClick={() => void intents.toggleSearch()}
        >
          Search
        </button>
        <button
          className="underline"
          type="button"
          onClick={() => void intents.toggleShelf()}
        >
          Shelf <span aria-label="Due reminders">{shelf.due_count}</span>
        </button>
        <button
          className="underline"
          type="button"
          onClick={() => void intents.toggleStewards()}
        >
          Stewards
        </button>
      </nav>
      {shelf.latest_reminder !== null && (
        <ReminderNotice
          entry={shelf.latest_reminder}
          dismiss={intents.dismissReminder}
        />
      )}
      {search.open && <SearchPalette search={search} intents={intents} />}
      {shelf.open && <ShelfView shelf={shelf} intents={intents} />}
      {vault.open && <VaultPanel vault={vault} intents={intents} />}
      {stewards.open && (
        <StewardsPanel
          stewards={stewards}
          rooms={session.rooms}
          intents={intents}
        />
      )}
      <LoungePanel lounges={lounges} intents={intents} />
      <ErrorNotice error={session.error} />
      <CampusPicker
        campuses={session.campuses}
        selectCampus={intents.selectCampus}
      />
      <CreateCampusForm
        busy={session.busy}
        createCampus={intents.createCampus}
      />
      <RoomList
        rooms={session.rooms}
        activeRoomId={session.active_room_id}
        selectRoom={intents.selectRoom}
      />
      {inbox.open && <InboxView inbox={inbox} intents={intents} />}
      <RoomView
        timeline={timeline}
        intents={intents}
        presets={shelf.presets}
        customTime={shelf.picker_anchor_ms !== null}
        stewards={stewards.stewards}
      />
      {thread.root_message_id !== null && (
        <ThreadPanel
          thread={thread}
          palette={timeline.reaction_palette}
          intents={intents}
          stewards={stewards.stewards}
        />
      )}
      <SnapshotView network={network} />
    </main>
  );
}
