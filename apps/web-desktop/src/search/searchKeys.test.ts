import type { SearchSnapshot } from "@pulse/protocol";
import { describe, expect, test } from "vitest";
import {
  MESSAGE_A,
  emptySearch,
  expectOnlyCalled,
  noopIntents,
  searchHitOf,
  searchOf,
  type IntentName,
} from "../test/fixtures";
import {
  runSearchKeyIntent,
  searchKeyIntent,
  type SearchKeyIntent,
} from "./searchKeys";

const open = searchOf([searchHitOf(MESSAGE_A)]);

const input = (
  key: string,
  overrides: Partial<{
    metaKey: boolean;
    ctrlKey: boolean;
    editableTarget: boolean;
  }> = {},
) => ({
  key,
  metaKey: false,
  ctrlKey: false,
  editableTarget: false,
  ...overrides,
});

describe("searchKeyIntent", () => {
  test.each<
    [string, ReturnType<typeof input>, SearchSnapshot, SearchKeyIntent | null]
  >([
    ["cmd+k opens a closed palette", input("k", { metaKey: true }), emptySearch, "toggle_search"],
    ["ctrl+k opens a closed palette", input("k", { ctrlKey: true }), emptySearch, "toggle_search"],
    ["cmd+K with caps lock", input("K", { metaKey: true }), emptySearch, "toggle_search"],
    ["cmd+k closes an open palette", input("k", { metaKey: true }), open, "toggle_search"],
    [
      "cmd+k works from an editable target",
      input("k", { metaKey: true, editableTarget: true }),
      open,
      "toggle_search",
    ],
    ["cmd+j is not a search key", input("j", { metaKey: true }), open, null],
    ["cmd+i is left to the inbox", input("i", { metaKey: true }), emptySearch, null],
    ["ArrowDown moves next", input("ArrowDown"), open, "next"],
    ["ArrowUp moves previous", input("ArrowUp"), open, "previous"],
    ["Enter opens the selection", input("Enter"), open, "open"],
    ["Escape closes the palette", input("Escape"), open, "close"],
    [
      "navigation works from the palette input",
      input("ArrowDown", { editableTarget: true }),
      open,
      "next",
    ],
    ["ArrowDown is ignored while closed", input("ArrowDown"), emptySearch, null],
    ["ArrowUp is ignored while closed", input("ArrowUp"), emptySearch, null],
    ["Enter is ignored while closed", input("Enter"), emptySearch, null],
    ["Escape is ignored while closed", input("Escape"), emptySearch, null],
    ["a plain k does nothing", input("k"), open, null],
    ["a plain j does nothing", input("j"), open, null],
    ["a modified arrow does nothing", input("ArrowDown", { metaKey: true }), open, null],
    ["a modified enter does nothing", input("Enter", { ctrlKey: true }), open, null],
    ["an unmapped key does nothing", input("x"), open, null],
  ])("%s", (_name, key, search, expected) => {
    expect(searchKeyIntent(key, search)).toBe(expected);
  });
});

describe("runSearchKeyIntent", () => {
  test.each<[SearchKeyIntent, IntentName, unknown[]]>([
    ["toggle_search", "toggleSearch", []],
    ["close", "toggleSearch", []],
    ["next", "moveSearchSelection", ["next"]],
    ["previous", "moveSearchSelection", ["previous"]],
    ["open", "openSearchSelection", []],
  ])("%s calls %s once", async (intent, call, args) => {
    const intents = noopIntents();
    await runSearchKeyIntent(intent, intents);
    expectOnlyCalled(intents, call, args);
  });
});
