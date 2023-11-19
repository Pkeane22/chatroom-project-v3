CREATE TABLE users (
    id UUID NOT NULL,
    username varchar PRIMARY KEY,
    hash_password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    last_login_at TIMESTAMPTZ NOT NULL,
    UNIQUE (id)
);
