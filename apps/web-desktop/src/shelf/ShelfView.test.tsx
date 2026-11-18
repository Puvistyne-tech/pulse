import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import type { ReminderPreset, ShelfSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test, vi } from "vitest";
import {
  MESSAGE_A,
  MESSAGE_B,
  MESSAGE_C,
  MESSAGE_D,
  ROOM_A,
  allPresets,
  expectOnlyCalled,
  noopIntents,
  shelfEntryOf,
  shelfOf,
} from "../test/fixtures";
import { ReminderNotice } from "./ReminderNotice";
import { ShelfMenu, presetLabels } from "./ShelfMenu";
import { ShelfView } from "./ShelfView";

afterEach(cleanup);

function renderShelf(shelf: ShelfSnapshot) {
  const intents = noopIntents();
  const view = render(<ShelfView shelf={shelf} intents={intents} />);
  return { ...view, intents };
}

const fullShelf = shelfOf({
  due: [shelfEntryOf("due", MESSAGE_B), shelfEntryOf("due", MESSAGE_A)],
  upcoming: [shelfEntryOf("upcoming", MESSAGE_C)],
  done: [shelfEntryOf("done", MESSAGE_D)],
});

const sectionLabels = ["Due", "Upcoming", "Done"] as const;

describe("sections", () => {
  test("the three sections render in order", () => {
    renderShelf(fullShelf);
    const headings = screen.getAllByRole("heading", { level: 3 });
    expect(headings.map((heading) => heading.textContent)).toEqual([
      "Due",
      "Upcoming",
      "Done",
    ]);
  });

  test.each<[(typeof sectionLabels)[number], string[]]>([
    ["Due", ["shelved e2", "shelved e1"]],
    ["Upcoming", ["shelved e3"]],
    ["Done", ["shelved e4"]],
  ])("%s lists its entries in snapshot order", (label, previews) => {
    renderShelf(fullShelf);
    const section = screen.getByRole("region", { name: label });
    const rows = within(section).getAllByRole("listitem");
    expect(rows.map((row) => row.querySelector("span.block:nth-of-type(2)")?.textContent)).toEqual(previews);
  });

  test("an empty shelf renders three empty sections", () => {
    renderShelf(shelfOf({}));
    for (const label of sectionLabels) {
      const section = screen.getByRole("region", { name: label });
      expect(within(section).queryAllByRole("listitem")).toHaveLength(0);
    }
  });

  test.each<[string, ShelfSnapshot, string | null]>([
    ["no error", shelfOf({}), null],
    ["unavailable", shelfOf({}, { error: { code: "unavailable", field: null } }), "Pulse server unavailable"],
    [
      "invalid reminder time",
      shelfOf({}, { error: { code: "invalid_input", field: "remind_at" } }),
      "Choose a reminder time that is not too far in the past or future",
    ],
  ])("error %s", (_name, shelf, notice) => {
    renderShelf(shelf);
    expect(screen.queryByRole("alert")?.textContent ?? null).toBe(notice);
  });

  test("entries awaiting confirmation say so", () => {
    renderShelf(
      shelfOf({ upcoming: [shelfEntryOf("upcoming", MESSAGE_A, { pending: true })] }),
    );
    expect(screen.getByText(/Saving/)).not.toBeNull();
  });

  test("confirmed entries do not say saving", () => {
    renderShelf(fullShelf);
    expect(screen.queryByText(/Saving/)).toBeNull();
  });

  test("the close button toggles the shelf", () => {
    const { intents } = renderShelf(fullShelf);
    fireEvent.click(screen.getByRole("button", { name: "Close shelf" }));
    expectOnlyCalled(intents, "toggleShelf");
  });
});

describe("entry actions", () => {
  test.each<[string, "Complete" | "Reopen", "complete" | "reopen"]>([
    ["Due", "Complete", "complete"],
    ["Upcoming", "Complete", "complete"],
    ["Done", "Reopen", "reopen"],
  ])("%s offers %s", (label, button, action) => {
    const { intents } = renderShelf(fullShelf);
    const section = screen.getByRole("region", { name: label });
    const target = within(section).getAllByRole("button", { name: button })[0]!;
    fireEvent.click(target);
    const message = { Due: MESSAGE_B, Upcoming: MESSAGE_C, Done: MESSAGE_D }[label];
    expectOnlyCalled(intents, "updateShelf", [ROOM_A, message, action, null, null]);
    const other = button === "Complete" ? "Reopen" : "Complete";
    expect(within(section).queryByRole("button", { name: other })).toBeNull();
  });
});

type MenuRow = [string, boolean, ReminderPreset[], string[]];

const menuRows: MenuRow[] = [
  [
    "unshelved with every preset",
    false,
    allPresets,
    ["Save to shelf", ...allPresets.map((preset) => presetLabels[preset]), "Remind at chosen time"],
  ],
  [
    "shelved with every preset",
    true,
    allPresets,
    [...allPresets.map((preset) => presetLabels[preset]), "Remind at chosen time", "Remove from shelf"],
  ],
  ["unshelved without presets", false, [], ["Save to shelf", "Remind at chosen time"]],
  ["shelved without presets", true, [], ["Remind at chosen time", "Remove from shelf"]],
  [
    "a subset of presets",
    false,
    ["in_one_hour", "tomorrow"],
    ["Save to shelf", "In one hour", "Tomorrow", "Remind at chosen time"],
  ],
];

