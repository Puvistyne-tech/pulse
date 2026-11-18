import type { AttachmentView } from "@pulse/protocol";

export function AttachmentList({
  attachments,
}: {
  attachments: AttachmentView[];
}) {
  if (attachments.length === 0) {
    return null;
  }
  return (
    <ul aria-label="Attachments" className="space-y-1 text-sm">
      {attachments.map((attachment) => (
        <li key={attachment.attachment_id}>
          {attachment.url === null ? (
            <span>{attachment.file_name}</span>
          ) : attachment.kind === "image" ? (
            <img
              alt={attachment.file_name}
              className="max-h-48 rounded"
              src={attachment.url}
            />
          ) : (
            <a
              className="underline"
              href={attachment.url}
              rel="noreferrer"
              target="_blank"
            >
              {attachment.file_name}
            </a>
          )}
        </li>
      ))}
    </ul>
  );
}
