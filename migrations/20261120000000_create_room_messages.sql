CREATE TABLE room_messages (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    message_id UUID NOT NULL,
    previous_message_id UUID,
    author_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    client_nonce UUID NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, room_id, message_id),
    FOREIGN KEY (campus_id, room_id) REFERENCES rooms(campus_id, room_id) ON DELETE CASCADE,
    UNIQUE (campus_id, author_id, client_nonce),
    CHECK (previous_message_id IS NULL OR previous_message_id < message_id)
);
