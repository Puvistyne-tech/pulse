import type { InboxSnapshot } from "@pulse/protocol";
import { describe, expect, test } from "vitest";
import {
  emptyInbox,
  expectOnlyCalled,
  noopIntents,
  type IntentName,
} from "../test/fixtures";
import {
  inboxKeyIntent,
  runInboxKeyIntent,
  type InboxKeyIntent,
  type KeyInput,
} from "./inboxKeys";

const plain = (key: string, editableTarget = false): KeyInput => ({
  key,
  metaKey: false,
  ctrlKey: false,
  editableTarget,
});

const withMeta = (key: string, editableTarget = false): KeyInput => ({
  ...plain(key, editableTarget),
  metaKey: true,
});

const withCtrl = (key: string, editableTarget = false): KeyInput => ({
  ...plain(key, editableTarget),
  ctrlKey: true,
});

const closed: InboxSnapshot = emptyInbox;
const open: InboxSnapshot = { ...emptyInbox, open: true };
const replying: InboxSnapshot = { ...open, replying: true };

type Row = [string, KeyInput, InboxSnapshot, InboxKeyIntent | null];

const rows: Row[] = [
  ["cmd+i toggles a closed inbox", withMeta("i"), closed, "toggle_inbox"],
  ["cmd+i toggles an open inbox", withMeta("i"), open, "toggle_inbox"],
  ["ctrl+i toggles", withCtrl("i"), closed, "toggle_inbox"],
  ["cmd+I lowercases", withMeta("I"), closed, "toggle_inbox"],
  ["ctrl+I lowercases", withCtrl("I"), open, "toggle_inbox"],
  ["cmd+i works from an editable target", withMeta("i", true), open, "toggle_inbox"],
  ["cmd+j is not next", withMeta("j"), open, null],
  ["ctrl+e is not archive", withCtrl("e"), open, null],
  ["cmd+r is not reply", withMeta("r"), open, null],
  ["cmd+escape is ignored", withMeta("Escape"), replying, null],
  ["j moves next", plain("j"), open, "next"],
  ["k moves previous", plain("k"), open, "previous"],
  ["e archives", plain("e"), open, "archive"],
  ["r replies", plain("r"), open, "reply"],
  ["J lowercases to next", plain("J"), open, "next"],
  ["K lowercases to previous", plain("K"), open, "previous"],
  ["E lowercases to archive", plain("E"), open, "archive"],
  ["R lowercases to reply", plain("R"), open, "reply"],
  ["j is ignored while the inbox is closed", plain("j"), closed, null],
  ["k is ignored while the inbox is closed", plain("k"), closed, null],
  ["e is ignored while the inbox is closed", plain("e"), closed, null],
  ["r is ignored while the inbox is closed", plain("r"), closed, null],
  ["j is ignored in an editable target", plain("j", true), open, null],
  ["r is ignored in an editable target", plain("r", true), open, null],
  ["unmapped key is ignored", plain("x"), open, null],
  ["digit is ignored", plain("1"), open, null],
  ["escape cancels a reply", plain("Escape"), replying, "cancel_reply"],
  [
    "escape cancels a reply from an editable target",
    plain("Escape", true),
    replying,
    "cancel_reply",
  ],
  ["escape without a reply is ignored", plain("Escape"), open, null],
  ["escape on a closed inbox is ignored", plain("Escape"), closed, null],
];

describe("inboxKeyIntent", () => {
  test.each(rows)("%s", (_name, input, inbox, expected) => {
    expect(inboxKeyIntent(input, inbox)).toBe(expected);
  });
});

const runs: ReadonlyArray<[InboxKeyIntent, IntentName, unknown[]]> = [
  ["toggle_inbox", "toggleInbox", []],
  ["next", "moveInboxSelection", ["next"]],
  ["previous", "moveInboxSelection", ["previous"]],
  ["archive", "archiveInboxSelection", []],
  ["reply", "startInboxReply", []],
  ["cancel_reply", "cancelInboxReply", []],
];

describe("runInboxKeyIntent", () => {
  test.each(runs)("%s calls %s once", async (intent, call, args) => {
    const intents = noopIntents();
    await runInboxKeyIntent(intent, intents);
    expectOnlyCalled(intents, call, args);
  });
});
