import type { AttachmentView } from "@pulse/protocol";

export function AttachmentLink({ attachment }: { attachment: AttachmentView }) {
  if (attachment.url === null) {
    return <span className="text-subtle">{attachment.file_name}</span>;
  }
  if (attachment.kind === "image") {
    return (
      <img
        alt={attachment.file_name}
        className="max-h-60 rounded-lg border border-line"
        src={attachment.url}
      />
    );
  }
  return (
    <a
      className="inline-flex items-center gap-2 rounded-lg border border-line bg-muted px-3 py-2 font-medium text-ink hover:border-accent hover:text-accent"
      href={attachment.url}
      rel="noreferrer"
      target="_blank"
    >
      {attachment.file_name}
    </a>
  );
}

export function AttachmentList({
  attachments,
}: {
  attachments: AttachmentView[];
}) {
  if (attachments.length === 0) {
    return null;
  }
  return (
    <ul aria-label="Attachments" className="flex flex-wrap gap-2 text-sm">
      {attachments.map((attachment) => (
        <li key={attachment.attachment_id}>
          <AttachmentLink attachment={attachment} />
        </li>
      ))}
    </ul>
  );
}
