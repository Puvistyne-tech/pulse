CREATE TABLE accounts (
    account_id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE auth_tokens (
    token_hash BYTEA PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX auth_tokens_account_idx ON auth_tokens (account_id);

CREATE TABLE campus_members (
    campus_id UUID NOT NULL REFERENCES campuses(campus_id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('owner', 'admin', 'member', 'guest')),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, account_id)
);

CREATE INDEX campus_members_account_idx ON campus_members (account_id);

CREATE TABLE rooms (
    campus_id UUID NOT NULL REFERENCES campuses(campus_id) ON DELETE CASCADE,
    room_id UUID NOT NULL,
    slug TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (campus_id, room_id),
    UNIQUE (campus_id, slug)
);
