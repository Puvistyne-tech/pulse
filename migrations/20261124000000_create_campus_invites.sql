CREATE TABLE campus_invites (
    campus_id UUID NOT NULL REFERENCES campuses(campus_id) ON DELETE CASCADE,
    secret_hash BYTEA NOT NULL,
    created_by UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (campus_id, secret_hash)
);
