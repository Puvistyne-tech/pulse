import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { JSX } from "react";
import type { CampusInvite, MemberEntry, Role } from "@pulse/protocol";
import { afterEach, expect, test } from "vitest";
import {
  GRACE_ID,
  CAMPUS_A,
  account,
  CAMPUS_B,
  ROOM_A,
  ROOM_B,
  campusA,
  campusB,
  noopIntents,
  rooms,
  type IntentName,
} from "../test/fixtures";
import { CampusPicker } from "./CampusPicker";
import { CreateCampusForm } from "./CreateCampusForm";
import { InvitePanel } from "./InvitePanel";
import { JoinCampusForm } from "./JoinCampusForm";
import { MembersPanel } from "./MembersPanel";
import { RoomList } from "./RoomList";

afterEach(cleanup);

test.each([
  ["plain", "gamma", "Gamma Campus"],
  ["untrimmed", " gamma ", " Gamma  Campus "],
])("create campus submits %s input as typed", (_name, slug, displayName) => {
  const intents = noopIntents();
  render(<CreateCampusForm busy={false} createCampus={intents.createCampus} />);
  fireEvent.change(screen.getByLabelText("Campus slug"), {
    target: { value: slug },
  });
  fireEvent.change(screen.getByLabelText("Campus name"), {
    target: { value: displayName },
  });
  fireEvent.click(screen.getByRole("button", { name: "Create campus" }));
  expect(intents.createCampus).toHaveBeenCalledExactlyOnceWith({
    slug,
    display_name: displayName,
  });
});

const busyForms: ReadonlyArray<[string, (busy: boolean) => JSX.Element]> = [
  ["Create campus", (busy) => <CreateCampusForm busy={busy} createCampus={noopIntents().createCampus} />],
  ["Join campus", (busy) => <JoinCampusForm busy={busy} joinCampus={noopIntents().joinCampus} />],
  ["Create invite code", (busy) => <InvitePanel invite={null} busy={busy} createInvite={noopIntents().createInvite} />],
];

test.each(busyForms.flatMap(([label, form]) => [true, false].map((busy) => [label, busy, form] as const)))(
  "%s disabled is %s while busy",
  (label, busy, form) => {
    render(form(busy));
    const button = screen.getByRole("button", { name: label });
    expect((button as HTMLButtonElement).disabled).toBe(busy);
  },
);

test.each(["0193a5c0-0000-7000-8000-00000000000a.secret", "  padded  "])(
  "join campus submits %s as typed",
  (code) => {
    const intents = noopIntents();
    render(<JoinCampusForm busy={false} joinCampus={intents.joinCampus} />);
    fireEvent.change(screen.getByLabelText("Invite code"), { target: { value: code } });
    fireEvent.click(screen.getByRole("button", { name: "Join campus" }));
    expect(intents.joinCampus).toHaveBeenCalledExactlyOnceWith({ code });
  },
);

test.each<[string, CampusInvite | null, string, boolean]>([
  ["no invite yet", null, "Create invite code", false],
  ["an issued invite", { campus_id: CAMPUS_A, code: "issued-code", expires_at_ms: 0 }, "Create another code", true],
])("invite panel with %s", (_name, invite, label, showsCode) => {
  const intents = noopIntents();
  render(<InvitePanel invite={invite} busy={false} createInvite={intents.createInvite} />);
  expect(screen.queryByDisplayValue("issued-code") !== null).toBe(showsCode);
  fireEvent.click(screen.getByRole("button", { name: label }));
  expect(intents.createInvite).toHaveBeenCalledOnce();
});

test.each([
  ["Alpha Campus", CAMPUS_A],
  ["Beta Campus", CAMPUS_B],
])("picking %s selects its campus id", (label, campusId) => {
  const intents = noopIntents();
  render(
    <CampusPicker
      campuses={[campusA, campusB]}
      selectCampus={intents.selectCampus}
    />,
  );
  fireEvent.click(screen.getByText(label));
  expect(intents.selectCampus).toHaveBeenCalledExactlyOnceWith(campusId);
});

