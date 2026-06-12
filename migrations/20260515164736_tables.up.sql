CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users
(
    id                    UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name                  TEXT,
    email                 TEXT UNIQUE NOT NULL,
    password              TEXT,
    email_verified        BOOLEAN              DEFAULT false,
    image                 TEXT,
    phone                 TEXT,
    phone_verified        BOOLEAN              DEFAULT false,
    is_active             BOOLEAN              DEFAULT true,
    two_factor            BOOLEAN              DEFAULT false,
    lockout_end           TIMESTAMPTZ,
    concurrency_stamp     TEXT,
    failed_login_count    INT                  DEFAULT 0,
    last_failed_attempted TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ          DEFAULT NOW()
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

CREATE TABLE roles
(
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE user_roles
(
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    role_id INT  NOT NULL REFERENCES roles (id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE user_claims
(
    id          SERIAL PRIMARY KEY,
    user_id     UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    claim_type  TEXT NOT NULL,
    claim_value TEXT NOT NULL
);

CREATE TABLE role_claims
(
    id          SERIAL PRIMARY KEY,
    role_id     INT  NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    claim_type  TEXT NOT NULL,
    claim_value TEXT NOT NULL
);

CREATE TABLE passkeys
(
    id          UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id     UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    cred_id     BYTEA       NOT NULL UNIQUE,
    device_type TEXT,
    passkey     JSONB       NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_passkeys_cred_id ON passkeys (cred_id);