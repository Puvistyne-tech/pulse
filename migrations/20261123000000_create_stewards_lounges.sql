ALTER TABLE accounts ADD COLUMN kind TEXT NOT NULL DEFAULT 'person' CHECK (kind IN ('person', 'steward'));
ALTER TABLE accounts ALTER COLUMN email DROP NOT NULL, ALTER COLUMN password_hash DROP NOT NULL;
ALTER TABLE accounts ADD CONSTRAINT accounts_credentials_by_kind
    CHECK (
        (kind = 'person' AND email IS NOT NULL AND password_hash IS NOT NULL)
        OR (kind = 'steward' AND email IS NULL AND password_hash IS NULL)
    );
ALTER TABLE campus_members DROP CONSTRAINT campus_members_role_check;
ALTER TABLE campus_members ADD CONSTRAINT campus_members_role_check
    CHECK (role IN ('owner', 'admin', 'member', 'guest', 'steward'));

CREATE TABLE stewards (
    campus_id UUID NOT NULL,
    steward_id UUID NOT NULL UNIQUE,
    token_hash BYTEA NOT NULL UNIQUE,
    credential_revision BIGINT NOT NULL DEFAULT 1,
    work_cursor UUID,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, steward_id),
    FOREIGN KEY (campus_id, steward_id) REFERENCES campus_members(campus_id, account_id) ON DELETE CASCADE
);

CREATE TABLE steward_rooms (
    campus_id UUID NOT NULL,
    steward_id UUID NOT NULL,
    room_id UUID NOT NULL,
    PRIMARY KEY (campus_id, steward_id, room_id),
    FOREIGN KEY (campus_id, steward_id) REFERENCES stewards(campus_id, steward_id) ON DELETE CASCADE,
    FOREIGN KEY (campus_id, room_id) REFERENCES rooms(campus_id, room_id) ON DELETE CASCADE
);

CREATE INDEX steward_rooms_room_idx ON steward_rooms (campus_id, room_id);

CREATE TABLE lounges (
    campus_id UUID NOT NULL REFERENCES campuses(campus_id) ON DELETE CASCADE,
    lounge_id UUID NOT NULL,
    room_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, lounge_id),
    UNIQUE (campus_id, room_id),
    FOREIGN KEY (campus_id, room_id) REFERENCES rooms(campus_id, room_id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX lounges_campus_scope_idx ON lounges (campus_id) WHERE room_id IS NULL;

INSERT INTO lounges (campus_id, lounge_id, room_id) SELECT campus_id, uuidv7(), NULL FROM campuses;
INSERT INTO lounges (campus_id, lounge_id, room_id) SELECT campus_id, uuidv7(), room_id FROM rooms;
