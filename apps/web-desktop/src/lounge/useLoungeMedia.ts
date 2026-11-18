import { useEffect, useRef } from "react";
import type { LoungeTicket } from "@pulse/protocol";
import { pulseClient } from "../client/pulseClient";
import { connectLoungeMedia } from "./loungeMedia";

export function useLoungeMedia(
  ticket: LoungeTicket | null,
  canSpeak: boolean,
): void {
  const latest = useRef(ticket);
  latest.current = ticket;
  const key =
    ticket === null ? null : `${ticket.livekit_url}|${ticket.room_name}`;

  useEffect(() => {
    const current = latest.current;
    if (key === null || current === null) {
      return;
    }
    return connectLoungeMedia(current, canSpeak, (state) => {
      pulseClient.reportLoungeMedia(state).catch(() => undefined);
    });
  }, [key, canSpeak]);
}
