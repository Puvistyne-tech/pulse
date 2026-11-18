import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import type { StewardSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test, vi } from "vitest";
import {
  ROOM_A,
  ROOM_B,
  STEWARD_A,
  STEWARD_B,
  expectOnlyCalled,
  issuedTokenOf,
  noopIntents,
  rooms,
  stewardEntryOf,
  stewardsOf,
} from "../test/fixtures";
import { StewardMentionPicker } from "./StewardMentionPicker";
import { StewardsPanel } from "./StewardsPanel";

afterEach(cleanup);

function renderPanel(stewards: StewardSnapshot) {
  const intents = noopIntents();
  const view = render(<StewardsPanel stewards={stewards} rooms={rooms} intents={intents} />);
  return { ...view, intents };
}

const scribe = stewardEntryOf(STEWARD_A, "Scribe");
const retired = stewardEntryOf(STEWARD_B, "Archivist", { revoked: true, room_ids: [ROOM_B] });
const roster = stewardsOf([scribe, retired]);

describe("directory", () => {
  test("stewards render in snapshot order with revoked marked", () => {
    renderPanel(roster);
    const rows = screen.getAllByRole("listitem");
    expect(rows.map((row) => row.querySelector("span.font-medium")?.textContent)).toEqual([
      "Scribe",
      "Archivist",
    ]);
    expect(rows.map((row) => row.getAttribute("data-revoked"))).toEqual(["false", "true"]);
  });

  test.each<[string, StewardSnapshot, string[]]>([
    [
      "managers act on active stewards only",
      roster,
      ["Rotate token for Scribe", "Revoke Scribe"],
    ],
    ["members get no actions", stewardsOf([scribe, retired], { can_manage: false }), []],
  ])("%s", (_name, stewards, labels) => {
    renderPanel(stewards);
    const actions = screen
      .queryAllByRole("button")
      .map((button) => button.getAttribute("aria-label"))
      .filter((label) => label !== null);
    expect(actions).toEqual(labels);
    expect(screen.queryByRole("button", { name: "Create steward" }) !== null).toBe(
      stewards.can_manage,
    );
  });

  test("a revoked steward lists its rooms read-only", () => {
    renderPanel(roster);
    const row = screen.getAllByRole("listitem")[1]!;
    expect(row.textContent).toContain("#courtyard");
    expect(within(row).queryAllByRole("checkbox")).toHaveLength(0);
  });

  test.each<[string, Partial<StewardSnapshot>, string | null]>([
    ["no error", {}, null],
    [
      "invalid rooms",
      { error: { code: "invalid_input", field: "rooms" } },
      "Choose a valid set of rooms for that steward",
    ],
  ])("error %s", (_name, overrides, notice) => {
    renderPanel(stewardsOf([scribe], overrides));
    expect(screen.queryByRole("alert")?.textContent ?? null).toBe(notice);
  });

  test("the close button toggles the panel", () => {
    const { intents } = renderPanel(roster);
    fireEvent.click(screen.getByRole("button", { name: "Close stewards" }));
    expectOnlyCalled(intents, "toggleStewards");
  });
});

