import type { LoungeMediaState } from "@pulse/protocol";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { ticketOf } from "../test/fixtures";
import { connectLoungeMedia } from "./loungeMedia";

const fake = vi.hoisted(() => {
  type Handler = (...args: unknown[]) => void;
  class FakeRoom {
    handlers = new Map<string, Handler>();
    connect = vi.fn(async (_url: string, _token: string) => {
      await fake.connectGate;
      if (fake.connectError !== null) {
        throw fake.connectError;
      }
    });
    disconnect = vi.fn(async () => {});
    localParticipant = {
      setMicrophoneEnabled: vi.fn(async (_enabled: boolean) => {
        if (fake.microphoneError !== null) {
          throw fake.microphoneError;
        }
      }),
    };
    constructor() {
      fake.rooms.push(this);
    }
    on(event: string, handler: Handler) {
      this.handlers.set(event, handler);
      return this;
    }
    emit(event: string, ...args: unknown[]) {
      this.handlers.get(event)?.(...args);
    }
  }
  const fake = {
    rooms: [] as FakeRoom[],
    connectError: null as Error | null,
    connectGate: null as Promise<void> | null,
    microphoneError: null as Error | null,
    FakeRoom,
  };
  return fake;
});

vi.mock("livekit-client", () => ({
  Room: fake.FakeRoom,
  RoomEvent: {
    TrackSubscribed: "trackSubscribed",
    TrackUnsubscribed: "trackUnsubscribed",
    Disconnected: "disconnected",
  },
  Track: { Kind: { Audio: "audio", Video: "video" } },
}));

const ticket = ticketOf();

function start(canSpeak: boolean) {
  const reports: LoungeMediaState[] = [];
  const stop = connectLoungeMedia(ticket, canSpeak, (state) =>
    reports.push(state),
  );
  return { reports, stop };
}

const settled = (reports: LoungeMediaState[], last: LoungeMediaState) =>
  vi.waitFor(() => expect(reports.at(-1)).toBe(last));

beforeEach(() => {
  fake.rooms.length = 0;
  fake.connectError = null;
  fake.connectGate = null;
  fake.microphoneError = null;
});

afterEach(() => {
  document.body.innerHTML = "";
});

describe("connecting", () => {
  test.each<[string, boolean, boolean]>([
    ["a speaker publishes the microphone", true, true],
    ["a listener never touches the microphone", false, false],
  ])("%s", async (_name, canSpeak, microphoneEnabled) => {
    const { reports } = start(canSpeak);
    await settled(reports, "connected");
    expect(reports).toEqual(["connecting", "connected"]);
    const [room] = fake.rooms;
    expect(room?.connect).toHaveBeenCalledExactlyOnceWith(
      ticket.livekit_url,
      ticket.token,
    );
    expect(room?.localParticipant.setMicrophoneEnabled.mock.calls).toEqual(
      microphoneEnabled ? [[true]] : [],
    );
  });

  test.each<[string, Error | null, Error | null]>([
    ["a connect error", new Error("refused"), null],
    ["a microphone error", null, new Error("denied")],
  ])("%s reports failed", async (_name, connectError, microphoneError) => {
    fake.connectError = connectError;
    fake.microphoneError = microphoneError;
    const { reports } = start(true);
    await settled(reports, "failed");
    expect(reports).toEqual(["connecting", "failed"]);
  });

  test("an unexpected disconnect reports failed", async () => {
    const { reports } = start(false);
    await settled(reports, "connected");
    fake.rooms[0]?.emit("disconnected");
    expect(reports).toEqual(["connecting", "connected", "failed"]);
  });
});

describe("cleanup", () => {
  test("stopping before the module loads creates no room and reports nothing", async () => {
    const { reports, stop } = start(true);
    stop();
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(fake.rooms).toHaveLength(0);
    expect(reports).toEqual([]);
  });

  test("stopping after connecting disconnects once and goes quiet", async () => {
    const { reports, stop } = start(true);
    await settled(reports, "connected");
    stop();
    fake.rooms[0]?.emit("disconnected");
    expect(fake.rooms[0]?.disconnect).toHaveBeenCalledOnce();
    expect(reports).toEqual(["connecting", "connected"]);
  });

  test("stopping while connecting suppresses the late report", async () => {
    let release: () => void = () => {};
    const gate = new Promise<void>((resolve) => {
      release = resolve;
    });
    fake.connectGate = gate;
    const { reports, stop } = start(true);
    await vi.waitFor(() => expect(fake.rooms).toHaveLength(1));
    stop();
    release();
    await gate;
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(fake.rooms[0]?.disconnect).toHaveBeenCalledOnce();
    expect(reports).toEqual(["connecting"]);
  });
});

describe("remote audio", () => {
  const trackOf = (kind: string) => {
    const element = document.createElement("audio");
    return {
      kind,
      element,
      attach: vi.fn(() => element),
      detach: vi.fn(() => [element]),
    };
  };

  test.each<[string, string, boolean]>([
    ["an audio track is attached", "audio", true],
    ["a video track is ignored", "video", false],
  ])("%s", async (_name, kind, attached) => {
    const { reports } = start(false);
    await settled(reports, "connected");
    const track = trackOf(kind);
    fake.rooms[0]?.emit("trackSubscribed", track);
    expect(document.body.contains(track.element)).toBe(attached);
    expect(track.attach).toHaveBeenCalledTimes(attached ? 1 : 0);
  });

  test("unsubscribing and stopping remove attached audio", async () => {
    const { reports, stop } = start(false);
    await settled(reports, "connected");
    const first = trackOf("audio");
    const second = trackOf("audio");
    fake.rooms[0]?.emit("trackSubscribed", first);
    fake.rooms[0]?.emit("trackSubscribed", second);
    fake.rooms[0]?.emit("trackUnsubscribed", first);
    expect(document.body.contains(first.element)).toBe(false);
    expect(document.body.contains(second.element)).toBe(true);
    stop();
    expect(document.body.contains(second.element)).toBe(false);
  });
});
