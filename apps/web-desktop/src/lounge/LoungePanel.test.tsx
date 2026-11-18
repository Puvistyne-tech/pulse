import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import type { LoungeSnapshot } from "@pulse/protocol";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import {
  LOUNGE_A,
  LOUNGE_B,
  emptyLounges,
  expectOnlyCalled,
  loungeEntryOf,
  loungesOf,
  noopIntents,
  participantOf,
  ticketOf,
} from "../test/fixtures";
import { LoungePanel } from "./LoungePanel";
import { connectLoungeMedia } from "./loungeMedia";

vi.mock("./loungeMedia", () => ({ connectLoungeMedia: vi.fn(() => vi.fn()) }));

const connect = vi.mocked(connectLoungeMedia);

beforeEach(() => connect.mockClear());
afterEach(cleanup);

function renderPanel(lounges: LoungeSnapshot) {
  const intents = noopIntents();
  const view = render(<LoungePanel lounges={lounges} intents={intents} />);
  return { ...view, intents };
}

const campusLounge = loungeEntryOf("campus", {
  participants: [participantOf("Grace"), participantOf("Linus", { can_speak: false })],
});
const roomLounge = loungeEntryOf("room");
const joinedCampus = { ...campusLounge, joined: true };

describe("rendering", () => {
  test("an empty snapshot renders nothing", () => {
    const { container } = renderPanel(emptyLounges);
    expect(container.firstChild).toBeNull();
  });

  test("lounges render in snapshot order with their scope labels", () => {
    renderPanel(loungesOf([roomLounge, campusLounge]));
    const rows = screen.getAllByRole("listitem").filter((row) => row.hasAttribute("data-joined"));
    expect(rows.map((row) => row.querySelector("span.font-medium")?.textContent)).toEqual([
      "#commons lounge",
      "Campus lounge",
    ]);
  });

  test("participants show names, self and listeners", () => {
    renderPanel(
      loungesOf([
        {
          ...campusLounge,
          joined: true,
          participants: [participantOf("Ada", { from_self: true }), ...campusLounge.participants],
        },
      ]),
    );
    const list = screen.getByRole("list", { name: "Participants" });
    expect(within(list).getAllByRole("listitem").map((item) => item.textContent)).toEqual([
      "You",
      "Grace",
      "Linus listening",
    ]);
  });

  test.each<[string, Partial<LoungeSnapshot>, string | null]>([
    ["idle shows no status", { media: "idle" }, null],
    ["connecting", { media: "connecting" }, "Connecting audio"],
    ["connected", { media: "connected" }, "Audio connected"],
    ["failed", { media: "failed" }, "Audio unavailable"],
  ])("media %s", (_name, overrides, status) => {
    renderPanel(loungesOf([joinedCampus], overrides));
    expect(screen.queryByRole("status")?.textContent ?? null).toBe(status);
  });

  test("media status is hidden while not joined", () => {
    renderPanel(loungesOf([campusLounge], { media: "connected" }));
    expect(screen.queryByRole("status")).toBeNull();
  });

  test.each<[string, Partial<LoungeSnapshot>, string | null]>([
    ["no error", {}, null],
    [
      "unavailable",
      { error: { code: "unavailable", field: null } },
      "Pulse server unavailable",
    ],
  ])("error %s", (_name, overrides, notice) => {
    renderPanel(loungesOf([campusLounge], overrides));
    expect(screen.queryByRole("alert")?.textContent ?? null).toBe(notice);
  });
});

describe("join and leave", () => {
  test.each<[string, LoungeSnapshot, string, string]>([
    ["the campus lounge", loungesOf([campusLounge, roomLounge]), "Join Campus lounge", LOUNGE_A],
    ["a room lounge", loungesOf([campusLounge, roomLounge]), "Join #commons lounge", LOUNGE_B],
  ])("joining %s calls the client", (_name, lounges, label, loungeId) => {
    const { intents } = renderPanel(lounges);
    fireEvent.click(screen.getByRole("button", { name: label }));
    expectOnlyCalled(intents, "joinLounge", [loungeId]);
  });

  test("leaving calls the client", () => {
    const { intents } = renderPanel(loungesOf([joinedCampus, roomLounge]));
    fireEvent.click(screen.getByRole("button", { name: "Leave" }));
    expectOnlyCalled(intents, "leaveLounge");
  });

  test.each<[string, Partial<LoungeSnapshot>, string[]]>([
    ["joinable", {}, ["Join Campus lounge", "Join #commons lounge"]],
    ["not allowed to join", { can_join: false }, []],
    ["joining", { joining: true }, ["Join Campus lounge", "Join #commons lounge"]],
  ])("join buttons when %s", (_name, overrides, labels) => {
    renderPanel(loungesOf([campusLounge, roomLounge], overrides));
    const buttons = screen.queryAllByRole("button", { name: /^Join/ });
    expect(buttons.map((button) => button.getAttribute("aria-label"))).toEqual(labels);
    const joining = overrides.joining === true;
    expect(buttons.every((button) => (button as HTMLButtonElement).disabled === joining)).toBe(true);
  });
});

describe("media connection", () => {
  test("a ticket connects with the speaking permission", () => {
    renderPanel(loungesOf([joinedCampus], { ticket: ticketOf(), can_speak: false }));
    expect(connect).toHaveBeenCalledExactlyOnceWith(ticketOf(), false, expect.any(Function));
  });

  test("no ticket never connects", () => {
    renderPanel(loungesOf([campusLounge]));
    expect(connect).not.toHaveBeenCalled();
  });

  test("clearing the ticket runs the disconnect", () => {
    const disconnect = vi.fn();
    connect.mockReturnValueOnce(disconnect);
    const intents = noopIntents();
    const { rerender } = render(
      <LoungePanel lounges={loungesOf([joinedCampus], { ticket: ticketOf() })} intents={intents} />,
    );
    expect(disconnect).not.toHaveBeenCalled();
    rerender(<LoungePanel lounges={loungesOf([campusLounge])} intents={intents} />);
    expect(disconnect).toHaveBeenCalledOnce();
  });

  test.each<[string, Parameters<typeof ticketOf>[0], number]>([
    ["a refreshed token on the same room", { token: "ticket-token-b", expires_at_ms: 900_000 }, 1],
    ["another room name", { room_name: "pulse-other" }, 2],
    ["another server", { livekit_url: "ws://127.0.0.1:7999" }, 2],
  ])("%s", (_name, change, connections) => {
    const intents = noopIntents();
    const { rerender } = render(
      <LoungePanel lounges={loungesOf([joinedCampus], { ticket: ticketOf() })} intents={intents} />,
    );
    rerender(
      <LoungePanel
        lounges={loungesOf([joinedCampus], { ticket: ticketOf(change) })}
        intents={intents}
      />,
    );
    expect(connect).toHaveBeenCalledTimes(connections);
  });
});
