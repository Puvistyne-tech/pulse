import type { LoungeMediaState, LoungeTicket } from "@pulse/protocol";
import type { Room } from "livekit-client";

export function connectLoungeMedia(
  ticket: LoungeTicket,
  canSpeak: boolean,
  report: (state: LoungeMediaState) => void,
): () => void {
  let cancelled = false;
  let room: Room | null = null;
  const attached = new Set<HTMLElement>();

  const run = async () => {
    const { Room: LivekitRoom, RoomEvent, Track } = await import("livekit-client");
    if (cancelled) {
      return;
    }
    const next = new LivekitRoom();
    room = next;
    next.on(RoomEvent.TrackSubscribed, (track) => {
      if (track.kind === Track.Kind.Audio) {
        const element = track.attach();
        attached.add(element);
        document.body.appendChild(element);
      }
    });
    next.on(RoomEvent.TrackUnsubscribed, (track) => {
      for (const element of track.detach()) {
        attached.delete(element);
        element.remove();
      }
    });
    next.on(RoomEvent.Disconnected, () => {
      if (!cancelled) {
        report("failed");
      }
    });
    report("connecting");
    await next.connect(ticket.livekit_url, ticket.token);
    if (cancelled) {
      return;
    }
    if (canSpeak) {
      await next.localParticipant.setMicrophoneEnabled(true);
    }
    if (!cancelled) {
      report("connected");
    }
  };

  run().catch(() => {
    if (!cancelled) {
      report("failed");
    }
  });

  return () => {
    cancelled = true;
    for (const element of attached) {
      element.remove();
    }
    attached.clear();
    void room?.disconnect();
  };
}
