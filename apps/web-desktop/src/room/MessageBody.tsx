import type { BodySegment } from "@pulse/protocol";

export function MessageBody({ segments }: { segments: BodySegment[] }) {
  return (
    <span>
      {segments.map((segment, index) =>
        segment.kind === "mention" ? (
          <strong key={index} className="rounded bg-accent-soft px-1 font-semibold text-accent">
            {segment.text}
          </strong>
        ) : segment.kind === "link" && segment.url !== null ? (
          <a
            key={index}
            className="text-accent underline decoration-accent/40 underline-offset-2 hover:decoration-accent"
            href={segment.url}
            rel="noreferrer"
            target="_blank"
          >
            {segment.text}
          </a>
        ) : (
          <span key={index}>{segment.text}</span>
        ),
      )}
    </span>
  );
}
