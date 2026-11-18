import type { BodySegment } from "@pulse/protocol";

export function MessageBody({ segments }: { segments: BodySegment[] }) {
  return (
    <span>
      {segments.map((segment, index) =>
        segment.kind === "mention" ? (
          <strong key={index} className="text-indigo-700">
            {segment.text}
          </strong>
        ) : segment.kind === "link" && segment.url !== null ? (
          <a
            key={index}
            className="underline"
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
