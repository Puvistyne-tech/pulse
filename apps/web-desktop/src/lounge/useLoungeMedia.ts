import { useEffect, useRef } from "react";
import type { LoungeMediaState, LoungeTicket } from "@pulse/protocol";
import { connectLoungeMedia } from "./loungeMedia";

export function useLoungeMedia(
  ticket: LoungeTicket | null,
  canSpeak: boolean,
  report: (state: LoungeMediaState) => Promise<void>,
): void {
  const latest = useRef(ticket);
  latest.current = ticket;
  const reporter = useRef(report);
  reporter.current = report;
  const key =
    ticket === null ? null : `${ticket.livekit_url}|${ticket.room_name}`;

  useEffect(() => {
    const current = latest.current;
    if (key === null || current === null) {
      return;
    }
    return connectLoungeMedia(current, canSpeak, (state) => {
      reporter.current(state).catch(() => undefined);
    });
  }, [key, canSpeak]);
}
