import { useState, type FormEvent } from "react";
import type { AccountId, AddMemberRequest, MemberEntry, Role, UpdateMemberRequest } from "@pulse/protocol";
import { Avatar } from "../ui/Avatar";
import { roleLabels } from "../ui/roleLabels";
import { ui } from "../ui/styles";

interface MembersPanelProps {
  members: MemberEntry[];
  assignableRoles: Role[];
  busy: boolean;
  addMember(request: AddMemberRequest): Promise<void>;
  updateMember(accountId: AccountId, request: UpdateMemberRequest): Promise<void>;
  removeMember(accountId: AccountId): Promise<void>;
}


function AddMemberForm({
  assignableRoles,
  busy,
  addMember,
}: Pick<MembersPanelProps, "assignableRoles" | "busy" | "addMember">) {
  const [email, setEmail] = useState("");
  const [role, setRole] = useState<Role>(assignableRoles.includes("member") ? "member" : assignableRoles[0]!);

  function submit(event: FormEvent) {
    event.preventDefault();
    void addMember({ email, role });
  }

  return (
    <form className="flex items-end gap-2" onSubmit={submit}>
      <label className={`${ui.label} flex-1`}>
        Add by email
        <input
          className={ui.input}
          type="email"
          placeholder="grace@example.org"
          value={email}
          onChange={(event) => setEmail(event.target.value)}
        />
      </label>
      <label className={ui.label}>
        <span className="sr-only">New member role</span>
        <select
          className={ui.input}
          aria-label="New member role"
          value={role}
          onChange={(event) => setRole(event.target.value as Role)}
        >
          {assignableRoles.map((option) => (
            <option key={option} value={option}>
              {roleLabels[option]}
            </option>
          ))}
        </select>
      </label>
      <button className={ui.primaryButton} type="submit" disabled={busy}>
        Add
      </button>
    </form>
  );
}

export function MembersPanel({
  members,
  assignableRoles,
  busy,
  addMember,
  updateMember,
  removeMember,
}: MembersPanelProps) {
  return (
    <div className="space-y-4">
      {assignableRoles.length > 0 && (
        <AddMemberForm assignableRoles={assignableRoles} busy={busy} addMember={addMember} />
      )}
      <ul aria-label="Members" className="max-h-72 divide-y divide-line overflow-y-auto rounded-xl border border-line">
        {members.map((member) => (
          <li key={member.account_id} className="flex items-center gap-3 px-3 py-2">
            <Avatar name={member.display_name} />
            <div className="min-w-0 flex-1">
              <p className="truncate text-sm font-medium text-ink">{member.display_name}</p>
              <p className="truncate text-xs text-subtle">{member.email}</p>
            </div>
            {member.manageable ? (
              <>
                <select
                  className="rounded-md border border-line bg-surface px-2 py-1 text-xs text-ink"
                  aria-label={`Role of ${member.display_name}`}
                  value={member.role}
                  disabled={busy}
                  onChange={(event) => void updateMember(member.account_id, { role: event.target.value as Role })}
                >
                  {assignableRoles.map((option) => (
                    <option key={option} value={option}>
                      {roleLabels[option]}
                    </option>
                  ))}
                </select>
                <button
                  className={ui.ghostButton}
                  type="button"
                  disabled={busy}
                  aria-label={`Remove ${member.display_name}`}
                  onClick={() => void removeMember(member.account_id)}
                >
                  Remove
                </button>
              </>
            ) : (
              <span className={`${ui.pill} bg-muted text-subtle`}>{roleLabels[member.role]}</span>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