describe("admin intents", () => {
  test.each<[string, string, "rotateStewardToken" | "revokeSteward"]>([
    ["rotating", "Rotate token for Scribe", "rotateStewardToken"],
    ["revoking", "Revoke Scribe", "revokeSteward"],
  ])("%s forwards the steward id", (_name, label, call) => {
    const { intents } = renderPanel(roster);
    fireEvent.click(screen.getByRole("button", { name: label }));
    expectOnlyCalled(intents, call, [STEWARD_A]);
  });

  test.each<[string, string, boolean, string[]]>([
    ["adding a room", "#courtyard", true, [ROOM_A, ROOM_B]],
    ["removing a room", "#commons", false, []],
  ])("%s assigns the resulting set", (_name, label, checked, roomIds) => {
    const { intents } = renderPanel(roster);
    const group = screen.getByRole("group", { name: "Rooms for Scribe" });
    const box = within(group).getByLabelText(label) as HTMLInputElement;
    expect(box.checked).toBe(!checked);
    fireEvent.click(box);
    expectOnlyCalled(intents, "assignStewardRooms", [STEWARD_A, roomIds]);
  });

  test("pending disables every action", () => {
    renderPanel(stewardsOf([scribe], { pending: true }));
    const controls = [
      ...screen.getAllByRole("button"),
      ...screen.getAllByRole("checkbox"),
    ].filter((control) => control.getAttribute("aria-label") !== null || control.tagName === "INPUT");
    expect(controls.length).toBeGreaterThan(0);
    expect(controls.every((control) => (control as HTMLInputElement).disabled || control.closest("fieldset")?.disabled)).toBe(true);
    expect((screen.getByRole("button", { name: "Create steward" }) as HTMLButtonElement).disabled).toBe(true);
  });

  test("creating sends the name and chosen rooms then resets the form", () => {
    const { intents } = renderPanel(stewardsOf([]));
    const submit = screen.getByRole("button", { name: "Create steward" }) as HTMLButtonElement;
    expect(submit.disabled).toBe(true);
    const name = screen.getByLabelText("Steward name") as HTMLInputElement;
    fireEvent.change(name, { target: { value: "Courier" } });
    const group = screen.getByRole("group", { name: "Rooms for the new steward" });
    fireEvent.click(within(group).getByLabelText("#courtyard"));
    fireEvent.click(submit);
    expectOnlyCalled(intents, "createSteward", ["Courier", [ROOM_B]]);
    expect(name.value).toBe("");
    expect((within(group).getByLabelText("#courtyard") as HTMLInputElement).checked).toBe(false);
  });
});

describe("issued token", () => {
  test("is absent by default", () => {
    renderPanel(roster);
    expect(screen.queryByRole("status", { name: "New steward token" })).toBeNull();
  });

  test("is shown once with its owner and dismisses through the client", () => {
    const { intents } = renderPanel(stewardsOf([scribe], { issued: issuedTokenOf() }));
    const notice = screen.getByRole("status", { name: "New steward token" });
    expect(notice.textContent).toContain("Scribe");
    expect(notice.textContent).toContain("pst_fixture-steward-a");
    fireEvent.click(within(notice).getByRole("button", { name: "Dismiss token" }));
    expectOnlyCalled(intents, "dismissStewardToken");
  });

  test("rerendering without the token removes it from the page", () => {
    const intents = noopIntents();
    const { rerender } = render(
      <StewardsPanel
        stewards={stewardsOf([scribe], { issued: issuedTokenOf() })}
        rooms={rooms}
        intents={intents}
      />,
    );
    rerender(<StewardsPanel stewards={stewardsOf([scribe])} rooms={rooms} intents={intents} />);
    expect(document.body.textContent).not.toContain("pst_fixture-steward-a");
  });
});

describe("mention picker", () => {
  const stewards = [
    scribe,
    retired,
    stewardEntryOf("0193a5c0-0000-7000-8000-0000000000c5", "Elsewhere", { room_ids: [ROOM_B] }),
  ];

  test.each<[string, string, string[]]>([
    ["a room with an assigned steward", ROOM_A, ["Scribe"]],
    ["a room with only a revoked steward", ROOM_B, ["Elsewhere"]],
    ["a room with none", "0193a5c0-0000-7000-8000-0000000000d9", []],
  ])("%s", (_name, roomId, names) => {
    render(<StewardMentionPicker stewards={stewards} roomId={roomId} onPick={vi.fn()} />);
    expect(screen.queryAllByRole("button").map((button) => button.textContent)).toEqual(names);
  });

  test("picking inserts the mention token", () => {
    const onPick = vi.fn();
    render(<StewardMentionPicker stewards={stewards} roomId={ROOM_A} onPick={onPick} />);
    fireEvent.click(screen.getByRole("button", { name: "Scribe" }));
    expect(onPick).toHaveBeenCalledExactlyOnceWith(scribe.mention_token);
  });
});
