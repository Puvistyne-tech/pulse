import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { AttachmentKind, VaultSnapshot, VaultTab } from "@pulse/protocol";
import { afterEach, describe, expect, test } from "vitest";
import {
  MESSAGE_A,
  MESSAGE_B,
  MESSAGE_C,
  ROOM_A,
  attachmentViewOf,
  expectOnlyCalled,
  noopIntents,
  vaultEntryOf,
  vaultOf,
} from "../test/fixtures";
import { VaultPanel } from "./VaultPanel";

afterEach(cleanup);

function renderVault(vault: VaultSnapshot) {
  const intents = noopIntents();
  const view = render(<VaultPanel vault={vault} intents={intents} />);
  return { ...view, intents };
}

const tabLabels: ReadonlyArray<[VaultTab, string]> = [
  ["media", "Media"],
  ["files", "Files"],
  ["links", "Links"],
  ["pinned", "Pinned"],
];

describe("tabs", () => {
  test("the four tabs render in order", () => {
    renderVault(vaultOf([]));
    const labels = screen.getAllByRole("tab").map((tab) => tab.textContent);
    expect(labels).toEqual(["Media", "Files", "Links", "Pinned"]);
  });

  test.each(tabLabels)("%s is the selected tab only when current", (current, label) => {
    renderVault(vaultOf([], { tab: current }));
    const selected = screen
      .getAllByRole("tab")
      .filter((tab) => tab.getAttribute("aria-selected") === "true")
      .map((tab) => tab.textContent);
    expect(selected).toEqual([label]);
  });

  test.each(tabLabels)("clicking %s opens that tab in the room", (tab, label) => {
    const { intents } = renderVault(vaultOf([]));
    fireEvent.click(screen.getByRole("tab", { name: label }));
    expectOnlyCalled(intents, "openVault", [ROOM_A, tab]);
  });

  test("tabs are disabled without a room", () => {
    const { intents } = renderVault(vaultOf([], { room_id: null }));
    for (const tab of screen.getAllByRole("tab")) {
      expect((tab as HTMLButtonElement).disabled).toBe(true);
      fireEvent.click(tab);
    }
    expectOnlyCalled(intents, null);
  });
});

describe("entries", () => {
  test("entries render in snapshot order with their author and preview", () => {
    const { container } = renderVault(
      vaultOf([
        vaultEntryOf("links", MESSAGE_B, { link: "https://example.org/b" }),
        vaultEntryOf("links", MESSAGE_A, { link: "https://example.org/a" }),
      ]),
    );
    const rows = [...container.querySelectorAll("li")];
    expect(rows.map((row) => row.querySelector("a")?.textContent)).toEqual([
      "https://example.org/b",
      "https://example.org/a",
    ]);
    expect(rows[0]?.textContent).toContain("Grace");
    expect(rows[0]?.textContent).toContain("preview e2");
  });

  test.each<[string, AttachmentKind, string | null, string, string | null]>([
    ["an image with a ticket", "image", "https://files.example.test/i", "IMG", "https://files.example.test/i"],
    ["a video with a ticket", "video", "https://files.example.test/v", "A", "https://files.example.test/v"],
    ["audio with a ticket", "audio", "https://files.example.test/a", "A", "https://files.example.test/a"],
    ["a file with a ticket", "file", "https://files.example.test/f", "A", "https://files.example.test/f"],
    ["an image without a ticket", "image", null, "SPAN", null],
    ["a file without a ticket", "file", null, "SPAN", null],
  ])("%s", (_name, kind, url, tag, target) => {
    const { container } = renderVault(
      vaultOf([
        vaultEntryOf("media", MESSAGE_A, {
          attachment: attachmentViewOf(kind, { url, file_name: "named" }),
        }),
      ]),
    );
    const item = container.querySelector("li")!;
    const image = item.querySelector("img");
    const anchor = item.querySelector("a");
    const plain = item.querySelector("span:not(.block)");
    expect([image !== null, anchor !== null, plain !== null]).toEqual([
      tag === "IMG",
      tag === "A",
      tag === "SPAN",
    ]);
    expect(image?.getAttribute("alt") ?? "named").toBe("named");
    expect(image?.getAttribute("src") ?? target).toBe(target);
    expect(anchor?.getAttribute("href") ?? target).toBe(target);
    expect(anchor?.getAttribute("rel") ?? "noreferrer").toBe("noreferrer");
    expect((anchor ?? plain)?.textContent ?? "named").toBe("named");
  });

  test("links open in a new window without a referrer", () => {
    renderVault(
      vaultOf([vaultEntryOf("links", MESSAGE_A, { link: "https://example.org/a" })]),
    );
    const anchor = screen.getByRole("link", { name: "https://example.org/a" });
    expect(anchor.getAttribute("href")).toBe("https://example.org/a");
    expect(anchor.getAttribute("rel")).toBe("noreferrer");
    expect(anchor.getAttribute("target")).toBe("_blank");
  });

  test("pinned entries show their preview text", () => {
    renderVault(
      vaultOf([vaultEntryOf("pinned", MESSAGE_C, { pinned_at_ms: 5, preview: "keep this" })]),
    );
    expect(screen.getByText("keep this")).not.toBeNull();
  });
});

describe("status and paging", () => {
  test.each<[string, Partial<VaultSnapshot>, boolean, boolean]>([
    ["loading", { loading: true }, true, false],
    ["idle", { loading: false }, false, false],
    ["more pages", { has_more: true }, false, true],
    ["loading with more pages", { loading: true, has_more: true }, true, true],
  ])("%s", (_name, overrides, loading, loadMore) => {
    renderVault(vaultOf([], overrides));
    expect(screen.queryByText("Loading") !== null).toBe(loading);
    expect(screen.queryByRole("button", { name: "Load more" }) !== null).toBe(loadMore);
  });

  test("load more asks for the next page", () => {
    const { intents } = renderVault(vaultOf([], { has_more: true }));
    fireEvent.click(screen.getByRole("button", { name: "Load more" }));
    expectOnlyCalled(intents, "loadMoreVault");
  });

  test.each<[string, VaultSnapshot["error"], string | null]>([
    ["no error", null, null],
    ["forbidden", { code: "forbidden", field: null }, "You are not allowed to do that"],
    ["unknown room", { code: "not_found", field: null }, "That item could not be found"],
  ])("error %s", (_name, error, notice) => {
    renderVault(vaultOf([], { error }));
    expect(screen.queryByRole("alert")?.textContent ?? null).toBe(notice);
  });

  test("the close button closes the vault", () => {
    const { intents } = renderVault(vaultOf([]));
    fireEvent.click(screen.getByRole("button", { name: "Close vault" }));
    expectOnlyCalled(intents, "closeVault");
  });
});
