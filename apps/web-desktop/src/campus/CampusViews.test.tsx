import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, test } from "vitest";
import {
  CAMPUS_A,
  CAMPUS_B,
  campusA,
  campusB,
  noopIntents,
  rooms,
} from "../test/fixtures";
import { CampusPicker } from "./CampusPicker";
import { CreateCampusForm } from "./CreateCampusForm";
import { RoomList } from "./RoomList";

afterEach(cleanup);

test.each([
  ["plain", "gamma", "Gamma Campus"],
  ["untrimmed", " gamma ", " Gamma  Campus "],
])("create campus submits %s input as typed", (_name, slug, displayName) => {
  const intents = noopIntents();
  render(<CreateCampusForm busy={false} createCampus={intents.createCampus} />);
  fireEvent.change(screen.getByLabelText("Campus slug"), {
    target: { value: slug },
  });
  fireEvent.change(screen.getByLabelText("Campus name"), {
    target: { value: displayName },
  });
  fireEvent.click(screen.getByRole("button", { name: "Create campus" }));
  expect(intents.createCampus).toHaveBeenCalledExactlyOnceWith({
    slug,
    display_name: displayName,
  });
});

test.each([
  [true, true],
  [false, false],
])("busy %s disables create campus %s", (busy, disabled) => {
  render(<CreateCampusForm busy={busy} createCampus={noopIntents().createCampus} />);
  const button = screen.getByRole("button", { name: "Create campus" });
  expect((button as HTMLButtonElement).disabled).toBe(disabled);
});

test.each([
  ["Alpha Campus", CAMPUS_A],
  ["Beta Campus", CAMPUS_B],
])("picking %s selects its campus id", (label, campusId) => {
  const intents = noopIntents();
  render(
    <CampusPicker
      campuses={[campusA, campusB]}
      selectCampus={intents.selectCampus}
    />,
  );
  fireEvent.click(screen.getByText(label));
  expect(intents.selectCampus).toHaveBeenCalledExactlyOnceWith(campusId);
});

test.each([
  ["both default rooms", rooms, ["#commons", "#courtyard"]],
  ["reversed snapshot order", [...rooms].reverse(), ["#courtyard", "#commons"]],
  ["no rooms", [], []],
])("room list renders %s", (_name, list, expected) => {
  render(<RoomList rooms={list} />);
  const labels = screen.queryAllByRole("listitem").map((item) => item.textContent);
  expect(labels).toEqual(expected);
});
