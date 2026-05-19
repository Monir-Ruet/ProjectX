CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users
(
    id                UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name              TEXT,
    password          TEXT,
    email             TEXT UNIQUE NOT NULL,
    email_verified    TIMESTAMPTZ,
    image             TEXT,
    role              TEXT        NOT NULL DEFAULT 'user',
    phone             TEXT,
    phone_verified    BOOLEAN              DEFAULT false,
    is_active         BOOLEAN              DEFAULT true,
    twofactor         BOOLEAN              DEFAULT false,
    lockout_end       TIMESTAMPTZ,
    concurrency_stamp TEXT,
    num_failed        INTEGER     NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users (email);

CREATE TABLE sessions
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id    UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    ip         TEXT        NOT NULL,
    user_agent TEXT        NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_user_id ON sessions (user_id);

CREATE TABLE provider
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id    UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name       TEXT        NOT NULL,
    account_id TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (name, account_id)
);

CREATE INDEX idx_provide_user_id ON provider (user_id);
CREATE INDEX idx_provide_account_id ON provider (account_id);