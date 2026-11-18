import type { ClientSnapshot } from "@pulse/protocol";
import { CampusPicker } from "./campus/CampusPicker";
import { CreateCampusForm } from "./campus/CreateCampusForm";
import { RoomList } from "./campus/RoomList";
import type { PulseIntents } from "./client/usePulseClient";
import { InboxView } from "./inbox/InboxView";
import { useInboxKeys } from "./inbox/useInboxKeys";
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
  const { session, network, timeline, thread, inbox } = snapshot;
  useInboxKeys(inbox, intents);

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
      <RoomView timeline={timeline} intents={intents} />
      {thread.root_message_id !== null && (
        <ThreadPanel
          thread={thread}
          palette={timeline.reaction_palette}
          intents={intents}
        />
      )}
      <SnapshotView network={network} />
    </main>
  );
}
