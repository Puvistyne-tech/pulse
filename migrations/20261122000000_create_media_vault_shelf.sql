CREATE TABLE attachments (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    attachment_id UUID NOT NULL,
    uploader_id UUID NOT NULL,
    object_key TEXT NOT NULL,
    file_name TEXT NOT NULL,
    content_type TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('image', 'video', 'audio', 'file')),
    byte_size BIGINT NOT NULL CHECK (byte_size > 0),
    message_id UUID,
    position SMALLINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, attachment_id),
    UNIQUE (object_key),
    FOREIGN KEY (campus_id, room_id) REFERENCES rooms(campus_id, room_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, uploader_id)
        REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, room_id, message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE,
    CHECK (
        object_key = 'campuses/' || campus_id::text || '/rooms/' || room_id::text
            || '/attachments/' || attachment_id::text
    ),
    CHECK ((message_id IS NULL) = (position IS NULL))
);

CREATE UNIQUE INDEX attachments_message_position_idx
    ON attachments (campus_id, message_id, position)
    WHERE message_id IS NOT NULL;

CREATE INDEX attachments_vault_idx
    ON attachments (campus_id, room_id, kind, message_id DESC)
    WHERE message_id IS NOT NULL;

CREATE TABLE message_links (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    message_id UUID NOT NULL,
    position SMALLINT NOT NULL,
    url TEXT NOT NULL,
    PRIMARY KEY (campus_id, message_id, position),
    FOREIGN KEY (campus_id, room_id, message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE
);

CREATE INDEX message_links_room_idx
    ON message_links (campus_id, room_id, message_id DESC);

CREATE TABLE message_pins (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    message_id UUID NOT NULL,
    pinned_by UUID NOT NULL,
    pinned_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, room_id, message_id),
    FOREIGN KEY (campus_id, room_id, message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, pinned_by)
        REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE
);

CREATE INDEX message_pins_room_idx
    ON message_pins (campus_id, room_id, pinned_at DESC);

CREATE TABLE shelf_items (
    campus_id UUID NOT NULL,
    account_id UUID NOT NULL,
    shelf_item_id UUID NOT NULL,
    room_id UUID NOT NULL,
    message_id UUID NOT NULL,
    remind_at TIMESTAMPTZ,
    reminded_at TIMESTAMPTZ,
    done_at TIMESTAMPTZ,
    revision BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, shelf_item_id),
    UNIQUE (campus_id, account_id, message_id),
    FOREIGN KEY (campus_id, room_id, message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, account_id)
        REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE,
    CHECK (reminded_at IS NULL OR (remind_at IS NOT NULL AND remind_at <= reminded_at))
);

CREATE INDEX shelf_items_account_idx
    ON shelf_items (campus_id, account_id);

CREATE INDEX shelf_items_due_idx
    ON shelf_items (campus_id, remind_at)
    WHERE reminded_at IS NULL AND done_at IS NULL AND remind_at IS NOT NULL;
