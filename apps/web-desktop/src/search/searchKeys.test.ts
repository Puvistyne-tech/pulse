import type { SearchSnapshot } from "@pulse/protocol";
import { describe, expect, test } from "vitest";
import {
  MESSAGE_A,
  emptySearch,
  expectOnlyCalled,
  keyInput,
  noopIntents,
  searchHitOf,
  searchOf,
  type IntentName,
} from "../test/fixtures";
import type { KeyInput } from "../inbox/inboxKeys";
import {
  runSearchKeyIntent,
  searchKeyIntent,
  type SearchKeyIntent,
} from "./searchKeys";

const open = searchOf([searchHitOf(MESSAGE_A)]);

describe("searchKeyIntent", () => {
  test.each<
    [string, KeyInput, SearchSnapshot, SearchKeyIntent | null]
  >([
    ["cmd+k opens a closed palette", keyInput("k", { metaKey: true }), emptySearch, "toggle_search"],
    ["ctrl+k opens a closed palette", keyInput("k", { ctrlKey: true }), emptySearch, "toggle_search"],
    ["cmd+K with caps lock", keyInput("K", { metaKey: true }), emptySearch, "toggle_search"],
    ["cmd+k closes an open palette", keyInput("k", { metaKey: true }), open, "toggle_search"],
    [
      "cmd+k works from an editable target",
      keyInput("k", { metaKey: true, editableTarget: true }),
      open,
      "toggle_search",
    ],
    ["cmd+j is not a search key", keyInput("j", { metaKey: true }), open, null],
    ["cmd+i is left to the inbox", keyInput("i", { metaKey: true }), emptySearch, null],
    ["ArrowDown moves next", keyInput("ArrowDown"), open, "next"],
    ["ArrowUp moves previous", keyInput("ArrowUp"), open, "previous"],
    ["Enter opens the selection", keyInput("Enter"), open, "open"],
    ["Escape closes the palette", keyInput("Escape"), open, "close"],
    [
      "navigation works from the palette input",
      keyInput("ArrowDown", { editableTarget: true }),
      open,
      "next",
    ],
    ["ArrowDown is ignored while closed", keyInput("ArrowDown"), emptySearch, null],
    ["ArrowUp is ignored while closed", keyInput("ArrowUp"), emptySearch, null],
    ["Enter is ignored while closed", keyInput("Enter"), emptySearch, null],
    ["Escape is ignored while closed", keyInput("Escape"), emptySearch, null],
    ["a plain k does nothing", keyInput("k"), open, null],
    ["a plain j does nothing", keyInput("j"), open, null],
    ["a modified arrow does nothing", keyInput("ArrowDown", { metaKey: true }), open, null],
    ["a modified enter does nothing", keyInput("Enter", { ctrlKey: true }), open, null],
    ["an unmapped key does nothing", keyInput("x"), open, null],
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
