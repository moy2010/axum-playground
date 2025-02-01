CREATE TABLE IF NOT EXISTS users
(
    id            UUID PRIMARY KEY    NOT NULL,
    email_address TEXT                NOT NULL,
    name          TEXT                NOT NULL,
    created_at    TEXT                NOT NULL,
    updated_at    TEXT                NULL
);