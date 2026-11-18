import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { SearchSnapshot } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  MESSAGE_A,
  MESSAGE_B,
  MESSAGE_C,
  expectOnlyCalled,
  noopIntents,
  searchHitOf,
  searchOf,
  snippetOf,
} from "../test/fixtures";
import { SearchPalette } from "./SearchPalette";

afterEach(cleanup);

function renderPalette(search: SearchSnapshot) {
  const intents = noopIntents();
  const view = render(<SearchPalette search={search} intents={intents} />);
  return { ...view, intents };
}

const threeHits = searchOf(
  [
    searchHitOf(MESSAGE_A, { snippet: snippetOf(["first ", false], ["alpha", true]) }),
    searchHitOf(MESSAGE_B, { topic_title: "Plans", author_display_name: "Linus" }),
    searchHitOf(MESSAGE_C, { snippet: snippetOf(["alpha", true], [" last", false]) }),
  ],
  { selected_index: 1 },
);

describe("results", () => {
  test("hits render in snapshot order", () => {
    renderPalette(threeHits);
    const rows = screen.getAllByRole("listitem");
    expect(rows).toHaveLength(3);
    expect(rows[0]?.textContent).toContain("first alpha");
    expect(rows[1]?.textContent).toContain("Plans");
    expect(rows[2]?.textContent).toContain("alpha last");
  });

  test.each<[string, SearchSnapshot, number | null]>([
    ["the snapshot selection", threeHits, 1],
    ["the first hit", searchOf(threeHits.hits), 0],
    ["no selection", searchOf(threeHits.hits, { selected_index: null }), null],
  ])("%s is the aria-selected row", (_name, search, selected) => {
    renderPalette(search);
    const flags = screen
      .getAllByRole("listitem")
      .map((row) => row.getAttribute("aria-selected"));
    expect(flags).toEqual(
      [0, 1, 2].map((index) => String(index === selected)),
    );
  });

  test("matched spans are marked and others are not", () => {
    const { container } = renderPalette(threeHits);
    const marked = [...container.querySelectorAll("mark")].map(
      (node) => node.textContent,
    );
    expect(marked).toEqual(["alpha", "alpha", "alpha"]);
  });

  test.each<[string, SearchSnapshot, string[], string[]]>([
    ["no hits", searchOf([]), [], []],
    ["a topic title", threeHits, ["Plans"], ["Linus"]],
  ])("%s", (_name, search, strong, authors) => {
    const { container } = renderPalette(search);
    expect([...container.querySelectorAll("strong")].map((n) => n.textContent?.trim())).toEqual(strong);
    for (const author of authors) {
      expect(container.textContent).toContain(author);
    }
  });
});

describe("status", () => {
  test.each<[string, Partial<SearchSnapshot>, string | null]>([
    ["searching shows progress", { searching: true }, "Searching"],
    ["idle shows no progress", { searching: false }, null],
  ])("%s", (_name, overrides, text) => {
    renderPalette(searchOf([], overrides));
    expect(screen.queryByText("Searching") !== null).toBe(text !== null);
  });

  test.each<[string, SearchSnapshot["error"], string | null]>([
    ["no error", null, null],
    ["unavailable", { code: "unavailable", field: null }, "Pulse server unavailable"],
  ])("error %s", (_name, error, notice) => {
    renderPalette(searchOf([], { error }));
    expect(screen.queryByRole("alert")?.textContent ?? null).toBe(notice);
  });
});

describe("intents", () => {
  test.each(["a", "alpha beta", ""])("typing %j searches for it", (query) => {
    const { intents } = renderPalette(searchOf([], { query: "seed" }));
    fireEvent.change(screen.getByLabelText("Search messages"), {
      target: { value: query },
    });
    expectOnlyCalled(intents, "search", [query]);
  });

  test.each([
    [0, MESSAGE_A],
    [1, MESSAGE_B],
    [2, MESSAGE_C],
  ])("clicking hit %i opens its message", (index, messageId) => {
    const { intents } = renderPalette(threeHits);
    const buttons = screen
      .getAllByRole("listitem")
      .map((row) => row.querySelector("button")!);
    fireEvent.click(buttons[index]!);
    expectOnlyCalled(intents, "openSearchHit", [messageId]);
  });

  test("the close button toggles the palette", () => {
    const { intents } = renderPalette(threeHits);
    fireEvent.click(screen.getByRole("button", { name: "Close search" }));
    expectOnlyCalled(intents, "toggleSearch");
  });

  test("the input starts with the snapshot query", () => {
    renderPalette(searchOf([], { query: "seed" }));
    expect((screen.getByLabelText("Search messages") as HTMLInputElement).value).toBe("seed");
  });
});
