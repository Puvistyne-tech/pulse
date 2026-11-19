import { useState, type FormEvent } from "react";
import { Bot, KeyRound, ShieldOff } from "lucide-react";
import type {
  Room,
  RoomId,
  StewardEntry,
  StewardSnapshot,
} from "@pulse/protocol";
import type { PulseIntents } from "../client/usePulseClient";
import { ErrorNotice } from "../session/ErrorNotice";
import { Panel } from "../ui/Panel";
import { ui } from "../ui/styles";

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
      className="flex flex-wrap gap-1.5 text-xs"
      disabled={disabled}
    >
      {rooms.map((room) => (
        <label
          key={room.room_id}
          className="inline-flex cursor-pointer items-center gap-1.5 rounded-full border border-line px-2.5 py-1 font-medium text-subtle transition has-[:checked]:border-accent has-[:checked]:bg-accent-soft has-[:checked]:text-accent has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-60"
        >
          <input
            type="checkbox"
            className="h-3.5 w-3.5 accent-[var(--pulse-accent)]"
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
    <li
      data-revoked={steward.revoked}
      className={`space-y-2.5 rounded-xl border border-line p-3 text-sm ${steward.revoked ? "opacity-60" : ""}`}
    >
      <div className="flex items-center gap-2">
        <span className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent-soft text-accent">
          <Bot size={16} aria-hidden="true" />
        </span>
        <span className="flex-1 font-medium text-ink">{steward.display_name}</span>
        {steward.revoked && <span className={`${ui.pill} bg-danger-soft text-danger`}>Revoked</span>}
        {steward.manageable && (
          <>
            <button
              className={ui.ghostButton}
              type="button"
              disabled={pending}
              aria-label={`Rotate token for ${steward.display_name}`}
              onClick={() => void intents.rotateStewardToken(id)}
            >
              <KeyRound size={13} aria-hidden="true" />
              Rotate token
            </button>
            <button
              className={`${ui.ghostButton} hover:text-danger`}
              type="button"
              disabled={pending}
              aria-label={`Revoke ${steward.display_name}`}
              onClick={() => void intents.revokeSteward(id)}
            >
              <ShieldOff size={13} aria-hidden="true" />
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
        <p className="text-xs text-subtle">
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
    <form className="space-y-3 rounded-xl border border-dashed border-line bg-muted p-3" onSubmit={submit}>
      <h3 className={ui.sectionTitle}>New steward</h3>
      <input
        aria-label="Steward name"
        className={ui.input}
        placeholder="Steward name"
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
        className={ui.primaryButton}
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
    <Panel
      label="Stewards"
      title={
        <>
          <Bot size={16} aria-hidden="true" className="text-accent" />
          Stewards
        </>
      }
      closeLabel="Close stewards"
      onClose={() => void intents.toggleStewards()}
    >
      <ErrorNotice error={stewards.error} />
      {stewards.issued !== null && (
        <div
          role="status"
          aria-label="New steward token"
          className="space-y-2 rounded-xl border border-warning/40 bg-warning-soft p-3 text-sm"
        >
          <p className="text-ink">
            Token for {stewards.issued.display_name}. Copy it now; it is shown
            only once.
          </p>
          <code className="block break-all rounded-lg bg-surface px-2 py-1.5 font-mono text-xs text-ink">
            {stewards.issued.token}
          </code>
          <button
            className={ui.secondaryButton}
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
    </Panel>
  );
}