describe("shelf menu", () => {
  test.each(menuRows)("%s", (_name, shelved, presets, labels) => {
    render(<ShelfMenu presets={presets} shelved={shelved} onUpdate={vi.fn()} />);
    expect(screen.queryByRole("menu")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    const menu = screen.getByRole("menu");
    expect(within(menu).getAllByRole("button").map((button) => button.textContent)).toEqual(labels);
  });

  test.each<[string, boolean, string, unknown[]]>([
    ["saving unshelved", false, "Save to shelf", ["save", null, null]],
    ["removing", true, "Remove from shelf", ["remove", null, null]],
    ["a preset on an unshelved message", false, "In one hour", ["save", "in_one_hour", null]],
    ["a preset on a shelved message", true, "Next week", ["reschedule", "next_week", null]],
  ])("%s calls onUpdate and closes the menu", (_name, shelved, label, args) => {
    const onUpdate = vi.fn();
    render(<ShelfMenu presets={allPresets} shelved={shelved} onUpdate={onUpdate} />);
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    fireEvent.click(screen.getByRole("button", { name: label }));
    expect(onUpdate).toHaveBeenCalledExactlyOnceWith(...args);
    expect(screen.queryByRole("menu")).toBeNull();
  });

  test.each<[string, boolean, string]>([
    ["unshelved saves", false, "save"],
    ["shelved reschedules", true, "reschedule"],
  ])("a custom time %s with epoch milliseconds", (_name, shelved, action) => {
    const onUpdate = vi.fn();
    render(<ShelfMenu presets={allPresets} shelved={shelved} onUpdate={onUpdate} />);
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    const apply = screen.getByRole("button", { name: "Remind at chosen time" }) as HTMLButtonElement;
    expect(apply.disabled).toBe(true);
    fireEvent.change(screen.getByLabelText("Remind at"), {
      target: { value: "2026-11-20T10:30" },
    });
    expect(apply.disabled).toBe(false);
    fireEvent.click(apply);
    expect(onUpdate).toHaveBeenCalledExactlyOnceWith(
      action,
      null,
      Date.parse("2026-11-20T10:30"),
    );
  });

  test.each<[string, boolean, boolean]>([
    ["offered by default", true, true],
    ["withheld when disabled", false, false],
  ])("the custom time is %s", (_name, customTime, shown) => {
    render(
      <ShelfMenu presets={[]} shelved={false} customTime={customTime} onUpdate={vi.fn()} />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    expect(screen.queryByLabelText("Remind at") !== null).toBe(shown);
    expect(screen.queryByRole("button", { name: "Remind at chosen time" }) !== null).toBe(shown);
  });

  test.each<[string, number | null, boolean]>([
    ["a picker anchor", 3_300_000, true],
    ["no picker anchor", null, false],
  ])("the shelf view with %s", (_name, anchor, shown) => {
    renderShelf(
      shelfOf(
        { upcoming: [shelfEntryOf("upcoming", MESSAGE_C)] },
        { picker_anchor_ms: anchor, picker_latest_ms: anchor },
      ),
    );
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    expect(screen.queryByLabelText("Remind at") !== null).toBe(shown);
  });

  test("the toggle button reflects whether the menu is open", () => {
    render(<ShelfMenu presets={[]} shelved={false} onUpdate={vi.fn()} />);
    const toggle = screen.getByRole("button", { name: "Shelf" });
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
    fireEvent.click(toggle);
    expect(toggle.getAttribute("aria-expanded")).toBe("true");
    fireEvent.click(toggle);
    expect(screen.queryByRole("menu")).toBeNull();
  });

  test("entries in the shelf view open a shelved menu for their message", () => {
    const { intents } = renderShelf(
      shelfOf({ upcoming: [shelfEntryOf("upcoming", MESSAGE_C)] }),
    );
    fireEvent.click(screen.getByRole("button", { name: "Shelf" }));
    fireEvent.click(screen.getByRole("button", { name: "In three hours" }));
    expectOnlyCalled(intents, "updateShelf", [
      ROOM_A,
      MESSAGE_C,
      "reschedule",
      "in_three_hours",
      null,
    ]);
  });
});

describe("reminder notice", () => {
  test("shows the author and preview and dismisses", () => {
    const dismiss = vi.fn(async () => {});
    render(
      <ReminderNotice
        entry={shelfEntryOf("due", MESSAGE_A, { preview: "call back" })}
        dismiss={dismiss}
      />,
    );
    const notice = screen.getByRole("status", { name: "Reminder" });
    expect(notice.textContent).toContain("Grace: call back");
    fireEvent.click(screen.getByRole("button", { name: "Dismiss" }));
    expect(dismiss).toHaveBeenCalledOnce();
  });
});
