CREATE TABLE direct_notes (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    member_key TEXT NOT NULL,
    created_by UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, room_id),
    UNIQUE (campus_id, member_key),
    FOREIGN KEY (campus_id, room_id) REFERENCES rooms(campus_id, room_id) ON DELETE CASCADE
);

CREATE TABLE direct_note_members (
    campus_id UUID NOT NULL,
    room_id UUID NOT NULL,
    account_id UUID NOT NULL,
    PRIMARY KEY (campus_id, room_id, account_id),
    FOREIGN KEY (campus_id, room_id) REFERENCES direct_notes(campus_id, room_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, account_id) REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE
);

CREATE INDEX direct_note_members_account_idx ON direct_note_members (campus_id, account_id);