test.each([
  ["both default rooms", rooms, ["#commons", "#courtyard"]],
  ["reversed snapshot order", [...rooms].reverse(), ["#courtyard", "#commons"]],
  ["no rooms", [], []],
])("room list renders %s", (_name, list, expected) => {
  render(
    <RoomList
      rooms={list}
      activeRoomId={null}
      selectRoom={noopIntents().selectRoom}
    />,
  );
  const labels = screen.queryAllByRole("listitem").map((item) => item.textContent);
  expect(labels).toEqual(expected);
});

test.each([
  ["#commons", ROOM_A],
  ["#courtyard", ROOM_B],
])("clicking %s selects its room id", (label, roomId) => {
  const intents = noopIntents();
  render(
    <RoomList rooms={rooms} activeRoomId={null} selectRoom={intents.selectRoom} />,
  );
  fireEvent.click(screen.getByText(label));
  expect(intents.selectRoom).toHaveBeenCalledExactlyOnceWith(roomId);
});

test.each([
  ["first room", ROOM_A, "#commons"],
  ["second room", ROOM_B, "#courtyard"],
  ["no active room", null, null],
])("active room marking for %s", (_name, activeRoomId, marked) => {
  render(
    <RoomList
      rooms={rooms}
      activeRoomId={activeRoomId}
      selectRoom={noopIntents().selectRoom}
    />,
  );
  const current = screen
    .getAllByRole("button")
    .filter((button) => button.getAttribute("aria-current") === "true")
    .map((button) => button.textContent);
  expect(current).toEqual(marked === null ? [] : [marked]);
});

const roster: MemberEntry[] = [
  { account_id: account.account_id, display_name: "Ada", email: "ada@example.org", role: "owner", manageable: false },
  { account_id: GRACE_ID, display_name: "Grace", email: "grace@example.org", role: "member", manageable: true },
];

function renderMembers(assignableRoles: Role[]) {
  const intents = noopIntents();
  render(
    <MembersPanel
      members={roster}
      assignableRoles={assignableRoles}
      busy={false}
      addMember={intents.addMember}
      updateMember={intents.updateMember}
      removeMember={intents.removeMember}
    />,
  );
  return intents;
}

test.each<[string, Role[], boolean]>([
  ["a manager", ["admin", "member", "guest"], true],
  ["a viewer", [], false],
])("members panel for %s offers adding %s", (_who, assignableRoles, offersAdd) => {
  renderMembers(assignableRoles);
  expect(screen.queryByLabelText("Add by email") !== null).toBe(offersAdd);
  expect(screen.getByText("Owner")).not.toBeNull();
  expect(screen.queryByRole("button", { name: "Remove Ada" })).toBeNull();
});

test.each<[string, (intents: ReturnType<typeof noopIntents>) => void, IntentName, unknown[]]>([
  [
    "adding by email",
    () => {
      fireEvent.change(screen.getByLabelText("Add by email"), { target: { value: "linus@example.org" } });
      fireEvent.change(screen.getByLabelText("New member role"), { target: { value: "guest" } });
      fireEvent.click(screen.getByRole("button", { name: "Add" }));
    },
    "addMember",
    [{ email: "linus@example.org", role: "guest" }],
  ],
  [
    "changing a role",
    () => fireEvent.change(screen.getByLabelText("Role of Grace"), { target: { value: "admin" } }),
    "updateMember",
    [GRACE_ID, { role: "admin" }],
  ],
  [
    "removing a member",
    () => fireEvent.click(screen.getByRole("button", { name: "Remove Grace" })),
    "removeMember",
    [GRACE_ID],
  ],
])("members panel %s calls its intent", (_name, act, intent, args) => {
  const intents = renderMembers(["admin", "member", "guest"]);
  act(intents);
  expect(intents[intent]).toHaveBeenCalledExactlyOnceWith(...args);
});
