import type { BodySegment } from "@pulse/protocol";

export function MessageBody({ segments }: { segments: BodySegment[] }) {
  return (
    <span>
      {segments.map((segment, index) =>
        segment.kind === "mention" ? (
          <strong key={index} className="text-indigo-700">
            {segment.text}
          </strong>
        ) : (
          <span key={index}>{segment.text}</span>
        ),
      )}
    </span>
  );
}
