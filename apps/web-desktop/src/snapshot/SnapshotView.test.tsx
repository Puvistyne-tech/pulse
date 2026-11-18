import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, expect, test } from "vitest";
import { snapshotFixtures } from "../test/fixtures";
import { SnapshotView } from "./SnapshotView";

afterEach(cleanup);

test.each(snapshotFixtures)("renders %s snapshot", (_name, snapshot, expected) => {
  render(<SnapshotView snapshot={snapshot} />);
  expect(screen.getByText(expected)).toBeTruthy();
});
