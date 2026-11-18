import { expect, test } from "vitest";
import { signedInWithRooms } from "../test/fixtures";
import { newerSnapshot } from "./snapshotOrder";

const at = (sequence: number) => ({ ...signedInWithRooms, sequence });

test.each([
  ["no current snapshot", null, at(3), "next"],
  ["older current", at(2), at(3), "next"],
  ["equal sequence", at(3), at(3), "next"],
  ["newer current", at(4), at(3), "current"],
] as const)("%s keeps the %s snapshot", (_name, current, next, winner) => {
  const result = newerSnapshot(current, next);
  expect(result).toBe(winner === "next" ? next : current);
});

test("a stale sequence never regresses a folded stream", () => {
  const stream = [at(2), at(5), at(3), at(5), at(4), at(6)];
  const folded = stream.reduce<ReturnType<typeof at> | null>(
    (current, next) => newerSnapshot(current, next),
    null,
  );
  expect(folded?.sequence).toBe(6);
});
