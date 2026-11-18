ALTER TABLE rooms ADD COLUMN activity_seq BIGINT NOT NULL DEFAULT 0;

ALTER TABLE room_messages
    ADD COLUMN thread_root_id UUID,
    ADD COLUMN activity_seq BIGINT NOT NULL DEFAULT 0,
    ADD CONSTRAINT room_messages_thread_root_fk
        FOREIGN KEY (campus_id, room_id, thread_root_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE,
    ADD CONSTRAINT room_messages_thread_root_order
        CHECK (thread_root_id IS NULL OR thread_root_id < message_id);

CREATE INDEX room_messages_thread_idx
    ON room_messages (campus_id, thread_root_id, message_id)
    WHERE thread_root_id IS NOT NULL;

CREATE INDEX room_messages_activity_idx
    ON room_messages (campus_id, room_id, activity_seq);

CREATE TABLE threads (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    root_message_id UUID NOT NULL,
    topic_title TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, room_id, root_message_id),
    FOREIGN KEY (campus_id, room_id, root_message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE
);

CREATE TABLE thread_subscriptions (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    root_message_id UUID NOT NULL,
    account_id UUID NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('following', 'muted', 'archived')),
    read_through UUID,
    revision BIGINT NOT NULL DEFAULT 1,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, root_message_id, account_id),
    FOREIGN KEY (campus_id, room_id, root_message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, account_id)
        REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE
);

CREATE INDEX thread_subscriptions_account_idx
    ON thread_subscriptions (campus_id, account_id);

CREATE TABLE message_reactions (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    message_id UUID NOT NULL,
    account_id UUID NOT NULL,
    emoji TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, message_id, account_id, emoji),
    FOREIGN KEY (campus_id, room_id, message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, account_id)
        REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE
);

CREATE TABLE message_mentions (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    message_id UUID NOT NULL,
    account_id UUID NOT NULL,
    PRIMARY KEY (campus_id, message_id, account_id),
    FOREIGN KEY (campus_id, room_id, message_id)
        REFERENCES room_messages(campus_id, room_id, message_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, account_id)
        REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE
);

CREATE INDEX message_mentions_account_idx
    ON message_mentions (campus_id, account_id, message_id);
