import { useState, type FormEvent } from "react";
import type {
  Room,
  RoomId,
  StewardEntry,
  StewardSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";

interface RoomChecklistProps {
  label: string;
  rooms: Room[];
  selected: RoomId[];
  disabled: boolean;
  onChange(roomIds: RoomId[]): void;
}

function RoomChecklist({
  label,
  rooms,
  selected,
  disabled,
  onChange,
}: RoomChecklistProps) {
  return (
    <fieldset
      aria-label={label}
      className="flex flex-wrap gap-3 text-xs"
      disabled={disabled}
    >
      {rooms.map((room) => (
        <label key={room.room_id} className="flex items-center gap-1">
          <input
            type="checkbox"
            checked={selected.includes(room.room_id)}
            onChange={(event) =>
              onChange(
                event.target.checked
                  ? [...selected, room.room_id]
                  : selected.filter((id) => id !== room.room_id),
              )
            }
          />
          #{room.slug}
        </label>
      ))}
    </fieldset>
  );
}

interface StewardRowProps {
  steward: StewardEntry;
  rooms: Room[];
  pending: boolean;
  intents: PulseIntents;
}

function StewardRow({
  steward,
  rooms,
  pending,
  intents,
}: StewardRowProps) {
  const id = steward.steward_id;
  return (
    <li data-revoked={steward.revoked} className="space-y-1 text-sm">
      <div className="flex items-center gap-3">
        <span className="font-medium">{steward.display_name}</span>
        {steward.revoked && <span className="text-xs">Revoked</span>}
        {steward.manageable && (
          <>
            <button
              className="text-xs underline disabled:opacity-50"
              type="button"
              disabled={pending}
              aria-label={`Rotate token for ${steward.display_name}`}
              onClick={() => void intents.rotateStewardToken(id)}
            >
              Rotate token
            </button>
            <button
              className="text-xs underline disabled:opacity-50"
              type="button"
              disabled={pending}
              aria-label={`Revoke ${steward.display_name}`}
              onClick={() => void intents.revokeSteward(id)}
            >
              Revoke
            </button>
          </>
        )}
      </div>
      {steward.manageable ? (
        <RoomChecklist
          label={`Rooms for ${steward.display_name}`}
          rooms={rooms}
          selected={steward.room_ids}
          disabled={pending}
          onChange={(roomIds) => void intents.assignStewardRooms(id, roomIds)}
        />
      ) : (
        <p className="text-xs text-slate-600">
          {rooms
            .filter((room) => steward.room_ids.includes(room.room_id))
            .map((room) => `#${room.slug}`)
            .join(" ")}
        </p>
      )}
    </li>
  );
}

interface CreateStewardFormProps {
  rooms: Room[];
  pending: boolean;
  createSteward: PulseIntents["createSteward"];
}

function CreateStewardForm({
  rooms,
  pending,
  createSteward,
}: CreateStewardFormProps) {
  const [displayName, setDisplayName] = useState("");
  const [roomIds, setRoomIds] = useState<RoomId[]>([]);

  function submit(event: FormEvent) {
    event.preventDefault();
    void createSteward(displayName, roomIds);
    setDisplayName("");
    setRoomIds([]);
  }

  return (
    <form className="space-y-1" onSubmit={submit}>
      <input
        aria-label="Steward name"
        className="w-full rounded border border-slate-300 px-2 py-1"
        type="text"
        value={displayName}
        onChange={(event) => setDisplayName(event.target.value)}
      />
      <RoomChecklist
        label="Rooms for the new steward"
        rooms={rooms}
        selected={roomIds}
        disabled={pending}
        onChange={setRoomIds}
      />
      <button
        className="rounded border px-2 py-1 text-sm disabled:opacity-50"
        type="submit"
        disabled={pending || displayName === ""}
      >
        Create steward
      </button>
    </form>
  );
}

interface StewardsPanelProps {
  stewards: StewardSnapshot;
  rooms: Room[];
  intents: PulseIntents;
}

export function StewardsPanel({ stewards, rooms, intents }: StewardsPanelProps) {
  return (
    <section aria-label="Stewards" className="space-y-2">
      <header className="flex items-center justify-between">
        <h2 className="font-semibold">Stewards</h2>
        <button
          className="text-sm underline"
          type="button"
          onClick={() => void intents.toggleStewards()}
        >
          Close stewards
        </button>
      </header>
      <ErrorNotice error={stewards.error} />
      {stewards.issued !== null && (
        <div
          role="status"
          aria-label="New steward token"
          className="space-y-1 rounded border border-slate-300 p-2 text-sm"
        >
          <p>
            Token for {stewards.issued.display_name}. Copy it now; it is shown
            only once.
          </p>
          <code className="block break-all">{stewards.issued.token}</code>
          <button
            className="text-xs underline"
            type="button"
            onClick={() => void intents.dismissStewardToken()}
          >
            Dismiss token
          </button>
        </div>
      )}
      <ul className="space-y-2">
        {stewards.stewards.map((steward) => (
          <StewardRow
            key={steward.steward_id}
            steward={steward}
            rooms={rooms}
            pending={stewards.pending}
            intents={intents}
          />
        ))}
      </ul>
      {stewards.can_manage && (
        <CreateStewardForm
          rooms={rooms}
          pending={stewards.pending}
          createSteward={intents.createSteward}
        />
      )}
    </section>
  );
}
