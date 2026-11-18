import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, test } from "vitest";
import {
  CAMPUS_A,
  CAMPUS_B,
  ROOM_A,
  ROOM_B,
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
  render(
    <RoomList
      rooms={list}
      activeRoomId={null}
      selectRoom={noopIntents().selectRoom}
    />,
  );
  const labels = screen.queryAllByRole("listitem").map((item) => item.textContent);
  expect(labels).toEqual(expected);
});

test.each([
  ["#commons", ROOM_A],
  ["#courtyard", ROOM_B],
])("clicking %s selects its room id", (label, roomId) => {
  const intents = noopIntents();
  render(
    <RoomList rooms={rooms} activeRoomId={null} selectRoom={intents.selectRoom} />,
  );
  fireEvent.click(screen.getByText(label));
  expect(intents.selectRoom).toHaveBeenCalledExactlyOnceWith(roomId);
});

test.each([
  ["first room", ROOM_A, "#commons"],
  ["second room", ROOM_B, "#courtyard"],
  ["no active room", null, null],
])("active room marking for %s", (_name, activeRoomId, marked) => {
  render(
    <RoomList
      rooms={rooms}
      activeRoomId={activeRoomId}
      selectRoom={noopIntents().selectRoom}
    />,
  );
  const current = screen
    .getAllByRole("button")
    .filter((button) => button.getAttribute("aria-current") === "true")
    .map((button) => button.textContent);
  expect(current).toEqual(marked === null ? [] : [marked]);
});
