import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, expect, test } from "vitest";
import { networkFixtures } from "../test/fixtures";
import { SnapshotView } from "./SnapshotView";

afterEach(cleanup);

test.each(networkFixtures)("renders %s network", (_name, network, expected) => {
  render(<SnapshotView network={network} />);
  expect(screen.getByText(expected)).toBeTruthy();
});
